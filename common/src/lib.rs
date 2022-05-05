use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Requester {
    Sender,
    Receiver,
    None,
}

impl From<String> for Requester {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "sender" => Requester::Sender,
            "\"sender\"" => Requester::Sender,
            "receiver" => Requester::Receiver,
            "\"receiver\"" => Requester::Receiver,
            _ => Requester::None,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ReceiverSendData {
    pub requester: Requester,
    pub secret_phrase: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SenderSendData {
    pub ip: String,
    pub port: u16,
    pub requester: Requester,
    pub secret_phrase: String,
    pub file_name: String,
    pub file_size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReceiverGetData {
    pub ip: String,
    pub port: u16,
    pub file_name: String,
    pub file_size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HashMapData {
    pub ip: String,
    pub port: u16,
    pub file_name: String,
    pub file_size: u64,
}
