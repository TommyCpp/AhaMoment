pub use super::Result;

pub trait KvsEngine{
    fn set(&mut self, key: String, value: String) -> Result<()>;
    fn get(&mut self, key: String) -> Result<Option<String>>;
    fn remove(&mut self, key: String) -> Result<()>;
}


pub const KV_STORE_NAME: &str = "kvs";
pub const SLED_STORE_NAME: &str = "sled";

pub mod kv;
pub mod sled;

pub use self::kv::KvStore;
pub use self::sled::SledStore;
use std::path::Path;
