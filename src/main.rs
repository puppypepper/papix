use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let mut file = File::open("hello.html").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let mut file = File::open("404.html").unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
