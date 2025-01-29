use clap::Parser;
use crossterm::terminal::size;

use crate::conway::Point;

use super::{cli::Cli, Mode};

#[derive(Debug, Clone, Copy)]
pub struct Configuration {
    pub size: Point,
    pub char: char,
    pub fps: u64,
    pub mode: Mode,
    pub rand_points: usize,
    pub duration: Option<usize>,
}

impl Configuration {
    pub fn configure() -> Configuration {
        let cli = Cli::parse();

        let (mut x, mut y): (usize, usize) = match size() {
            Ok((x, y)) => (x.into(), y.into()),
            Err(_) => (80, 40),
        };

        if let Some(value) = cli.dx {
            x = value;
        }

        if let Some(value) = cli.dy {
            y = value;
        }

        Configuration {
            size: Point::new(y, x),
            char: cli.def_char,
            fps: cli.fps,
            mode: cli.mode,
            rand_points: cli.random,
            duration: cli.duration,
        }
    }
}
