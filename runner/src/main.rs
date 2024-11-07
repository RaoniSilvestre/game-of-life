use configuration::Config;
use runner::Runner;

fn main() {
    let config = Config::configure();
    let mut runner = Runner::new(config);
    runner.run();
}
