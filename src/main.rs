use std::{
    env,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

struct Args {
    binding: String,
}
impl Args {
    pub fn collect() -> Args {
        let raw: Vec<String> = env::args().collect();
        Args {
            binding: Args::get_binding_from_raw_args(raw),
        }
    }

    fn get_binding_from_raw_args(raw: Vec<String>) -> String {
        for arg in raw {
            if arg.starts_with("binding=") {
                return arg[8..].to_string();
            }
        }
        //return String::from("0.0.0.0:80"); TODO - Use this when ready.
        return String::from("0.0.0.0:7878");
    }
}

fn main() {
    println!("Initializing...");

    let args = Args::collect();

    let listener = TcpListener::bind(&args.binding)
        .expect("Failed to bind TCP listener");
    println!("Bound to {}", &args.binding);

    let mut request_counter: u64 = 0;
    for incoming_request in listener.incoming() {
        request_counter += 1;
        println!("=== Request {request_counter} Start ===");
        match incoming_request {
            Ok(stream) => handle_connection(stream),
            Err(err) => println!("Error: {}", err),
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

