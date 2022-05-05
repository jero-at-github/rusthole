use common::{ReceiverGetData, ReceiverSendData, Requester, SenderSendData};
use question::{Answer, Question};
use std::fs::metadata;
use std::{error::Error, path::Path};
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::secret_phrase::{get_phrase, print_secret_phrase};

mod secret_phrase;

const SYNC_SERVER_IP: &str = "127.0.0.1";
const SYNC_SERVER_PORT: &str = "8081";
const SENDER_PORT: &str = "8080";

pub async fn exec_sender(path: String) -> Result<(), Box<dyn Error>> {
    let local_ip = "127.0.0.1";

    // Check if the path exists
    let path_file = Path::new(path.as_str());
    let file_size = metadata(path_file)?.len();
    let file_name = path_file.file_name().unwrap().to_str().unwrap().to_string();
    let path_exists = path_file.exists();

    if !path_exists {
        return Err("Path doesn't exist!")?;
    }

    // Clean terminal
    print!("{esc}c", esc = 27 as char);

    // Generate the secret phrase
    let secret_phrase = get_phrase().unwrap();

    // Send secret phras to sync server
    connect_sender_to_server(&secret_phrase, file_name.clone(), file_size).await?;

    // Start listener
    let listener = TcpListener::bind(format!("{}:{}", local_ip, SENDER_PORT))
        .await
        .unwrap();

    // Print file size, file name and secret phrase
    println!("Sending {:?} Bytes file named {:?}", file_size, file_name);
    print_secret_phrase(&secret_phrase);

    // Wait for connection
    let (mut stream, _addr) = listener.accept().await.unwrap();

    // Send file content
    let contents = fs::read(path).await?;
    stream.write_all(contents.as_slice()).await.unwrap();

    Ok(())
}

async fn connect_sender_to_server(
    secret_phrase: &str,
    file_name: String,
    file_size: u64,
) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(format!("{}:{}", SYNC_SERVER_IP, SYNC_SERVER_PORT)).await?;

    let data = SenderSendData {
        ip: stream.peer_addr().unwrap().ip().to_string(),
        port: stream.peer_addr().unwrap().port(),
        requester: Requester::Sender,
        secret_phrase: secret_phrase.to_string(),
        file_name,
        file_size,
    };

    let data_serialized = serde_json::to_string(&data).unwrap();
    stream.write_all(data_serialized.as_bytes()).await.unwrap();

    Ok(())
}

pub async fn exec_receiver(secret_phrase: &str) -> Result<(), Box<dyn Error>> {
    print!("{esc}c", esc = 27 as char);
    let data: ReceiverGetData = connect_recv_to_server(secret_phrase).await?;

    let path = format!("./{}", data.file_name);
    //TODO: check if file already exists, in affirmative case abort
    File::create(&path).await?;

    println!(
        "Receiving file ({} Bytes) into: {}",
        data.file_size, data.file_name
    );

    let answer = Question::new("Ok?")
        .default(Answer::YES)
        .show_defaults()
        .confirm();

    if answer == Answer::YES {
    } else {
        //TODO: println!("Aborting...");
    }
    //TODO: 100%|██████████████████████████████████████████████████████████████████████████████████████████████| 14.0/14.0 [00:00<00:00, 95.5B/s]

    // Connecting to listener
    let stream = TcpStream::connect(format!("{}:{}", data.ip, SENDER_PORT)).await?;
    println!("Receiving (->tcp:{:?})", stream.local_addr());

    // Read streamed content file
    let mut reader = BufReader::new(stream);
    let mut reader_content = Vec::new();
    reader.read_to_end(&mut reader_content).await.unwrap();

    // Write content file
    fs::write(path, reader_content).await?;

    println!("Received file written to {}", data.file_name);

    Ok(())
}

async fn connect_recv_to_server(secret_phrase: &str) -> Result<ReceiverGetData, Box<dyn Error>> {
    // Connect to sync server
    let mut stream = TcpStream::connect(format!("{}:{}", SYNC_SERVER_IP, SYNC_SERVER_PORT)).await?;

    // Send data
    let data = ReceiverSendData {
        requester: Requester::Receiver,
        secret_phrase: secret_phrase.to_string(),
    };
    let data_serialized = serde_json::to_string(&data).unwrap();
    stream.write_all(data_serialized.as_bytes()).await.unwrap();

    // Read data
    let mut reader = BufReader::new(stream);
    let mut buffer = vec![0; 1024];

    let num_bytes = reader.read(&mut buffer[..]).await.unwrap();

    if num_bytes > 0 {
        let data: ReceiverGetData = serde_json::from_slice(&buffer[..num_bytes])?;

        return Ok(ReceiverGetData {
            ip: data.ip,
            port: data.port,
            file_name: data.file_name,
            file_size: data.file_size,
        });
    } else {
        return Err("Error in receiver: no response from sync-server.")?;
    }
}
