use std::collections::{HashMap, BTreeMap};
use crate::{Result, Error};
use std::path::Path;
use std::{fs as fs, io};
use std::fs::{File, OpenOptions};
use serde::{Serialize, Deserialize};
use std::io::{Write, Read, LineWriter, BufReader, SeekFrom, BufRead, Seek, BufWriter};
use std::ops::Add;

pub struct KvStore {
    data_map: BTreeMap<String, CommandLog>,
    file_readers: HashMap<u32, BufReaderWithPos>,
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
        let mut store = KvStore {
            data_map: BTreeMap::new(),
            file_readers: HashMap::new(),
            log: String::from(log_file_str),
        };
        store.read_log()?;
        Ok(store)
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        match self.data_map.get(&key) {
            Some(val_ref) => Ok(None),
            None => Ok(None),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
//        let (key_cp, value_cp) = (key.clone(), value.clone());
//        let _ = self.data_map.insert(key, value); //ignore the return value
//        self.append_log(Log::Set { key: key_cp, value: value_cp });
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        match self.data_map.remove(&key) {
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
//        let mut file = self.open_file(true)?;
//        let mut s = String::new();
//        match file.read_to_string(&mut s) {
//            Ok(us) => {
//                let mut lines: Vec<&str> = s.split('\n').collect();
//                for line in lines {
//                    if line.len() != 0 {
//                        let log: Log = serde_json::from_str(line).unwrap();
//                        match log {
//                            Log::Set { key, value } => {
//                                self.data_map.insert(key, value);
//                            }
//                            Log::Remove { key } => {
//                                self.data_map.remove(&key);
//                            }
//                            Log::Get { key } => {}
//                        }
//                    }
//                }
//            }
//            Err(why) => panic!("Cannot read file, {}", why)
//        }
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

#[derive(Copy, Clone)]
struct CommandLog {
    file_id: u64,
    pos: u64,
    len: u64,
}

struct BufReaderWithPos {
    reader: BufReader<File>,
    pos: u64,
}

impl BufReaderWithPos {
    fn new(mut file: File) -> Result<Self> {
        let pos = file.seek(SeekFrom::Current(0))?;
        let reader = BufReader::new(file);
        Ok(BufReaderWithPos {
            reader,
            pos,
        })
    }
}

impl Read for BufReaderWithPos {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl Seek for BufReaderWithPos {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

struct BufWriterWithPos {
    writer: BufWriter<File>,
    pos: u64,
}

impl BufWriterWithPos {
    fn new(mut file: File) -> Result<Self> {
        let pos = file.seek(SeekFrom::Current(0))?;
        let writer = BufWriter::new(file);
        Ok(BufWriterWithPos {
            writer,
            pos,
        })
    }
}

impl Write for BufWriterWithPos {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl Seek for BufWriterWithPos {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}
