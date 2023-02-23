use crate::common::{AppError, PATH};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::{
    fs::File,
    io::{BufReader, Read, Write},
};

pub async fn run(id: u64, name: String, mut payload: Multipart) -> Result<String, AppError> {
    let mut filename = String::from(PATH);
    filename.push_str(&id.to_string());
    std::fs::create_dir(&filename).map_err(|e| {
        eprintln!("{e}");
        AppError::InternalError
    })?;
    filename.push_str("/");
    filename.push_str(&name);
    let mut file = File::create(filename).map_err(|_| AppError::InternalError)?;

    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item.map_err(|e| AppError::MultipartError {
            error: e.to_string(),
        })?;
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
    Ok("File uploaded".into())
}
