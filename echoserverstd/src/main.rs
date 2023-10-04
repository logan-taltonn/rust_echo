use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::env::args; 
use std::{thread, time::Duration};

// intermediary - delay sim
// acts as a toy real-world delay performing computation or otherwise that would cause service of incoming streams into the server to
    // not be able to service in time -> needing asynch io
// link between client and server are on port 8001 | link between server and delay sim on port 8000
// introduces delay to simulate computation & to showcase async nature

// consts
const INTERMEDIARY_SERVER_ADDR: &str = "127.0.0.1:8000";

fn main() {
    // read args -> introduce delay to simulate real-world scenario
    let delay = args().nth(1).unwrap_or_default().parse::<u64>().unwrap_or_default(); // read in a delay if passed essentially, lots of type casting
    // start connection + delay prompt
    println!("echo server std starting {}", INTERMEDIARY_SERVER_ADDR);
    println!("echo server has a delay of {} ms", delay);
    // bind to any addr 
    let listener = TcpListener::bind(INTERMEDIARY_SERVER_ADDR).unwrap();
    // prompt
    println!("echo server std listening {}", INTERMEDIARY_SERVER_ADDR);
    // cycle thru incoming streams
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, delay);
        // println!("connection established."); // nc 127.0.0.1 8000 in another terminal
    }
}

fn handle_connection(mut stream:TcpStream, delay:u64) {
    // read buf
    let mut buf = [0;1024];
    let len = stream.read(&mut buf).unwrap();
    let msg = String::from_utf8_lossy(&buf[..len]);
    println!("received: {}", msg);
    // introduce delay, sleep thread
    thread::sleep(Duration::from_millis(delay));
    // write to buf
    let _ = stream.write_all(msg.as_bytes());
    println!("sent: {}", msg);
}