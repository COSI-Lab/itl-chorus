//! Room handlers using actix web
//!
//! rooms:
//!      POST    /room                  - create a new room and redirects to it
//!      GET     /room/{id}             - get room info
//!      GET     /room/{id}/chat        - connect to the chat service
//!      GET     /room/{id}/playback    - connect to the playback service
//!      GET     /room/{id}/host        - become the host of the room (at most one host per room)

use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use common::RoomInfo;

use crate::{actors::ChatClient, room::Room, Rooms};

/// Create a new room and return the id
#[post("/room")]
pub async fn create_room(rooms: web::Data<Rooms>) -> impl Responder {
    // Create a new room actor
    let room = Room::new();
    let name = room.uuid();

    log::debug!("Created room {}", name);

    // Add the room to the hashmap
    rooms.write().await.insert(name, room);

    let info = RoomInfo {
        id: name,
        clients: 0,
    };

    // Return the id of the room
    HttpResponse::Ok().json(info)
}

#[get("/room/{id}")]
pub async fn get_room_info(rooms: web::Data<Rooms>, id: web::Path<uuid::Uuid>) -> impl Responder {
    let rooms = rooms.read().await;
    let room = match rooms.get(&id) {
        Some(room) => room,
        None => return HttpResponse::NotFound().finish(),
    };

    let info = room.info();

    HttpResponse::Ok().json(info)
}

/// Connect to the room chat
#[get("/room/{id}/chat")]
pub async fn room_ws(
    req: HttpRequest,
    stream: web::Payload,
    rooms: web::Data<Rooms>,
    id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let rooms = rooms.read().await;
    let room = match rooms.get(&id) {
        Some(room) => room,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    let client = ChatClient::new(room.chat());

    ws::start(client, &req, stream)
}
