use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, net::SocketAddr};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

#[derive(Debug, Serialize, Deserialize)]
enum Requester {
    Sender,
    Receiver,
}

#[derive(Serialize, Deserialize)]
struct ReceiverData {
    requester: Requester,
    secret_phrase: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut phrases = HashMap::<String, SocketAddr>::new();

    let ip = "127.0.0.1";
    let port = 8081;

    // Start listener
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();

    // Clean terminal
    print!("{esc}c", esc = 27 as char);
    // Wait for connection
    println!("Sync server accepting connection at {}:{}", ip, port);

    loop {
        let (mut stream, addr) = listener.accept().await.unwrap();
        let (stream_reader, mut stream_writer) = stream.split();

        let mut reader = BufReader::new(stream_reader);
        let mut buffer = vec![0; 1024];

        // Read secret phrase
        let num_bytes = reader.read(&mut buffer[..]).await.unwrap();

        // Store the secret phrase and corresponding IP in memory
        if num_bytes > 0 {
            let data: ReceiverData = serde_json::from_slice(&buffer[..num_bytes])?;
            let requester: Requester = data.requester;
            let secret_phrase: String = data.secret_phrase;

            match requester {
                Requester::Sender => {
                    phrases.insert(secret_phrase, addr);

                    println!("HashMap content:");
                    for (key, value) in &phrases {
                        println!("{}: {}", key, value);
                    }
                }
                Requester::Receiver => {
                    if phrases.contains_key(&secret_phrase) {
                        // send back IP of sender to receiver
                        let sender_addr = phrases.get(&secret_phrase).unwrap();

                        let data = json!({
                            "ip": sender_addr.ip(),
                            "port": sender_addr.port(),
                        });

                        stream_writer
                            .write_all(data.to_string().as_bytes())
                            .await
                            .unwrap();
                    }
                }
            }
        }
    }
}
