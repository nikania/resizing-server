use actix_web::{get, post, FromRequest, HttpResponse, Responder};

use crate::{resizing, common::AppError};
use serde::{Deserialize, Serialize};
use serde_json;

// don't understand which REST method to use, how to do routing
#[post("/upload")]
async fn upload(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
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
    pub file_extension: String,
    pub dimensions: Dimensions,
}

#[post("/resize")]
async fn resize(req_body: String) -> Result<String, AppError> {
    let body = serde_json::from_str::<ResizeData>(&req_body)
        .map_err(|e| AppError::BadRequest { error: e.to_string() })?;

    let result = resizing::run(body)?;
    Ok(result)
}

#[post("/convert")]
async fn convert(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
