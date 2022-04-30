use clap::{Parser, Subcommand};
use std::error::Error;
use tokio::{
    fs,
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
    Receive { path: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Commands::Send { path } => exec_server(path.clone()).await,
        Commands::Receive { path } => exec_client().await,
    }
}

async fn exec_client() -> Result<(), Box<dyn Error>> {
    let host = "localhost";
    let port = 8080;
    let recv_path_file = "samples/client/received.txt";

    // Connecting to listener
    let stream = TcpStream::connect(format!("{}:{}", host, port)).await?;

    // Read streamed content file
    let mut reader = BufReader::new(stream);
    let mut reader_content = Vec::new();
    reader.read_to_end(&mut reader_content).await.unwrap();

    // Write content file
    fs::write(recv_path_file, reader_content).await?;

    Ok(())
}

async fn exec_server(path: String) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let host = "localhost";
    let port = 8080;
    let sent_file_path = "samples/server/video_sample.mp4";

    // Start listener
    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    // Wait for connection
    println!("Accepting connection at {}:{}", host, port);
    let (mut socket, _addr) = listener.accept().await.unwrap();

    // Send file content
    let (_socket_reader, mut socket_writer) = socket.split();
    let contents = fs::read(path).await?;
    socket_writer.write_all(contents.as_slice()).await.unwrap();

    Ok(())
}
