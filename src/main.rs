use std::{
    collections::HashMap, env, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}
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

struct Redirects(HashMap<String, String>);
impl Redirects {
    pub fn collect() -> Redirects {
        let mut map = HashMap::new();
        map.insert("collectio.nz".to_string(), "http://localhost:8000".to_string());
        Redirects(map)
    }

    pub fn get(&self, host: &str) -> Option<&str> {
        self.0.get(host).map(String::as_str)
    }
}

fn main() {
    println!("Initializing...");

    let args = Args::collect();
    let redirects = Redirects::collect();

    let listener = TcpListener::bind(&args.binding)
        .expect("Failed to bind TCP listener");
    println!("Bound to {}", &args.binding);

    let mut request_counter: u64 = 0;
    for incoming_request in listener.incoming() {
        request_counter += 1;
        println!("=== Request {request_counter} ===");
        match incoming_request {
            Ok(stream) => handle_connection(stream, &redirects),
            Err(err) => println!("Error: {}", err),
        }
        println!("================");
    }
}

fn handle_connection(mut stream: TcpStream, redirects: &Redirects) {
    let request = parse_request(&stream);
    log_request(&request);
    let response = determine_response(&request, redirects);
    println!("=== Response ===");
    println!("{}", &response);
    stream.write_all(response.as_bytes()).unwrap();
}

fn parse_request(stream: &TcpStream) -> Vec<String> {
    BufReader::new(stream)
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect()
}

fn log_request(request: &Vec<String>) {
    for line in request {
        println!("{}", line);
    }
}

fn determine_response(request: &Vec<String>, redirects: &Redirects) -> String {
    match find_host(request) {
        Some(host) => {
            match redirects.get(&host) {
                Some(redirect_location) => response_redirect(redirect_location),
                None => response_host_not_configured(&host),
            }
        },
        None => response_host_not_specified(),
    }
}

fn find_host(request: &Vec<String>) -> Option<String> {//TODO - Proper request parsing.
    request.iter()
        .find(|line| line.to_lowercase().starts_with("host: "))//TODO - Change case in parsing.
        .map(|s| s[6..].to_string())
}

fn response_host_not_specified() -> String {
    format!(
        "{}\r\n{}\r\n\r\n{}",
        "HTTP/1.1 400 Bad Request",
        format!("Content-Type: text/plain"),
        "Host not specified in request headers",
    )
}

fn response_host_not_configured(host: &str) -> String {
    format!(
        "{}\r\n{}\r\n\r\n{}",
        "HTTP/1.1 404 Not Found",
        format!("Content-Type: text/plain"),
        format!("Host not configured: {}", host),
    )
}

fn response_redirect(location: &str) -> String {
    format!(
        "{}\r\n{}\r\n\r\n",
        "HTTP/1.1 308 Permanent Redirect",
        format!("Location: {location}"),
    )
}

