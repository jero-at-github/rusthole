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

/*

#[derive(Debug, Serialize, Deserialize)]
enum Requester {
    Sender,
    Receiver,
    None,
}

impl From<String> for Requester {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "sender" => Requester::Sender,
            "receiver" => Requester::Receiver,
            _ => Requester::None,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ReceiverSendData {
    requester: Requester,
    secret_phrase: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SenderSendData {
    requester: Requester,
    secret_phrase: String,
    file_name: String,
    file_size: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct HashMapData {
    ip: String,
    port: u16,
    file_name: String,
    file_size: u64,
}

#[derive(Serialize, Deserialize)]
struct ReceiverGetData {
    ip: String,
    port: u16,
    file_name: String,
    file_size: u64,
}
*/
