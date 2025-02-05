use std::time::Duration;

use crossterm::event::{
    poll, read, Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind,
};
use tokio::sync::mpsc;

use crate::conway::Point;

use super::{Runner, RunnerEvent};

impl Runner {
    pub async fn event_listener(tx: mpsc::Sender<RunnerEvent>) {
        loop {
            if poll(Duration::from_micros(10)).unwrap() {
                Self::read_terminal_input(&tx).await;
            }
        }
    }

    async fn read_terminal_input(tx: &mpsc::Sender<RunnerEvent>) {
        match read().expect("Erro ao ler input do terminal") {
            Event::Mouse(e) => Self::mouse_event_handler(tx, e).await,
            Event::Key(e) => Self::keyboard_event_handler(tx, e).await,
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => todo!(),
        }
    }

    async fn mouse_event_handler(tx: &mpsc::Sender<RunnerEvent>, event: MouseEvent) {
        match event.kind {
            MouseEventKind::Down(mouse_button) => {
                let point = Point::new(event.row.into(), event.column.into());

                match mouse_button {
                    MouseButton::Left | MouseButton::Middle => {
                        let runner_event = RunnerEvent::Revive(point);

                        let _ = tx.send(runner_event).await;
                    }
                    MouseButton::Right => {
                        let runner_event = RunnerEvent::Kill(point);

                        let _ = tx.send(runner_event).await;
                    }
                }
            }
            MouseEventKind::Drag(mouse_button) => {
                let point = Point::new(event.row.into(), event.column.into());

                match mouse_button {
                    MouseButton::Left | MouseButton::Middle => {
                        let runner_event = RunnerEvent::Revive(point);

                        tx.send(runner_event).await.unwrap();
                    }
                    MouseButton::Right => {
                        let runner_event = RunnerEvent::Kill(point);

                        tx.send(runner_event).await.unwrap();
                    }
                }
            }
            _ => {}
        }
    }

    async fn keyboard_event_handler(tx: &mpsc::Sender<RunnerEvent>, event: KeyEvent) {
        match event.code {
            KeyCode::Enter => tx.send(RunnerEvent::ToggleRun).await.unwrap(),
            KeyCode::Char('q') => tx.send(RunnerEvent::Quit).await.unwrap(),
            _ => {}
        }
    }
}
