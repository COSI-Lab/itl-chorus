use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use futures_util::TryStreamExt;
use std::fs;

/*
file server:
    routes:
        POST    /midi           - add another file
        GET     /midi           - list of files
        GET     /midi/{file}    - download a midi file
        DELETE  /midi/{file}    - delete a midi file
*/

const MIDI_DIR: &str = "midi";
const MAX_FILE_SIZE: usize = 10_485_760; // 10MB

/// Upload and verify midi files
#[post("/midi")]
async fn upload(mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    // We may need to return multiple results to the user, so we'll collect them
    let mut results: Vec<Result<String, String>> = Vec::new();

    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field.content_disposition();

        let file_name = match content_disposition.get_filename() {
            Some(file_name) => file_name.to_owned(),
            None => {
                results.push(Err("No file name provided".to_string()));
                continue;
            }
        };

        // read file into memory
        let mut file = vec![];
        while let Some(chunk) = field.try_next().await? {
            if (file.len() + chunk.len()) > MAX_FILE_SIZE {
                results.push(Err(format!(
                    "File {} is too large. Max size is {} bytes",
                    file_name, MAX_FILE_SIZE
                )));
                continue;
            }
            file.extend_from_slice(&chunk);
        }

        // verify the uploaded file is a midi
        if midly::parse(&file).is_err() {
            results.push(Err(format!("File {} is not a valid midi", file_name)));
            continue;
        }

        // save file to disk
        let path = format!("{}/{}", MIDI_DIR, file_name);
        fs::write(&path, file)?;

        results.push(Ok(format!("File {} uploaded successfully", file_name)));
    }

    Ok(HttpResponse::Ok().json(results))
}

/// Return information about all available files as JSON
/// For now, just return a list of filenames
#[get("/midi")]
async fn list() -> impl Responder {
    match std::fs::read_dir(MIDI_DIR) {
        Ok(entries) => {
            let filenames: Vec<String> = entries
                .filter_map(Result::ok)
                .filter_map(|entry| entry.file_name().into_string().ok())
                .collect();

            HttpResponse::Ok().json(filenames)
        }
        Err(err) => {
            log::error!("Error reading midi directory: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Download a single file by name
#[get("/midi/{file}")]
async fn download(path: web::Path<String>) -> actix_web::Result<NamedFile> {
    let path = format!("{}/{}", MIDI_DIR, sanitize_filename::sanitize(&*path));

    // verify the file exists
    if fs::metadata(&path).is_err() {
        return actix_web::Result::Err(actix_web::error::ErrorNotFound("File not found"));
    }

    Ok(NamedFile::open(path)?)
}

/// Delete a saved file by name
#[delete("/midi/{file}")]
async fn delete(path: web::Path<String>) -> HttpResponse {
    let path = format!("{}/{}", MIDI_DIR, sanitize_filename::sanitize(&*path));

    // verify the file exists
    if fs::metadata(&path).is_err() {
        return HttpResponse::NotFound().finish();
    }

    // delete the file
    match fs::remove_file(&path) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            log::error!("Error deleting file: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
