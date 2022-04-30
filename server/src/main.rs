use std::net::SocketAddr;

use tokio::{net::TcpListener, sync::broadcast};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(1);
}
