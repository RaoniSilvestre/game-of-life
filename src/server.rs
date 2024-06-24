use std::sync::Arc;
use tokio::sync::Mutex;

use ezsockets::Server;
use gol::{configuration::Config, game::Runner, websocket::server::GameServer};
use tracing::info;

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();
    let config = Config::configure();
    let mut runner: Runner = Runner::new(config);

    runner.start();

    let arc_runner = Arc::new(Mutex::new(runner));

    let (server, _) = Server::create(|handle| GameServer {
        game_runner: arc_runner.clone(),
        handle,
        config,
    });

    tokio::task::spawn(async move {
        loop {
            let mut locked_runner = arc_runner.lock().await;
            info!("Atualizando jogo");
            locked_runner.update();
            Runner::sleep(locked_runner.config.fps).await;
            drop(locked_runner)
        }
    });

    ezsockets::tungstenite::run(server, "127.0.0.1:8080")
        .await
        .unwrap();
}
