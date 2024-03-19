use std::{
    collections::HashMap,
    io::Read,
    net::{TcpListener, TcpStream},
    str,
};

#[path = "./log.rs"]
mod http_log;
use http_log::HttpLog;

#[path = "./config.rs"]
mod config;

pub struct HttpFields {
    pub body: String,
    pub original_url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
}

pub fn port() -> String {
    let port = config::get("PORT");
    return if port != "" { port } else { "7878".to_string() };
}

pub fn start() -> TcpListener {
    let dns: &str = "0.0.0.0";
    let tcp: String = dns.to_owned() + ":" + &port();

    return TcpListener::bind(tcp).unwrap();
}

pub fn request(mut stream: &TcpStream) -> HttpFields {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let data = str::from_utf8(&buffer).unwrap();

    let request_vec: Vec<_> = data.split("\r\n").collect();
    let mut skip_line: bool = false;
    let mut line_count: i32 = 0;
    let mut body: &str = "";
    let mut http_method_path: &str = "";
    let mut http_headers: HashMap<String, String> = HashMap::new();

    for line in &request_vec {
        line_count += 1;

        if line_count == 1 {
            http_method_path = line;
            continue;
        }

        if line.is_empty() {
            skip_line = true;
            continue;
        }

        if skip_line {
            body = line.trim_end_matches("\0");
            break;
        }

        let header_split: Vec<&str> = line.split(": ").collect();
        http_headers.insert(header_split[0].to_string(), header_split[1].to_string());
    }

    let access_log = HttpLog {
        ip: stream.peer_addr().unwrap().to_string(),
        http_method_path: http_method_path.to_string(),
    };

    access_log.emit(&body.to_string());

    let original_url = url(http_method_path);
    let query_params = query_params(original_url.clone().as_str());

    return HttpFields {
        body: body.to_string(),
        original_url,
        method: method(http_method_path),
        headers: http_headers,
        query_params,
    };
}

fn url(http_method_path: &str) -> String {
    let splitted: Vec<&str> = http_method_path.split(" ").collect();

    if splitted.len() > 1 {
        return String::from(splitted[1]);
    } else {
        return String::from("");
    }
}

fn method(http_method_path: &str) -> String {
    let splitted: Vec<&str> = http_method_path.split(" ").collect();

    return String::from(splitted[0]);
}

fn query_params(url: &str) -> HashMap<String, String> {
    let splitted: Vec<&str> = url.split("?").collect();

    if splitted.len() > 1 {
        return splitted[1]
            .split('&')
            .filter_map(|s| {
                s.split_once('=')
                    .and_then(|t| Some((t.0.to_owned(), t.1.to_owned())))
            })
            .collect();
    } else {
        return HashMap::new();
    }
}
