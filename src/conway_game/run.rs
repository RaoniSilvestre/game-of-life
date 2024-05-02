use super::super::utils::get_rand_points_list;
use super::cell::Cell;
use super::conway::ConwayGame;
use std::io::Stdout;
use std::{thread::sleep, time::Duration};
use terminal::*;

pub fn run(mut game: ConwayGame) -> Result<(), error::ErrorKind> {
    let mut terminal = terminal::stdout();
    terminal.act(Action::HideCursor)?;
    terminal.act(Action::ClearTerminal(Clear::All))?;

    let initial_state = get_rand_points_list(4000, game.size);

    game.start_state(initial_state);

    loop {
        paint_screen(&mut game.matrix, &mut terminal);
        game.matrix = ConwayGame::get_updated_points(&mut game.matrix);

        sleep(Duration::from_millis(100));
    }
}

// Pinta o estado salvo na tela
pub fn paint_screen(matrix: &mut [Vec<Cell>], terminal: &mut Terminal<Stdout>) {
    let alive_cells = ConwayGame::get_alive_cells(matrix);

    // let _ = alive_cells
    //     .iter()
    //     .map(|alive_cells| println!("{:#?} - {:#?}", alive_cells.state, alive_cells.point))
    //     .collect::<()>();
    terminal.act(Action::ClearTerminal(Clear::All)).unwrap();

    for cell in alive_cells.iter() {
        terminal
            .act(Action::MoveCursorTo(cell.point.row, cell.point.col))
            .unwrap();
        println!("#");
    }
}
