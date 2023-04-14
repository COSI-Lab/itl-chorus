use actix_web::{App, HttpServer};

mod midi;
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
 * Messages are sent via websockets
 * rooms:
 *      POST    /room           - create a new room, returns a room id
 *      GET     /room/{id}      - loads the html page for the rooms
 *      GET     /room/{id}/ws   - websocket connection for the room, the first connection is the host
 */

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HashMap of rooms
    let rooms = std::sync::Arc::new(std::sync::Mutex::new(room::Rooms::new()));

    println!("Starting server on http://127.0.0.1:8081");

    HttpServer::new(move || {
        App::new()
            .app_data(rooms.clone())
            .service(midi::delete)
            .service(midi::upload)
            .service(midi::list)
            .service(midi::download)
            .service(actix_files::Files::new("/", "../frontend/dist").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
