use tokio::{fs, io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
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
    let contents = fs::read(sent_file_path).await?;
    socket_writer.write_all(contents.as_slice()).await.unwrap();

    Ok(())
}
