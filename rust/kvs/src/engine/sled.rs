use crate::{KvsEngine, KvError};
use sled;
use std::path::Path;
use super::Result;
use sled::{Tree, Db};

#[derive(Clone)]
pub struct SledStore(Db);

impl SledStore {
    pub fn open(path: &Path) -> Result<SledStore> {
        let db = sled::Db::start_default(path)?;
        Ok(SledStore(db))
    }

    pub fn new(db: Db) -> SledStore {
        SledStore(db)
    }
}

impl KvsEngine for SledStore {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        let tree: &Tree = &self.0;
        tree.set(key, value.into_bytes()).map(|_| ())?;
        tree.flush()?;
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        let tree: &Tree = &self.0;
        Ok(tree
            .get(key)?
            .map(|i_vec| AsRef::<[u8]>::as_ref(&i_vec).to_vec())
            .map(String::from_utf8)
            .transpose()?)
    }

    fn remove(&mut self, key: String) -> Result<()> {
        let tree: &Tree = &self.0;
        if tree.get(key.clone())?.is_none() {
            return Err(KvError::NotFoundError);
        }
        tree.del(key)?;
        tree.flush()?;
        Ok(())
    }
}