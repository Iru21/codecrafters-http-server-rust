mod request;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use itertools::Itertools;
use crate::request::{HTTPRequest, HTTPResponse};

fn send_response(mut stream: TcpStream, response: HTTPResponse) {
    let response_string = response.to_string();
    stream.write(response_string.as_bytes()).unwrap();
}

fn handle(mut stream: TcpStream) {
    println!("Connection established!");
    let request_data = &mut [0; 512];
    stream.read(request_data).unwrap();
    let request = HTTPRequest::from_string(String::from_utf8_lossy(request_data).to_string());
    if request.method == "GET" {
        match request.path.as_str() {
            "/" => {
                send_response(stream, HTTPResponse::new(200, "Hello, world!".to_string(), HashMap::new()))
            },
            "/user-agent" => {
                let user_agent = request.headers.iter().find(|x| x.starts_with("User-Agent:")).unwrap().split(": ").collect_vec()[1].to_string();
                let length = user_agent.len().to_string();
                let headers = HashMap::from([
                    ("Content-Type", "text/plain"),
                    ("Content-Length", length.as_str()),
                ]);
                send_response(stream, HTTPResponse::new(200, user_agent, headers));
            }
            _ if request.path.starts_with("/echo/") => {
                let echo = request.path.replace("/echo/", "");
                let res = format!("Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", echo.len(), echo);
                let response = format!("HTTP/1.1 200 OK\r\n{}", res);
                stream.write(response.as_bytes()).unwrap();
            },
            _ => {
                let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                stream.write(response.as_bytes()).unwrap();
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Server listening on port 4221");

    for stream in listener.incoming() {
        match stream {
            Ok(data) => handle(data),
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
