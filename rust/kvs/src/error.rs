use std::io;
use crate::error::Error::{IoError, SerdeError};

#[derive(Debug)]
pub enum Error {
    NotFoundError,
    IoError(io::Error),
    SerdeError(serde_json::error::Error),
    InternalError(String),
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: io::Error) -> Self {
        IoError(err)
    }
}

impl std::convert::From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Self {
        SerdeError(err)
    }
}
