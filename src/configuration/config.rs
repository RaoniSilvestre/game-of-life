use super::parser::{Cli, Mode};
use clap::Parser;

use crate::game::elements::Point;

#[derive(Debug)]
pub struct Config {
    pub size: Point,
    pub char: char,
    pub fps: u64,
    pub mode: Mode,
    pub rand_points: usize,
    pub duration: Option<usize>,
}

impl Config {
    pub fn configure() -> Config {
        let cli = Cli::parse();
        let config = Config {
            size: Point::new(cli.dx, cli.dy),
            char: cli.def_char,
            fps: cli.fps,
            mode: cli.mode,
            rand_points: cli.random,
            duration: cli.duration,
        };

        config
    }
}
