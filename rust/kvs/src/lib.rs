#[macro_use]
extern crate log;
extern crate env_logger;


pub use kv::KvStore;
pub use crate::error::KvError;
pub use engine::*;
pub use engine::KV_STORE_NAME;
pub use engine::kv;
pub use server::KvServer;
pub use common::*;
pub use engine::SLED_STORE_NAME;


pub mod engine;
mod server;
mod error;
mod common;
mod client;

pub type Result<T> = std::result::Result<T, KvError>;


pub const DEFAULT_IP: &str = "127.0.0.0";
pub const DEFAULT_PORT: u32 = 4000;

