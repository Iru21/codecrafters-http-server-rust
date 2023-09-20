use std::io::{Read, Write};
use std::net::TcpListener;
use itertools::Itertools;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("Server listening on port 4221");

    for stream in listener.incoming() {
        match stream {
            Ok(mut data) => {
                println!("Connection established!");
                let request = &mut [0; 512];
                data.read(request).unwrap();
                let text = String::from_utf8_lossy(request).to_string();
                let lines = text.lines().collect_vec();
                let start_line = lines[0].split_whitespace().collect_vec();
                if start_line[0] == "GET" {
                    match start_line[1] {
                        "/" => {
                            let response = "HTTP/1.1 200 OK\r\n\r\n";
                            data.write(response.as_bytes()).unwrap();
                        }
                        _ => {
                            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                            data.write(response.as_bytes()).unwrap();
                        }
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
