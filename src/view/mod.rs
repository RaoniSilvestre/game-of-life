use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::Terminal;

use ratatui::prelude::*;
use std::io;
use std::io::Stdout;

use crate::conway::Cell;

#[derive(Debug)]
pub struct BasicPainter {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

pub trait Paint {
    fn paint(&mut self, cells: &[Cell]);
}

impl Default for BasicPainter {
    fn default() -> Self {
        Self {
            terminal: Terminal::new(CrosstermBackend::new(io::stdout())).unwrap(),
        }
    }
}

impl Paint for BasicPainter {
    fn paint(&mut self, alive_cells: &[Cell]) {
        self.terminal
            .draw(|f| {
                let size = f.area();

                let cell_char = '█';

                let mut buffer = vec![vec![' '; size.width as usize]; size.height as usize];

                alive_cells.iter().for_each(|cell| {
                    if cell.row() < size.height as usize && cell.col() < size.width as usize {
                        buffer[cell.row()][cell.col()] = cell_char;
                    }
                });

                let screen_content: String = buffer
                    .iter()
                    .map(|row| row.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join("\n");

                let paragraph = Paragraph::new(screen_content)
                    .style(Style::default().fg(Color::Black).bg(Color::White))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .title("Conway's game"),
                    );

                f.render_widget(paragraph, size);
            })
            .unwrap();
    }
}
