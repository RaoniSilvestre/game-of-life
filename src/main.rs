use cli_game_of_life::conway_game::elements::point::Point;
use cli_game_of_life::conway_game::{conway::ConwayGame, run::run};
use core::panic;
use terminal::*;

fn main() -> error::Result<()> {
    let game = ConwayGame::new(Point { row: 160, col: 36 });

    match run(game) {
        Ok(_) => {}
        Err(_) => panic!("Jogou parou de rodar subitamente!"),
    }
    Ok(())
}
