use actix::prelude::*;
use actix_web_actors::ws;
use common::client_to_server::ClientToServerWS;

use super::{Broadcast, Envelope, EnvelopeExt, JoinInner, LeaveInner, Room};

pub struct Client {
    room: Addr<Room>, // The room actor
    addr: Option<Addr<Self>>,
}

impl EnvelopeExt for Client {
    fn envelope<T>(&self, msg: T) -> super::Envelope<T, Self> {
        Envelope {
            inner: msg,
            addr: self.addr.clone().unwrap(),
        }
    }
}

impl Client {
    pub fn new(room: Addr<Room>) -> Self {
        Self { room, addr: None }
    }

    fn handle_client_message(&mut self, msg: ClientToServerWS) {
        match msg {
            ClientToServerWS::Upload(msg) => {
                log::debug!("Received message from ({:?}) {}", self.addr, msg.msg);
                self.room.do_send(self.envelope(msg));
            }
        }
    }
}

impl Actor for Client {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr = Some(ctx.address());

        self.room.do_send(self.envelope(JoinInner {}))
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        self.room.do_send(self.envelope(LeaveInner {}))
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Client {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            // Ping handler
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            // Close handler
            Ok(ws::Message::Close(reason)) => {
                log::debug!("Closing {:?} : {:?}", self.addr, reason);
                ctx.stop();
            }
            // Text message handler
            Ok(ws::Message::Text(text)) => match serde_json::from_str::<ClientToServerWS>(&text) {
                Ok(msg) => self.handle_client_message(msg),
                Err(err) => {
                    log::warn!("Error parsing message: {}", err);
                }
            },
            _ => (), // Ignore byte messages and errors
        }
    }
}

impl Handler<Broadcast> for Client {
    type Result = ();

    fn handle(&mut self, msg: Broadcast, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.inner.as_ref().as_str())
    }
}
