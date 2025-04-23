use std::{
    io::{self},
    process::exit,
    time::Duration,
};

use anyhow::{Context, Ok};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tokio::{
    sync::mpsc::{self, Receiver},
    time,
};

use crate::{
    conway::{Cell, State},
    view::{BasicPainter, Paint},
    Res,
};

use super::{Runner, RunnerEvent};

pub enum Painting {
    Points(Vec<Cell>),
}

impl Runner {
    pub async fn painter_handler(mut rx: Receiver<Painting>) -> Res<()> {
        let mut painter = BasicPainter::new()?;

        while let Some(paint) = rx.recv().await {
            match paint {
                Painting::Points(cells) => {
                    painter.paint(&cells)?;
                }
            }
        }

        Ok(())
    }

    pub async fn run(&mut self) -> Res<()> {
        let (event_tx, mut event_rx) = mpsc::channel::<RunnerEvent>(32);
        let (painter_tx, painter_rx) = mpsc::channel::<Painting>(32);
        let tx_input = event_tx.clone();
        let tx_tick = event_tx.clone();
        let tick = self.tick();

        enable_raw_mode().context("Falha ao habilitar raw mode")?;

        let mut stdout = io::stdout();

        stdout.execute(EnterAlternateScreen)?;
        stdout.execute(EnableMouseCapture)?;

        tokio::spawn(async move {
            if let Err(e) = Self::event_listener(tx_input).await {
                eprintln!("Erro no event listener... {e}")
            };
        });

        tokio::spawn(async move {
            Self::tick_waiter(tx_tick, tick).await;
        });

        tokio::spawn(async move {
            if let Err(e) = Self::painter_handler(painter_rx).await {
                eprintln!("Erro no paint handler... {e}")
            }
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
                    disable_raw_mode().context("Falha ao desabilitar raw mode")?;
                    stdout.execute(LeaveAlternateScreen)?;
                    stdout.execute(DisableMouseCapture)?;
                    exit(0);
                }
            }

            painter_tx
                .send(Painting::Points(self.state()))
                .await
                .context("Falha ao enviar estado atual para pintar")?;
        }

        Ok(())
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
