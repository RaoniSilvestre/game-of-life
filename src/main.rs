use core::panic;
use std::thread::sleep;
use std::time::Duration;

use cli_game_of_life::conway_game::point::Point;
use cli_game_of_life::conway_game::{conway::ConwayGame, run::run};
use terminal::*;

fn main() -> error::Result<()> {
    let terminal = terminal::stdout();

    let (rows, cols) = match terminal.get(Value::TerminalSize) {
        Ok(Retrieved::TerminalSize(row, col)) => (row, col),
        _ => panic!("Não foi possível recuperar tamanho do terminal"),
    };

    println!("{rows} - {cols}");
    sleep(Duration::from_millis(1000));

    let game = ConwayGame::new(Point { row: 130, col: 36 });

    match run(game) {
        Ok(_) => {}
        Err(_) => panic!("Jogou parou de rodar subitamente!"),
    }
    Ok(())
}
