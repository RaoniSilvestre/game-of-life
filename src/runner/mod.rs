mod event_listener;
mod painter;
mod run;
mod tick;

use run::Painting;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{
    configuration::Configuration,
    conway::{Cell, Point},
    view::BasicPainter,
    ConwayGame,
};

#[derive(Debug)]
pub struct EventListener(Sender<RunnerEvent>);

#[derive(Debug)]
pub struct TickWaiter {
    tx: Sender<RunnerEvent>,
    tick: u64,
}

#[derive(Debug)]
pub struct PaintHandler {
    rx: Receiver<Painting>,
    painter: BasicPainter,
}

#[derive(Debug)]
pub struct Runner {
    game: ConwayGame,
    config: Configuration,
    stop: bool,
}

#[derive(Debug)]
pub struct RunnerChannels {
    event_rx: Receiver<RunnerEvent>,
    painter_tx: Sender<Painting>,
}

#[derive(Debug)]
pub enum RunnerEvent {
    Tick,
    Revive(Point),
    Kill(Point),
    ToggleRun,
    Quit,
}

struct RunnerStructs {
    event_listener: EventListener,
    tick_waiter: TickWaiter,
    painter: PaintHandler,
    channels: RunnerChannels,
}

impl Runner {
    pub fn new(config: Configuration) -> Self {
        let game = ConwayGame::new(config.size);

        Runner {
            game,
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

    fn add_cell(&mut self, cell: Cell) {
        self.game.matrix[cell.row()][cell.col()] = cell;
    }

    pub fn tick(&self) -> u64 {
        self.config.fps
    }
}
