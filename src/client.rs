use std::io;

use ezsockets::ClientConfig;
use gol::websocket::client::Client;
use ratatui::{prelude::CrosstermBackend, Terminal};

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();
    let config = ClientConfig::new("ws://127.0.0.1:8080");
    let (_handle, future) = ezsockets::connect(
        |_client| Client {
            vec_points: vec![],
            terminal: Terminal::new(CrosstermBackend::new(io::stdout())).unwrap(),
        },
        config,
    )
    .await;
    future.await.unwrap();
}
