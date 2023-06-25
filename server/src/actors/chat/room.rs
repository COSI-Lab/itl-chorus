use std::sync::Arc;

use actix::prelude::*;
use common::server_to_client;

use crate::{
    actors::{Envelope, EnvelopeExt},
    names::Names,
};

use super::{ChatClient, ClientJoin, ClientLeave, ClientMessage};

// The room actor is responsible for managing the room.
//
// Rooms have a host and a list of clients.
pub struct ChatRoom {
    addr: Option<Addr<Self>>,
    clients: Vec<Addr<ChatClient>>,
    names: Names<Addr<ChatClient>>,

    // Stores a history of sent messages.
    messages: Vec<Arc<str>>,
}

impl Actor for ChatRoom {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr = Some(ctx.address());
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {}
}

impl EnvelopeExt for ChatRoom {
    fn envelope<T>(&self, msg: T) -> Envelope<T, Self> {
        Envelope {
            inner: msg,
            addr: self.addr.clone().unwrap(),
        }
    }
}

impl ChatRoom {
    pub fn new() -> Self {
        Self {
            clients: Vec::new(),
            names: Names::new(),
            addr: None,
            messages: Vec::new(),
        }
    }

    pub fn broadcast(&mut self, msg: server_to_client::Chat) {
        let wrapped: Arc<str> = Arc::from(serde_json::to_string(&msg).unwrap());

        self.messages.push(wrapped.clone());
        for client in &self.clients {
            client.do_send(self.envelope(wrapped.clone()));
        }
    }
}

impl Handler<ClientMessage> for ChatRoom {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.broadcast(server_to_client::Chat::Message {
            name: *self.names.get(&msg.addr).unwrap(),
            msg: msg.inner.msg,
        });
    }
}

impl Handler<ClientJoin> for ChatRoom {
    type Result = ();

    fn handle(&mut self, msg: ClientJoin, _ctx: &mut Self::Context) -> Self::Result {
        // TODO: Handle case where we run out of names
        let name = self.names.generate().unwrap();

        let addr = msg.addr;
        self.names.insert(addr.clone(), name);
        self.clients.push(addr.clone());

        // Send the client the history of messages
        for msg in &self.messages {
            addr.do_send(self.envelope(msg.clone()));
        }

        self.broadcast(server_to_client::Chat::Joined { name });
    }
}

impl Handler<ClientLeave> for ChatRoom {
    type Result = ();

    fn handle(&mut self, msg: ClientLeave, _ctx: &mut Self::Context) -> Self::Result {
        self.clients.retain(|client| client != &msg.addr);
        let name = self.names.remove(&msg.addr).unwrap();

        self.broadcast(server_to_client::Chat::Left { name });
    }
}
