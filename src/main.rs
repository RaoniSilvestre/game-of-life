use gol::configuration::Config;
use gol::game::Runner;

#[tokio::main]
pub async fn main() {
    let config = Config::configure();
    let mut runner = Runner::new(config);
    runner.run().await
}
