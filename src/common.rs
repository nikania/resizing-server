use actix_web::ResponseError;
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
    #[display(fmt = "Not found: {}", error)]
    NotFoundError { error: String },
    #[display(fmt = "Bad request: {}", error)]
    BadRequest { error: String },
    #[display(fmt = "Multipart upload error: {}", error)]
    MultipartError { error: String },
    #[display(fmt = "Upload error: {}", error)]
    UploadError { error: String },
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::InternalError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFoundError { .. } => actix_web::http::StatusCode::NOT_FOUND,
            Self::BadRequest { .. } => actix_web::http::StatusCode::BAD_REQUEST,
            Self::MultipartError { .. } => actix_web::http::StatusCode::BAD_REQUEST,
            Self::UploadError { .. } => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub const PATH: &str = "./data/";
