use std::collections::{HashMap, BTreeMap};
use crate::{Result, Error};
use std::path::{Path, PathBuf};
use std::{fs as fs, io};
use std::fs::{File, OpenOptions};
use serde::{Serialize, Deserialize};
use std::io::{Write, Read, LineWriter, BufReader, SeekFrom, BufRead, Seek, BufWriter};
use std::ops::Add;
use serde_json::Deserializer;
use crate::Error::NotFoundError;

const LOG_SIZE: u64 = 128;

pub struct KvStore {
    data_map: BTreeMap<String, CommandOps>,
    file_readers: HashMap<u64, BufReaderWithPos>,
    file_writer: BufWriterWithPos,
    current_active_log: u64,
    log_dir: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum CommandLog {
    Set {
        key: String,
        value: String,
    },
    Remove {
        key: String
    },
}


impl KvStore {
    pub fn open(path: &Path) -> Result<KvStore> {
        let log_file_names = KvStore::get_log_files_sorted(path)?;
        let mut readers_map = HashMap::<u64, BufReaderWithPos>::new();
        let active_file_name: u64 = log_file_names.last().unwrap_or(&0) + 1;
        for log_file_name in log_file_names {
            let reader = BufReaderWithPos::new(fs::File::open(get_file_name_from_log_id(log_file_name, path))?)?;
            readers_map.insert(log_file_name, reader);
        }
        //create new log file
        let active_file = File::create(get_file_name_from_log_id(active_file_name, path))?;
        let mut store = KvStore {
            data_map: BTreeMap::new(),
            file_readers: readers_map,
            file_writer: BufWriterWithPos::new(active_file)?,
            current_active_log: active_file_name,
            log_dir: path.to_path_buf(),
        };

        store.read_log()?;
        Ok(store)
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        match self.data_map.get(&key) {
            Some(&command_ops) => {
                let mut value = Vec::<u8>::new();
                let mut active_file_reader: BufReaderWithPos;
                let mut reader_ref: &mut BufReaderWithPos;
                if command_ops.file_id == self.current_active_log {
                    //if the log is in current active log
                    active_file_reader = BufReaderWithPos::new(
                        OpenOptions::new()
                            .read(true)
                            .open(
                                get_file_name_from_log_id(self.current_active_log, self.log_dir.as_path())
                            )?)?;
                    reader_ref = &mut active_file_reader;
                }else {
                    reader_ref = self.file_readers.get_mut(&command_ops.file_id).unwrap();
                }
                reader_ref.seek(SeekFrom::Start(command_ops.pos));
                let chunk = reader_ref.take(command_ops.len);
                let command: CommandLog = serde_json::from_reader(chunk)?;
                match command {
                    CommandLog::Set { key, value } => {
                        Ok(Some(value))
                    }
                    _ => { Err(Error::InternalError(String::from("The log should not be remove command log"))) }
                }
            }
            None => Ok(None),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command_log = CommandLog::Set {
            key: key.clone(),
            value,
        };
        let mut serialized = serde_json::to_string(&command_log).unwrap();
        let old_len = self.file_writer.pos;
        let len = self.file_writer.write(serialized.as_bytes())?;
        self.file_writer.flush();
        let command_ops = CommandOps {
            file_id: self.current_active_log,
            pos: old_len,
            len: self.file_writer.pos - old_len,
        };
        self.data_map.insert(key, command_ops);
        if len as u64 > LOG_SIZE {
            self.swap_active_file()?
        }
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if !self.data_map.contains_key(&key) {
            Err(NotFoundError)
        } else {
            let command_log = CommandLog::Remove {
                key: key.clone(),
            };
            let mut serialized = serde_json::to_string(&command_log).unwrap();
            let old_len = self.file_writer.pos;
            let len = self.file_writer.write(serialized.as_bytes())?;
            self.file_writer.flush();
            let command_ops = CommandOps {
                file_id: self.current_active_log,
                pos: old_len,
                len: self.file_writer.pos - old_len,
            };
            self.data_map.remove(key.as_str());
            if len as u64 > LOG_SIZE {
                self.swap_active_file()?
            }
            Ok(())
        }
    }


    fn sorted_file_keys(&self) -> Result<Vec<u64>> {
        let mut keys: Vec<u64> = self.file_readers.keys().map(|&k| k).collect();
        keys.sort_unstable();
        Ok(keys)
    }

    fn swap_active_file(&mut self) -> Result<()> {
        let keys: Vec<u64> = self.sorted_file_keys()?;
        let current_active_file_id = keys.last().unwrap() + 1;
        self.file_writer.writer.flush();
        let new_active_file_id = current_active_file_id + 1;
        let new_active_file = File::create(get_file_name_from_log_id(new_active_file_id, self.log_dir.as_path()))?;
        self.file_writer = BufWriterWithPos::new(new_active_file)?;
        let current_active_file = File::open(get_file_name_from_log_id(current_active_file_id, self.log_dir.as_path()))?;
        self.file_readers.insert(current_active_file_id, BufReaderWithPos::new(current_active_file)?);
        Ok(())
    }

    fn read_log(&mut self) -> Result<()> {
        let mut keys = self.sorted_file_keys()?;
        for log_file_id in keys {
            let reader = self.file_readers.get_mut(&log_file_id).unwrap();
            let mut pos = reader.seek(SeekFrom::Start(0))?;
            let mut stream = Deserializer::from_reader(reader).into_iter::<CommandLog>();
            while let Some(command) = stream.next() {
                let new_pos = stream.byte_offset() as u64;
                match command? {
                    CommandLog::Set {
                        key, value
                    } => {
                        self.data_map.insert(key, CommandOps {
                            pos,
                            len: new_pos - pos,
                            file_id: log_file_id,
                        })
                    }
                    CommandLog::Remove { key } => {
                        self.data_map.remove(key.as_str())
                    }
                };
                pos = new_pos
            }
        }
        Ok(())
    }


    fn get_log_files_sorted(path: &Path) -> Result<Vec<u64>> {
        let mut logs: Vec<u64> = fs::read_dir(path)?
            .flat_map(|res| -> Result<_> { Ok(res?.path()) })
            .filter(|path| { path.is_file() && path.extension().unwrap() == "log" })
            .map(|path| {
                let filename = path.file_name().unwrap().to_str().unwrap();
                String::from(filename).trim_end_matches(".log").parse::<u64>()
            })
            .flatten()
            .collect();
        logs.sort_unstable();
        Ok(logs)
    }
}

fn get_file_name_from_log_id(id: u64, base: &Path) -> PathBuf {
    base.join(format!("{}.log", id))
}


#[derive(Copy, Clone)]
struct CommandOps {
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
