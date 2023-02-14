use actix_web::{get, post, HttpResponse, Responder};

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

#[post("/resize")]
async fn resize(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/convert")]
async fn convert(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
