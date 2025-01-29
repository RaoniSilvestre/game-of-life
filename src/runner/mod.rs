use std::{thread::sleep, time::Duration};

use tracing::{debug, info};

use crate::{
    configuration::{Configuration, Mode},
    conway::Cell,
    utils::random_generator,
    view::{BasicPainter, Paint},
    ConwayGame,
};

pub struct Runner {
    pub game: ConwayGame,
    pub painter: BasicPainter,
    pub config: Configuration,
}

impl Runner {
    pub fn new(config: Configuration) -> Self {
        let game = ConwayGame::new(config.size);
        let painter = BasicPainter::default();
        Runner {
            game,
            painter,
            config,
        }
    }

    pub fn run(&mut self) {
        match self.config.mode {
            Mode::Random => self.random_run(),
            Mode::Test => self.testing_run(),
        }
    }

    fn start(&mut self) {
        debug!("Inicializando jogo");
        let initial_state = random_generator(self.config.rand_points, self.game.size);
        self.game.start_state(initial_state);
    }

    fn update(&mut self) {
        self.game.update_living_cells()
    }

    fn state(&self) -> Vec<Cell> {
        ConwayGame::get_alive_cells(&self.game.matrix)
    }

    fn render(&mut self) {
        let alives = self.state();
        self.painter.paint(&alives);
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
        info!("Rodando em modo de teste!");
        sleep(Duration::from_secs(10));
    }
}
