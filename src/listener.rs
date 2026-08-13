use std::{sync::mpsc::SyncSender, thread, time::Duration};

use crossterm::event::{read, Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};

use crate::{runner::RunnerEvent, Res};

#[derive(Debug)]
pub struct EventListener(SyncSender<RunnerEvent>);

#[derive(Debug)]
pub struct Tick {
    tx: SyncSender<RunnerEvent>,
    tick: u64,
}

impl Tick {
    pub fn new(tx: SyncSender<RunnerEvent>, tick: u64) -> Self {
        Self { tx, tick }
    }

    pub fn tick(&self) {
        loop {
            thread::sleep(Duration::from_micros(1000000 / self.tick));
            let _ = self.tx.send(RunnerEvent::Tick);
        }
    }
}

impl EventListener {
    pub fn new(tx: SyncSender<RunnerEvent>) -> Self {
        EventListener(tx)
    }

    pub fn listen(&self) -> Res<()> {
        loop {
            match read()? {
                Event::Mouse(e) => self.mouse_event_handler(e),
                Event::Key(e) => self.keyboard_event_handler(e),
                _ => Ok(()),
            }?
        }
    }

    fn mouse_event_handler(&self, event: MouseEvent) -> Res<()> {
        let tx = &self.0;

        match event.kind {
            MouseEventKind::Down(mouse_button) | MouseEventKind::Drag(mouse_button) => {
                let point = (event.row.into(), event.column.into());

                let runner_event = match mouse_button {
                    MouseButton::Left | MouseButton::Middle => RunnerEvent::PaintLeft(point),
                    MouseButton::Right => RunnerEvent::PaintRight(point),
                };

                tx.send(runner_event)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn keyboard_event_handler(&self, event: KeyEvent) -> Res<()> {
        let tx = &self.0;

        match event.code {
            KeyCode::Enter => Ok(tx.send(RunnerEvent::ToggleRun)?),
            KeyCode::Char('q') => Ok(tx.send(RunnerEvent::Quit)?),
            _ => Ok(()),
        }
    }
}
