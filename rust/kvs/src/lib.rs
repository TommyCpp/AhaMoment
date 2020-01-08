pub use kv::KvStore;
use crate::Error::{IoError, SerdeError};
use std::io;

mod kv;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotFoundError,
    IoError(io::Error),
    SerdeError(serde_json::error::Error),
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: io::Error) -> Self {
        IoError(err)
    }
}

impl std::convert::From<serde_json::error::Error> for Error{
    fn from(err: serde_json::error::Error) -> Self {
        SerdeError(err)
    }
}