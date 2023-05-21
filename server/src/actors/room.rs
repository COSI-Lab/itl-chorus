use std::sync::Arc;

use actix::prelude::*;
use common::server_to_client::{ChatMessage, ServerToClientWS};

use crate::names::Names;

use super::{Client, Envelope, EnvelopeExt, GetRoomInfo, Join, Leave, Message};

// The room actor is responsible for managing the room.
//
// Rooms have a host and a list of clients.

pub struct Room {
    addr: Option<Addr<Self>>,
    uuid: uuid::Uuid,
    clients: Vec<Addr<Client>>,
    names: Names<Addr<Client>>,
}

impl Actor for Room {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::debug!("Room {} started", self.uuid);
        self.addr = Some(ctx.address());
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::debug!("Room {} stopped", self.uuid);
    }
}

impl EnvelopeExt for Room {
    fn envelope<T>(&self, msg: T) -> super::Envelope<T, Self> {
        Envelope {
            inner: msg,
            addr: self.addr.clone().unwrap(),
        }
    }
}

impl Room {
    pub fn new() -> Self {
        Self {
            uuid: uuid::Uuid::new_v4(),
            clients: Vec::new(),
            names: Names::new(),
            addr: None,
        }
    }

    pub fn uuid(&self) -> uuid::Uuid {
        self.uuid
    }

    pub fn broadcast(&self, msg: ServerToClientWS) {
        let parsed = Arc::new(serde_json::to_string(&msg).unwrap());

        for client in &self.clients {
            client.do_send(self.envelope(parsed.clone()));
        }
    }
}

impl Handler<Message> for Room {
    type Result = ();

    fn handle(&mut self, msg: Message, _ctx: &mut Self::Context) -> Self::Result {
        log::debug!("Received message from client {:?}", msg.addr);

        self.broadcast(ServerToClientWS::Chat(ChatMessage {
            name: *self.names.get(&msg.addr).unwrap(),
            msg: msg.inner.msg,
        }));
    }
}

impl Handler<Join> for Room {
    type Result = ();

    fn handle(&mut self, msg: Join, _ctx: &mut Self::Context) -> Self::Result {
        let name = self.names.generate().unwrap(); // TODO: Error
        self.names.insert(msg.addr.clone(), name);
        self.clients.push(msg.addr);
    }
}

impl Handler<Leave> for Room {
    type Result = ();

    fn handle(&mut self, msg: Leave, _ctx: &mut Self::Context) -> Self::Result {
        self.clients.retain(|client| client != &msg.addr);
        self.names.remove(&msg.addr);
    }
}

impl Handler<GetRoomInfo> for Room {
    type Result = Result<common::RoomInfo, ()>;

    fn handle(&mut self, _msg: GetRoomInfo, _ctx: &mut Self::Context) -> Self::Result {
        Ok(common::RoomInfo {
            id: self.uuid,
            clients: self.clients.len(),
        })
    }
}
