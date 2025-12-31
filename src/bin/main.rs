use little_crab::ThreadPool;
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1907").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_request(stream);
        });
    }
}

fn handle_request(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, path) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "templates/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "templates/404.html")
    };
    let contents = fs::read_to_string(path).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
