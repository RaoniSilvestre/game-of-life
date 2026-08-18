use std::{
    fs::{self},
    path::PathBuf,
};

use clap::Parser;

use crate::Res;

/// Basic simulator for automaton in terminal.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Frames per second
    #[arg(short, long, default_value_t = 1)]
    fps: u64,
    /// Scene file
    #[arg(short, long)]
    scene: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Configuration {
    pub fps: u64,
    pub scene: Option<toml::Value>,
}

impl Configuration {
    pub fn configure() -> Res<Configuration> {
        let cli = Cli::parse();

        let scene = cli.scene.map(parse_file).transpose()?;

        Ok(Configuration {
            fps: cli.fps,
            scene,
        })
    }
}

fn parse_file(s: PathBuf) -> Res<toml::Value> {
    let f = fs::read_to_string(s)?;
    Ok(toml::de::from_str::<toml::Value>(&f)?)
}
