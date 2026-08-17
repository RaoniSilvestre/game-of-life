use game_of_life::{Automaton, Configuration, Conway, Res, Runner};

fn main() -> Res<()> {
    let config = Configuration::configure();

    let conway = Conway::new(80, 40);
    let mut runner = Runner::new(config, conway);

    runner.run()
}
