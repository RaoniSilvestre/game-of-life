use std::time::Duration;

use tokio::{
    sync::mpsc::Sender,
    time::{self},
};

use super::{RunnerEvent, TickWaiter};

impl TickWaiter {
    pub fn new(tx: Sender<RunnerEvent>, tick: u64) -> Self {
        Self { tx, tick }
    }

    pub async fn tick(&self) {
        let mut interval = time::interval(Duration::from_micros(1000000 / self.tick));

        loop {
            interval.tick().await;
            let _ = self.tx.send(RunnerEvent::Tick).await;
        }
    }
}
