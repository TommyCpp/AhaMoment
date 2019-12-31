pub use kv::KvStore;
use crate::Error::{IoError};
use std::io;

mod kv;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotFoundError,
    IoError(io::Error),
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: io::Error) -> Self {
        IoError(err)
    }
}