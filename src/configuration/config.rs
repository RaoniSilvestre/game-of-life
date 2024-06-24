use super::Config;
use crate::configuration::Cli;
use crate::game::elements::Point;
use clap::Parser;

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
