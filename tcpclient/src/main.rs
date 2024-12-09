use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::exit;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello".as_bytes()).unwrap_or_else(|e| {
        eprintln!("failed to write: {}", e);
        exit(1);
    });
    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap_or_else(|e| {
        eprintln!("failed to read: {}", e);
        exit(1);
    });
    println!(
        "Got response from server:{:?}",
        str::from_utf8(&buffer).unwrap();
    );
}
