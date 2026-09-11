use anyhow::Context;
use game_of_life::{
    AutomatonType, Configuration, Conway, ConwayScene, FireScene, FireSim, Res, Runner,
};

#[macroquad::main("Game of Life")]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Erro: {e}");
        std::process::exit(1);
    }
}

async fn run() -> Res<()> {
    let mut config = Configuration::configure()?;

    match config.scene.take() {
        Some(value) => {
            let automaton_type = value.get("type").context("cena sem campo `type`")?;
            match AutomatonType::try_from(automaton_type)? {
                AutomatonType::Conway => {
                    let scene: ConwayScene = value.try_into()?;
                    Runner::<Conway>::new(config, scene).run().await
                }
                AutomatonType::FireSim => {
                    let scene: FireScene = value.try_into()?;
                    Runner::<FireSim>::new(config, scene).run().await
                }
            }
        }
        None => Runner::<Conway>::new(config, ConwayScene::empty(80, 40)).run().await,
    }
}
