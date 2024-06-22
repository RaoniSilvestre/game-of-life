use super::elements::Point;
use crate::configuration::{Config, Mode};
use crate::game::ConwayGame;
use core::time::Duration;
use rand::Rng;
use std::io::Stdout;
use std::thread::sleep;
use terminal::*;

pub struct Runner {
    pub game: ConwayGame,
    pub terminal: Terminal<Stdout>,
}

impl Runner {
    pub fn run(config: Config) {
        match config.mode {
            Mode::Random => Self::random_run(config),
            Mode::Test => Self::testing_run(config),
        }
    }

    pub fn random_run(config: Config) {
        let mut game = ConwayGame::new(config.size);

        let mut terminal = get_terminal();
        let mut painter = ConwayGame::painting_factory(config.char, &mut terminal);

        let initial_state = random_generator(config.rand_points, game.size);

        game.start_state(initial_state);

        let mut counter = 1;
        loop {
            painter(&game.matrix);
            game.update_living_cells();
            sleep(Duration::from_millis(1000 / config.fps));

            if let Some(duration) = config.duration {
                if counter == duration {
                    break;
                }
                counter += 1;
            }
        }
    }
    pub fn testing_run(config: Config) {
        println!("{:?}", config);
        println!("Rodando em modo de teste!");
    }
}

// Funções auxiliares!
fn get_terminal() -> Terminal<Stdout> {
    let terminal = terminal::stdout();
    terminal.act(Action::HideCursor).unwrap();
    terminal.act(Action::ClearTerminal(Clear::All)).unwrap();
    terminal
}

fn random_generator(n: usize, size: Point) -> Vec<Point> {
    let mut points_list = Vec::new();
    let mut rng_1 = rand::thread_rng();
    let mut rng_2 = rand::thread_rng();
    for _ in 0..n {
        let new_point = Point {
            row: rng_1.gen_range(1..size.row - 1),
            col: rng_2.gen_range(1..size.col - 1),
        };

        points_list.push(new_point)
    }
    points_list
}
