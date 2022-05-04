use clap::{Parser, Subcommand};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use std::fs::File as StdFile;
use std::io::BufReader as StdBufReader;
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

#[derive(Serialize, Deserialize)]
struct ReceiverData {
    ip: String,
    port: u16,
}

/// Send files from computer to computer
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a file
    Send { path: String },
    /// Send a file in test mode
    TestSend,
    /// Receives a file
    Receive { secret_phrase: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Commands::Send { path } => exec_sender(path.clone()).await,
        Commands::TestSend => exec_sender("./samples/received.mp4".into()).await,
        Commands::Receive { secret_phrase } => exec_receiver(secret_phrase).await,
    }
}

const SYNC_SERVER_IP: &str = "127.0.0.1";
const SYNC_SERVER_PORT: &str = "8081";
const SENDER_PORT: &str = "8080";

async fn exec_receiver(secret_phrase: &str) -> Result<(), Box<dyn Error>> {
    let path = "./downloads/received.mp4";

    File::create(path).await?;

    print!("{esc}c", esc = 27 as char);

    let (ip, port) = connect_to_sync_server("receiver".into(), secret_phrase).await?;

    print!("ip: {} port: {}", ip, port);

    // Connecting to listener
    let stream = TcpStream::connect(format!("{}:{}", ip, SENDER_PORT)).await?;

    // Read streamed content file
    let mut reader = BufReader::new(stream);
    let mut reader_content = Vec::new();
    reader.read_to_end(&mut reader_content).await.unwrap();

    // Write content file
    fs::write(path, reader_content).await?;

    Ok(())
}

async fn exec_sender(path: String) -> Result<(), Box<dyn Error>> {
    let local_ip = "127.0.0.1";

    print!("{esc}c", esc = 27 as char);

    // Generate the secret phrase
    let secret_phrase = get_phrase().unwrap();

    // Send secret phras to sync server
    connect_to_sync_server("sender".into(), &secret_phrase).await?;

    // Start listener
    let listener = TcpListener::bind(format!("{}:{}", local_ip, SENDER_PORT))
        .await
        .unwrap();

    // Print the secret phrase
    print_secret_phrase(&secret_phrase);

    // Wait for connection
    let (mut stream, _addr) = listener.accept().await.unwrap();

    // Send file content
    let contents = fs::read(path).await?;
    stream.write_all(contents.as_slice()).await.unwrap();

    Ok(())
}

async fn connect_to_sync_server(
    requester: String,
    secret_phrase: &str,
) -> Result<(String, String), Box<dyn Error>> {
    let mut ip = String::new();
    let mut port = String::new();

    let mut stream = TcpStream::connect(format!("{}:{}", SYNC_SERVER_IP, SYNC_SERVER_PORT)).await?;

    let data = json!({
        "requester":  requester,
        "secret_phrase": secret_phrase,
    });

    stream.write_all(data.to_string().as_bytes()).await.unwrap();

    if requester == "receiver" {
        let mut reader = BufReader::new(stream);
        let mut buffer = vec![0; 1024];

        let num_bytes = reader.read(&mut buffer[..]).await.unwrap();

        if num_bytes > 0 {
            let data: ReceiverData = serde_json::from_slice(&buffer[..num_bytes])?;
            ip = data.ip;
            port = data.port.to_string();
        }
    }

    Ok((ip, port))
}

fn print_secret_phrase(secret_phrase: &str) {
    println!("Rusthole code is: {}", secret_phrase);
    println!("On the other computer, please run:");
    println!();
    println!("rusthole receive {}", secret_phrase);
}

fn get_phrase() -> Result<String, Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    let json_file = StdFile::open("./bip39-es.json")?;
    let json_reader = StdBufReader::new(json_file);
    let bip39_list: serde_json::Value =
        serde_json::from_reader(json_reader).expect("JSON was not well-formatted");

    let phrase = format!(
        "{}-{}-{}",
        rng.gen_range(0..=9u8),
        bip39_list["list"][rng.gen_range(0..=2047)]
            .as_str()
            .unwrap(),
        bip39_list["list"][rng.gen_range(0..=2047)]
            .as_str()
            .unwrap()
    );

    Ok(phrase)
}
