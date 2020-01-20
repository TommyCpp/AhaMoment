use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use crate::KvError;
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Request {
    Get { key: String },
    Remove { key: String },
    Set { key: String, value: String },
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Response {
    Success { val: String },
    Error { error: String },
}

impl Response {
    pub fn success(val: String) -> Response {
        Response::Success {
            val
        }
    }

    pub fn error(error: KvError) -> Response {
        Response::Error {
            error: format!("{}", error)
        }
    }
}