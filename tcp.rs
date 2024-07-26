// Importing necessary modules from the Rust libraries
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // this is a buffer to read data from the client
    let mut buffer = [0; 1024];
    // this line reads data from the stream and stores it in the buffer
    stream.read(&mut buffer).expect("Failed to read from client");

    let request = String::from_utf8_lossy(&buffer[..])
}