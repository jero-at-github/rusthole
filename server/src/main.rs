use tokio::{fs, io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let ip = "127.0.0.1";
    let port = 8080;

    // Start listener
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();

    // Wait for connection
    println!("Sync server accepting connection at {}:{}", ip, port);
    let (mut socket, _addr) = listener.accept().await.unwrap();

    Ok(())
}
