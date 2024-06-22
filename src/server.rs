use async_trait::async_trait;

// Create our own session that implements `SessionExt`

type SessionID = u16;
type Session = ezsockets::Session<SessionID, Message>;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct EchoSession {
    handle: Session,
    id: SessionID,
    counter: usize,
}

#[async_trait]
impl ezsockets::SessionExt for EchoSession {
    type ID = SessionID;
    type Call = Message;

    fn id(&self) -> &Self::ID {
        &self.id
    }

    async fn on_text(&mut self, text: String) -> Result<(), ezsockets::Error> {
        let _ = self.handle.text(text.clone()); // Send response to the client
        let id = self.handle.id;

        tracing::info!("Message from {id}: {text:?}");
        Ok(())
    }

    async fn on_binary(&mut self, bytes: Vec<u8>) -> Result<(), ezsockets::Error> {
        let point: Point = From::from(&bytes[..]);
        println!("{:?}", point);
        Ok(())
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), ezsockets::Error> {
        match call {
            Message::Increment => {
                self.counter += 1;
                info!("{} : {}", self.id, self.counter)
            }
            Message::Share => {
                let _ = self
                    .handle
                    .binary(Point {
                        x: self.counter,
                        y: self.counter,
                    })
                    .unwrap();
            }
        }
        Ok(())
    }
}

// Create our own server that implements `ServerExt`

use ezsockets::Server;
use std::{net::SocketAddr, time::Duration};
use tracing::info;
use tracing_subscriber::fmt::format;

#[derive(Debug)]
enum Message {
    Increment,
    Share,
}

struct EchoServer {}

#[async_trait]
impl ezsockets::ServerExt for EchoServer {
    type Session = EchoSession;
    type Call = ();

    async fn on_connect(
        &mut self,
        socket: ezsockets::Socket,
        _request: ezsockets::Request,
        address: SocketAddr,
    ) -> Result<Session, Option<ezsockets::CloseFrame>> {
        let id = address.port();
        let session = Session::create(
            |handle| {
                tokio::spawn({
                    let session = handle.clone();
                    async move {
                        loop {
                            session.call(Message::Increment).unwrap();
                            session.call(Message::Share).unwrap();
                            // info!("Adding counter to {}", session.id);
                            tokio::time::sleep(Duration::from_secs(3)).await;
                        }
                    }
                });
                EchoSession {
                    id,
                    handle,
                    counter: 0,
                }
            },
            id,
            socket,
        );
        Ok(session)
    }

    async fn on_disconnect(
        &mut self,
        _id: <Self::Session as ezsockets::SessionExt>::ID,
        _reason: Result<Option<ezsockets::CloseFrame>, ezsockets::Error>,
    ) -> Result<(), ezsockets::Error> {
        Ok(())
    }

    async fn on_call(&mut self, call: Self::Call) -> Result<(), ezsockets::Error> {
        let () = call;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let (server, _) = Server::create(|_handle| EchoServer {});

    ezsockets::tungstenite::run(server, "127.0.0.1:8080")
        .await
        .unwrap();
}
