// use std::io::prelude::*;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;

// client
// role: send messages to server (currently hardcoded message -> change to whatever message you want)
// connects to server port 8001, where the user interacts with server
// details: send streams to server

const ECHO_SERVER_ADDRESS: &str = "127.0.0.1:8001";

#[tokio::main]
async fn main() {
    // create connection
    println!("connecting to {}", ECHO_SERVER_ADDRESS); // maybe change
    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        // connected
        println!("connected to echo server {}:{}", stream.local_addr().unwrap().ip(), stream.local_addr().unwrap().port());
        // write resp to
        let msg = "Hello World";
        let _ = stream.write(msg.as_bytes()).await; // can do write_all instead of flush below
        let _ = stream.flush().await;
        println!("sent: {}", msg);
        // read res back
        let mut buf = [0;1024];
        let len = stream.read(&mut buf).await.unwrap();
        let msg = String::from_utf8_lossy(&buf);
        println!("receieved: {}", msg);
    } else {
        println!("failed to connect to echo server {}" , ECHO_SERVER_ADDRESS);
    }

}