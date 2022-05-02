use std::{collections::HashMap, net::IpAddr};

use tokio::{
    io::{AsyncBufReadExt, BufReader},
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
        let _num_bytes = reader.read_line(&mut reader_content).await.unwrap();

        // Check if the secret phrase is well-formed

        // Store the secret phrase and corresponding IP  in memory
        // if num_bytes == 0 {
        //     return Err(Box("No"));
        // }
        phrases.insert(reader_content, addr.ip());

        println!(
            "Received connection request for secret phrase: {:?}",
            phrases
        );
    }
}
