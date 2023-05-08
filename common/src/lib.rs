//! This module defines messages sent between the frontend and the server

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomInfo {
    pub id: uuid::Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub name: String,
    pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadMessage {
    pub msg: String,
}
