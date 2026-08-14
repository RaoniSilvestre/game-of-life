use anyhow::Context;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Widget};
use std::io::{stdout, Stdout};
use std::sync::mpsc::Receiver;

use crate::game::GameState;
use crate::Res;

pub fn listen(rx: Receiver<GameState>) -> Res<()> {
    let mut terminal = ratatui::init();
    execute!(stdout(), EnableMouseCapture)?;

    terminal.clear()?;

    loop {
        let cells = rx.recv()?;
        paint(&mut terminal, cells)?
    }
}

pub fn quit() {
    execute!(stdout(), DisableMouseCapture).unwrap();
    ratatui::restore();
    std::process::exit(0)
}

fn paint(terminal: &mut Terminal<CrosstermBackend<Stdout>>, cells: GameState) -> Res<()> {
    terminal
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
