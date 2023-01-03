use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let connection_listener =
        TcpListener::bind("127.0.0.1:3000").expect("Failed to bind to address");
    println!("Running on port 3000");

    for stream in connection_listener.incoming() {
        let mut stream = stream.expect("Error on connection stream");
        println!("Connection established");

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
