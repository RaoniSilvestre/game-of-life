use gol::configuration::Config;
use gol::game::game_runner::Runner;

fn main() {
    let config = Config::configure();
    Runner::run(config)
}
