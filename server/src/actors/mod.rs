use std::ops::Deref;

use actix::prelude::*;
use common::RoomInfo;

mod client;
mod room;

pub use client::Client;
pub use room::Room;

trait IntoMessage {
    type Target;

    fn into_message(self) -> Self::Target;
}

#[derive(Message)]
#[rtype(result = "Result<RoomInfo, ()>")]
pub struct GetRoomInfo;

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

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct ChatMessage(common::Message);

impl Deref for ChatMessage {
    type Target = common::Message;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoMessage for common::Message {
    type Target = ChatMessage;

    fn into_message(self) -> Self::Target {
        ChatMessage(self)
    }
}
