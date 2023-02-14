use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use routes::{upload, download, crop, convert, resize};

mod routes;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// upload - store all img in one place with session num
// resize # pixels or percentage
// crop
// convert /jpgtopng /pngtojpg ..etc
// download

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(upload)
            .service(download)
            .service(crop)
            .service(resize)
            .service(convert)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
