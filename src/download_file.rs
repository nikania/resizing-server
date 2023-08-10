use crate::common::{AppError, PATH};
use actix_web::web::Bytes;
use futures::{stream};
use futures_util::{Stream};
use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn run(id: u64, name: String) 
    -> Result<(String, impl Stream<Item = Result<Bytes, std::io::Error>>), AppError> {
    // open file
    let filename = format!("{PATH}/{id}/{name}");
    let file = File::open(filename)
        .map_err(|e| AppError::NotFoundError { error: e.to_string() })?;
    let reader = BufReader::new(file);
    let strm = stream::iter(reader.bytes().map(|r| r.map(|b| vec![b]).map(Bytes::from)));
    Ok((name, strm))
}