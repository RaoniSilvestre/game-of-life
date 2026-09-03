use anyhow::Context;
use crossterm::cursor::Show;
use crossterm::event::DisableMouseCapture;
use crossterm::execute;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Widget};
use std::io::{stdout, Stdout};
use std::sync::{mpsc::Receiver, Arc};

use crate::automaton::{Automaton, Board};
use crate::Res;

pub fn listen<A: Automaton>(
    terminal: Terminal<CrosstermBackend<Stdout>>,
    rx: Receiver<Snapshot<A::Cell>>,
) -> Res<()> {
    let mut terminal = terminal;
    loop {
        let cells = rx.recv()?;
        paint::<A>(&mut terminal, cells)?
    }
}

pub fn quit() {
    execute!(stdout(), DisableMouseCapture, Show).unwrap();
    ratatui::restore();
    std::process::exit(0)
}

fn paint<A: Automaton>(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    cells: Snapshot<A::Cell>,
) -> Res<()> {
    terminal
        .draw(|f| {
            let size = f.area();
            f.render_widget(cells, size);
        })
        .context("Erro ao pintar tela!")?;

    Ok(())
}

#[derive(Clone, Debug)]
pub struct Snapshot<C: Into<Color>> {
    pub name: String,
    pub iterations: usize,
    pub board: Arc<Board<C>>,
}

impl<C: Into<Color> + Copy + Default> Widget for Snapshot<C> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        const CELL_CHAR: char = '█';

        let block = Block::default()
            .borders(Borders::ALL)
            .title_top(self.name.as_str())
            .title_bottom(format!(" Iterations: {} ", self.iterations));

        let inner = block.inner(area);
        block.render(area, buf);

        let width = self.board.width();
        for (idx, cell) in self.board.cells().iter().enumerate() {
            let row = idx / width;
            let col = idx % width;
            let x = inner.x + col as u16;
            let y = inner.y + row as u16;

            if let Some(buf_cell) = buf.cell_mut((x, y)) {
                buf_cell.set_char(CELL_CHAR).set_fg((*cell).into());
            }
        }
    }
}
