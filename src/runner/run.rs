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
use tokio::{
    sync::mpsc::{self, Receiver},
    time,
};
use tracing::info;

use crate::{
    conway::{Cell, State},
    view::{BasicPainter, Paint},
};

use super::{Runner, RunnerEvent};

pub enum Painting {
    Points(Vec<Cell>),
}

impl Runner {
    pub async fn painter_handler(mut rx: Receiver<Painting>) {
        let mut painter = BasicPainter::default();

        while let Some(paint) = rx.recv().await {
            match paint {
                Painting::Points(cells) => {
                    painter.paint(&cells);
                }
            }
        }
    }

    pub async fn run(&mut self) {
        let (event_tx, mut event_rx) = mpsc::channel::<RunnerEvent>(32);
        let (painter_tx, painter_rx) = mpsc::channel::<Painting>(32);
        let tx_input = event_tx.clone();
        let tx_tick = event_tx.clone();
        let tick = self.tick();

        info!("Habilitando modo raw!");
        enable_raw_mode().unwrap();

        execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();

        tokio::spawn(async move {
            Self::event_listener(tx_input).await;
        });

        tokio::spawn(async move {
            Self::tick_waiter(tx_tick, tick).await;
        });

        tokio::spawn(async move {
            Self::painter_handler(painter_rx).await;
        });

        self.start();

        while let Some(event) = event_rx.recv().await {
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

            painter_tx
                .send(Painting::Points(self.state()))
                .await
                .unwrap();
        }
    }

    async fn tick_waiter(tx: mpsc::Sender<RunnerEvent>, tick: u64) {
        let mut interval = time::interval(Duration::from_micros(1000000 / tick));

        loop {
            interval.tick().await;
            tx.send(RunnerEvent::Tick)
                .await
                .expect("Não foi possível enviar mensagem de tick");
        }
    }
}
