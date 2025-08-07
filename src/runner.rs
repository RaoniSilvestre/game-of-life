use std::{
    io::{self},
    process::exit,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    thread,
};

use anyhow::{Context, Ok};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use crate::{
    game::{ConwayGame, GameState, Point},
    listener::{EventListener, Tick},
    view::{BasicPainter, Paint},
    Configuration, Res,
};

#[derive(Debug)]
pub struct Runner {
    game: ConwayGame,
    config: Configuration,
    stop: bool,
}

#[derive(Debug)]
pub struct PaintHandler {
    rx: Receiver<GameState>,
    painter: BasicPainter,
}

#[derive(Debug)]
pub struct RunnerChannels {
    event_rx: Receiver<RunnerEvent>,
    painter_tx: SyncSender<GameState>,
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
    tick_waiter: Tick,
    painter: PaintHandler,
    channels: RunnerChannels,
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

    pub fn tick(&self) -> u64 {
        self.config.fps
    }
}

impl Runner {
    pub fn run(&mut self) -> Res<()> {
        let RunnerStructs {
            event_listener,
            tick_waiter,
            mut painter,
            channels,
        } = self.get_structs()?;

        enable_raw_mode().context("Falha ao habilitar raw mode")?;

        let mut stdout = io::stdout();

        stdout.execute(EnterAlternateScreen)?;
        stdout.execute(EnableMouseCapture)?;

        thread::spawn(move || {
            if let Err(e) = event_listener.listen() {
                eprintln!("Erro no event listener... {e}")
            }
        });

        thread::spawn(move || {
            tick_waiter.tick();
        });

        thread::spawn(move || {
            if let Err(e) = painter.paint() {
                eprintln!("Erro no paint handler... {e}")
            }
        });

        loop {
            let event = channels.event_rx.recv()?;

            match event {
                RunnerEvent::Tick if self.stop => (),
                RunnerEvent::Tick => self.game.update(),
                RunnerEvent::Revive(p) => self.game.add(p),
                RunnerEvent::Kill(p) => self.game.remove(p),
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
                .send(self.game.state())
                .context("Falha ao enviar estado atual para pintar")?;
        }
    }

    fn get_structs(&self) -> Res<RunnerStructs> {
        let (event_tx, event_rx) = sync_channel::<RunnerEvent>(32);
        let (painter_tx, painter_rx) = sync_channel::<GameState>(32);
        let tx_input = event_tx.clone();
        let tx_tick = event_tx.clone();
        let tick = self.config.fps;

        let event_listener = EventListener::new(tx_input);
        let painter = PaintHandler::new(painter_rx)?;
        let tick_waiter = Tick::new(tx_tick, tick);

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

impl PaintHandler {
    pub fn new(rx: Receiver<GameState>) -> Res<Self> {
        let painter = BasicPainter::new()?;
        Ok(Self { rx, painter })
    }

    pub fn paint(&mut self) -> Res<()> {
        loop {
            let cells = self.rx.recv()?;
            self.painter.paint(cells)?
        }
    }
}
