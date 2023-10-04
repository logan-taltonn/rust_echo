use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt,AsyncReadExt};
use std::str::FromStr;
use uuid::Uuid;

// server
// role: read client inputs, send them back to client, ONLY AFTER sending to intermediary delay sim and getting a response back
// creates a unique uuid for that incoming stream from client or elsewhere, loops through incoming streams and spawns off a new async thread
    // once spawned, will move to next stream while previous one still processing, concurrently
    // so multiple instances of client or netcat all sending messages at the same time can be services in parallel
// must wait on intermediary delay -> to simulate real-world computation
// details: spawns off new async threads per incoming stream (& uuid) and sends back message to client (after delay)

// constants
const ECHO_SERVER_ADDR: &str = "127.0.0.1:8001";
const INTERMEDIARY_SERVER_ADDR: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    // start connection
    println!("echo server tokio starting {}", ECHO_SERVER_ADDR);
    // bind to addr
    let listener = TcpListener::bind(ECHO_SERVER_ADDR).await.unwrap();
    // listening prompt
    println!("echo server tokio listening {}", ECHO_SERVER_ADDR);
    // cycle thru incoming stream connections
    loop {
        // accept stream connection
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    // creaete unique id
    let id = Uuid::new_v4();
    // read buf
    let mut buffer = [0; 1024];
    let len = stream.read(&mut buffer).await.unwrap();
    let msg = String::from_utf8_lossy(&buffer[..len]);
    println!("{} - received: {}", id, msg);
    // call intemediary
    let itm_msg = call_intermediary(id, msg.to_owned().to_string()).await;
    let out = format!("intemediary says: {}", itm_msg);
    // write to buf
    let _ = stream.write_all(out.as_bytes()).await;
    println!("{} - sent: {}", id, msg);
}

async fn call_intermediary(id:Uuid, msg: String) -> String {
    println!("{} - connecting to echo server: {}",id, INTERMEDIARY_SERVER_ADDR);
    // connected or not
    if let Ok(mut stream) = TcpStream::connect(INTERMEDIARY_SERVER_ADDR).await {
        // connect success
        println!("{} - connected to intermediary: {}:{}", id, stream.local_addr().unwrap().ip(), stream.local_addr().unwrap().port());
        // hardcode msg to tokio echo server
        let _ = stream.write_all(msg.as_bytes()).await;
        println!("{} - sent: {}", id, msg);
        // read res
        let mut buf = [0; 1024];
        let len = stream.read(&mut buf).await.unwrap();
        let msg = String::from_utf8_lossy(&buf[..len]); 
        println!("{} - received from intermediary: {}", id, msg);
        // return msg
        return msg.to_owned().to_string();
    } else {
        // intermediary is not available
        println!("{} - couldn't connect to intermediary: {}", id, INTERMEDIARY_SERVER_ADDR);
        return String::from_str("intermediary is not available").unwrap();
    }
}