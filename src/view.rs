use anyhow::Context;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Widget};
use std::io::{self, Stdout};

use crate::automaton::{Board, Snapshot};
use crate::Res;

#[derive(Debug)]
pub struct BasicPainter {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

struct CellsWidget<'a, C> {
    board: &'a Board<C>,
}

impl<C> Widget for CellsWidget<'_, C>
where
    C: Into<Color> + Copy + PartialEq + Default,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        const CELL_CHAR: char = '█';

        for (i, &cell) in self.board.cells().iter().enumerate() {
            if cell == C::default() {
                continue;
            }

            let row = (i / self.board.width()) as u16;
            let col = (i % self.board.width()) as u16;

            let screen_x = area.x + col;
            let screen_y = area.y + row;

            if let Some(buf_cell) = buf.cell_mut((screen_x, screen_y)) {
                let color: Color = cell.into();
                buf_cell.set_char(CELL_CHAR).set_fg(color);
            }
        }
    }
}

impl BasicPainter {
    pub fn new() -> Res<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
        Ok(Self { terminal })
    }

    pub fn paint<C>(&mut self, snapshot: &Snapshot<C>) -> Res<()>
    where
        C: Into<Color> + Copy + PartialEq + Default,
    {
        let active = snapshot
            .board
            .cells()
            .iter()
            .filter(|&&cell| cell != C::default())
            .count();

        self.terminal
            .draw(|f| {
                let area = f.area();

                let block = Block::default()
                    .borders(Borders::ALL)
                    .title_top(snapshot.name.clone())
                    .title_bottom(format!(
                        " Iterations: {} | Active cells: {} ",
                        snapshot.iterations, active
                    ));

                let block_width = snapshot.board.width() as u16 + 2;
                let block_height = snapshot.board.height() as u16 + 2;
                let block_area = Rect::new(area.x, area.y, block_width, block_height);
                let inner_area = block.inner(block_area);

                f.render_widget(block, block_area);

                let cells_widget = CellsWidget {
                    board: &snapshot.board,
                };

                f.render_widget(cells_widget, inner_area);
            })
            .context("Erro ao pintar tela!")?;

        Ok(())
    }
}
