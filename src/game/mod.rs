pub mod conway;
pub mod elements;
pub mod game_runner;

use super::configuration::Config;
use elements::{Cell, Point};
use rand::Rng;
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::Stdout;

pub struct ConwayGame {
    pub matrix: Vec<Vec<Cell>>,
    pub size: Point,
}

pub struct Runner {
    pub game: ConwayGame,
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
    pub config: Config,
}

// Funções auxiliares
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
