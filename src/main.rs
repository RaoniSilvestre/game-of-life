use gol::configuration::Config;
use gol::game::Runner;

fn main() {
    let config = Config::configure();
    let mut runner = Runner::new(config);
    runner.run();
}
