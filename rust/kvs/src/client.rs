use std::net::{TcpStream, SocketAddr};
use crate::{Result, Request, Response, KvError};
use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, error};
use crate::Response::Error;
use std::process::exit;
use std::io::Read;

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

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let request = Request::Set {
            key,
            value,
        };

        if let Err(err) = self.handle_request(request) {
            Err(err)
        } else {
            Ok(())
        }
    }

    pub fn get(&mut self, key: String) -> Result<String> {
        let request = Request::Get {
            key
        };

        self.handle_request(request)
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let request = Request::Remove {
            key
        };

        if let Err(err) = self.handle_request(request) {
            Err(err)
        } else {
            Ok(())
        }
    }

    fn handle_request(&mut self, request: Request) -> Result<String> {
        serde_json::to_writer(&self.stream, &request); //set request

        let response: Response = serde_json::from_reader(&self.stream)?;
        match response {
            Response::Success { val } => {
                if let Request::Get { key } = request {
                    Ok(val)
                } else {
                    Ok(String::from(""))
                }
            }
            Response::Error { error } => {
                println!("{}",error);
                Err(KvError::NotFoundError)
            }
        }
    }
}