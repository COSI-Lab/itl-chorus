use std::collections::HashMap;

use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, middleware::Logger};
use env_logger::Env;
use tokio::sync::Mutex;

mod actors;
mod midi;
mod room;

/*
 * The index page that loads the SPA
 * index:
 *      GET     /index.html     - load the index page
 *
 * A file server to handle uploading and downloading midi files
 * file server:
 *      POST    /api/midi           - add another file
 *      GET     /api/midi           - list of files
 *      GET     /api/midi/{file}    - download a midi file
 *      DELETE  /api/midi/{file}    - delete a midi file
 *
 * Joinable rooms where users can join to participate in the playback
 * The host has full control over what music is played and can trigger playback
 *
 * Messages are sent via websocket
 * rooms:
 *      POST    /api/room           - create a new room and redirects to it
 *      GET     /api/room/{id}      - gets some info about the room. Typically the websocket is used instead
 *      GET     /api/room/{id}/ws   - websocket connection for the room, the first connection is the host
 */

// Rooms are stored in a hashmap from a uuid to a room actor
type Rooms = Mutex<HashMap<uuid::Uuid, actix::Addr<actors::Room>>>;

/// Any request that doesn't match a static file, and isn't a part of the API gets redirected to the index page
async fn index() -> Result<NamedFile, actix_web::Error> {
    Ok(NamedFile::open("../frontend/dist/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // set up logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("Starting server on http://localhost:8081");

    // Create a hashmap to store the rooms
    let rooms = web::Data::new(Rooms::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(rooms.clone())
            .service(
                // API routes
                web::scope("/api")
                    .service(midi::delete)
                    .service(midi::upload)
                    .service(midi::list)
                    .service(midi::download)
                    .service(room::create_room)
                    .service(room::room_ws)
            )
            .service(actix_files::Files::new("/", "../frontend/dist").index_file("index.html"))
            .default_service(web::to(index))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
