use cli_game_of_life::configuration::config::Config;
use cli_game_of_life::game::game_runner::Runner;

fn main() {
    let config = Config::configure();
    Runner::run(config)
}
