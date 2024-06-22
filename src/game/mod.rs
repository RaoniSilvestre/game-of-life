pub mod conway;
pub mod elements;
pub mod game_runner;

use elements::{Cell, Point};

pub struct ConwayGame {
    pub matrix: Vec<Vec<Cell>>,
    pub size: Point,
}
