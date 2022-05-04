use clap::{Parser, Subcommand};
use rusthole::{exec_receiver, exec_sender};
use std::error::Error;

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
