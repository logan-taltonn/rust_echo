use std::io::prelude::*;
use std::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "localhost:1234";

fn main() {
    // create connection
    println!("connecting to {}", ECHO_SERVER_ADDRESS); // maybe change
    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS) {
        // connected
        println!("connected to echo server {}:{}", stream.local_addr().unwrap().ip(), stream.local_addr().unwrap().port());
        // write resp to
        let msg = "Hello World";
        let _ = stream.write(msg.as_bytes());
        let _ = stream.flush();
        println!("sent: {}", msg);
        // read res back
        let mut buf = [0;1024];
        let len = stream.read(&mut buf).unwrap();
        let msg = String::from_utf8_lossy(&buf);
        println!("receieved: {}", msg);
    } else {
        println!("failed to connect to echo server {}" , ECHO_SERVER_ADDRESS);
    }

}