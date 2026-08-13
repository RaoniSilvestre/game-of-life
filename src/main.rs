use game_of_life::{run_with_scene, Configuration, Res};

fn main() -> Res<()> {
    let config = Configuration::configure()?;

    run_with_scene(config)
}
