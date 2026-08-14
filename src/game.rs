use std::collections::HashSet;

use rand::seq::IndexedRandom;

pub type Point = (i32, i32);
pub type GameState = HashSet<Point>;

#[derive(Debug, Default)]
pub struct ConwayGame {
    alives: GameState,
    iterations: usize,
}

impl ConwayGame {
    pub fn add(&mut self, p: Point) {
        self.alives.insert(p);
    }

    pub fn state(&self) -> GameState {
        self.alives.clone()
    }

    pub fn scatter(&mut self, (a, b): Point) {
        let x_min = a - 4;
        let x_max = a + 3;
        let y_min = b - 4;
        let y_max = b + 3;

        let mut possible_points: Vec<Point> = Vec::with_capacity(64);
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                possible_points.push((x, y));
            }
        }

        let mut rng = rand::rng();
        let chosen_points = possible_points.as_slice().choose_multiple(&mut rng, 16);

        for point in chosen_points {
            self.add(*point);
        }
    }

    pub fn update(&mut self) {
        self.iterations += 1;

        let mut new_alives = HashSet::new();

        for (x, y) in self.alives.iter() {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let point = (x + dx, y + dy);

                    let amount_of_neighbours = self.get_neighbours(point);
                    let is_alive = self.alives.contains(&point);

                    match (is_alive, amount_of_neighbours) {
                        (true, 2 | 3) => new_alives.insert(point),
                        (false, 3) => new_alives.insert(point),
                        _ => true,
                    };
                }
            }
        }

        self.alives = new_alives;
    }

    fn get_neighbours(&self, (x, y): Point) -> u8 {
        let mut alives = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let row = x + dx;
                let col = y + dy;

                if self.alives.contains(&(row, col)) {
                    alives += 1;
                }
            }
        }

        alives
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn hashset_from_slice(points: &[(i32, i32)]) -> HashSet<Point> {
        points.iter().cloned().collect()
    }

    #[test]
    fn test_get_neighbours() {
        let mut game = ConwayGame::default();
        // Cria um padrão simples:
        // X . X
        // . X .
        // X . X
        game.add((0, 0));
        game.add((0, 2));
        game.add((1, 1));
        game.add((2, 0));
        game.add((2, 2));

        assert_eq!(
            game.get_neighbours((1, 1)),
            4,
            "Célula viva no centro deve ter 4 vizinhos"
        );
        assert_eq!(
            game.get_neighbours((0, 0)),
            1,
            "Célula viva no canto deve ter 1 vizinho"
        );
        assert_eq!(
            game.get_neighbours((1, 0)),
            3,
            "Célula morta deve ter 3 vizinhos"
        );
        assert_eq!(
            game.get_neighbours((5, 5)),
            0,
            "Célula morta e isolada deve ter 0 vizinhos"
        );
    }

    #[test]
    fn test_add_around() {
        let mut game = ConwayGame::default();
        let center_point = (10, 10);
        game.scatter(center_point);

        // 1. Verifica se exatamente 16 células foram adicionadas
        assert_eq!(
            game.alives.len(),
            16,
            "A função add_around deve adicionar exatamente 16 células"
        );

        // 2. Verifica se todas as células estão dentro do grid 8x8 esperado
        let x_min = center_point.0 - 4;
        let x_max = center_point.0 + 3;
        let y_min = center_point.1 - 4;
        let y_max = center_point.1 + 3;

        for (x, y) in game.alives.iter() {
            assert!(
                *x >= x_min && *x <= x_max,
                "Célula X={} está fora dos limites",
                x
            );
            assert!(
                *y >= y_min && *y <= y_max,
                "Célula Y={} está fora dos limites",
                y
            );
        }
    }

    // --- Testes das Regras do Jogo ---

    #[test]
    fn test_underpopulation() {
        // Uma célula sozinha ou com apenas um vizinho deve morrer.
        let mut game = ConwayGame::default();
        game.add((0, 0)); // Célula sozinha
        game.add((5, 5)); // Células com 1 vizinho
        game.add((5, 6));

        game.update();
        assert!(
            game.alives.is_empty(),
            "Células com menos de 2 vizinhos devem morrer por solidão"
        );
    }

    #[test]
    fn test_overpopulation() {
        // Uma célula com mais de 3 vizinhos morre por superpopulação.
        // Teste com um quadrado 3x3 (centro tem 8 vizinhos e morre)
        let mut game = ConwayGame::default();
        for x in 0..=2 {
            for y in 0..=2 {
                game.add((x, y));
            }
        }
        assert!(
            game.alives.contains(&(1, 1)),
            "O centro deve estar vivo inicialmente"
        );

        game.update();
        assert!(
            !game.alives.contains(&(1, 1)),
            "O centro com 8 vizinhos deve morrer"
        );
    }

    #[test]
    fn test_reproduction() {
        // Uma célula morta com exatamente 3 vizinhos deve nascer.
        let mut game = ConwayGame::default();
        game.add((0, 0));
        game.add((0, 2));
        game.add((1, 1));

        assert!(
            !game.alives.contains(&(0, 1)),
            "O ponto (0,1) está morto inicialmente"
        );
        game.update();
        assert!(
            game.alives.contains(&(0, 1)),
            "O ponto (0,1) deveria nascer pois tem 3 vizinhos"
        );
    }

    #[test]
    fn test_still_life_block() {
        // "Block" é uma vida estática, não deve mudar.
        // XX
        // XX
        let mut game = ConwayGame::default();
        game.add((0, 0));
        game.add((0, 1));
        game.add((1, 0));
        game.add((1, 1));

        let initial_state = game.state();
        game.update();
        let next_state = game.state();

        assert_eq!(
            initial_state, next_state,
            "O padrão 'block' não deve mudar após uma atualização"
        );
    }

    #[test]
    fn test_oscillator_blinker() {
        // "Blinker" oscila entre uma linha vertical e horizontal.
        // Estado 1: .X.
        //           .X.
        //           .X.
        let mut game = ConwayGame::default();
        game.add((1, 0));
        game.add((1, 1));
        game.add((1, 2));

        // Estado 2: ...
        //           XXX
        //           ...
        let expected_state = hashset_from_slice(&[(0, 1), (1, 1), (2, 1)]);

        game.update();
        assert_eq!(
            game.state(),
            expected_state,
            "Blinker deveria ter mudado de vertical para horizontal"
        );

        // Estado 3: Volta ao original
        let initial_state = hashset_from_slice(&[(1, 0), (1, 1), (1, 2)]);
        game.update();
        assert_eq!(
            game.state(),
            initial_state,
            "Blinker deveria ter voltado ao estado vertical"
        );
    }
}
