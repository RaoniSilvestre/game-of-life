use std::{
    io::{self},
    process::exit,
};

use anyhow::{Context, Ok};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tokio::sync::mpsc::channel;

use crate::{
    conway::{Cell, State},
    Res,
};

use super::{
    EventListener, PaintHandler, Runner, RunnerChannels, RunnerEvent, RunnerStructs, TickWaiter,
};

pub enum Painting {
    Points(Vec<Cell>),
}

impl Runner {
    pub async fn run(&mut self) -> Res<()> {
        let RunnerStructs {
            event_listener,
            tick_waiter,
            mut painter,
            mut channels,
        } = self.get_structs()?;

        enable_raw_mode().context("Falha ao habilitar raw mode")?;

        let mut stdout = io::stdout();

        stdout.execute(EnterAlternateScreen)?;
        stdout.execute(EnableMouseCapture)?;

        tokio::spawn(async move {
            if let Err(e) = event_listener.listen().await {
                eprintln!("Erro no event listener... {e}")
            }
        });

        tokio::spawn(async move {
            tick_waiter.tick().await;
        });

        tokio::spawn(async move {
            if let Err(e) = painter.paint().await {
                eprintln!("Erro no paint handler... {e}")
            }
        });

        self.start();

        while let Some(event) = channels.event_rx.recv().await {
            match event {
                RunnerEvent::Tick if self.stop => (),
                RunnerEvent::Tick => self.update(),
                RunnerEvent::Revive(p) => self.add_cell(Cell::new(p, State::Alive)),
                RunnerEvent::Kill(p) => self.add_cell(Cell::new(p, State::Dead)),
                RunnerEvent::ToggleRun => self.stop = !self.stop,
                RunnerEvent::Quit => {
                    disable_raw_mode().context("Falha ao desabilitar raw mode")?;
                    stdout.execute(LeaveAlternateScreen)?;
                    stdout.execute(DisableMouseCapture)?;
                    exit(0);
                }
            }

            channels
                .painter_tx
                .send(Painting::Points(self.state()))
                .await
                .context("Falha ao enviar estado atual para pintar")?;
        }

        Ok(())
    }

    fn get_structs(&self) -> Res<RunnerStructs> {
        let (event_tx, event_rx) = channel::<RunnerEvent>(32);
        let (painter_tx, painter_rx) = channel::<Painting>(32);
        let tx_input = event_tx.clone();
        let tx_tick = event_tx.clone();
        let tick = self.config.fps;

        let event_listener = EventListener::new(tx_input);
        let painter = PaintHandler::new(painter_rx)?;
        let tick_waiter = TickWaiter::new(tx_tick, tick);

        let channels = RunnerChannels {
            painter_tx,
            event_rx,
        };

        Ok(RunnerStructs {
            event_listener,
            tick_waiter,
            painter,
            channels,
        })
    }
}
