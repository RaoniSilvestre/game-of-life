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
        BasicPainter {
            terminal: Terminal::new(CrosstermBackend::new(io::stdout())).unwrap(),
        }
    }
}

impl Paint for BasicPainter {
    fn paint(&mut self, alive_cells: &[Cell]) {
        self.terminal
            .draw(|f| {
                let size = f.area();
                // Defina o conteúdo para o "paragraph" - um simples caractere representando a célula viva
                let cell_char = '█'; // Você pode substituir isso por qualquer caractere

                // Criar um buffer para renderizar todas as células em forma de string
                let mut screen_content = String::new();
                let cells_per_row = size.width as usize;
                let cells_per_col = size.height as usize;

                // Inicializar o conteúdo da tela com o caractere de célula (como um padrão)
                for row in 0..cells_per_col {
                    for col in 0..cells_per_row {
                        if alive_cells
                            .iter()
                            .any(|cell| cell.point.row == row && cell.point.col == col)
                        {
                            screen_content.push(cell_char); // Adiciona o caractere da célula
                        } else {
                            screen_content.push(' '); // Adiciona um espaço vazio se não for uma célula viva
                        }
                    }
                    screen_content.push('\n'); // Nova linha após cada linha de células
                }

                // Renderiza o conteúdo no terminal com o Paragraph
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
