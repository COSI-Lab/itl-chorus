use actix::prelude::*;
use actix_web_actors::ws;
use common::client_to_server;

use crate::actors::{Envelope, EnvelopeExt};

use super::{Broadcast, ChatRoom, JoinInner, LeaveInner};

pub struct ChatClient {
    room: Addr<ChatRoom>, // The room actor
    addr: Option<Addr<Self>>,
}

impl EnvelopeExt for ChatClient {
    fn envelope<T>(&self, msg: T) -> Envelope<T, Self> {
        Envelope {
            inner: msg,
            addr: self.addr.clone().unwrap(),
        }
    }
}

impl ChatClient {
    pub fn new(room: Addr<ChatRoom>) -> Self {
        Self { room, addr: None }
    }

    fn handle_client_message(&mut self, msg: client_to_server::Chat) {
        self.room.do_send(self.envelope(msg));
    }
}

impl Actor for ChatClient {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr = Some(ctx.address());
        self.room.do_send(self.envelope(JoinInner {}))
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        self.room.do_send(self.envelope(LeaveInner {}))
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatClient {
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
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<client_to_server::Chat>(&text) {
                    Ok(msg) => self.handle_client_message(msg),
                    Err(err) => {
                        log::warn!("Error parsing message: {}", err);
                    }
                }
            }
            _ => (), // Ignore byte messages and errors
        }
    }
}

impl Handler<Broadcast> for ChatClient {
    type Result = ();

    fn handle(&mut self, msg: Broadcast, ctx: &mut Self::Context) -> Self::Result {
        // TODO: Use a binary protocol instead of JSON
        ctx.text(msg.inner.as_ref())
    }
}
