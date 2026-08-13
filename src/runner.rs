use crate::{
    Configuration, Res,
    automaton::{Automaton, Conway, Point, Snapshot},
    listener::{EventListener, Tick},
    view::BasicPainter,
};
use anyhow::{Context, anyhow, bail};
use crossterm::{
    ExecutableCommand,
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::Color;
use std::{
    io::{self},
    process::exit,
    sync::mpsc::{Receiver, SyncSender, sync_channel},
    thread,
};

pub fn run_with_scene(config: Configuration) -> Res<()> {
    match config.scene.automaton.as_str() {
        "conway" => Runner::<Conway>::new(config)?.run(),
        other => bail!("autômato desconhecido: {other}"),
    }
}

#[derive(Debug)]
pub struct Runner<A: Automaton> {
    automaton: A,
    config: Configuration,
    stop: bool,
}

#[derive(Debug)]
pub struct PaintHandler<C> {
    rx: Receiver<Snapshot<C>>,
    painter: BasicPainter,
}

#[derive(Debug)]
pub struct RunnerChannels<C> {
    event_rx: Receiver<RunnerEvent>,
    painter_tx: SyncSender<Snapshot<C>>,
}

#[derive(Debug)]
pub enum RunnerEvent {
    Tick,
    PaintLeft(Point),
    PaintRight(Point),
    ToggleRun,
    Quit,
}

struct RunnerStructs<C> {
    event_listener: EventListener,
    tick_waiter: Tick,
    painter: PaintHandler<C>,
    channels: RunnerChannels<C>,
}

impl<A: Automaton> Runner<A> {
    pub fn new(config: Configuration) -> Res<Self> {
        let scene = &config.scene;
        let automaton = A::from_rows(scene.width, scene.height, &scene.initial)?;

        Ok(Self {
            automaton,
            config,
            stop: true,
        })
    }

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
                RunnerEvent::Tick => self.automaton.step(),
                RunnerEvent::PaintLeft(point) => {
                    let (x, y) = self.screen_to_grid(point);
                    self.automaton.paint_left(x, y);
                }
                RunnerEvent::PaintRight(point) => {
                    let (x, y) = self.screen_to_grid(point);
                    self.automaton.paint_right(x, y);
                }
                RunnerEvent::ToggleRun => self.stop = !self.stop,
                RunnerEvent::Quit => {
                    disable_raw_mode().context("Falha ao desabilitar raw mode")?;
                    stdout.execute(LeaveAlternateScreen)?;
                    stdout.execute(DisableMouseCapture)?;
                    exit(0);
                }
            }

            let snapshot = Snapshot {
                name: A::name().to_string(),
                iterations: self.automaton.iterations(),
                board: self.automaton.board().clone(),
            };

            channels
                .painter_tx
                .send(snapshot)
                .map_err(|e| anyhow!("Falha ao enviar snapshot: {e}"))?;
        }
    }

    fn screen_to_grid(&self, (row, col): Point) -> Point {
        const BORDER: i32 = 1;
        (row - BORDER, col - BORDER)
    }

    fn get_structs(&self) -> Res<RunnerStructs<A::Cell>> {
        let (event_tx, event_rx) = sync_channel::<RunnerEvent>(32);
        let (painter_tx, painter_rx) = sync_channel::<Snapshot<A::Cell>>(32);
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

impl<C: Into<Color> + Copy + PartialEq + Default> PaintHandler<C> {
    pub fn new(rx: Receiver<Snapshot<C>>) -> Res<Self> {
        let painter = BasicPainter::new()?;
        Ok(Self { rx, painter })
    }

    pub fn paint(&mut self) -> Res<()> {
        loop {
            let snapshot = self.rx.recv()?;
            self.painter.paint(&snapshot)?
        }
    }
}
