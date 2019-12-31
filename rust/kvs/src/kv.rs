use std::collections::HashMap;
use crate::{Result, Error};
use std::path::Path;
use std::fs as fs;
use std::fs::{File, OpenOptions};
use serde::{Serialize, Deserialize};
use std::io::{Write, Read, LineWriter};
use std::ops::Add;

pub struct KvStore {
    hashmap: HashMap<String, String>,
    log: String, //log file path
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Log {
    Set {
        key: String,
        value: String,
    },
    Get {
        key: String
    },
    Remove {
        key: String
    },
}


impl KvStore {
    pub fn open(path: &Path) -> Result<KvStore> {
        let log_file = path.join("log.txt");
        let log_file_str = log_file.to_str().unwrap();
        if !log_file.exists() {
            //only create if not exists
            match File::create(log_file_str) {
                Err(why) => panic!("{}", why),
                _ => ()
            }
            Ok(KvStore {
                hashmap: HashMap::new(),
                log: String::from(log_file_str),
            })
        } else {
            let mut store = KvStore {
                hashmap: HashMap::new(),
                log: String::from(log_file_str),
            };
            store.read_log()?;
            Ok(store)
        }
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        match self.hashmap.get(&key) {
            Some(val_ref) => Ok(Some(String::from(val_ref))),
            None => Ok(None),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let (key_cp, value_cp) = (key.clone(), value.clone());
        let _ = self.hashmap.insert(key, value); //ignore the return value
        self.append_log(Log::Set { key: key_cp, value: value_cp });
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        match self.hashmap.remove(&key) {
            Some(val) => {
                self.append_log(Log::Remove { key });
                Ok(())
            }
            None => {
                Err(Error::NotFoundError)
            }
        }
    }

    fn append_log(&mut self, log: Log) -> Result<()> {
        let mut serialized = serde_json::to_string(&log).unwrap();
        serialized = serialized.add("\n");
        let mut file = self.open_file(false)?;
        match file.write_all(serialized.as_bytes()) {
            Ok(()) => Ok(()),
            Err(why) => panic!("Cannot write file, {}", why)
        }
    }

    fn read_log(&mut self) -> Result<()> {
        let mut file = self.open_file(true)?;
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Ok(us) => {
                let mut lines: Vec<&str> = s.split('\n').collect();
                for line in lines {
                    if line.len() != 0 {
                        let log: Log = serde_json::from_str(line).unwrap();
                        match log {
                            Log::Set { key, value } => {
                                self.hashmap.insert(key, value);
                            }
                            Log::Remove { key } => {
                                self.hashmap.remove(&key);
                            }
                            Log::Get { key } => {}
                        }
                    }
                }
            }
            Err(why) => panic!("Cannot read file, {}", why)
        }
        Ok(())
    }

    fn open_file(&mut self, is_read: bool) -> Result<File> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(self.log.clone());
        if is_read {
            file = File::open(self.log.clone());
        }
        match file {
            Err(why) => panic!("could not open file, {}", why),
            Ok(file) => Ok(file)
        }
    }
}
