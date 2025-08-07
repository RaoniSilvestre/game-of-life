use anyhow::Context;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Widget};
use std::io::{self, Stdout};

use crate::game::GameState;
use crate::Res;

#[derive(Debug)]
pub struct BasicPainter {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

pub trait Paint {
    fn paint(&mut self, cells: GameState) -> Res<()>;
}

struct CellsWidget<'a> {
    cells: &'a GameState,
}

impl Widget for CellsWidget<'_> {
    fn render(self, _: Rect, buf: &mut Buffer) {
        const CELL_CHAR: char = '█';

        for &(row, col) in self.cells.iter() {
            if row >= 0 && col >= 0 {
                let x = col as u16;
                let y = row as u16;

                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_char(CELL_CHAR);
                }
            }
        }
    }
}

impl BasicPainter {
    pub fn new() -> Res<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        Ok(Self { terminal })
    }
}

impl Paint for BasicPainter {
    fn paint(&mut self, cells: GameState) -> Res<()> {
        self.terminal
            .draw(|f| {
                let size = f.area();

                let block = Block::default()
                    .borders(Borders::ALL)
                    .title_top("Conway's game")
                    .title_bottom(format!(" Alive Cells: {} ", cells.len()));

                let inner_area = block.inner(size);

                f.render_widget(block, size);

                let cells_widget = CellsWidget { cells: &cells };

                f.render_widget(cells_widget, inner_area);
            })
            .context("Erro ao pintar tela!")?;

        Ok(())
    }
}
