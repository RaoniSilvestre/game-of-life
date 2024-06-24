use super::elements::{Cell, CellMatrix, Point, State};
use super::ConwayGame;
use std::io::Stdout;
use terminal::*;

impl ConwayGame {
    // Inicializar Jogo
    pub fn new(size: Point) -> Self {
        let matrix = CellMatrix::new(size);
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
        let matrix_in = self.matrix.clone();
        let update_state = Cell::update_state_factory(&matrix_in);
        let mut matrix_out: Vec<Vec<Cell>> = CellMatrix::new(self.size);

        for row in self.matrix.iter() {
            for cell in row.iter() {
                update_state(cell.point, &mut matrix_out);
            }
        }

        self.matrix = matrix_out;
    }

    // Pinta o estado salvo na tela
    pub fn paint_screen(tinta: char, alive_cells: &Vec<Cell>, terminal: &mut Terminal<Stdout>) {
        terminal.act(Action::ClearTerminal(Clear::All)).unwrap();

        for cell in alive_cells.iter() {
            let cell_row = (cell.point.row).try_into().unwrap();
            let cell_col = (cell.point.col).try_into().unwrap();
            terminal
                .act(Action::MoveCursorTo(cell_row, cell_col))
                .unwrap();
            println!("{}", tinta);
        }
    }

    pub fn painting_factory(
        tinta: char,
        terminal: &mut Terminal<Stdout>,
    ) -> impl FnMut(Vec<Cell>) + '_ {
        move |alives: Vec<Cell>| Self::paint_screen(tinta, &alives, terminal)
    }
}
