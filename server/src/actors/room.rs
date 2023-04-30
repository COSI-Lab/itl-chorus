use actix::prelude::*;

use super::{Join, Leave, Message, Client};

// The room actor is responsible for managing the room.
//
// Rooms have a host and a list of clients.

pub struct Room {
    name: uuid::Uuid,
    host: Option<Recipient<Message>>,
    clients: Vec<Addr<Client>>,
}

impl Actor for Room {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::info!("Room {} started", self.name);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("Room {} stopped", self.name);
    }
}

impl Room {
    pub fn new() -> Self {
        Self {
            name: uuid::Uuid::new_v4(),
            host: None,
            clients: Vec::new(),
        }
    }

    pub fn name(&self) -> uuid::Uuid {
        self.name
    }

    pub fn host(&self) -> Option<Recipient<Message>> {
        self.host.clone()
    }

    pub fn set_host(&mut self, host: Recipient<Message>) {
        self.host = Some(host);
    }

    pub fn send_message(&self, msg: Message) {
        for client in self.clients.iter() {
            client.do_send(msg.clone());
        }
    }
}

impl Handler<Join> for Room {
    type Result = ();

    fn handle(&mut self, msg: Join, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("{} joined room {}", msg.name, self.name);
        self.clients.push(msg.addr);

        self.send_message(Message {
            name: "Server".to_string(),
            msg: format!("{} joined the room", msg.name),
        });
    }
}

impl Handler<Leave> for Room {
    type Result = ();

    fn handle(&mut self, msg: Leave, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("{} left room {}", msg.name, self.name);
        self.clients.iter().position(|c| c == &msg.addr).map(|i| self.clients.remove(i));

        self.send_message(Message {
            name: "Server".to_string(),
            msg: format!("{} left the room", msg.name),
        });
    }
}

impl Handler<Message> for Room {
    type Result = ();

    fn handle(&mut self, msg: Message, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("{} sent message to room {}", msg.name, self.name);
        self.send_message(msg);
    }
}
