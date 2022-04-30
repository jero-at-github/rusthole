use std::{io::Read, net::SocketAddr};

use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (mut socket, addr) = listener.accept().await.unwrap();

    let (socket_reader, mut socket_writer) = socket.split();

    let contents = fs::read("samples/server/data.txt").await?;

    // let mut buffer = BytesMut::with_capacity(10);

    socket_writer.write_all(contents.as_slice()).await.unwrap();
    //let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(1);

    Ok(())
}
