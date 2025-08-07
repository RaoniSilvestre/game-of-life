use clap::Parser;

/// Basic simulator for Conway's Game of Life.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Frames per second
    #[arg(short, long, default_value_t = 1)]
    pub fps: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct Configuration {
    pub fps: u64,
}

impl Configuration {
    pub fn configure() -> Configuration {
        let cli = Cli::parse();

        Configuration { fps: cli.fps }
    }
}
