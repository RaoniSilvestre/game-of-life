use ezsockets::ClientConfig;
use gol::game::elements::Point;
use gol::websocket::client::Client;
use std::io::BufRead;

#[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt::init();
    let config = ClientConfig::new("ws://localhost:8080");
    let (handle, future) = ezsockets::connect(
        |_client| Client {
            p: Point::new(1, 2),
        },
        config,
    )
    .await;
    tokio::spawn(async move {
        future.await.unwrap();
    });

    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        if let Ok(pars) = line {
            let parts: Vec<&str> = pars.split_whitespace().collect();

            if parts.len() != 2 {
                tracing::error!(
                    "Linha malformada: esperava 2 valores, mas recebeu {}",
                    parts.len()
                );
                continue;
            }

            let row: usize = match parts[0].parse() {
                Ok(val) => val,
                Err(_) => {
                    tracing::error!("Falha ao parsear x: {}", parts[0]);
                    continue;
                }
            };

            let col: usize = match parts[1].parse() {
                Ok(val) => val,
                Err(_) => {
                    tracing::error!("Falha ao parsear y: {}", parts[1]);
                    continue;
                }
            };

            let point = Point::new(row, col);

            handle.binary(point).unwrap();
        }
    }
}
