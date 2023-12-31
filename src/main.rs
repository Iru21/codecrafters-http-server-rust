mod request;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{env, fs, thread};
use itertools::Itertools;
use crate::request::{HTTPRequest, HTTPResponse};

fn send_response(mut stream: TcpStream, response: HTTPResponse) {
    let response_string = response.to_string();
    stream.write(response_string.as_bytes()).unwrap();
}

fn send_404(stream: TcpStream) {
    let response = HTTPResponse::new(404, "".to_string(), HashMap::new());
    send_response(stream, response);
}

fn handle(mut stream: TcpStream, directory: String) {
    println!("Connection established!");
    let request_data = &mut [0; 1024];
    stream.read(request_data).unwrap();
    let request_data = String::from_utf8_lossy(request_data).trim_end_matches(char::from(0)).to_string();
    let request = HTTPRequest::from_string(request_data);
    match request.method.as_str() {
        "GET" => {
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
                },
                _ if request.path.starts_with("/files/") => {
                    let filename = request.path.replace("/files/", "");
                    let path = format!("{}/{}", directory, filename);
                    let file = std::fs::read_to_string(path);

                    match file {
                        Ok(data) => {
                            println!("data: {}", data);
                            let length = data.len().to_string();
                            let headers = HashMap::from([
                                ("Content-Type", "application/octet-stream"),
                                ("Content-Length", length.as_str()),
                            ]);
                            send_response(stream, HTTPResponse::new(200, data, headers));
                        },
                        Err(_) => {
                            send_404(stream);
                        }
                    }
                },
                _ if request.path.starts_with("/echo/") => {
                    let echo = request.path.replace("/echo/", "");
                    let length = echo.len().to_string();
                    let headers = HashMap::from([
                        ("Content-Type", "text/plain"),
                        ("Content-Length", length.as_str()),
                    ]);
                    send_response(stream, HTTPResponse::new(200, echo, headers));
                },
                _ => {
                    send_404(stream);
                }
            }
        },
        "POST" => {
            match request.path.as_str() {
                _ if request.path.starts_with("/files/") => {
                    let filename = request.path.replace("/files/", "");
                    let path = format!("{}/{}", directory, filename);
                    let mut file = fs::File::create(path).unwrap();
                    file.write(request.body.as_bytes()).unwrap();

                    send_response(stream, HTTPResponse::new(201, "".to_string(), HashMap::new()));

                },
                _ => {
                    send_404(stream);
                }
            }
        }
        _ => {
            send_404(stream);
        }
    }
}

fn main() {
    let args = env::args().collect_vec();

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Server listening on port 4221");

    for stream in listener.incoming() {
        match stream {
            Ok(data) => {
                let dir = args.get(2).unwrap_or(&String::from(".")).to_string();

                thread::spawn(move || {
                    handle(data, dir);
                });
            },
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
