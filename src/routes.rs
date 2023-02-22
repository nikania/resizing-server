use crate::{common::AppError, resizing, upload_file};
use actix_multipart::Multipart;
use actix_web::{get, post, FromRequest, HttpResponse, Responder};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

#[post("/upload")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, AppError> {
    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| AppError::MultipartError {
            error: e.to_string(),
        })?;
        let filename = "./data/some.png";
        let mut file = File::create(filename).map_err(|_| AppError::InternalError)?;
        let mut buf = Vec::new();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            match chunk {
                Ok(ch) => {
                    buf.append(&mut ch.to_vec());
                }
                Err(e) => {
                    return Err(AppError::MultipartError {
                        error: e.to_string(),
                    })
                }
            }
        }
        file.write_all(buf.as_slice())
            .map_err(|e| AppError::MultipartError {
                error: e.to_string(),
            })?;
    }

    let result = upload_file::run()?;
    Ok(HttpResponse::Ok().body(result))
}

#[get("/download")]
async fn download() -> impl Responder {
    HttpResponse::Ok().body("your file")
}

// {"download_filename":"0002463(1).jpeg","filesize":146038,"output_filesize":145436,"output_filenumber":1,"output_extensions":"[\"jpeg\"]","timer":"0.674","status":"TaskSuccess"}
pub struct AppResponse {
    download_filename: String,
    output_filenumber: String,
    output_extensions: String,
    status: String,
}

#[post("/crop")]
async fn crop(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

type Dimensions = (u32, u32);
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ResizeData {
    pub session: u64,
    pub filename: String,
    // TODO use image::ImageFormat instead
    pub file_extension: String,
    pub dimensions: Dimensions,
}

#[post("/resize")]
async fn resize(req_body: String) -> Result<String, AppError> {
    let body = serde_json::from_str::<ResizeData>(&req_body).map_err(|e| AppError::BadRequest {
        error: e.to_string(),
    })?;

    let result = resizing::run(body)?;
    Ok(result)
}

#[post("/convert")]
async fn convert(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
