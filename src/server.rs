use ezsockets::Server;
use gol::websocket::server::GameServer;

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();
    let (server, _) = Server::create(|_handle| GameServer {});

    ezsockets::tungstenite::run(server, "127.0.0.1:8080")
        .await
        .unwrap();
}
