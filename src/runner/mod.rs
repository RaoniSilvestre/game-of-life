mod event_listener;
mod run;

use crate::{
    configuration::Configuration,
    conway::{Cell, Point},
    view::{BasicPainter, Paint},
    ConwayGame,
};

#[derive(Debug)]
pub struct Runner {
    pub game: ConwayGame,
    pub painter: BasicPainter,
    pub config: Configuration,
    pub stop: bool,
}

#[derive(Debug)]
pub enum RunnerEvent {
    Tick,
    Revive(Point),
    Kill(Point),
    ToggleRun,
    Quit,
}

impl Runner {
    pub fn new(config: Configuration) -> Self {
        let game = ConwayGame::new(config.size);
        let painter = BasicPainter::default();
        Runner {
            game,
            painter,
            config,
            stop: true,
        }
    }

    fn start(&mut self) {
        self.game.start_state(Vec::new())
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

    fn add_cell(&mut self, cell: Cell) {
        self.game.matrix[cell.row()][cell.col()] = cell;
    }

    pub fn tick(&self) -> u64 {
        self.config.fps
    }
}
