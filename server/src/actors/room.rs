use actix::prelude::*;

use crate::actors::IntoMessage;

use super::{ChatMessage, Client, GetRoomInfo, Join, Leave, RoomInfo};

// The room actor is responsible for managing the room.
//
// Rooms have a host and a list of clients.

pub struct Room {
    name: uuid::Uuid,
    clients: Vec<Addr<Client>>,
}

impl Actor for Room {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::debug!("Room {} started", self.name);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::debug!("Room {} stopped", self.name);
    }
}

impl Room {
    pub fn new() -> Self {
        Self {
            name: uuid::Uuid::new_v4(),
            clients: Vec::new(),
        }
    }

    pub fn name(&self) -> uuid::Uuid {
        self.name
    }

    pub fn send_message(&self, msg: ChatMessage) {
        for client in self.clients.iter() {
            client.do_send(msg.clone());
        }
    }
}

impl Handler<GetRoomInfo> for Room {
    type Result = Result<RoomInfo, ()>;

    fn handle(&mut self, _msg: GetRoomInfo, _ctx: &mut Self::Context) -> Self::Result {
        log::debug!("Getting info for room {}", self.name);
        Ok(RoomInfo { id: self.name })
    }
}

impl Handler<Join> for Room {
    type Result = ();

    fn handle(&mut self, msg: Join, _ctx: &mut Self::Context) -> Self::Result {
        log::debug!("{} joined room {}", msg.name, self.name);
        self.clients.push(msg.addr);

        self.send_message(
            common::Message {
                name: "Server".to_string(),
                msg: format!("{} joined the room", msg.name),
            }
            .into_message(),
        );
    }
}

impl Handler<Leave> for Room {
    type Result = ();

    fn handle(&mut self, msg: Leave, _ctx: &mut Self::Context) -> Self::Result {
        log::debug!("{} left room {}", msg.name, self.name);
        self.clients
            .iter()
            .position(|c| c == &msg.addr)
            .map(|i| self.clients.remove(i));

        self.send_message(
            common::Message {
                name: "Server".to_string(),
                msg: format!("{} left the room", msg.name),
            }
            .into_message(),
        );
    }
}

impl Handler<ChatMessage> for Room {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, _ctx: &mut Self::Context) -> Self::Result {
        log::debug!("{} sent message to room {}", msg.name, self.name);
        self.send_message(msg);
    }
}
