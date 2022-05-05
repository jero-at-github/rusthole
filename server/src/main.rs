use common::{HashMapData, ReceiverSendData, Requester, SenderSendData};
use serde_json::Value;
use std::collections::HashMap;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut phrases = HashMap::<String, HashMapData>::new();

    let ip = "127.0.0.1";
    let port = 8081;

    // Start listener
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();

    // Clean terminal
    print!("{esc}c", esc = 27 as char);
    // Wait for connection
    println!("Sync server accepting connection at {}:{}", ip, port);

    loop {
        let (mut stream, _addr) = listener.accept().await.unwrap();
        let (stream_reader, mut stream_writer) = stream.split();

        let mut reader = BufReader::new(stream_reader);
        let mut buffer = vec![0; 1024];

        // Read secret phrase
        let num_bytes = reader.read(&mut buffer[..]).await.unwrap();

        // Store the secret phrase and corresponding IP in memory
        if num_bytes > 0 {
            let data: Value = serde_json::from_slice(&buffer[..num_bytes])?;
            let requester = Requester::from(data["requester"].to_string());

            match requester {
                Requester::Sender => {
                    let data: SenderSendData = serde_json::from_slice(&buffer[..num_bytes])?;
                    phrases.insert(
                        data.secret_phrase,
                        HashMapData {
                            ip: data.ip,
                            port: data.port,
                            file_name: data.file_name,
                            file_size: data.file_size,
                        },
                    );

                    println!("HashMap content:");
                    for (key, value) in &phrases {
                        println!("{}: {:?}", key, value);
                    }
                }
                Requester::Receiver => {
                    let data: ReceiverSendData = serde_json::from_slice(&buffer[..num_bytes])?;
                    if phrases.contains_key(&data.secret_phrase) {
                        // send back IP of sender to receiver
                        let data = phrases.get(&data.secret_phrase).unwrap();
                        let data_serialized = serde_json::to_string(&data)?;

                        stream_writer
                            .write_all(data_serialized.as_bytes())
                            .await
                            .unwrap();
                    }
                }
                Requester::None => {
                    Err("Requester can't be of type None")?;
                }
            }
        }
    }
}
