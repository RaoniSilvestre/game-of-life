use std::{io::stdout, sync::mpsc::sync_channel, thread};

use crossterm::{event::EnableMouseCapture, execute};

use crate::{
    automaton::{Automaton, Conway, Point},
    listener::{self},
    view::{self, Snapshot},
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
pub struct Runner<A: Automaton> {
    automaton: A,
    config: Configuration,
    stop: bool,
}

impl<A: Automaton> Runner<A> {
    pub fn new(config: Configuration, scene: A::Scene) -> Self {
        let automaton = A::from(scene);

        Runner {
            automaton,
            config,
            stop: true,
        }
    }

    pub fn run(&mut self) -> Res<()> {
        let (event_tx, event_rx) = sync_channel::<RunnerEvent>(32);
        let (painter_tx, painter_rx) = sync_channel::<Snapshot<A::Cell>>(32);

        let tick_rate = self.config.fps;

        let mut terminal = ratatui::init();
        execute!(stdout(), EnableMouseCapture)?;
        terminal.clear()?;

        // Input thread
        thread::spawn(move || {
            if let Err(e) = listener::listen(event_tx, tick_rate) {
                eprintln!("Erro no event listener... {e}");
                view::quit();
            }
        });

        // Output thread
        thread::spawn(|| {
            if let Err(e) = view::listen::<A>(terminal, painter_rx) {
                eprintln!("Erro no view listener... {e}");
                view::quit();
            }
        });

        // Processing thread
        loop {
            let event = event_rx.recv()?;

            match event {
                RunnerEvent::Tick if self.stop => (),
                RunnerEvent::Tick => self.automaton.step(),
                RunnerEvent::Revive(p) => self.automaton.paint_left(p),
                RunnerEvent::AddAround(p) => self.automaton.paint_right(p),
                RunnerEvent::ToggleRun => self.stop = !self.stop,
                RunnerEvent::Quit => view::quit(),
            }

            painter_tx.send(Snapshot {
                name: Conway::name().to_string(),
                iterations: self.automaton.iterations(),
                board: self.automaton.board().clone(),
            })?;
        }
    }

    pub fn tick(&self) -> u64 {
        self.config.fps
    }
}
