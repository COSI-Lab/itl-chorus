mod chat;
mod player;

use actix::{Actor, Addr};
pub use chat::{ChatClient, ChatRoom};

/// An envelope is a message with a "return address".
/// This allows actors to understand where a message came from.
#[derive(actix::Message)]
#[rtype(result = "()")]
struct Envelope<T, A: Actor> {
    pub inner: T,
    pub addr: Addr<A>,
}

trait EnvelopeExt: Actor {
    // An actor can create an envelope
    fn envelope<T>(&self, msg: T) -> Envelope<T, Self>;
}
