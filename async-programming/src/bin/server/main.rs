use async_std::io::{self, prelude::*};
use async_std::{net, task};
use std::error::Error;
use std::thread;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let server_task = tokio::spawn(async move {
        if let Err(err) = server().await {
            eprintln!("Server error: {}", err);
        }
    });

    let client_task = tokio::spawn(async move {
        if let Err(err) = client().await {
            eprintln!("Client error: {}", err);
        }
    });

    if let Err(err) = tokio::try_join!(server_task, client_task) {
        eprintln!("Join error: {}", err);
    }

    Ok(())
}

async fn server() -> Result<(), Box<dyn Error + Send + Sync>> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Listening on port 8080...");

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            //let mut buffer = [0; 1024];

            //let bytes_read = socket.read(&mut buffer).await?;
            //println!(
            //    "Received: {}",
            //    String::from_utf8_lossy(&buffer[..bytes_read])
            //);

            //let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
            //socket.write_all(response.as_bytes()).await?;
            socket.write("received".as_bytes()).await?;

            drop(socket);
            Ok::<(), Box<dyn Error + Send + Sync>>(())
        });
    }
}

async fn client() -> Result<(), Box<dyn Error + Send + Sync>> {
    let server_addr = "127.0.0.1:8080";
    let mut server_socket = TcpStream::connect(server_addr).await?;

    let message = "Hello from client!";
    server_socket.write_all(message.as_bytes()).await?;

    let mut buffer = [0; 1024];
    let bytes_read = server_socket.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Server response: {}", response);

    Ok(())
}
