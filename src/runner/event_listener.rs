use std::time::Duration;

use crossterm::event::{
    poll, read, Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind,
};
use tokio::sync::mpsc::Sender;

use crate::{conway::Point, Res};

use super::{EventListener, RunnerEvent};

impl EventListener {
    pub fn new(tx: Sender<RunnerEvent>) -> Self {
        EventListener(tx)
    }

    pub async fn listen(&self) -> Res<()> {
        loop {
            if poll(Duration::from_micros(10))? {
                self.read_terminal_input().await?;
            }
        }
    }

    async fn read_terminal_input(&self) -> Res<()> {
        match read()? {
            Event::Mouse(e) => self.mouse_event_handler(e).await,
            Event::Key(e) => self.keyboard_event_handler(e).await,
            _ => Ok(()),
        }
    }

    async fn mouse_event_handler(&self, event: MouseEvent) -> Res<()> {
        let tx = &self.0;

        match event.kind {
            MouseEventKind::Down(mouse_button) => {
                let point = Point::new(event.row.into(), event.column.into());

                match mouse_button {
                    MouseButton::Left | MouseButton::Middle => {
                        let runner_event = RunnerEvent::Revive(point);
                        tx.send(runner_event).await?;
                        Ok(())
                    }
                    MouseButton::Right => {
                        let runner_event = RunnerEvent::Kill(point);
                        tx.send(runner_event).await?;
                        Ok(())
                    }
                }
            }
            MouseEventKind::Drag(mouse_button) => {
                let point = Point::new(event.row.into(), event.column.into());

                match mouse_button {
                    MouseButton::Left | MouseButton::Middle => {
                        let runner_event = RunnerEvent::Revive(point);
                        tx.send(runner_event).await?;
                        Ok(())
                    }
                    MouseButton::Right => {
                        let runner_event = RunnerEvent::Kill(point);
                        tx.send(runner_event).await?;
                        Ok(())
                    }
                }
            }
            _ => Ok(()),
        }
    }

    async fn keyboard_event_handler(&self, event: KeyEvent) -> Res<()> {
        let tx = &self.0;

        match event.code {
            KeyCode::Enter => Ok(tx.send(RunnerEvent::ToggleRun).await?),
            KeyCode::Char('q') => Ok(tx.send(RunnerEvent::Quit).await?),
            _ => Ok(()),
        }
    }
}
