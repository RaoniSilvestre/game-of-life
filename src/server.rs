use std::sync::{Arc, Mutex};

use ezsockets::Server;
use gol::{configuration::Config, game::Runner, websocket::server::GameServer};

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();
    let config = Config::configure();

    let runner: Runner = Runner::new(config);

    let arc_runner = Arc::new(Mutex::new(runner));
    let _arc_runner = arc_runner.clone();

    let (server, _) = Server::create(|_handle| GameServer {
        game_runner: _arc_runner,
    });

    tokio::task::spawn(async move {
        loop {
            let mut locked_runner = arc_runner.lock().unwrap();
            locked_runner.update();
            locked_runner.sleep();
        }
    });

    ezsockets::tungstenite::run(server, "127.0.0.1:8080")
        .await
        .unwrap();
}
