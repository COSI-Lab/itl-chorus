mod client;
mod room;

use std::sync::Arc;

use actix::{Actor, Addr};
pub use client::Client;
use common::{
    client_to_server::{self, GetRoomInfoRequest},
    RoomInfo,
};
pub use room::Room;

/// An envelope is a message with a "return address". It allows actors to understand where a message
/// came from.
#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Envelope<T, A: Actor> {
    pub inner: T,
    pub addr: Addr<A>,
}

trait EnvelopeExt: Actor {
    // An actor can create an envelope
    fn envelope<T>(&self, msg: T) -> Envelope<T, Self>;
}

/// Created when a client sends a message to the server.
pub type Message = Envelope<client_to_server::UploadMessage, Client>;

pub struct JoinInner {}

/// Created when a client joins a room.
pub type Join = Envelope<JoinInner, Client>;

pub struct LeaveInner {}

/// Created when a client leaves a room.
pub type Leave = Envelope<LeaveInner, Client>;

/// Messages broadcasted to all clients in a room.
pub type Broadcast = Envelope<Arc<String>, Room>;

/// Used by the http server to request room info.
#[derive(actix::Message)]
#[rtype(result = "Result<RoomInfo, ()>")]
pub struct GetRoomInfo {
    pub req: GetRoomInfoRequest,
}
