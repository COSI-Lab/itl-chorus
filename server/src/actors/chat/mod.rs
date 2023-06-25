mod client;
mod room;

use common::client_to_server;
use std::sync::Arc;

use super::Envelope;

pub use client::ChatClient;
pub use room::ChatRoom;

/// Created when a client sends a message to the server.
type ClientMessage = Envelope<client_to_server::Chat, ChatClient>;

/// Created when a client joins a room.
type ClientJoin = Envelope<JoinInner, ChatClient>;
struct JoinInner();

/// Created when a client leaves a room.
type ClientLeave = Envelope<LeaveInner, ChatClient>;
struct LeaveInner();

/// Messages broadcasted to all clients in a room.
type Broadcast = Envelope<Arc<str>, ChatRoom>;
