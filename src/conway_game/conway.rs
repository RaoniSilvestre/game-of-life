use super::elements::cell::{Cell, State};
use super::elements::matrix::CellMatrix;
use super::elements::point::Point;
use std::io::Stdout;
use terminal::*;

pub struct ConwayGame {
    pub matrix: Vec<Vec<Cell>>,
    pub size: Point,
}

impl ConwayGame {
    // Inicializar Jogo
    pub fn new(size: Point) -> Self {
        let matrix = CellMatrix::new_with_size(size);
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
            self.matrix[point.row][point.col].state = State::Alive;
        }
    }

    pub fn update_living_cells(&mut self) {
        let mut matrix_out: Vec<Vec<Cell>> = CellMatrix::new_with_size(self.size);

        for row in self.matrix.iter() {
            for cell in row.iter() {
                Cell::update_state(cell.point, &self.matrix, &mut matrix_out);
            }
        }

        self.matrix = matrix_out;
    }

    // Pinta o estado salvo na tela
    pub fn paint_screen(&mut self, terminal: &mut Terminal<Stdout>) {
        let matrix = &self.matrix;
        let alive_cells = ConwayGame::get_alive_cells(matrix);

        terminal.act(Action::ClearTerminal(Clear::All)).unwrap();

        for cell in alive_cells.iter() {
            let cell_row = (cell.point.row + 10).try_into().unwrap();
            let cell_col = (cell.point.col + 4).try_into().unwrap();
            terminal
                .act(Action::MoveCursorTo(cell_row, cell_col))
                .unwrap();
            println!("#");
        }
    }
}
