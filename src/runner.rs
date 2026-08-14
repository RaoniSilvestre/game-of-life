use std::{sync::mpsc::sync_channel, thread};

use anyhow::Context;

use crate::{
    game::{ConwayGame, GameState, Point},
    listener::{self},
    view::{self},
    Configuration, Res,
};

#[derive(Debug)]
pub enum RunnerEvent {
    Tick,
    Revive(Point),
    AddAround(Point),
    ToggleRun,
    Quit,
}

#[derive(Debug)]
pub struct Runner {
    game: ConwayGame,
    config: Configuration,
    stop: bool,
}

impl Runner {
    pub fn new(config: Configuration) -> Self {
        let game = ConwayGame::default();

        Runner {
            game,
            config,
            stop: true,
        }
    }

    pub fn run(&mut self) -> Res<()> {
        let (event_tx, event_rx) = sync_channel::<RunnerEvent>(32);
        let (painter_tx, painter_rx) = sync_channel::<GameState>(32);

        let tick_rate = self.config.fps;

        // Input thread
        thread::spawn(move || {
            if let Err(e) = listener::listen(event_tx, tick_rate) {
                eprintln!("Erro no event listener... {e}");
                view::quit();
            }
        });

        // Output thread
        thread::spawn(|| {
            if let Err(e) = view::listen(painter_rx) {
                eprintln!("Erro no view listener... {e}");
                view::quit();
            }
        });

        // Processing thread
        loop {
            let event = event_rx.recv()?;

            match event {
                RunnerEvent::Tick if self.stop => (),
                RunnerEvent::Tick => self.game.update(),
                RunnerEvent::Revive(p) => self.game.add(p),
                RunnerEvent::AddAround(p) => self.game.scatter(p),
                RunnerEvent::ToggleRun => self.stop = !self.stop,
                RunnerEvent::Quit => view::quit(),
            }

            painter_tx
                .send(self.game.state())
                .context("Falha ao enviar estado atual para pintar")?;
        }
    }

    pub fn tick(&self) -> u64 {
        self.config.fps
    }
}
