use actix::prelude::*;

mod client;
mod room;

pub use client::Client;
pub use room::Room;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub name: String,
    pub addr: Addr<Client>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Leave {
    pub name: String,
    pub addr: Addr<Client>,
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct Message {
    pub name: String,
    pub msg: String,
}
