// Importing necessary modules from the Rust libraries
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // this is a buffer to read data from the client
    let mut buffer = [0; 1024];
    // this line reads data from the stream and stores it in the buffer
    stream.read(&mut buffer).expect("Failed to read from client");
    // this line converts the data in the buffer into a UTF-8 encoded string.
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    // Response content
    let content = "Hello, Client!";
    // Basic HTTP response
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
        content.len(),
        content
    );

    stream.write(response.as_bytes()).expect("Failed to write response!");
}

// Entry point
fn main() {
    // Server running and listen on :8080
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream{
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}