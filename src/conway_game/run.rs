use super::super::utils::get_rand_points_list;
use super::conway::ConwayGame;
use crate::utils::get_ready_terminal;
use std::{thread::sleep, time::Duration};
use terminal::*;

pub fn run(mut game: ConwayGame) -> Result<(), error::ErrorKind> {
    let mut terminal = get_ready_terminal();

    let initial_state = get_rand_points_list(6000, game.size);

    game.start_state(initial_state);

    loop {
        game.paint_screen(&mut terminal);
        game.update_living_cells();
        sleep(Duration::from_millis(100));
    }
}
