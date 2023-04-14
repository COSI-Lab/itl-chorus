/*
 * Messages are sent via websockets
 * rooms:
 *      POST    /room           - create a new room, returns a room id 
 *      GET     /room/{id}      - returns some information about the room
 *      GET     /room/{id}/ws   - websocket connection for the room, the first connection is the host
 */

use std::collections::HashMap;

use actix_web::{post, web, Responder, HttpResponse};
use rand::Rng;

struct Room {
    id: String,
    host: Option<String>,
    clients: HashMap<String, String>,
}

impl Room {
    fn new(id: String) -> Self {
        Self {
            id,
            host: None,
            clients: HashMap::new(),
        }
    }

    fn add_client(&mut self, name: String, ip: String) {
        self.clients.insert(name, ip);
    }

    fn remove_client(&mut self, name: String) {
        self.clients.remove(&name);
    }

    fn set_host(&mut self, name: String) {
        self.host = Some(name);
    }
}

pub(crate) struct Rooms {
    rooms: HashMap<String, Room>,
}

impl Rooms {
    pub(crate) fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }
}

/// Create a new room with a unique id
#[post("/room")]
async fn create_room(rooms: web::Data<std::sync::Arc<std::sync::Mutex<Rooms>>>) -> impl Responder {
    let get_code = || rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(4)
        .map(char::from)
        .collect::<String>();
        
    let mut rooms = rooms.lock().unwrap();

    let mut id = get_code();
    while rooms.rooms.contains_key(&id) {
        id = get_code();
    }

    rooms.rooms.insert(id.clone(), Room::new(id.clone()));
    HttpResponse::Ok().body(id)
}
