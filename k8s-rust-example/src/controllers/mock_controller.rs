use crate::server::HttpFields;
use serde_json::Value;
use std::{io::Write, net::TcpStream};

#[macro_use]
#[path = "./macros/response_status.rs"]
mod response_status;

#[path = "../services/match_mock.rs"]
mod match_mock;
use match_mock::{Http, MockFile, TypeOr};

pub fn mock(stream: TcpStream, http_fields: HttpFields) {
    let http_path = http_fields.original_url.as_str();
    let query_params = http_fields.query_params;
    let http_method = http_fields.method.as_str();

    // todo: Improve using dynamic route params
    //let first_path = path.split("/").collect::<Vec<&str>>()[1];
    // let re = Regex::new(format!("/{first_path}/([A-Z|a-z|0-9]*)").as_str()).unwrap();
    // let path_formatted = re.replace_all(path, "/register/:id");
    //println!("{:#}", re.is_match(Some(data).unwrap().to_string().as_str()));

    let ret = match_mock::execute(
        Http {
            path: http_path,
            method: http_method,
            request_body: http_fields.body,
            query_params,
            headers: http_fields.headers,
        },
        MockFile {
            file_path: String::new(),
        },
    );

    return match ret {
        TypeOr::Buffer(ret, content_type) => {
            stream_response(&stream, ret, String::from("200"), content_type.to_string())
        }
        TypeOr::Json(ret) => response(
            &stream,
            ret["$.body"].to_owned(),
            ret["$.status"].to_string(),
        ),
    };
}

fn response_format(status: String, length: String, content_type: String) -> String {
    let content_header = format!(
        "Content-Type: {content_type}\r\nAccess-Control-Allow-Origin: *\r\nAccept-Ranges: bytes"
    );

    let status = status!(status.as_str());

    return format!("{status}\r\n{length}\r\n{content_header}\r\n\r\n");
}

fn stream_response(mut stream: &TcpStream, data: Vec<u8>, status: String, content_type: String) {
    let length = format!("Content-Length: {}", data.len());
    let response = response_format(status, length, content_type.to_string());

    stream.write_all(response.as_bytes()).unwrap();

    stream.write_all(&data).unwrap_or_default();

    stream.flush().unwrap();
}

fn response(mut stream: &TcpStream, data: Value, status: String) {
    let content = format!("{}", data);
    let length = format!("Content-Length: {}", content.len());
    let response = response_format(status, length, "application/json".to_string());

    stream.write_all(response.as_bytes()).unwrap();

    stream.write(content.as_bytes()).unwrap_or_default();

    stream.flush().unwrap();
}
