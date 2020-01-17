#[macro_use]
extern crate log;
extern crate env_logger;


pub use kv::KvStore;
pub use crate::error::Error;
pub use common::KvsEngine;

mod kv;
mod error;
mod common;

pub type Result<T> = std::result::Result<T, Error>;


pub const DEFAULT_IP: &str = "127.0.0.0";
pub const DEFAULT_PORT: u32 = 4000;
