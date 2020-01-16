use std::{fs, io};
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

use crate::{Error, Result};
use crate::Error::NotFoundError;

const LOG_SIZE: u64 = 1024 * 10; //byte


pub struct KvStore {
    data_map: BTreeMap<String, CommandOps>,
    file_readers: HashMap<u64, BufReaderWithPos>,
    file_writer: BufWriterWithPos,
    current_active_log: u64,
    log_dir: PathBuf,
    log_size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum CommandLog {
    Set { key: String, value: String },
    Remove { key: String },
}

impl KvStore {
    pub fn open(path: &Path) -> Result<KvStore> {
        let log_file_names = KvStore::get_log_files_sorted(path)?;
        let mut readers_map = HashMap::<u64, BufReaderWithPos>::new();
        let active_file_name: u64 = log_file_names.last().unwrap_or(&0) + 1;
        let mut old_log_size = 0 as u64;
        for log_file_name in log_file_names {
            let file = fs::File::open(get_file_name_from_log_id(log_file_name, path))?;
            old_log_size += file.metadata().unwrap().len();
            let reader = BufReaderWithPos::new(file)?;

            readers_map.insert(log_file_name, reader);
        }
        //create new log file
        let active_file = File::create(get_file_name_from_log_id(active_file_name, path))?;
        readers_map.insert(
            active_file_name,
            BufReaderWithPos::new(fs::File::open(get_file_name_from_log_id(
                active_file_name,
                path,
            ))?)?,
        );
        let mut store = KvStore {
            data_map: BTreeMap::new(),
            file_readers: readers_map,
            file_writer: BufWriterWithPos::new(active_file)?,
            current_active_log: active_file_name,
            log_dir: path.to_path_buf(),
            log_size: old_log_size,
        };

        store.read_log()?;
        Ok(store)
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        match self.data_map.get(&key) {
            Some(&command_ops) => {
                let reader_ref: &mut BufReaderWithPos;
                reader_ref = self.file_readers.get_mut(&command_ops.file_id).unwrap();
                reader_ref.seek(SeekFrom::Start(command_ops.pos));
                let chunk = reader_ref.take(command_ops.len);
                let command: CommandLog = serde_json::from_reader(chunk)?;
                match command {
                    CommandLog::Set { key: _, value } => Ok(Some(value)),
                    _ => Err(Error::InternalError(String::from(
                        "The log should not be remove command log",
                    ))),
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
        self.log_size += len as u64;
        let command_ops = CommandOps {
            file_id: self.current_active_log,
            pos: old_len,
            len: self.file_writer.pos - old_len,
        };
        self.data_map.insert(key, command_ops);
        if self.log_size > LOG_SIZE {
            self.compaction()?
        }
        Ok(())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if !self.data_map.contains_key(&key) {
            Err(NotFoundError)
        } else {
            let command_log = CommandLog::Remove { key: key.clone() };
            let mut serialized = serde_json::to_string(&command_log).unwrap();
            let old_len = self.file_writer.pos;
            let len = self.file_writer.write(serialized.as_bytes())?;
            self.file_writer.flush();
            self.log_size += len as u64;
            let _command_ops = CommandOps {
                file_id: self.current_active_log,
                pos: old_len,
                len: self.file_writer.pos - old_len,
            };
            self.data_map.remove(key.as_str());
            if self.log_size > LOG_SIZE {
                self.compaction()?
            }
            Ok(())
        }
    }

    fn sorted_file_keys(&self) -> Result<Vec<u64>> {
        let mut keys: Vec<u64> = self.file_readers.keys().map(|&k| k).collect();
        keys.sort_unstable();
        Ok(keys)
    }

    fn compaction(&mut self) -> Result<()> {
        self.file_writer.writer.flush();
        //Creating new file
        let archive_log_file = self.current_active_log + 1;
        self.file_writer = BufWriterWithPos::new(File::create(get_file_name_from_log_id(
            archive_log_file,
            self.log_dir.as_path(),
        ))?)?;
        self.current_active_log = archive_log_file;

        //Copy active log to new file
        let mut new_data_map = BTreeMap::new();
        let mut pos = self.file_writer.pos;
        for (key, command_ops) in self.data_map.iter() {
            let mut log = vec![0; command_ops.len as usize];
            let mut reader_ref: &mut BufReaderWithPos;
            reader_ref = self.file_readers.get_mut(&command_ops.file_id).unwrap();
            reader_ref.seek(SeekFrom::Start(command_ops.pos));
            let mut chunk = reader_ref.take(command_ops.len);
            chunk.read(log.as_mut_slice());

            let len = self.file_writer.write(log.as_slice())?;
            new_data_map.insert(
                key.clone(),
                CommandOps {
                    file_id: self.current_active_log,
                    pos,
                    len: len as u64,
                },
            );

            pos += len as u64;
        }
        self.file_writer.flush();

        //gather old file names
        let old_files = self.sorted_file_keys()?;

        //clean readers, close all reader of old files
        self.file_readers.clear();
        self.file_readers.insert(
            archive_log_file,
            BufReaderWithPos::new(File::open(get_file_name_from_log_id(
                archive_log_file,
                self.log_dir.as_path(),
            ))?)?,
        );

        //delete old files
        for file_id in old_files {
            let path = get_file_name_from_log_id(file_id, self.log_dir.as_path());
            fs::remove_file(path)?;
        }

        //replace data map
        self.data_map = new_data_map;

        //create new log file
        self.file_writer.writer.flush();
        //Creating new file
        let new_active_file_id = self.current_active_log + 1;
        self.file_writer = BufWriterWithPos::new(File::create(get_file_name_from_log_id(
            new_active_file_id,
            self.log_dir.as_path(),
        ))?)?;
        self.file_readers.insert(
            new_active_file_id,
            BufReaderWithPos::new(File::open(get_file_name_from_log_id(
                new_active_file_id,
                self.log_dir.as_path(),
            ))?)?,
        );
        self.current_active_log = new_active_file_id;

        //reset log size
        self.log_size = self.file_writer.pos;
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
                    CommandLog::Set { key, value: _ } => self.data_map.insert(
                        key,
                        CommandOps {
                            pos,
                            len: new_pos - pos,
                            file_id: log_file_id,
                        },
                    ),
                    CommandLog::Remove { key } => self.data_map.remove(key.as_str()),
                };
                pos = new_pos
            }
        }
        Ok(())
    }

    fn get_log_files_sorted(path: &Path) -> Result<Vec<u64>> {
        let mut logs: Vec<u64> = fs::read_dir(path)?
            .flat_map(|res| -> Result<_> { Ok(res?.path()) })
            .filter(|path| path.is_file() && path.extension().unwrap() == "log")
            .map(|path| {
                let filename = path.file_name().unwrap().to_str().unwrap();
                String::from(filename)
                    .trim_end_matches(".log")
                    .parse::<u64>()
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
    //The reason why we need pos is because the underlying cursor in File may be moved by other file pointer.
}

impl BufReaderWithPos {
    fn new(mut file: File) -> Result<Self> {
        let pos = file.seek(SeekFrom::Current(0))?;
        let reader = BufReader::new(file);
        Ok(BufReaderWithPos { reader, pos })
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
        Ok(BufWriterWithPos { writer, pos })
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
