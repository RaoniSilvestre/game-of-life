use std::{
    io::{self, stdout},
    time::Duration,
};

use crossterm::{
    event::{self, read, EnableMouseCapture, Event, MouseButton, MouseEventKind},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};
use tokio::{sync::mpsc, time};
use tracing::{debug, error};

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
                RunnerEvent::Input((x, y)) => {
                    debug!("x: {x} | y: {y}");
                    self.update();
                }
                RunnerEvent::Tick => {
                    self.update();
                }
            }
        }
    }

    async fn event_listener(tx: mpsc::Sender<RunnerEvent>) {
        loop {
            if event::poll(Duration::from_millis(100)).unwrap() {
                match read().expect("Erro ao ler input do terminal") {
                    Event::Mouse(e) => {
                        if let MouseEventKind::Down(mdown) = e.kind {
                            debug!("MDOWN");
                            let send_event = tx.send(RunnerEvent::Input((e.row, e.column))).await;

                            if let Err(err) = send_event {
                                error!("Erro ao enviar: {}", err);
                            }
                        }
                    }
                    Event::FocusGained => todo!(),
                    Event::FocusLost => todo!(),
                    Event::Key(_) => todo!(),
                    Event::Paste(_) => todo!(),
                    Event::Resize(_, _) => todo!(),
                }
            }
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
