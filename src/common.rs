use derive_more::{Display, Error};
use actix_web::ResponseError;

#[derive(Debug,Display,Error)]
pub enum AppError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
    #[display(fmt = "Not found: {}", obj)]
    NotFoundError {obj:String},
    #[display(fmt = "Bad request: {}", error)]
    BadRequest {error:String}
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::InternalError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFoundError {..} => actix_web::http::StatusCode::NOT_FOUND,
            Self::BadRequest { .. } => actix_web::http::StatusCode::BAD_REQUEST
        }
    }
}