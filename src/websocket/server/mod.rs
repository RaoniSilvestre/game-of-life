pub mod session;

use crate::{
    configuration::Config,
    game::Runner,
    websocket::server::session::{GameSession, Session},
};
use async_trait::async_trait;
use ezsockets::Server;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

pub struct GameServer {
    pub game_runner: Arc<Mutex<Runner>>,
    pub handle: Server<GameServer>,
    pub config: Config,
}

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
        let config = self.config;
        let session = Session::create(
            |handle| {
                tokio::spawn({
                    let session = handle.clone();
                    async move {
                        loop {
                            Runner::sleep(config.fps).await;
                            info!("Enviando game state para : {}", id);
                            session.call(()).unwrap();
                        }
                    }
                });
                GameSession {
                    id,
                    handle,
                    game_runner: self.game_runner.clone(),
                    fps: self.config.fps,
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
