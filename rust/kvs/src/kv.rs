use std::collections::{HashMap, BTreeMap};
use crate::{Result, Error};
use std::path::{Path, PathBuf};
use std::{fs as fs, io};
use std::fs::{File, OpenOptions};
use serde::{Serialize, Deserialize};
use std::io::{Write, Read, LineWriter, BufReader, SeekFrom, BufRead, Seek, BufWriter};
use std::ops::Add;
use serde_json::Deserializer;

pub struct KvStore {
    data_map: BTreeMap<String, CommandOps>,
    file_readers: HashMap<u64, BufReaderWithPos>,
    log: PathBuf,
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
        let logs = KvStore::get_log_files_sorted(path)?;
        let mut readers_map = HashMap::<u64, BufReaderWithPos>::new();
        for log_file_name in logs {
            let reader = BufReaderWithPos::new(fs::File::open(get_file_name_from_log_id(log_file_name, path))?)?;
            readers_map.insert(log_file_name, reader);
        }
        let mut store = KvStore {
            data_map: BTreeMap::new(),
            file_readers: readers_map,
            log: path.to_path_buf(),
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
                self.append_log(CommandLog::Remove { key });
                Ok(())
            }
            None => {
                Err(Error::NotFoundError)
            }
        }
    }

    fn append_log(&mut self, log: CommandLog) -> Result<()> {
        let mut serialized = serde_json::to_string(&log).unwrap();
        serialized = serialized.add("\n");
        let mut file = self.open_file(false)?;
        match file.write_all(serialized.as_bytes()) {
            Ok(()) => Ok(()),
            Err(why) => panic!("Cannot write file, {}", why)
        }
    }

    fn read_log(&mut self) -> Result<()> {
        let mut keys: Vec<u64> = self.file_readers.keys().map(|&k| k).collect();
        keys.sort_unstable();
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
