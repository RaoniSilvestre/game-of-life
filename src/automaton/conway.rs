use rand::seq::IndexedRandom;
use ratatui::style::Color;

use crate::{
    automaton::{Automaton, Board, Point},
    Res,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConwayCell {
    #[default]
    Dead,
    Alive,
}

impl From<ConwayCell> for Color {
    fn from(cell: ConwayCell) -> Self {
        match cell {
            ConwayCell::Dead => Color::Black,
            ConwayCell::Alive => Color::Green,
        }
    }
}

#[derive(Debug)]
pub struct Conway {
    board: Board<ConwayCell>,
    next: Board<ConwayCell>,
    iterations: usize,
}

impl Conway {
    fn neighbours(&self, (x, y): Point) -> u8 {
        let mut alive = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                let p = (x + dx, y + dy);
                if dx == 0 && dy == 0 {
                    continue;
                }

                if self.board.get(p) == ConwayCell::Alive {
                    alive += 1;
                }
            }
        }

        alive
    }
}

impl Automaton for Conway {
    type Cell = ConwayCell;

    fn name() -> &'static str {
        "Conway's Game of Life"
    }

    fn new(width: usize, height: usize) -> Self {
        Self {
            board: Board::new(width, height),
            next: Board::new(width, height),
            iterations: 0,
        }
    }

    fn from_rows(width: usize, height: usize, rows: &[String]) -> Res<Self> {
        let mut automaton = Self::new(width, height);

        for (x, row) in rows.iter().enumerate() {
            for (y, c) in row.chars().enumerate() {
                let p = (x as i32, y as i32);
                let cell = match c {
                    'X' | 'x' | '█' | 'O' => ConwayCell::Alive,
                    _ => ConwayCell::Dead,
                };
                automaton.board.set(p, cell);
            }
        }

        Ok(automaton)
    }

    fn board(&self) -> &Board<Self::Cell> {
        &self.board
    }

    fn step(&mut self) {
        self.iterations += 1;

        let width = self.board.width();
        let height = self.board.height();

        for x in 0..height {
            for y in 0..width {
                let p = (x as i32, y as i32);
                let neighbours = self.neighbours(p);
                let cell = self.board.get(p);

                let next_cell = match (cell, neighbours) {
                    (ConwayCell::Alive, 2 | 3) => ConwayCell::Alive,
                    (ConwayCell::Dead, 3) => ConwayCell::Alive,
                    _ => ConwayCell::Dead,
                };

                self.next.set(p, next_cell);
            }
        }

        std::mem::swap(&mut self.board, &mut self.next);
    }

    fn paint_left(&mut self, p: Point) {
        let current = self.board.get(p);
        let next = match current {
            ConwayCell::Alive => ConwayCell::Dead,
            ConwayCell::Dead => ConwayCell::Alive,
        };
        self.board.set(p, next);
    }

    fn paint_right(&mut self, (x, y): Point) {
        let mut possible_points: Vec<Point> = Vec::with_capacity(64);
        for dx in -4..=3 {
            for dy in -4..=3 {
                possible_points.push((x + dx, y + dy));
            }
        }

        let mut rng = rand::rng();
        let chosen = possible_points.as_slice().choose_multiple(&mut rng, 16);

        for (px, py) in chosen {
            self.board.set((*px, *py), ConwayCell::Alive);
        }
    }

    fn iterations(&self) -> usize {
        self.iterations
    }
}

#[cfg(test)]
mod tests_conway {
    use super::*;

    fn conway_from(rows: &[&str]) -> Conway {
        let width = rows.first().map(|r| r.len()).unwrap_or(0);
        let height = rows.len();
        let rows: Vec<String> = rows.iter().map(|s| s.to_string()).collect();
        Conway::from_rows(width, height, &rows).unwrap()
    }

    fn alive_set(game: &Conway) -> Vec<(i32, i32)> {
        let board = game.board();
        let mut alive = Vec::new();

        for (i, &cell) in board.cells().iter().enumerate() {
            if cell == ConwayCell::Alive {
                let x = (i / board.width()) as i32;
                let y = (i % board.width()) as i32;
                alive.push((x, y));
            }
        }

        alive
    }

    #[test]
    fn test_from_rows_parses_x() {
        let game = conway_from(&[".X.", "X.X"]);
        assert_eq!(alive_set(&game), vec![(0, 1), (1, 0), (1, 2)]);
    }

    #[test]
    fn test_neighbours() {
        let game = conway_from(&["X.X", ".X.", "X.X"]);

        assert_eq!(
            game.neighbours((1, 1)),
            4,
            "Célula viva no centro deve ter 4 vizinhos"
        );
        assert_eq!(
            game.neighbours((0, 0)),
            1,
            "Célula viva no canto deve ter 1 vizinho"
        );
        assert_eq!(
            game.neighbours((1, 0)),
            3,
            "Célula morta deve ter 3 vizinhos"
        );
    }

    #[test]
    fn test_neighbours_out_of_bounds_count_as_dead() {
        let game = conway_from(&["X"]);
        assert_eq!(game.neighbours((0, 0)), 0);
    }

    #[test]
    fn test_underpopulation() {
        let mut game = conway_from(&["X"]);
        game.step();
        assert!(
            alive_set(&game).is_empty(),
            "Células com menos de 2 vizinhos devem morrer por solidão"
        );
    }

    #[test]
    fn test_overpopulation() {
        let mut game = conway_from(&["XXX", "XXX", "XXX"]);
        assert_eq!(game.board().get((1, 1)), ConwayCell::Alive);

        game.step();
        assert_eq!(
            game.board().get((1, 1)),
            ConwayCell::Dead,
            "O centro com 8 vizinhos deve morrer"
        );
    }

    #[test]
    fn test_reproduction() {
        let mut game = conway_from(&[".XXX.", ".....", "....."]);
        assert_eq!(game.board().get((1, 2)), ConwayCell::Dead);

        game.step();
        assert_eq!(
            game.board().get((1, 2)),
            ConwayCell::Alive,
            "Célula morta com exatamente 3 vizinhos deve nascer"
        );
    }

    #[test]
    fn test_still_life_block() {
        let mut game = conway_from(&["XX", "XX"]);

        let initial = alive_set(&game);
        game.step();
        assert_eq!(
            alive_set(&game),
            initial,
            "O padrão 'block' não deve mudar após uma atualização"
        );
    }

    #[test]
    fn test_oscillator_blinker() {
        let mut game = conway_from(&[".....", "..X..", "..X..", "..X..", "....."]);

        let expected_horizontal = vec![(2, 1), (2, 2), (2, 3)];
        game.step();
        assert_eq!(
            alive_set(&game),
            expected_horizontal,
            "Blinker deveria ter mudado de vertical para horizontal"
        );

        let expected_vertical = vec![(1, 2), (2, 2), (3, 2)];
        game.step();
        assert_eq!(
            alive_set(&game),
            expected_vertical,
            "Blinker deveria ter voltado ao estado vertical"
        );
    }

    #[test]
    fn test_paint_left_toggles() {
        let mut game = Conway::new(3, 3);
        let p = (1, 1);

        game.paint_left(p);
        assert!(alive_set(&game).contains(&p));

        game.paint_left(p);
        assert!(!alive_set(&game).contains(&p));
    }

    #[test]
    fn test_paint_right_scatters_alive_cells() {
        let mut game = Conway::new(20, 20);

        game.paint_right((10, 10));
        let alive = alive_set(&game);

        assert_eq!(
            alive.len(),
            16,
            "scatter deve acender exatamente 16 células"
        );

        for (x, y) in alive {
            assert!((6..=13).contains(&x) && (6..=13).contains(&y));
        }
    }

    #[test]
    fn test_cell_colors() {
        assert_eq!(Color::from(ConwayCell::Dead), Color::Black);
        assert_eq!(Color::from(ConwayCell::Alive), Color::Green);
    }
}
