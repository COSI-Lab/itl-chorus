use actix::prelude::*;
use actix_web_actors::ws;

use super::{ChatMessage, Join, Leave, Room};

pub struct Client {
    name: uuid::Uuid,
    addr: Addr<Room>, // The room actor
}

impl Client {
    pub fn new(addr: Addr<Room>) -> Self {
        Self {
            name: uuid::Uuid::new_v4(),
            addr,
        }
    }

    pub fn name(&self) -> uuid::Uuid {
        self.name
    }
}

impl Actor for Client {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Add the client to the room
        self.addr.do_send(Join {
            name: self.name.to_string(),
            addr: ctx.address(),
        });
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        // Remove the client from the room
        self.addr.do_send(Leave {
            name: self.name.to_string(),
            addr: ctx.address(),
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Client {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            // Ping handler
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            // Close handler
            Ok(ws::Message::Close(reason)) => {
                log::debug!("Closing websocket for ({:?}) {}", reason, self.name);
                ctx.stop();
            }
            // Text message handler
            Ok(ws::Message::Text(text)) => {
                if let Ok(msg) = serde_json::from_str::<common::UploadMessage>(&text) {
                    self.addr.do_send(ChatMessage(common::Message {
                        name: self.name.to_string(),
                        msg: msg.msg,
                    }))
                }
            }
            _ => (), // Ignore byte messages and errors
        }
    }
}

impl Handler<ChatMessage> for Client {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        log::debug!("Sending message to ({}) {}", self.name, msg.msg);

        // Send the message to the client
        ctx.text(serde_json::to_string(&msg.0).unwrap());
    }
}
