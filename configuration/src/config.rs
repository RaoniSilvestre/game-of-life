use super::Cli;
use super::Config;
use clap::Parser;
use game::elements::Point;

impl Config {
    pub fn configure() -> Config {
        let cli = Cli::parse();

        Config {
            size: Point::new(cli.dx, cli.dy),
            char: cli.def_char,
            fps: cli.fps,
            mode: cli.mode,
            rand_points: cli.random,
            duration: cli.duration,
        }
    }
}
