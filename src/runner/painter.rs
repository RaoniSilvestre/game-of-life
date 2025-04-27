use tokio::sync::mpsc::Receiver;

use crate::{
    view::{BasicPainter, Paint},
    Res,
};

use super::{run::Painting, PaintHandler};

impl PaintHandler {
    pub fn new(rx: Receiver<Painting>) -> Res<Self> {
        let painter = BasicPainter::new()?;
        Ok(Self { rx, painter })
    }

    pub async fn paint(&mut self) -> Res<()> {
        while let Some(paint) = self.rx.recv().await {
            match paint {
                Painting::Points(cells) => {
                    self.painter.paint(&cells)?;
                }
            }
        }

        Ok(())
    }
}
