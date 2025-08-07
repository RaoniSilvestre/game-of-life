use game_of_life::{Configuration, Res, Runner};

fn main() -> Res<()> {
    let config = Configuration::configure();

    let mut runner = Runner::new(config);

    runner.run()
}
