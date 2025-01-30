use std::{
    io::{self},
    process::exit,
    time::Duration,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tokio::{sync::mpsc, time};

use crate::conway::{Cell, State};

use super::{Runner, RunnerEvent};

impl Runner {
    pub async fn run(&mut self) {
        let (tx, mut rx) = mpsc::channel::<RunnerEvent>(32);
        let tx_input = tx.clone();
        let tx_tick = tx.clone();
        let tick = self.tick();

        enable_raw_mode().unwrap();
        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();

        tokio::spawn(async move {
            Self::event_listener(tx_input).await;
        });

        tokio::spawn(async move {
            Self::tick_waiter(tx_tick, tick).await;
        });

        self.start();
        while let Some(event) = rx.recv().await {
            match event {
                RunnerEvent::Tick => {
                    if !self.stop {
                        self.update();
                    }
                }
                RunnerEvent::Revive(p) => {
                    self.add_cell(Cell::new(p, State::Alive));
                }
                RunnerEvent::Kill(p) => {
                    self.add_cell(Cell::new(p, State::Dead));
                }
                RunnerEvent::ToggleRun => self.stop = !self.stop,
                RunnerEvent::Quit => {
                    disable_raw_mode().unwrap();
                    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture).unwrap();
                    exit(0);
                }
            }
            self.render();
        }
    }

    async fn tick_waiter(tx: mpsc::Sender<RunnerEvent>, tick: u64) {
        let mut interval = time::interval(Duration::from_millis(1000 / tick));

        loop {
            interval.tick().await;
            tx.send(RunnerEvent::Tick)
                .await
                .expect("Não foi possível enviar mensagem de tick");
        }
    }
}
