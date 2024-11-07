use std::{thread::sleep, time::Duration};

use configuration::{Config, Mode};
use game::{elements::Cell, ConwayGame};
use view::{BasicPainter, Paint};

mod utils;

pub struct Runner {
    pub game: ConwayGame,
    pub painter: BasicPainter,
    pub config: Config,
}

impl Runner {
    pub fn new(config: Config) -> Self {
        let game = ConwayGame::new(config.size);
        let painter = BasicPainter::new();
        Runner {
            game,
            painter,
            config,
        }
    }

    pub fn start(&mut self) {
        let initial_state = utils::random_generator(self.config.rand_points, self.game.size);
        self.game.start_state(initial_state)
    }

    fn update(&mut self) {
        self.game.update_living_cells()
    }

    fn state(&self) -> Vec<Cell> {
        ConwayGame::get_alive_cells(&self.game.matrix)
    }

    pub fn render(&mut self) {
        let alives = self.state();
        self.painter.paint(&alives);
    }

    pub fn run(&mut self) {
        match self.config.mode {
            Mode::Random => self.random_run(),
            Mode::Test => self.testing_run(),
        }
    }

    fn sleep(fps: u64) {
        sleep(Duration::from_millis(1000 / fps))
    }

    fn random_run(&mut self) {
        self.start();
        loop {
            self.render();
            Runner::sleep(self.config.fps);
            self.update();
        }
    }

    fn testing_run(&self) {
        println!("{:?}", self.config);
        println!("Rodando em modo de teste!");
        sleep(Duration::from_secs(10));
    }
}
