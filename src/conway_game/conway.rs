use crate::conway_game::cell::{Cell, State};
use crate::conway_game::point::Point;
use std::io::{Stdout, Write};
use std::usize;
use terminal::*;

pub struct ConwayGame {
    pub matrix: Vec<Vec<Cell>>,
    pub size: Point,
}

impl ConwayGame {
    // Inicializar Jogo
    pub fn new(size: Point) -> Self {
        let mut matrix: Vec<Vec<Cell>> =
            vec![
                vec![Cell::new(Point { row: 0, col: 0 }, State::Dead); size.col.into()];
                size.row.into()
            ];

        for i in 0..size.row {
            for j in 0..size.col {
                matrix[i as usize][j as usize].point = Point { row: i, col: j };
            }
        }

        ConwayGame { matrix, size }
    }

    pub fn get_alive_cells(matrix: &[Vec<Cell>]) -> Vec<Cell> {
        let mut alive_cells = Vec::new();
        for row in matrix.iter() {
            for cell in row.iter() {
                if cell.state == State::Alive {
                    alive_cells.push(*cell)
                }
            }
        }

        alive_cells
    }

    pub fn start_state(&mut self, points_of_living_cells: Vec<Point>) {
        for point in points_of_living_cells {
            self.matrix[point.row as usize][point.col as usize].state = State::Alive;
        }
    }

    pub fn get_updated_points(matrix: &mut [Vec<Cell>]) -> Vec<Vec<Cell>> {
        let mut matrix_out: Vec<Vec<Cell>> =
            vec![
                vec![Cell::new(Point { row: 0, col: 0 }, State::Dead); matrix[0].len()];
                matrix.len()
            ];
        for row in matrix.iter() {
            for cell in row.iter() {
                Cell::update_state(cell.point, matrix, &mut matrix_out);
            }
        }

        matrix_out.to_vec()
    }
}
