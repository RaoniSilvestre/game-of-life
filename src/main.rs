use game_of_life::{Configuration, Runner};
use tracing::debug;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .compact()
        .init();

    let config = Configuration::configure();

    debug!("Configuração especificada: {:?}", config);

    let mut runner = Runner::new(config);

    debug!("Runner inicializado");

    runner.run();
}
