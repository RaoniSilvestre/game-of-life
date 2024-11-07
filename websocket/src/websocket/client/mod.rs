use std::io::Stdout;

use crate::game::elements::{Cell, State};
use crate::game::{elements::Point, ConwayGame};
use async_trait::async_trait;
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;

pub struct Client {
    pub vec_points: Vec<Cell>,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

#[async_trait]
impl ezsockets::ClientExt for Client {
    type Call = ();

    async fn on_text(&mut self, text: String) -> Result<(), ezsockets::Error> {
        tracing::info!("Mensagem recebida: {text}");
        Ok(())
    }

    async fn on_binary(&mut self, bytes: Vec<u8>) -> Result<(), ezsockets::Error> {
        if bytes.len() == 16 {
            let point: Point = From::from(&bytes[..]);
            self.vec_points.push(Cell::new(point, State::Alive));
            return Ok(());
        } else if bytes.len() < 16 {
            ConwayGame::paint_screen('#', &self.vec_points, &mut self.terminal);
            self.vec_points = vec![];
            return Ok(());
        } else {
            return Err("eita".into());
        }
    }

    async fn on_call(&mut self, _call: Self::Call) -> Result<(), ezsockets::Error> {
        Ok(())
    }
}
