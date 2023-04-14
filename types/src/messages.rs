use serde::{Deserialize, Serialize};

// Messages that can be sent to the room host.
#[derive(Debug, Deserialize, Serialize)]
pub enum ToHost {
    // A client has joined the room.
    Joined {
        name: String,
        ip: std::net::IpAddr,
        user_agent: String,
    },
    // A client has left the room.
    Left {
        name: String,
    },
    // A client is ready to play a song.
    Ready {
        name: String,
        song: String,
    },
}

// Messages than can be sent from the room host.
#[derive(Debug, Deserialize, Serialize)]
pub enum FromHost {
    // Choose a midi to play.
    Choose { midi: String },
    // Destroy the room.
    Destroy,
}

// Messages that can be sent to a room client.
#[derive(Debug, Deserialize, Serialize)]
pub enum ToClient {
    // The host has chosen a midi to play.
    Choosen { midi: String },
    // The client has been given a name
    Name { name: String },
}

// Messages that can be sent from a room client.
#[derive(Debug, Deserialize, Serialize)]
pub enum FromClient {
    // Send once the client is ready to play.
    Ready { name: String, song: String },
}
