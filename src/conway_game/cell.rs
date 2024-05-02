use std::usize;

use crate::conway_game::point::Point;
use crate::conway_game::point::PointI32;

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
        for dx in -1..1 {
            for dy in -1..1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let neighbour_point = PointI32 {
                    row: ((self.point.row as i32) + dx),
                    col: ((self.point.col as i32) + dy),
                };

                if let Ok(point) = Point::point_from_pointi32(neighbour_point) {
                    if matrix[point.row as usize][point.col as usize].state == State::Alive {
                        alives += 1;
                    }
                }
            }
        }
        alives
    }

    // Update the state of cell (This should be called to every alive cell or any neighbour)
    pub fn update_state(point: Point, matrix_in: &[Vec<Cell>], matrix_out: &mut [Vec<Cell>]) {
        let mut actual_cell = matrix_in[point.row as usize][point.col as usize];
        let alive_neighbours = actual_cell.alive_neighbours(matrix_in);

        match actual_cell.state {
            State::Alive => {
                if alive_neighbours != 2 {
                    actual_cell.state = State::Dead;
                }
            }
            State::Dead => {
                if alive_neighbours >= 1 {
                    actual_cell.state = State::Alive;
                }
            }
        }

        matrix_out[point.row as usize][point.col as usize] = actual_cell;
    }
}
