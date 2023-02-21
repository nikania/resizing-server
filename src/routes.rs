use actix_web::{get, post, HttpResponse, Responder, FromRequest};

use crate::resizing;
use serde::{Serialize, Deserialize};
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
pub struct Result {
    download_filename: String,
    output_filenumber: String,
    output_extensions: String,
    status: String
}

#[post("/crop")]
async fn crop(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


type Dimensions = (u32,u32);
#[derive(Serialize,Deserialize,Debug,Default)]
pub struct ResizeData {
    pub session: u64,
    pub filename: String,
    pub dimensions: Dimensions,
}

#[post("/resize")]
async fn resize(req_body: String) -> impl Responder {
    // let exp = ResizeData {session:123, filename: "sdf".to_owned(), dimensions:(200,300)};
    // let ser = serde_json::to_string(&exp).unwrap();
    // println!("serialized: {ser}");
    let body = serde_json::from_str::<ResizeData>(&req_body);
    match body {
        Ok(body) => {
            let result = resizing::run(body);
            HttpResponse::Ok().body(result)
        },
        Err(err) => {
            HttpResponse::BadRequest().body(err.to_string())
        }
    }
    
}

#[post("/convert")]
async fn convert(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
