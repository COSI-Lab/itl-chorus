//! Room handlers using actix web
//! 
//! Messages are sent via websocket
//! rooms:
//!      POST    /room           - create a new room and redirects to it
//!      GET     /room/{id}/ws   - websocket connection for the room, the first connection is the host

use crate::{actors::{Room, Client}, Rooms};

use actix::Actor;
use actix_web::{post, web, HttpResponse, Responder, HttpRequest, get};
use actix_web_actors::ws;

/// Create a new room and redirect to it
#[post("/room")]
pub async fn create_room(rooms: web::Data<Rooms>) -> impl Responder {
    // Create a new room actor
    let room = Room::new();

    let name = room.name();
    
    log ::info!("Created room {}", name);

    // Add the room to the hashmap
    rooms.lock().await.insert(name, room.start());

    // Redirect to the room
    HttpResponse::Found()
        .append_header(("Location", format!("/room/{}", name)))
        .finish()
}

/// Websocket connection for the room
#[get("/room/{id}/ws")]
pub async fn room_ws(req: HttpRequest, stream: web::Payload, rooms: web::Data<Rooms>, id: web::Path<uuid::Uuid>) -> Result<HttpResponse, actix_web::Error> {
    let rooms = rooms.lock().await;

    let room = match rooms.get(&id) {
        Some(room) => room,
        None => return Ok(HttpResponse::NotFound().finish())
    };

    let client = Client::new(room.clone());
    let name = client.name();

    log::info!("{} joined room {}", name, id);

    ws::start(client, &req, stream)
}