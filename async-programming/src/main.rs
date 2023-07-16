use async_std::io::{self, prelude::*};
use async_std::{net, task};

fn main() {
    call();
}

fn call() {
    //if let Ok(res) = task::block_on(fetch_string(r#"https://www.google.com"#)) {
    //    println!("{}", res);
    //}
    let response = fetch_string("localhost:8080");
    task::block_on(async {
        if let Ok(res) = response.await {
            println!("{:?}", res);
        };
    });
}

async fn fetch_string(address: &str) -> io::Result<String> {
    let mut socket = net::TcpStream::connect(address).await?;
    let mut buf = String::new();
    socket.read_to_string(&mut buf).await?;
    Ok(buf)
}
