use std::io::{Read, Write};
use std::net::TcpListener;
use std::process::exit;

fn main() {
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000");
    for stream in connection_listener.incoming() {
        let mut stream = stream.unwrap_or_else(move |e| {
            eprintln!("Error: {}", e);
            exit(1);
        });
        println!("Connection established");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            exit(1);
        });
        stream.write(&mut buffer).unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            exit(1);
        });
    }
}
