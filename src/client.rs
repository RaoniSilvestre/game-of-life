use async_trait::async_trait;
use ezsockets::ClientConfig;
use std::io::BufRead;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl From<&[u8]> for Point {
    // add code here
    fn from(bytes: &[u8]) -> Self {
        let x = usize::from_le_bytes(bytes[0..8].try_into().expect("Slice with incorrect lenght"));
        let y = usize::from_le_bytes(
            bytes[8..16]
                .try_into()
                .expect("Slice with incorrect lenght"),
        );
        Point { x, y }
    }
}

impl Into<Vec<u8>> for Point {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(16);
        bytes.extend_from_slice(&self.x.to_le_bytes());
        bytes.extend_from_slice(&self.y.to_le_bytes());
        bytes
    }
}

struct Client {}

#[async_trait]
impl ezsockets::ClientExt for Client {
    type Call = ();

    async fn on_text(&mut self, text: String) -> Result<(), ezsockets::Error> {
        tracing::info!("Mensagem recebida: {text}");
        Ok(())
    }

    async fn on_binary(&mut self, bytes: Vec<u8>) -> Result<(), ezsockets::Error> {
        let point: Point = From::from(&bytes[..]);

        tracing::info!("{:?}", point);
        Ok(())
    }

    async fn on_call(&mut self, _call: Self::Call) -> Result<(), ezsockets::Error> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let config = ClientConfig::new("ws://localhost:8080");
    let (handle, future) = ezsockets::connect(|_client| Client {}, config).await;
    tokio::spawn(async move {
        future.await.unwrap();
    });

    let stdin = std::io::stdin();
    let lines = stdin.lock().lines();
    for line in lines {
        // Tente parsear a linha em dois valores usize
        if let Ok(pars) = line {
            let parts: Vec<&str> = pars.split_whitespace().collect();

            if parts.len() != 2 {
                tracing::error!(
                    "Linha malformada: esperava 2 valores, mas recebeu {}",
                    parts.len()
                );
                continue;
            }

            let x: usize = match parts[0].parse() {
                Ok(val) => val,
                Err(_) => {
                    tracing::error!("Falha ao parsear x: {}", parts[0]);
                    continue;
                }
            };

            let y: usize = match parts[1].parse() {
                Ok(val) => val,
                Err(_) => {
                    tracing::error!("Falha ao parsear y: {}", parts[1]);
                    continue;
                }
            };

            // Crie a struct Point
            let point = Point { x, y };

            handle.binary(point).unwrap();
        }
    }
}
