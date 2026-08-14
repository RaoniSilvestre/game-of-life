use std::{sync::mpsc::SyncSender, thread, time::Duration};

use crossterm::event::{read, Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};

use crate::{runner::RunnerEvent, Res};

pub fn listen(tx: SyncSender<RunnerEvent>, tick_rate: u64) -> Res<()> {
    let tick_tx = tx.clone();

    thread::spawn(move || {
        tick(tick_tx, tick_rate);
    });

    loop {
        match read()? {
            Event::Mouse(e) => mouse_event_handler(&tx, e),
            Event::Key(e) => keyboard_event_handler(&tx, e),
            _ => Ok(()),
        }?
    }
}

fn mouse_event_handler(tx: &SyncSender<RunnerEvent>, event: MouseEvent) -> Res<()> {
    match event.kind {
        MouseEventKind::Down(mouse_button) => {
            let point = (event.row.into(), event.column.into());

            match mouse_button {
                MouseButton::Left | MouseButton::Middle => {
                    let runner_event = RunnerEvent::Revive(point);
                    tx.send(runner_event)?;
                    Ok(())
                }
                MouseButton::Right => {
                    let runner_event = RunnerEvent::AddAround(point);
                    tx.send(runner_event)?;
                    Ok(())
                }
            }
        }
        MouseEventKind::Drag(mouse_button) => {
            let point = (event.row.into(), event.column.into());

            match mouse_button {
                MouseButton::Left | MouseButton::Middle => {
                    let runner_event = RunnerEvent::Revive(point);
                    tx.send(runner_event)?;
                    Ok(())
                }
                MouseButton::Right => {
                    let runner_event = RunnerEvent::AddAround(point);
                    tx.send(runner_event)?;
                    Ok(())
                }
            }
        }
        _ => Ok(()),
    }
}

fn keyboard_event_handler(tx: &SyncSender<RunnerEvent>, event: KeyEvent) -> Res<()> {
    match event.code {
        KeyCode::Enter => Ok(tx.send(RunnerEvent::ToggleRun)?),
        KeyCode::Char('q') => Ok(tx.send(RunnerEvent::Quit)?),
        _ => Ok(()),
    }
}

pub fn tick(tx: SyncSender<RunnerEvent>, tick: u64) {
    loop {
        thread::sleep(Duration::from_micros(1000000 / tick));
        let _ = tx.send(RunnerEvent::Tick);
    }
}
