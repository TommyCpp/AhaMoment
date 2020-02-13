use std::io;
use crate::error::KvError::{IoError, SerdeError};
use std::string::FromUtf8Error;
use std::net::AddrParseError;
use std::fmt::Display;
use failure::Fail;
use rayon::ThreadPoolBuildError;

#[derive(Debug, Fail)]
pub enum KvError {
    #[fail(display = "Key Not found")]
    NotFoundError,
    #[fail(display = "Io error: {}", _0)]
    IoError(io::Error),
    #[fail(display = "Serde error: {}", _0)]
    SerdeError(serde_json::error::Error),
    #[fail(display = "Internal error: {}", _0)]
    InternalError(String),
    #[fail(display = "Sled error: {}", _0)]
    SledError(sled::Error),
    #[fail(display = "Utf8 conversion error")]
    Utf8Error(FromUtf8Error),
    #[fail(display = "Ip addr conversation error")]
    AddrParseError(AddrParseError),
    #[fail(display = "Error when creating thread pool")]
    ThreadPoolBuildError(ThreadPoolBuildError),
}

impl std::convert::From<std::io::Error> for KvError {
    fn from(err: io::Error) -> Self {
        IoError(err)
    }
}

impl std::convert::From<serde_json::error::Error> for KvError {
    fn from(err: serde_json::error::Error) -> Self {
        SerdeError(err)
    }
}

impl From<sled::Error> for KvError {
    fn from(err: sled::Error) -> KvError {
        KvError::SledError(err)
    }
}

impl From<FromUtf8Error> for KvError {
    fn from(err: FromUtf8Error) -> KvError {
        KvError::Utf8Error(err)
    }
}

impl From<AddrParseError> for KvError {
    fn from(err: AddrParseError) -> Self {
        KvError::AddrParseError(err)
    }
}

impl From<rayon::ThreadPoolBuildError> for KvError {
    fn from(err: ThreadPoolBuildError) -> Self {
        KvError::ThreadPoolBuildError(err)
    }
}