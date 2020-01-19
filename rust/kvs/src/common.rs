use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum CommandProtocol {
    Get { key: String },
    Remove { key: String },
    Set { key: String, value: String },
}