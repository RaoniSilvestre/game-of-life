use crossterm::style::Color;
use ratatui::layout::Rect;
use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::Block;
use ratatui::Terminal;

use super::elements::{Cell, CellMatrix, Point, State};
use super::ConwayGame;
use crossterm::{execute, terminal};
use ratatui::prelude::*;
use std::io::{self, Stdout};

impl ConwayGame {
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
    pub fn paint_screen(
        _tinta: char,
        alive_cells: &Vec<Cell>,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) {
        terminal
            .draw(|f| {
                let size = f.area();
                // Definir o fundo rosa para toda a tela
                let background = Block::default().style(Style::default().bg(style::Color::Red));
                f.render_widget(background, size);

                for cell in alive_cells.iter() {
                    let area = Rect::new(cell.point.col as u16, cell.point.row as u16, 1, 1); // Bloco maior para a célula viva
                                                                                              // Bloco roxo para a célula viva
                    let cell_block =
                        Block::default().style(Style::default().bg(style::Color::Magenta));
                    f.render_widget(cell_block, area);
                }
            })
            .unwrap();
    }

    pub fn painting_factory(
        tinta: char,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> impl FnMut(Vec<Cell>) + '_ {
        move |alives: Vec<Cell>| Self::paint_screen(tinta, &alives, terminal)
    }
}
