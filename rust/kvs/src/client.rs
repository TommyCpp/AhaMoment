use std::net::{TcpStream, SocketAddr};
use crate::{Result, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, error};

pub struct KvClient {
    stream: TcpStream
}

impl KvClient {
    pub fn new(addr: SocketAddr) -> Result<KvClient> {
        Ok(
            KvClient {
                stream: TcpStream::connect(addr)?,
            }
        )
    }

    pub fn set(&mut self, key: String, value: String) {
        let request = Request::Set {
            key,
            value,
        };

        self.send_request(request);
    }

    pub fn get(&mut self, key: String) {
        let request = Request::Get {
            key
        };

        self.send_request(request);
    }

    pub fn remove(&mut self, key: String){
        let request = Request::Remove {
            key
        };

        self.send_request(request);
    }

    fn send_request(&mut self, request: Request) {
        serde_json::to_writer(&self.stream, &request); //set request

        let mut de = serde_json::Deserializer::from_reader(&self.stream);
        match Response::deserialize(&mut de) {
            Ok(response) => {
                match response {
                    Response::Success { val } => {
                        match request {
                            Request::Set { key, value } => {
                                info!("Insert key {}, value {}", key, value)
                            }
                            Request::Get { key } => {
                                info!("The value of key {} is {}", key, val)
                            }
                            Request::Remove { key } => {
                                info!("The value with key {} is removed", key)
                            }
                        }
                    }
                    Response::Error { error } => {
                        error!("{}", error)
                    }
                }
            }
            Err(err) => {
                error!("{}", err)
            }
        }
    }
}