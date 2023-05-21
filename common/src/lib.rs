//! This module defines messages sent between the frontend and the server
//! These fall into 4 categories:
//!     Server to "Client"
//!     "Client" to Server
//!     Server to "Host"
//!     "Host" to Server
//!
//! Ostensibly, "Host" and "Client" are both _clients_ in the client-server model. However, they
//! have different roles in the application.

pub mod names;

use serde::{Deserialize, Serialize};

pub mod client_to_server {
    use super::*;

    // ---- HTTP requests ----

    /// Room info requests are sent to /api/room/{id}
    #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
    pub struct GetRoomInfoRequest {}

    // ---- Websocket messages ----

    #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
    pub struct UploadMessage {
        pub msg: String,
    }

    /// Data sent from the client to the server over the websocket
    #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
    pub enum ClientToServerWS {
        Upload(UploadMessage),
    }
}

pub mod server_to_client {
    use crate::names::Name;

    use super::*;

    // ---- Websocket messages ----

    #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
    pub struct ChatMessage {
        pub name: Name,
        pub msg: String,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
    pub struct JoinMessage {
        pub name: String,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
    pub struct LeaveMessage {
        pub name: String,
    }

    /// Data sent from the server to the client over the websocket
    #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
    pub enum ServerToClientWS {
        Chat(ChatMessage),
        Join(JoinMessage),
        Leave(LeaveMessage),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct RoomInfo {
    pub id: uuid::Uuid,
    pub clients: usize,
}
