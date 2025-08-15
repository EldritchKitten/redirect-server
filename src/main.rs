use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    println!("Initializing...");
    // 127.0.0.1:7878
    // 192.168.1.148:7878
    // 31.94.26.170
    let listener = TcpListener::bind("0.0.0.0:7878")
        .expect("Failed to bind TCP listener");
    let mut request_counter: u64 = 0;

    for stream in listener.incoming() {
        request_counter += 1;
        println!("=== Request {request_counter} Start ===");
        //let stream = stream.unwrap();
        if stream.is_ok() {
            let stream = stream.unwrap();
            handle_connection(stream);
        }
        println!("=== Request {request_counter} End ===");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    println!("Request: {http_request:#?}");

    let response = response_redirect();

    stream.write_all(response.as_bytes()).unwrap();
}

fn response_redirect() -> String {
    let location = "http://localhost:8000";

    let location_header = format!("Location: {location}");
    let headers = format!("{location_header}");

    let status_line = "HTTP/1.1 301 Moved Permanently";

    return format!("{status_line}\r\n{headers}");
}

fn response_possessed() -> String {
    let content = ":3";
    let length = content.len();

    let length_header = format!("Content-Length: {length}");
    let type_header = format!("Content-Type: text/plain");
    let headers = format!("{length_header}\r\n{type_header}");

    let status_line = "HTTP/1.1 200 OK";

    return format!("{status_line}\r\n{headers}\r\n\r\n{content}");
}

