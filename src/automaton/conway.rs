use std::sync::Arc;

use rand::seq::IndexedRandom;
use macroquad::color::Color;
use serde::{Deserialize, Serialize};

use crate::automaton::{Automaton, Board, Point};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConwayScene {
    pub width: usize,
    pub height: usize,
    pub rows: Vec<String>,
}

impl ConwayScene {
    pub fn empty(width: usize, height: usize) -> Self {
        let rows = vec![".".repeat(width); height];
        Self {
            width,
            height,
            rows,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConwayCell {
    #[default]
    Dead,
    Alive,
}

impl From<ConwayCell> for Color {
    fn from(cell: ConwayCell) -> Self {
        match cell {
            ConwayCell::Dead => Color::from_rgba(0, 0, 0, 255),
            ConwayCell::Alive => Color::from_rgba(0, 255, 0, 255),
        }
    }
}

#[derive(Debug)]
pub struct Conway {
    board: Arc<Board<ConwayCell>>,
    next: Arc<Board<ConwayCell>>,
    iterations: usize,
}

impl Conway {
    fn neighbours(board: &Board<ConwayCell>, (x, y): Point) -> u8 {
        let mut alive = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                let p = (x + dx, y + dy);
                if dx == 0 && dy == 0 {
                    continue;
                }

                if board.get(p) == ConwayCell::Alive {
                    alive += 1;
                }
            }
        }

        alive
    }
}

impl Automaton for Conway {
    type Cell = ConwayCell;
    type Scene = ConwayScene;

    fn name() -> &'static str {
        "Conway's Game of Life"
    }

    fn new(width: usize, height: usize) -> Self {
        Self {
            board: Arc::new(Board::new(width, height)),
            next: Arc::new(Board::new(width, height)),
            iterations: 0,
        }
    }

    fn from(scene: Self::Scene) -> Self {
        let mut automaton = Self::new(scene.width, scene.height);
        let board = Arc::make_mut(&mut automaton.board);

        for (x, row) in scene.rows.iter().enumerate() {
            for (y, c) in row.chars().enumerate() {
                let p = (x as i32, y as i32);
                let cell = match c {
                    'X' | 'x' | '█' | 'O' => ConwayCell::Alive,
                    _ => ConwayCell::Dead,
                };
                board.set(p, cell);
            }
        }

        automaton
    }

    fn board(&self) -> Arc<Board<Self::Cell>> {
        self.board.clone()
    }

    fn step(&mut self) {
        self.iterations += 1;

        let width = self.board.width();
        let height = self.board.height();
        let next_board = Arc::make_mut(&mut self.next);

        for x in 0..height {
            for y in 0..width {
                let p = (x as i32, y as i32);
                let neighbours = Self::neighbours(&self.board, p);
                let cell = self.board.get(p);

                let next_cell = match (cell, neighbours) {
                    (ConwayCell::Alive, 2 | 3) => ConwayCell::Alive,
                    (ConwayCell::Dead, 3) => ConwayCell::Alive,
                    _ => ConwayCell::Dead,
                };

                next_board.set(p, next_cell);
            }
        }

        std::mem::swap(&mut self.board, &mut self.next);
    }

    fn paint_left(&mut self, p: Point) {
        let board = Arc::make_mut(&mut self.board);
        let current = board.get(p);
        let next = match current {
            ConwayCell::Alive => ConwayCell::Dead,
            ConwayCell::Dead => ConwayCell::Alive,
        };
        board.set(p, next);
    }

    fn paint_right(&mut self, (x, y): Point) {
        let board = Arc::make_mut(&mut self.board);

        let mut possible_points: Vec<Point> = Vec::with_capacity(64);
        for dx in -4..=3 {
            for dy in -4..=3 {
                possible_points.push((x + dx, y + dy));
            }
        }

        let mut rng = rand::rng();
        let chosen = possible_points.as_slice().choose_multiple(&mut rng, 16);

        for (px, py) in chosen {
            board.set((*px, *py), ConwayCell::Alive);
        }
    }

    fn iterations(&self) -> usize {
        self.iterations
    }
}
