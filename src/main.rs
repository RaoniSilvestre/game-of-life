use anyhow::{bail, Context};
use game_of_life::{AutomatonType, Configuration, Conway, ConwayScene, Res, Runner};

fn main() -> Res<()> {
    let mut config = Configuration::configure()?;

    match config.scene.take() {
        Some(value) => {
            let automaton_type = value.get("type").context("cena sem campo `type`")?;
            match AutomatonType::try_from(automaton_type)? {
                AutomatonType::Conway => {
                    let scene: ConwayScene = value.try_into()?;
                    Runner::<Conway>::new(config, scene).run()
                }
                AutomatonType::FireSim => bail!("autômato FireSim ainda não implementado"),
            }
        }
        None => Runner::<Conway>::new(config, ConwayScene::empty(80, 40)).run(),
    }
}
