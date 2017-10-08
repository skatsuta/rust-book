extern crate hello;

use hello::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs::File;

const HOST_PORT: &str = "127.0.0.1:8080";
const MAIN_HTML: &str = "hello.html";
const NOT_FOUND_HTML: &str = "404.html";
const GET_REQUEST: &[u8] = b"GET / HTTP/1.1\r\n";
const THREAD_COUNT: usize = 4;

fn main() {
    let listener = TcpListener::bind(HOST_PORT).unwrap();
    let pool = ThreadPool::new(THREAD_COUNT);

    println!("Server has started listening on port 8080...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| { handle_connection(stream); });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let (status_line, filename) = if buffer.starts_with(GET_REQUEST) {
        ("HTTP/1.1 200 OK", MAIN_HTML)
    } else {
        ("HTTP/1.1 404 NOT FOUND", NOT_FOUND_HTML)
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}\r\n\r\n{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
