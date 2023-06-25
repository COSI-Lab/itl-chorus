use actix::{Actor, Addr};
use common::RoomInfo;

use crate::actors::ChatRoom;

// There's an actor for each service in the room
pub struct Room {
    uuid: uuid::Uuid,
    chat: Addr<ChatRoom>,
}

impl Room {
    pub fn new() -> Self {
        let uuid = uuid::Uuid::new_v4();

        Self {
            uuid,
            chat: ChatRoom::new().start(),
        }
    }

    pub fn chat(&self) -> Addr<ChatRoom> {
        self.chat.clone()
    }

    pub fn uuid(&self) -> uuid::Uuid {
        self.uuid
    }

    pub fn info(&self) -> RoomInfo {
        RoomInfo {
            id: self.uuid,
            clients: 0,
        }
    }
}
