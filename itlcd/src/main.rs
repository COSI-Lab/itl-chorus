use actix_multipart::Multipart;
use actix_web::{delete, get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use futures_util::TryStreamExt as _;
use std::fs;
use std::fs::File;
use std::io::Write;

/*
file server:
    routes:
        POST    /midi           - add another file
        GET     /midi           - list of files
        GET     /midi/{file}    - download a midi file
        DELETE  /midi/{file}    - delete a midi file
*/

/// Upload and verify a midi file
#[post("/midi")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
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

/// Return information about all avaiable files as JSON
#[get("/midi")]
async fn list() -> impl Responder {
    HttpResponse::Ok().body("")
}

/// Download a single file by name
#[get("/midi/{file}")]
async fn download() -> impl Responder {
    HttpResponse::Ok().body("")
}

/// Delete a saved file by name
#[delete("/midi/{file}")]
async fn delete(path: web::Path<String>) -> HttpResponse {
    match File::open(path.clone()) {
        //have Chris explain this
        Ok(_) => match fs::remove_file(format!("./tmp/{}", path)) {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::BadRequest().finish(),
        },
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(delete)
            .service(upload)
            .service(list)
            .service(download)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
