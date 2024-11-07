use super::Cli;
use super::Config;
use clap::Parser;
use crossterm::terminal::size;
use game::elements::Point;

impl Config {
    pub fn configure() -> Config {
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

        Config {
            size: Point::new(y, x),
            char: cli.def_char,
            fps: cli.fps,
            mode: cli.mode,
            rand_points: cli.random,
            duration: cli.duration,
        }
    }
}
