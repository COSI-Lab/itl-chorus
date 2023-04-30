use std::collections::HashMap;

use actix_web::{App, HttpServer, web};
use tokio::sync::Mutex;

mod midi;
mod actors;
mod room;

/*
 * The index page that loads the SPA
 * index:
 *      GET     /index.html     - load the index page
 *
 * A file server to handle uploading and downloading midi files
 * file server:
 *      POST    /midi           - add another file
 *      GET     /midi           - list of files
 *      GET     /midi/{file}    - download a midi file
 *      DELETE  /midi/{file}    - delete a midi file
 *
 * Joinable rooms where users can join to participate in the playback
 * The host has full control over what music is played and can trigger playback
 *
 * Messages are sent via websocket
 * rooms:
 *      POST    /room           - create a new room and redirects to it
 *      GET     /room/{id}      - loads the html page for the rooms
 *      GET     /room/{id}/ws   - websocket connection for the room, the first connection is the host
 */

// Rooms are stored in a hashmap from a uuid to a room actor
type Rooms = Mutex<HashMap<uuid::Uuid, actix::Addr<actors::Room>>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on http://127.0.0.1:8081");

    // Create a hashmap to store the rooms
    let rooms = web::Data::new(Rooms::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(rooms.clone())
            .service(midi::delete)
            .service(midi::upload)
            .service(midi::list)
            .service(midi::download)
            .service(room::create_room)
            .service(room::room_ws)
            .service(actix_files::Files::new("/", "../frontend/dist").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
