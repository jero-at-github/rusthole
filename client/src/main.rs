use clap::{Parser, Subcommand};
use rand::Rng;
use std::error::Error;
use std::fs::File as StdFile;
use std::io::BufReader as StdBufReader;
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

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
    Receive,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Commands::Send { path } => exec_sender(path.clone()).await,
        Commands::TestSend => exec_sender("./samples/client/received.mp4".into()).await,
        Commands::Receive => exec_receiver().await,
    }
}

async fn exec_receiver() -> Result<(), Box<dyn Error>> {
    let host = "88.152.112.218";
    let port = 8080;
    let path = "../../samples/client/received.mp4";

    File::create(path).await?;

    // Connecting to listener
    let stream = TcpStream::connect(format!("{}:{}", host, port)).await?;

    // Read streamed content file
    let mut reader = BufReader::new(stream);
    let mut reader_content = Vec::new();
    reader.read_to_end(&mut reader_content).await.unwrap();

    // Write content file
    fs::write(path, reader_content).await?;

    Ok(())
}

async fn exec_sender(path: String) -> Result<(), Box<dyn Error>> {
    let ip = "127.0.0.1";
    let port = 8080;

    // Start listener
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();

    // Generate and print the secret phrase
    let secret_phrase = get_phrase().unwrap();
    print_secret_phrase(secret_phrase);

    // Wait for connection
    println!("Accepting connection at {}:{}", ip, port);
    let (mut socket, _addr) = listener.accept().await.unwrap();

    // Send file content
    let (_socket_reader, mut socket_writer) = socket.split();
    let contents = fs::read(path).await?;
    socket_writer.write_all(contents.as_slice()).await.unwrap();

    Ok(())
}

fn print_secret_phrase(secret_phrase: String) {
    println!("Rusthole code is: {}", secret_phrase);
    println!("On the other computer, please run:");
    println!("");
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
        rng.gen_range(0..=9 as u8),
        bip39_list["list"][rng.gen_range(0..=2047)]
            .as_str()
            .unwrap(),
        bip39_list["list"][rng.gen_range(0..=2047)]
            .as_str()
            .unwrap()
    );

    Ok(phrase)
}
