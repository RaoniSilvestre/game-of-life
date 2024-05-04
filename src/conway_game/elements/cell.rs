use core::panic;
use std::usize;

use super::matrix;
use super::point::Point;
use super::point::PointI32;

#[derive(PartialEq, Clone, Copy, Debug, Eq)]
pub enum State {
    Alive,
    Dead,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub state: State,
    pub point: Point,
}

impl Cell {
    // Inicializar celula
    pub fn new(point: Point, state: State) -> Self {
        Cell { state, point }
    }

    // Neighbours logic
    pub fn alive_neighbours(&self, matrix: &[Vec<Cell>]) -> usize {
        let mut alives = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let neighbor_row = (self.point.row as i32) + dx;
                let neighbor_col = (self.point.col as i32) + dy;

                if Self::is_inside_boundaries(neighbor_row, neighbor_col, matrix) {
                    if matrix[neighbor_row as usize][neighbor_col as usize].state == State::Alive {
                        alives += 1;
                    }
                }
            }
        }

        alives
    }

    fn is_inside_boundaries(neighbor_row: i32, neighbor_col: i32, matrix: &[Vec<Cell>]) -> bool {
        if neighbor_col < 0
            || neighbor_row < 0
            || neighbor_row >= matrix.len() as i32
            || neighbor_col >= matrix[0].len() as i32
        {
            return false;
        }

        true
    }

    pub fn update_state(point: Point, matrix_in: &[Vec<Cell>], matrix_out: &mut [Vec<Cell>]) {
        let row = point.row as usize;
        let col = point.col as usize;

        let actual_cell = matrix_in[row][col];

        let alive_neighbours = actual_cell.alive_neighbours(matrix_in);

        let new_state = match actual_cell.state {
            State::Alive => {
                if alive_neighbours == 2 || alive_neighbours == 3 {
                    State::Alive
                } else {
                    State::Dead
                }
            }
            State::Dead => {
                if alive_neighbours == 3 {
                    State::Alive
                } else {
                    State::Dead
                }
            }
        };

        matrix_out[row][col].state = new_state;
    }
}
