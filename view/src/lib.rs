use game::elements::Cell;
use ratatui::layout::Rect;
use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::Block;
use ratatui::Terminal;

use ratatui::prelude::*;
use std::io;
use std::io::Stdout;

#[derive(Debug)]
pub struct BasicPainter {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

pub trait Paint {
    fn paint(&mut self, cells: &Vec<Cell>);
}

impl BasicPainter {
    pub fn new() -> BasicPainter {
        BasicPainter {
            terminal: Terminal::new(CrosstermBackend::new(io::stdout())).unwrap(),
        }
    }
}

impl Paint for BasicPainter {
    fn paint(&mut self, alive_cells: &Vec<Cell>) {
        self.terminal
            .draw(|f| {
                let size = f.area();
                let background = Block::default().style(Style::default().bg(style::Color::Red));
                f.render_widget(background, size);

                for cell in alive_cells.iter() {
                    let area = Rect::new(cell.point.col as u16, cell.point.row as u16, 1, 1);
                    let cell_block =
                        Block::default().style(Style::default().bg(style::Color::Magenta));
                    f.render_widget(cell_block, area);
                }
            })
            .unwrap();
    }
}
