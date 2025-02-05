use clap::Parser;
use crossterm::terminal::size;

use crate::conway::Point;

use super::cli::Cli;

#[derive(Debug, Clone, Copy)]
pub struct Configuration {
    pub size: Point,
    pub fps: u64,
}

impl Configuration {
    pub fn configure() -> Configuration {
        let cli = Cli::parse();

        let (x, y): (usize, usize) = match size() {
            Ok((x, y)) => (x.into(), y.into()),
            Err(_) => (80, 40),
        };

        Configuration {
            size: Point::new(y, x),
            fps: cli.fps,
        }
    }
}
