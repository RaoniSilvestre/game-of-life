//! Autômato celular de propagação de incêndio florestal (SFB).
//!
//! Port em Rust do simulador Python em `modelagem-incendios-automatos/`:
//! - `sfb.py`     -> regras de transição, matrizes de preferência e grid.
//! - `projeto.py` -> cores por estado/vegetação e interatividade.
//!
//! A lógica é a mesma do original; apenas os índices numéricos do NumPy foram
//! substituídos por enumerações tipadas.

use rand::Rng;
use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::automaton::{Automaton, Board, Point};

/// Tipo de vegetação de uma célula.
///
/// sfb.py:24 — "array identificando os tipos: 0 - baixa, 1 - média, 2 - alta".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vegetation {
    Low,
    Medium,
    High,
}

/// As matrizes de preferência e os parâmetros por vegetação (deterioração,
/// exposição) são armazenados em `Vec`s indexados por tipo. O Python usa os
/// inteiros 0/1/2 em `grid_vegetacao`; aqui fazemos a mesma indexação.
impl From<Vegetation> for usize {
    fn from(value: Vegetation) -> Self {
        match value {
            Vegetation::Low => 0,
            Vegetation::Medium => 1,
            Vegetation::High => 2,
        }
    }
}

/// Estado de queima de uma célula.
///
/// sfb.py:178-180 (máscaras em `check`): 0 = suscetível, 1 = inflamada, 2 = queimada.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FireState {
    #[default]
    Susceptible,
    Burning,
    Burnt,
}

/// Célula do grid: junta o estado (que muda) com a vegetação (fixa).
///
/// No Python esses dois conceitos vivem em arrays separados
/// (`grid_array` e `grid_vegetacao`); aqui a vegetação fica na própria célula
/// para que o `Board` consiga colorir cada célula sozinho (via `Into<Color>`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FireCell {
    pub state: FireState,
    pub vegetation: Vegetation,
}

impl Default for FireCell {
    fn default() -> Self {
        Self {
            state: FireState::Susceptible,
            vegetation: Vegetation::Low,
        }
    }
}

/// Cores por estado/vegetação, espelhando projeto.py:25-41 e projeto.py:281-285.
///
/// - suscetível: cor base da vegetação (`veg_colors`).
/// - inflamada : `Vermelho` [226, 50, 54].
/// - queimada  : `Preto` (usamos um cinza escuro para não sumir no terminal).
impl From<FireCell> for Color {
    fn from(cell: FireCell) -> Self {
        match cell.state {
            FireState::Burning => Color::Rgb(226, 50, 54),
            FireState::Burnt => Color::Rgb(40, 40, 40),
            FireState::Susceptible => match cell.vegetation {
                Vegetation::Low => Color::Rgb(144, 238, 144),
                Vegetation::Medium => Color::Rgb(0, 128, 0),
                Vegetation::High => Color::Rgb(0, 50, 0),
            },
        }
    }
}

/// Direção do vento.
///
/// sfb.py:163 — `dir_vento = "norte"` (padrão). Os quatro valores possíveis são
/// "norte", "sul", "leste" e "oeste" (projeto.py:149).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WindDirection {
    #[default]
    Norte,
    Sul,
    Leste,
    Oeste,
}

impl WindDirection {
    /// Quantas rotações de 90° (anti-horário, `np.rot90`) aplicar à matriz base.
    ///
    /// sfb.py:43-46 (e análogos em 85-88 e 127-130):
    /// norte = base, sul = k=2, leste = k=3, oeste = k=1.
    fn rotation_k(self) -> usize {
        match self {
            WindDirection::Norte => 0,
            WindDirection::Sul => 2,
            WindDirection::Leste => 3,
            WindDirection::Oeste => 1,
        }
    }
}

/// As matrizes por direção ficam num `Vec` na mesma ordem em que
/// `PrefMatrices::build` as preenche (`[Norte, Sul, Leste, Oeste]`), então o
/// índice é o próprio discriminante do enum.
impl From<WindDirection> for usize {
    fn from(value: WindDirection) -> Self {
        match value {
            WindDirection::Norte => 0,
            WindDirection::Sul => 1,
            WindDirection::Leste => 2,
            WindDirection::Oeste => 3,
        }
    }
}

/// Velocidade do vento (1 ou 2).
///
/// sfb.py:164 — `vel_vento = 1` (padrão). O valor 2 amplia a vizinhança e usa as
/// matrizes 5x5 (sfb.py:273).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WindSpeed {
    #[default]
    Um,
    Dois,
}

impl WindSpeed {
    /// Raio da vizinhança: 1 para velocidade 1, 2 para velocidade 2.
    ///
    /// sfb.py:273 — `alcance = 1 if vel_vento == 1 else 2`.
    fn alcance(self) -> i32 {
        match self {
            WindSpeed::Um => 1,
            WindSpeed::Dois => 2,
        }
    }
}

/// As matrizes são agrupadas por velocidade num array de 2 posições: o Python
/// chaveia por `(vel_vento, dir_vento)` (sfb.py:62-71); aqui `vel_vento = 1`
/// vira o índice 0 e `vel_vento = 2` vira o índice 1.
impl From<WindSpeed> for usize {
    fn from(value: WindSpeed) -> Self {
        match value {
            WindSpeed::Um => 0,
            WindSpeed::Dois => 1,
        }
    }
}

/// Distribuição inicial da vegetação.
///
/// sfb.py:399-407 (`random_grid`): 0 = realismo, 1 = risco, 2 = didático.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Distribution {
    #[default]
    Realismo,
    Risco,
    Didatico,
}

/// A distribuição define as probabilidades de cada vegetação (baixa, média,
/// alta), exatamente como em sfb.py:399-407:
/// - realismo: [0.4, 0.35, 0.25]
/// - risco   : [0.3, 0.4, 0.3]
/// - didático: [1/3, 1/3, 1/3]
impl From<Distribution> for [f64; 3] {
    fn from(value: Distribution) -> Self {
        match value {
            Distribution::Realismo => [0.4, 0.35, 0.25],
            Distribution::Risco => [0.3, 0.4, 0.3],
            Distribution::Didatico => [1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0],
        }
    }
}

/// Cena serializável (TOML) do autômato SFB.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FireScene {
    pub width: usize,
    pub height: usize,
    #[serde(default)]
    pub wind_direction: WindDirection,
    #[serde(default)]
    pub wind_speed: WindSpeed,
    #[serde(default)]
    pub distribution: Distribution,
    #[serde(default)]
    pub fire: Vec<(usize, usize)>,
}

type Matrix = Vec<Vec<f64>>;

fn matrix_from(rows: &[&[f64]]) -> Matrix {
    rows.iter().map(|r| r.to_vec()).collect()
}

/// Rotação de 90° anti-horário, equivalente a `np.rot90(m, k)`.
///
/// sfb.py usa `np.rot90` para derivar as matrizes por direção (ex.: linhas
/// 44-46); `k` é o número de rotações de 90° no sentido anti-horário.
fn rot90<T: Copy + Default>(m: &[Vec<T>], k: usize) -> Vec<Vec<T>> {
    let mut out = m.to_vec();

    for _ in 0..(k % 4) {
        let n = out.len();
        let mut rotated = vec![vec![T::default(); n]; n];

        for r in 0..n {
            for c in 0..n {
                rotated[r][c] = out[c][n - 1 - r];
            }
        }

        out = rotated;
    }

    out
}

/// Matriz de preferência base (vento norte) por vegetação e velocidade.
///
/// Os valores são cópia literal do sfb.py:
/// - baixa v1: sfb.py:38-42, baixa v2: sfb.py:49-55
/// - média v1: sfb.py:80-84, média v2: sfb.py:91-97
/// - alta  v1: sfb.py:122-126, alta v2: sfb.py:133-139
fn base_matrix(veg: Vegetation, speed: WindSpeed) -> Matrix {
    let rows: &[&[f64]] = match (veg, speed) {
        (Vegetation::Low, WindSpeed::Um) => {
            &[&[0.01, 0.01, 0.01], &[0.01, 0.0, 0.01], &[0.01, 0.02, 0.01]]
        }
        (Vegetation::Medium, WindSpeed::Um) => {
            &[&[0.01, 0.02, 0.01], &[0.02, 0.0, 0.02], &[0.02, 0.03, 0.02]]
        }
        (Vegetation::High, WindSpeed::Um) => {
            &[&[0.01, 0.02, 0.01], &[0.03, 0.0, 0.03], &[0.03, 0.04, 0.03]]
        }
        (Vegetation::Low, WindSpeed::Dois) => &[
            &[0.0, 0.0, 0.0, 0.0, 0.0],
            &[0.0, 0.01, 0.01, 0.01, 0.0],
            &[0.0, 0.01, 0.0, 0.01, 0.0],
            &[0.0, 0.02, 0.03, 0.02, 0.0],
            &[0.0, 0.01, 0.02, 0.01, 0.0],
        ],
        (Vegetation::Medium, WindSpeed::Dois) => &[
            &[0.0, 0.0, 0.0, 0.0, 0.0],
            &[0.0, 0.01, 0.01, 0.01, 0.0],
            &[0.0, 0.02, 0.0, 0.02, 0.0],
            &[0.0, 0.03, 0.04, 0.03, 0.0],
            &[0.0, 0.01, 0.03, 0.01, 0.0],
        ],
        (Vegetation::High, WindSpeed::Dois) => &[
            &[0.0, 0.0, 0.0, 0.0, 0.0],
            &[0.0, 0.01, 0.01, 0.01, 0.0],
            &[0.0, 0.03, 0.0, 0.03, 0.0],
            &[0.0, 0.04, 0.05, 0.04, 0.0],
            &[0.0, 0.01, 0.04, 0.01, 0.0],
        ],
    };

    matrix_from(rows)
}

/// Matrizes de preferência pré-computadas para todas as combinações de
/// vegetação × velocidade × direção (mesma estrutura dos dicionários
/// `matrizes_pref_veg_*` do sfb.py:62-71, 104-113 e 146-155).
#[derive(Debug, Clone)]
struct PrefMatrices {
    mats: Vec<Vec<Vec<Matrix>>>,
}

impl PrefMatrices {
    fn build() -> Self {
        let vegs = [Vegetation::Low, Vegetation::Medium, Vegetation::High];
        let dirs = [
            WindDirection::Norte,
            WindDirection::Sul,
            WindDirection::Leste,
            WindDirection::Oeste,
        ];
        let speeds = [WindSpeed::Um, WindSpeed::Dois];

        let mut mats = Vec::with_capacity(3);
        for veg in vegs {
            let mut speed_mats = Vec::with_capacity(2);
            for speed in speeds {
                let base = base_matrix(veg, speed);
                let mut dir_mats = Vec::with_capacity(4);
                for dir in dirs {
                    dir_mats.push(rot90(&base, dir.rotation_k()));
                }
                speed_mats.push(dir_mats);
            }
            mats.push(speed_mats);
        }

        Self { mats }
    }

    fn get(&self, veg: Vegetation, speed: WindSpeed, dir: WindDirection) -> &Matrix {
        &self.mats[usize::from(veg)][usize::from(speed)][usize::from(dir)]
    }
}

#[derive(Debug)]
pub struct FireSim {
    board: Board<FireCell>,
    next: Board<FireCell>,
    exposure: Vec<f64>,
    timer: Vec<i32>,
    wind_direction: WindDirection,
    wind_speed: WindSpeed,
    iterations: usize,
    matrices: PrefMatrices,
}

impl FireSim {
    fn with_size(width: usize, height: usize) -> Self {
        Self {
            board: Board::new(width, height),
            next: Board::new(width, height),
            exposure: vec![0.0; width * height],
            timer: vec![-1; width * height],
            wind_direction: WindDirection::Norte,
            wind_speed: WindSpeed::Um,
            iterations: 0,
            matrices: PrefMatrices::build(),
        }
    }

    /// Distribui a vegetação aleatoriamente conforme a distribuição escolhida.
    ///
    /// Equivalente a sfb.py:399-407 (`random_grid`): sorteia 0/1/2 com as
    /// probabilidades da distribuição.
    fn random_vegetation(&mut self, distribution: Distribution) {
        let probs: [f64; 3] = distribution.into();
        let (p0, p1) = (probs[0], probs[0] + probs[1]);

        let mut rng = rand::rng();
        let width = self.board.width();
        let height = self.board.height();

        for x in 0..height {
            for y in 0..width {
                let r: f64 = rng.random();
                let veg = if r < p0 {
                    Vegetation::Low
                } else if r < p1 {
                    Vegetation::Medium
                } else {
                    Vegetation::High
                };

                let mut cell = self.board.get((x as i32, y as i32));
                cell.vegetation = veg;
                self.board.set((x as i32, y as i32), cell);
            }
        }
    }

    /// Acende uma célula (define como inflamada).
    ///
    /// Equivalente a `handle_mouse(..., status=1)` (sfb.py:409-415) e ao clique
    /// esquerdo de projeto.py:410-412.
    fn ignite(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.board.height() || y >= self.board.width() {
            return;
        }

        let mut cell = self.board.get((x as i32, y as i32));
        cell.state = FireState::Burning;
        self.board.set((x as i32, y as i32), cell);
    }

    /// Tempo de deterioração (gerações queimando até virar queimada).
    ///
    /// sfb.py:73 (baixa = 3), sfb.py:115 (média = 2), sfb.py:157 (alta = 1).
    fn deterioration(veg: Vegetation) -> i32 {
        match veg {
            Vegetation::Low => 3,
            Vegetation::Medium => 2,
            Vegetation::High => 1,
        }
    }

    /// Incremento da exposição acumulada quando a célula não pega fogo.
    ///
    /// sfb.py:192 (baixa = 0.02), sfb.py:206 (média = 0.04), sfb.py:220 (alta = 0.06).
    fn exposure_increment(veg: Vegetation) -> f64 {
        match veg {
            Vegetation::Low => 0.02,
            Vegetation::Medium => 0.04,
            Vegetation::High => 0.06,
        }
    }

    /// Teto da exposição acumulada.
    ///
    /// sfb.py:297 (baixa = 0.08), sfb.py:323 (média = 0.11), sfb.py:349 (alta = 0.14).
    fn exposure_cap(veg: Vegetation) -> f64 {
        match veg {
            Vegetation::Low => 0.08,
            Vegetation::Medium => 0.11,
            Vegetation::High => 0.14,
        }
    }

    /// Probabilidade de uma célula suscetível pegar fogo nesta geração.
    ///
    /// Porta a parte suscetível de `rules` (sfb.py:272-350): soma as
    /// preferências dos vizinhos inflamados e adiciona a exposição acumulada;
    /// se a soma for < 0.01 a probabilidade é 0 (sfb.py:290-293).
    fn ignition_probability(&self, (x, y): Point, alcance: i32, veg: Vegetation) -> f64 {
        let matrix = self.matrices.get(veg, self.wind_speed, self.wind_direction);
        let width = self.board.width();
        let height = self.board.height();
        let mut soma = 0.0;

        for i in -alcance..=alcance {
            for j in -alcance..=alcance {
                if i == 0 && j == 0 {
                    continue;
                }

                let xi = x + i;
                let yj = y + j;

                if xi < 0 || yj < 0 || xi as usize >= height || yj as usize >= width {
                    continue;
                }

                if self.board.get((xi, yj)).state == FireState::Burning {
                    // sfb.py:288 — `matriz_preferencia[j + alcance][i + alcance]`.
                    soma += matrix[(i + alcance) as usize][(j + alcance) as usize];
                }
            }
        }

        if soma < 0.01 {
            0.0
        } else {
            (soma + self.exposure[x as usize * width + y as usize]).min(1.0)
        }
    }
}

impl Automaton for FireSim {
    type Cell = FireCell;
    type Scene = FireScene;

    fn name() -> &'static str {
        "Simulação de Incêndios (SFB)"
    }

    fn new(width: usize, height: usize) -> Self {
        let mut sim = Self::with_size(width, height);
        sim.random_vegetation(Distribution::Realismo);
        sim
    }

    fn from(scene: FireScene) -> Self {
        let mut sim = Self::with_size(scene.width, scene.height);
        sim.random_vegetation(scene.distribution);
        sim.wind_direction = scene.wind_direction;
        sim.wind_speed = scene.wind_speed;

        for (x, y) in scene.fire {
            sim.ignite(x as i32, y as i32);
        }

        sim
    }

    fn board(&self) -> &Board<FireCell> {
        &self.board
    }

    fn step(&mut self) {
        self.iterations += 1;

        let width = self.board.width();
        let height = self.board.height();
        let alcance = self.wind_speed.alcance();
        let mut rng = rand::rng();

        for x in 0..height {
            for y in 0..width {
                let p: Point = (x as i32, y as i32);
                let idx = x * width + y;
                let cell = self.board.get(p);
                let mut next = cell;

                match cell.state {
                    // sfb.py:352-378 — célula inflamada: decrementa o tempo de
                    // deterioração e vira queimada ao chegar em 0.
                    FireState::Burning => {
                        if self.timer[idx] == 0 {
                            next.state = FireState::Burnt;
                            self.timer[idx] = -1;
                            self.exposure[idx] = 0.0;
                        } else {
                            self.timer[idx] -= 1;
                        }
                    }
                    // sfb.py:272-350 — célula suscetível: pode inflamar ou
                    // acumular exposição.
                    FireState::Susceptible => {
                        let prob = self.ignition_probability(p, alcance, cell.vegetation);
                        if rng.random::<f64>() <= prob {
                            next.state = FireState::Burning;
                        } else {
                            let inc = Self::exposure_increment(cell.vegetation);
                            let cap = Self::exposure_cap(cell.vegetation);
                            self.exposure[idx] = (self.exposure[idx] + inc).min(cap);
                        }
                    }
                    FireState::Burnt => {}
                }

                self.next.set(p, next);
            }
        }

        std::mem::swap(&mut self.board, &mut self.next);

        // sfb.py:200/214/228 (`check`) — ao inflamar, o contador de deterioração
        // é inicializado com o tempo da vegetação da célula.
        for idx in 0..(width * height) {
            let cell = self.board.cells()[idx];
            if cell.state == FireState::Burning && self.timer[idx] == -1 {
                self.timer[idx] = Self::deterioration(cell.vegetation);
            }
        }
    }

    /// Clique esquerdo/scroll: acende a célula sob o cursor (projeto.py:410-412).
    fn paint_left(&mut self, p: Point) {
        self.ignite(p.0, p.1);
    }

    /// Clique direito: acende uma vizinhança 3x3 ao redor do cursor.
    fn paint_right(&mut self, (x, y): Point) {
        for dx in -1..=1 {
            for dy in -1..=1 {
                self.ignite(x + dx, y + dy);
            }
        }
    }

    fn iterations(&self) -> usize {
        self.iterations
    }
}
