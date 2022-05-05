// use std::fmt::format;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    println!("Hello, world!");

    let linstener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in linstener.incoming() {
        let stream  = stream.unwrap();

        println!("connection established");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // let response="HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
