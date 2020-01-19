use crate::{KvsEngine, SLED_STORE_NAME, CommandProtocol};
use crate::engine::kv::KvStore;
use crate::engine::sled::SledStore;
use std::path::Path;
use crate::error::KvError;
use crate::Result;
use std::net::{SocketAddr, TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, error};
use std::io::{BufReader, BufWriter, Write};


pub struct KvServer<T> {
    engine: T,
    addr: SocketAddr,
}

impl<T: KvsEngine> KvServer<T> {
    pub fn new(addr: SocketAddr, engine: T) -> Result<KvServer<T>> {
        Ok(KvServer {
            engine,
            addr,
        })
    }

    pub fn serve(&mut self) {
        let listener = TcpListener::bind(self.addr).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(err) = self.handle(stream) {
                        error!("Error:{}", err)
                    }
                }
                Err(err) => {
                    error!("Network connection error")
                }
            }
        }
    }

    fn handle(&mut self, stream: TcpStream) -> Result<()> {
        let mut de = serde_json::Deserializer::from_reader(&stream);
        match CommandProtocol::deserialize(&mut de) {
            Ok(command) => {
                match command {
                    CommandProtocol::Get { key } => {
                        match self.engine.get(key) {
                            Ok(v) => {
                                let mut writer = BufWriter::new(&stream);
                                let res = v.unwrap_or(String::from("None"));
                                writer.write(res.as_bytes());
                                Ok(())
                            }
                            Err(err) => Err(err)
                        }
                    }
                    CommandProtocol::Remove { key } => {
                        self.engine.remove(key)?;
                        Ok(())
                    }
                    CommandProtocol::Set { key, value } => {
                        self.engine.set(key, value)?;
                        Ok(())
                    }
                }
            }
            Err(err) => Err(KvError::SerdeError(err))
        }
    }
}

