use std::collections::HashMap;
use itertools::Itertools;

pub struct HTTPResponse {
    status_code: u32,
    body: String,
    headers: HashMap<String, String>,
}

impl HTTPResponse {
    pub fn new(status_code: u32, body: String, headers: HashMap<&str, &str>) -> HTTPResponse {
        HTTPResponse {
            status_code,
            body,
            headers: headers.
                iter().
                map(|(k, v)| (k.to_string(), v.to_string())).
                collect(),
        }
    }

    pub fn to_string(&self) -> String {
        let status = match self.status_code {
            200 => "OK",
            404 => "NOT FOUND",
            _ => "INTERNAL SERVER ERROR",
        };
        let headers = self.headers.iter().map(|(k, v)| format!("{}: {}", k, v)).collect_vec().join("\r\n");
        format!("HTTP/1.1 {} {}\r\n{}\r\n\r\n{}", self.status_code, status, headers, self.body)
    }
}

pub struct HTTPRequest {
    pub method: String,
    pub path: String,
    pub headers: Vec<String>,
}

impl HTTPRequest {
    pub fn from_string(request: String) -> HTTPRequest {
        let lines = request.lines().collect_vec();
        let start_line = lines[0].split_whitespace().collect_vec();
        let method = start_line[0].to_string();
        let path = start_line[1].to_string();
        let headers = lines[1..].iter().map(|x| x.to_string()).collect_vec();
        HTTPRequest {
            method,
            path,
            headers,
        }
    }
}