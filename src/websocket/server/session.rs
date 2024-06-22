use super::Message;
use crate::game::elements::Point;
use async_trait::async_trait;
use tracing::info;

pub type SessionID = u16;
pub type Session = ezsockets::Session<SessionID, Message>;

#[derive(Debug)]
pub struct GameSession {
    pub handle: Session,
    pub id: SessionID,
    pub counter: usize,
}

#[async_trait]
impl ezsockets::SessionExt for GameSession {
    type ID = SessionID;
    type Call = Message;

    fn id(&self) -> &Self::ID {
        &self.id
    }

    async fn on_text(&mut self, text: String) -> Result<(), ezsockets::Error> {
        let _ = self.handle.text(text.clone()); // Send response to the client
        let id = self.handle.id;

        tracing::info!("Message from {id}: {text:?}");
        Ok(())
    }

    async fn on_binary(&mut self, bytes: Vec<u8>) -> Result<(), ezsockets::Error> {
        let point: Point = From::from(&bytes[..]);
        println!("{:?}", point);
        Ok(())
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), ezsockets::Error> {
        match call {
            Message::Increment => {
                self.counter += 1;
                info!("{} : {}", self.id, self.counter)
            }
            Message::Share => {
                let _ = self
                    .handle
                    .binary(Point {
                        row: self.counter,
                        col: self.counter,
                    })
                    .unwrap();
            }
        }
        Ok(())
    }
}
