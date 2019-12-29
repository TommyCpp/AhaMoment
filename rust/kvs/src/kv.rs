use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    hashmap: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            hashmap: HashMap::new(),
        }
    }

    pub fn get(&self, key: String) -> Option<String> {
        match self.hashmap.get(&key) {
            Some(val_ref) => Some(String::from(val_ref)),
            _ => None,
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        let _ = self.hashmap.insert(key, value); //ignore the return value
    }

    pub fn remove(&mut self, key: String) {
        let _ = self.hashmap.remove(&key);
    }
}
