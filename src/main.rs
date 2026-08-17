use game_of_life::{Configuration, Conway, ConwayScene, Res, Runner};

fn main() -> Res<()> {
    let config = Configuration::configure()?;

    let scene = ConwayScene {
        height: 40,
        width: 80,
        rows: vec![],
    };
    let mut runner = Runner::<Conway>::new(config, scene);

    runner.run()
}
