use crate::{KvsEngine, SLED_STORE_NAME, Request, Response};
use crate::engine::kv::KvStore;
use crate::engine::sled::SledStore;
use std::path::Path;
use crate::error::KvError;
use crate::Result;
use std::net::{SocketAddr, TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, error};
use std::io::{BufReader, BufWriter, Write};
use crate::thread_pool::{ThreadPool, NaiveThreadPool};


pub struct KvServer<T: KvsEngine, P: ThreadPool> {
    engine: T,
    addr: SocketAddr,
    thread_pool: P,
}

impl<T: KvsEngine, P: ThreadPool> KvServer<T, P> {
    pub fn new(addr: SocketAddr, engine: T, thread_pool: P) -> Result<KvServer<T, P>> {
        Ok(KvServer {
            engine,
            addr,
            thread_pool,
        })
    }

    pub fn serve(&self) {
        let listener = TcpListener::bind(self.addr).unwrap();
        for stream in listener.incoming() {
            let engine = self.engine.clone();
            self.thread_pool.spawn(move || {
                match stream {
                    Ok(stream) => {
                        if let Err(err) = handle(engine, stream) {
                            error!("Error:{}", err)
                        }
                    }
                    Err(err) => {
                        error!("Network connection error")
                    }
                }
            })
        }
    }
}

fn handle<E: KvsEngine>(engine: E, stream: TcpStream) -> Result<()> {
    let mut de = serde_json::Deserializer::from_reader(&stream);
    match Request::deserialize(&mut de) {
        Ok(command) => {
            match command {
                Request::Get { key } => {
                    match engine.get(key) {
                        Ok(v) => {
                            let res = v.unwrap_or(String::from("Key not found"));
                            let response = Response::Success {
                                val: res,
                            };
                            serde_json::to_writer(&stream, &response);
                            Ok(())
                        }
                        Err(err) => {
                            let response = Response::Error {
                                error: format!("{}", err)
                            };
                            serde_json::to_writer(&stream, &response);
                            Err(err)
                        }
                    }
                }
                Request::Remove { key } => {
                    if let Err(err) = engine.remove(key) {
                        serde_json::to_writer(&stream, &Response::Error {
                            error: format!("{}", err)
                        });
                        Err(err)
                    } else {
                        serde_json::to_writer(&stream, &Response::Success {
                            val: String::from("")
                        });
                        Ok(())
                    }
                }
                Request::Set { key, value } => {
                    engine.set(key, value)?;
                    serde_json::to_writer(&stream, &Response::Success {
                        val: String::from("")
                    });
                    Ok(())
                }
            }
        }
        Err(err) => Err(KvError::SerdeError(err))
    }
}

