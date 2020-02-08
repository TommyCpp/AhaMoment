pub use super::Result;

pub trait KvsEngine: Clone + Send + 'static {
    fn set(&self, key: String, value: String) -> Result<()>;
    fn get(&self, key: String) -> Result<Option<String>>;
    fn remove(&self, key: String) -> Result<()>;
}


pub const KV_STORE_NAME: &str = "kvs";
pub const SLED_STORE_NAME: &str = "sled";

pub mod kv;
pub mod sled;

pub use self::kv::KvStore;
pub use self::sled::SledStore;
use std::path::Path;
