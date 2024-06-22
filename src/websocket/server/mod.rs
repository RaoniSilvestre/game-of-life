pub mod session;

use crate::game::game_runner::Runner;
use crate::websocket::server::session::{GameSession, Session};
use async_trait::async_trait;
use std::{net::SocketAddr, time::Duration};

#[derive(Debug)]
pub enum Message {
    Increment,
    Share,
}

pub struct GameServer {}

#[async_trait]
impl ezsockets::ServerExt for GameServer {
    type Session = GameSession;
    type Call = ();

    async fn on_connect(
        &mut self,
        socket: ezsockets::Socket,
        _request: ezsockets::Request,
        address: SocketAddr,
    ) -> Result<Session, Option<ezsockets::CloseFrame>> {
        let id = address.port();

        let session = Session::create(
            |handle| {
                tokio::spawn({
                    let session = handle.clone();
                    async move {
                        loop {
                            session.call(Message::Increment).unwrap();
                            session.call(Message::Share).unwrap();
                            // info!("Adding counter to {}", session.id);
                            tokio::time::sleep(Duration::from_secs(3)).await;
                        }
                    }
                });
                GameSession {
                    id,
                    handle,
                    counter: 0,
                }
            },
            id,
            socket,
        );
        Ok(session)
    }

    async fn on_disconnect(
        &mut self,
        _id: <Self::Session as ezsockets::SessionExt>::ID,
        _reason: Result<Option<ezsockets::CloseFrame>, ezsockets::Error>,
    ) -> Result<(), ezsockets::Error> {
        Ok(())
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), ezsockets::Error> {
        let () = call;
        Ok(())
    }
}
