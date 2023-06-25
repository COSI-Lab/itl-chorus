pub mod name;

use serde::{Deserialize, Serialize};

pub mod client_to_server {
    use super::*;

    // ---- Websocket messages ----

    // --- CHAT ---
    /// Send a chat message to the server
    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Chat {
        pub msg: String,
    }

    // --- PLAYBACK ---
    /// Sent by a client indicating that they have finished loading their instruments
    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Loaded {
        pub instruments: Vec<String>,
    }
}

pub mod host_to_all {
    use serde::{Deserialize, Serialize};

    /// Sent by the host indicating that they have selected a song
    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct SongSelect {
        pub song: String,
    }

    /// Sent by the host requesting playback to begin
    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Begin {}

    /// Sent by the host requesting playback to pause
    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Pause {}

    /// Sent by the host to destroy the room
    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Destroy {}

    /// Data sent from the host to the server (which is then broadcasted to all clients)
    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub enum HostToAllWS {
        SongSelect(SongSelect),
        Begin(Begin),
        Pause(Pause),
        Destroy(Destroy),
    }
}

pub mod server_to_client {
    use crate::name::Name;

    use super::*;

    // ---- Websocket messages ----

    // --- CHAT ---
    /// A message destined for the chat component
    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub enum Chat {
        /// User generated chat message
        Message { name: Name, msg: String },
        /// Sent by the server when a user joins the room
        Joined { name: Name },
        /// Sent by the server when a user leaves the room
        Left { name: Name },
        /// When the host preforms an action a message is sent to all clients
        HostAction { msg: String },
    }

    // --- PLAYBACK ---
    /// Tells clients which instruments they need to load
    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct LoadInstruments {
        pub instruments: Vec<String>,
    }

    /// Event that can be handled by the midi player
    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Event {
        pub delta: f32,
        pub kind: EventKind,
    }

    /// Inner data of an event
    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub enum EventKind {
        NoteOn { channel: u8, key: u8, velocity: u8 },
        NoteOff { channel: u8, key: u8, velocity: u8 },
    }

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub enum Playback {
        LoadInstruments(LoadInstruments),
        Event(Event),
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RoomInfo {
    pub id: uuid::Uuid,
    pub clients: usize,
}
