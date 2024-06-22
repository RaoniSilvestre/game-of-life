use crate::game::elements::Point;
use async_trait::async_trait;

pub struct Client {
    pub p: Point,
}

#[async_trait]
impl ezsockets::ClientExt for Client {
    type Call = ();

    async fn on_text(&mut self, text: String) -> Result<(), ezsockets::Error> {
        tracing::info!("Mensagem recebida: {text}");
        tracing::info!("{:?}", self.p);
        Ok(())
    }

    async fn on_binary(&mut self, bytes: Vec<u8>) -> Result<(), ezsockets::Error> {
        let point: Point = From::from(&bytes[..]);

        tracing::info!("{:?}", point);
        Ok(())
    }

    async fn on_call(&mut self, _call: Self::Call) -> Result<(), ezsockets::Error> {
        Ok(())
    }
}
