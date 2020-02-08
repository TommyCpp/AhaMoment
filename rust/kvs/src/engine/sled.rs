use crate::{KvsEngine, KvError};
use sled;
use std::path::Path;
use super::Result;
use sled::{Tree, Db};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SledStore(Arc<Mutex<Db>>);

impl SledStore {
    pub fn open(path: &Path) -> Result<SledStore> {
        let db = sled::Db::start_default(path)?;
        Ok(SledStore(Arc::new(Mutex::new(db))))
    }

    pub fn new(db: Db) -> SledStore {
        SledStore(Arc::new(Mutex::new(db)))
    }
}

impl KvsEngine for SledStore {
    fn set(&self, key: String, value: String) -> Result<()> {
        let tree: &Tree = &self.0.lock().unwrap();
        tree.set(key, value.into_bytes()).map(|_| ())?;
        tree.flush()?;
        Ok(())
    }

    fn get(&self, key: String) -> Result<Option<String>> {
        let tree: &Tree = &self.0.lock().unwrap();
        Ok(tree
            .get(key)?
            .map(|i_vec| AsRef::<[u8]>::as_ref(&i_vec).to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }

    fn remove(&self, key: String) -> Result<()> {
        let tree: &Tree = &self.0.lock().unwrap();
        if tree.get(key.clone())?.is_none() {
            return Err(KvError::NotFoundError);
        }
        tree.del(key)?;
        tree.flush()?;
        Ok(())
    }
}