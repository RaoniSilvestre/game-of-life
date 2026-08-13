use clap::Parser;

use crate::scene::Scene;
use crate::Res;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Frames per second (overrides the scene)
    #[arg(short, long)]
    pub fps: Option<u64>,

    /// Path to the scene file
    #[arg(long, default_value = "scenes/conway.toml")]
    pub scene: String,
}

#[derive(Debug, Clone)]
pub struct Configuration {
    pub fps: u64,
    pub scene: Scene,
}

impl Configuration {
    pub fn configure() -> Res<Configuration> {
        let cli = Cli::parse();

        let scene = Scene::from_path(&cli.scene)?;
        let fps = cli.fps.or(scene.fps).unwrap_or(1);

        Ok(Configuration { fps, scene })
    }
}
