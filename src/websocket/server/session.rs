use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use crate::game::{elements::Point, Runner};
use async_trait::async_trait;

pub type SessionID = u16;
pub type Session = ezsockets::Session<SessionID, ()>;

pub struct GameSession {
    pub handle: Session,
    pub id: SessionID,
    pub game_runner: Arc<Mutex<Runner>>,
    pub fps: u64,
}

#[async_trait]
impl ezsockets::SessionExt for GameSession {
    type ID = SessionID;
    type Call = ();

    fn id(&self) -> &Self::ID {
        &self.id
    }

    async fn on_text(&mut self, text: String) -> Result<(), ezsockets::Error> {
        let _ = self.handle.text(text.clone());
        let id = self.handle.id;

        info!("Message from {id}: {text:?}");
        Ok(())
    }

    async fn on_binary(&mut self, bytes: Vec<u8>) -> Result<(), ezsockets::Error> {
        let point: Point = From::from(&bytes[..]);
        println!("{:?}", point);
        Ok(())
    }

    async fn on_call(&mut self, _: Self::Call) -> Result<(), ezsockets::Error> {
        info!("{} : Iniciando call", self.id);
        let game_state = self.game_runner.clone().lock().await.state();
        let length = game_state.len();

        info!("{} : Enviando {} pontos", self.id, length);
        let _ = self.handle.binary(vec![]);
        for cell in game_state {
            let _ = self.handle.binary(cell.point);
        }

        info!("{} : Pontos enviados", self.id);
        Ok(())
    }
}
