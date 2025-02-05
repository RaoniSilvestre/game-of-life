use game_of_life::{Configuration, Runner};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .compact()
        .init();

    let config = Configuration::configure();

    let mut runner = Runner::new(config);

    runner.run().await;
}
