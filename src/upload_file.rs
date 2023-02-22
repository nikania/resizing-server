use crate::common::AppError;

pub struct FileData {
    pub name: String,
    pub extension: String,
}

pub fn run() -> Result<String, AppError> {
    let filename = "upload".to_string();
    Ok("ok".into())
}
