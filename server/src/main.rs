use serde_json::Value;
use std::{collections::HashMap, net::IpAddr};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut phrases = HashMap::<String, IpAddr>::new();

    let ip = "127.0.0.1";
    let port = 8081;

    // Start listener
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();

    // Wait for connection
    println!("Sync server accepting connection at {}:{}", ip, port);

    loop {
        let (socket, addr) = listener.accept().await.unwrap();

        let mut reader = BufReader::new(socket);
        let mut reader_content = String::new();

        // Read secret phrase
        let num_bytes = reader.read_line(&mut reader_content).await.unwrap();

        // Store the secret phrase and corresponding IP  in memory
        if num_bytes != 0 {
            let data: Value = serde_json::from_str(reader_content.as_str())?;
            let requester = data["requester"].to_string();
            let secret_phrase = data["secret_phrase"].to_string();

            if requester == "sender" {
                phrases.insert(secret_phrase, addr.ip());

                println!(
                    "Received connection request for secret phrase: {:?}",
                    reader_content
                );

                for (key, value) in &phrases {
                    println!("{}: {}", key, value);
                }
            } else {
                if phrases.contains_key(&secret_phrase) {
                    // send sender IP back
                }
            }
        }
    }
}
