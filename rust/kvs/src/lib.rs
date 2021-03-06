#[macro_use]
extern crate log;
extern crate env_logger;


pub use kv::KvStore;
pub use crate::error::KvError;
pub use engine::*;
pub use server::KvServer;
pub use client::*;
pub use common::*;
pub use thread_pool::*;


pub mod engine;
pub mod thread_pool;
mod server;
mod error;
mod common;
mod client;

pub type Result<T> = std::result::Result<T, KvError>;


pub const DEFAULT_IP: &str = "127.0.0.1";
pub const DEFAULT_PORT: u32 = 4000;

