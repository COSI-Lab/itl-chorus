use std::io::Write;
use std::fs;
use std::fs::File;
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;
use libitlc::DeleteRequest;

async fn downloads() -> impl Responder {
    HttpResponse::Ok().body("Hello Chris, this is the downloads")
}

async fn file() -> impl Responder {
    HttpResponse::Ok().body("Hello Chris, this is the files\nHello Maryangela")
}

async fn submissions(mut payload: Multipart) -> Result<HttpResponse, Error> {
    //HttpResponse::Ok().body(req_body)
    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field.content_disposition();

        let filename = match content_disposition.get_filename() {
            Some(name) => name,
            None => return Ok(HttpResponse::BadRequest().finish()),
        };

        let filepath = format!("./tmp/{filename}");

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
    }

    Ok(HttpResponse::Ok().into())
}

async fn delete(item: web::Json<DeleteRequest>) -> HttpResponse {
    
    match File::open(item.0.delete_name.clone()){ //have Chris explain this
        Ok(_) => {
            match fs::remove_file(format!("./tmp/{}", item.0.delete_name)) {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(e) => {
                     HttpResponse::BadRequest().finish()
                }
            }
        },
        Err(_) => HttpResponse::NotFound().finish(),
    }
    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            //.service(hello) //service for handlers using routing macros
            //.service(echo) //route for manually routed handlers
            // .route("/hey", web::get().to(manual_hello))
            .route("/submissions", web::post().to(submissions))
            .route("/downloads", web::get().to(downloads))
            .route("/downloads/file", web::get().to(file))
            .route("/delete", web::delete().to(delete))
            // .service(downloads)
            // .service(file)
            // .service(submissions)
            // .service(delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

//yay
/*
file server:
    routes: the /route (/hey)
        Post request: submit an entry (changes)
        delete request:
        REDACTED FOR CHRIS
        POST /fileserver/submission - add another file
        DELETE /fileserver/delete
        GET /fileserver/download - list of files
        GET /fileserver/download/<file>

        get requests only have a header

*/
