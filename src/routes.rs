use crate::{common::AppError, resizing, upload_file, download_file};
use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse, Responder, http::header::{ContentType, ContentDisposition}};
use serde::{Deserialize, Serialize};
use serde_json;

#[post("/upload/{id}/{name}")]
async fn upload(
    path: web::Path<(u64, String)>,
    payload: Multipart,
) -> Result<HttpResponse, AppError> {
    let (id, name) = path.into_inner();
    let result = upload_file::run(id, name, payload).await?;
    Ok(HttpResponse::Ok().body(result))
}

#[get("/download/{id}/{name}")]
async fn download(path: web::Path<(u64, String)>) -> Result<HttpResponse, AppError> {
    let (id, name) = path.into_inner();
    let (filename, stream) = download_file::run(id, name)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .insert_header(ContentDisposition::attachment(filename))
        .streaming(stream))
}

// {"download_filename":"0002463(1).jpeg","filesize":146038,"output_filesize":145436,"output_filenumber":1,"output_extensions":"[\"jpeg\"]","timer":"0.674","status":"TaskSuccess"}
pub struct AppResponse {
    download_filename: String,
    output_filenumber: String,
    output_extensions: String,
    status: String,
}

#[post("/crop/{id}")]
async fn crop(req_body: String) -> Result<HttpResponse, AppError> {
   Ok( HttpResponse::Ok().body(req_body))
}

type Dimensions = (u32, u32);
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ResizeData {
    pub filename: String,
    // TODO use image::ImageFormat instead
    pub file_extension: String,
    pub dimensions: Dimensions,
}

#[post("/resize/{id}")]
async fn resize(path: web::Path<u64>, req_body: String) -> Result<String, AppError> {
    let id = path.into_inner();
    let body = serde_json::from_str::<ResizeData>(&req_body).map_err(|e| AppError::BadRequest {
        error: e.to_string(),
    })?;

    let result = resizing::run(id, body)?;
    Ok(result)
}

#[post("/convert/{id}")]
async fn convert(req_body: String) -> Result<HttpResponse, AppError> {
    todo!()
    // Ok(HttpResponse::Ok().body(req_body))
}
