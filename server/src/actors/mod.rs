use actix::prelude::*;

mod room;
mod client;

pub use room::Room;
pub use client::Client;

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
