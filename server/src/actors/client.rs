use actix::prelude::*;
use actix_web_actors::ws;

use super::{room::Room, Join, Leave, Message};

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
                // Send the message to the room
                self.addr.do_send(Message {
                    name: self.name.to_string(),
                    msg: text.to_string(),
                });
            }
            _ => (), // Ignore byte messages and errors
        }
    }
}

impl Handler<Message> for Client {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        // Send the message to the client
        ctx.text(format!("{}: {}", msg.name, msg.msg,));
    }
}
