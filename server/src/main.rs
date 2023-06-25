use std::collections::HashMap;

use actix_files::NamedFile;
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use room::Room;
use tokio::sync::RwLock;

mod actors;
mod api;
mod names;
mod room;

/*
 * Any unmapped request gets redirected to the index page
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

// Rooms are stored in a hashmap with the key being the room id
type Rooms = RwLock<HashMap<uuid::Uuid, Room>>;

// Route to static files
// const STATIC_PATH: &str = "dist";
const STATIC_PATH: &str = "../frontend/dist";

/// Any request that doesn't match a static file, and isn't a part of the API gets redirected to the index page
async fn index() -> Result<NamedFile, actix_web::Error> {
    Ok(NamedFile::open(format!("{}/index.html", STATIC_PATH))?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // set up logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("Starting server on http://localhost:8000");

    // Create a hashmap to store the rooms
    let rooms = web::Data::new(Rooms::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(rooms.clone())
            .service(
                // API routes
                web::scope("/api")
                    .service(api::midi::delete)
                    .service(api::midi::upload)
                    .service(api::midi::list)
                    .service(api::midi::download)
                    .service(api::room::create_room)
                    .service(api::room::room_ws),
            )
            .service(actix_files::Files::new("/", STATIC_PATH).index_file("index.html"))
            .default_service(web::to(index))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
