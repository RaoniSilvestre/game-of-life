use cli_game_of_life::config::Config;
use cli_game_of_life::conway_game::game_runner::Runner;

fn main() {
    let config = Config::configure();
    Runner::run(config)
}
