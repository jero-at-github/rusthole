use clap::{Parser, Subcommand};
use std::error::Error;
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
    /// Receives a file
    Receive,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Commands::Send { path } => exec_server(path.clone()).await,
        Commands::Receive => exec_client().await,
    }
}

async fn exec_client() -> Result<(), Box<dyn Error>> {
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

async fn exec_server(path: String) -> Result<(), Box<dyn std::error::Error + 'static>> {
    // let ip = public_ip::addr().await.unwrap();
    let ip = "127.0.0.1";
    let port = 8080;

    // Start listener
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();

    // Wait for connection
    println!("Accepting connection at {}:{}", ip, port);
    let (mut socket, _addr) = listener.accept().await.unwrap();

    // Send file content
    let (_socket_reader, mut socket_writer) = socket.split();
    let contents = fs::read(path).await?;
    socket_writer.write_all(contents.as_slice()).await.unwrap();

    Ok(())
}
