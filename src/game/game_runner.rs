use super::elements::Cell;
use super::random_generator;
use super::Runner;
use crate::configuration::{Config, Mode};
use crate::game::ConwayGame;
use core::time::Duration;
use std::thread::sleep;

impl Runner {
    pub fn new(config: Config) -> Self {
        let game = ConwayGame::new(config.size);
        let terminal = terminal::stdout();
        Runner {
            game,
            terminal,
            config,
        }
    }

    pub fn start(&mut self) {
        let initial_state = random_generator(self.config.rand_points, self.game.size);
        self.game.start_state(initial_state)
    }

    pub fn update(&mut self) {
        self.game.update_living_cells()
    }

    pub fn state(&self) -> Vec<Cell> {
        ConwayGame::get_alive_cells(&self.game.matrix)
    }

    pub fn print(&mut self) {
        let mut painter = ConwayGame::painting_factory(self.config.char, &mut self.terminal);
        painter(&self.game.matrix)
    }

    pub fn run(&mut self) {
        match self.config.mode {
            Mode::Random => self.random_run(),
            Mode::Test => self.testing_run(),
        }
    }

    pub fn sleep(&self) {
        sleep(Duration::from_millis(1000 / self.config.fps))
    }

    fn random_run(&mut self) {
        self.start();
        loop {
            self.print();
            self.sleep();
            self.update();
        }
    }

    fn testing_run(&self) {
        println!("{:?}", self.config);
        println!("Rodando em modo de teste!");
        sleep(Duration::from_secs(10));
    }
}
