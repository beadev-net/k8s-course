use serde_json::{
    json,
    Value::{self, Null},
};

use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Read,
    io::Write,
};

trait DataFile {
    fn get_path(&self) -> String;
}

pub struct MockFile {
    pub file_path: String,
}

impl DataFile for MockFile {
    fn get_path(&self) -> String {
        let args: Vec<String> = env::args().collect();

        let file = String::from(if self.file_path != "" {
            self.file_path.to_string()
        } else {
            "mock_data.json".to_string()
        });

        let mut i = 0;

        let file_path = loop {
            let arg = args.get(i).unwrap_or(&file);

            if arg.len() > 2 && &arg[..3] == "-f=" {
                break arg[3..].to_string();
            }

            if arg == &file {
                break file.to_string();
            }

            i += 1;
        };

        return file_path;
    }
}

pub struct Http<'a> {
    pub path: &'a str,
    pub method: &'a str,
    pub request_body: String,
    pub query_params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
}

#[derive(Debug)]
pub enum TypeOr<B, J> {
    Buffer(B, J),
    Json(J),
}

pub fn execute(http: Http, file: MockFile) -> TypeOr<Vec<u8>, Value> {
    let file_path = file.get_path();

    let file_string = fs::read_to_string(&file_path)
        .expect(format!("\x1b[31m<< Unable to read file {file_path} >>\x1b[0m").as_str());

    let mut data: Value = serde_json::from_str(&file_string).expect("Unable to parse");

    let http_request_body: Value = match http.request_body != "" {
        true => serde_json::from_str(&http.request_body).expect("Unable to parse"),
        false => Null,
    };

    let path = http.path.split("?").collect::<Vec<&str>>()[0];

    let file_data_request_body = data[path][http.method]["$.request"]["$.body"].to_owned();

    if http.path == "/_cat/routes" {
        return TypeOr::Json(json!({
            "$.body": data,
            "$.status": "200",
        }));
    }

    if data[path][http.method] == Null {
        return TypeOr::Json(json!({
            "$.body": {
                "error": "URI Path or HTTP Method Not found",
                "path": http.path,
                "method": http.method,
            },
            "$.status": "404",
        }));
    }

    if check_http_request_body_is_different_from_data_request_body(
        file_data_request_body.to_string(),
        http_request_body.to_string(),
    ) || check_data_request_body_is_null_and_http_request_body_is_not_null(
        file_data_request_body.to_string(),
        http_request_body.to_string(),
    ) {
        return TypeOr::Json(json!({
            "$.body": {
                "error": "Request body does not match",
                "request": http_request_body,
            },
            "$.status": "400",
        }));
    }

    let _ = &data[path][http.method]
        .as_object_mut()
        .unwrap()
        .remove("$.request");

    if data[path][http.method]["$.response"]["$.file"] != Null {
        let filename = data[path][http.method]["$.response"]["$.file"]
            .as_str()
            .unwrap();

        if filename.starts_with("http") || filename.starts_with("data:") {
            write!(std::io::stdout(), "Downloading file from {}\n", filename).unwrap();
            return TypeOr::Buffer(
                filename.as_bytes().to_vec(),
                data[path][http.method]["$.response"]["$.content-type"].take(),
            );
        }

        let mut buffer = Vec::new();

        let mut f = match File::open(&filename) {
            Ok(file) => file,
            Err(_) => panic!("Unable to open asset file {}", filename),
        };

        f.read_to_end(&mut buffer).unwrap();

        return TypeOr::Buffer(
            buffer.to_vec(),
            data[path][http.method]["$.response"]["$.content-type"].take(),
        );
    }

    return TypeOr::Json(data[path][http.method]["$.response"].to_owned());
}

fn check_http_request_body_is_different_from_data_request_body(
    file_data_request_body: String,
    http_request_body: String,
) -> bool {
    return file_data_request_body != http_request_body && file_data_request_body != "null";
}

fn check_data_request_body_is_null_and_http_request_body_is_not_null(
    file_data_request_body: String,
    http_request_body: String,
) -> bool {
    return file_data_request_body == "null" && http_request_body != "null";
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_read_file_and_return_mock_successfully() {
        let ret = execute(
            Http {
                path: "/register",
                method: "POST",
                request_body: "".to_string(),
                query_params: HashMap::new(),
                headers: HashMap::new(),
            },
            MockFile {
                file_path: "src/services/test_mock_data.json".to_string(),
            },
        );

        match ret {
            TypeOr::Buffer(_, _) => panic!("Should not return binary data"),
            TypeOr::Json(ret) => {
                assert_eq!(ret["$.body"]["name"], "John Doe");
            }
        }
    }

    #[test]
    fn test_read_file_and_return_mock_not_found() {
        let ret = execute(
            Http {
                path: "/register",
                method: "GET",
                request_body: "".to_string(),
                query_params: HashMap::new(),
                headers: HashMap::new(),
            },
            MockFile {
                file_path: "src/services/test_mock_data.json".to_string(),
            },
        );

        match ret {
            TypeOr::Buffer(_, _) => panic!("Should not return binary data"),
            TypeOr::Json(ret) => {
                assert_eq!(ret["$.body"]["error"], "URI Path or HTTP Method Not found");
            }
        }
    }

    #[test]
    fn test_read_file_and_return_mock_request_body_does_not_match() {
        let ret = execute(
            Http {
                path: "/register",
                method: "POST",
                request_body: r#"{"name": "what_your_name"}"#.to_string(),
                query_params: HashMap::new(),
                headers: HashMap::new(),
            },
            MockFile {
                file_path: "src/services/test_mock_data.json".to_string(),
            },
        );

        match ret {
            TypeOr::Buffer(_, _) => panic!("Should not return binary data"),
            TypeOr::Json(ret) => {
                assert_eq!(ret["$.body"]["error"], "Request body does not match");
            }
        }
    }

    #[test]
    fn test_read_file_from_args_and_return_mock_request_body_match() {
        let ret = execute(
            Http {
                path: "/register",
                method: "POST",
                request_body: "".to_string(),
                query_params: HashMap::new(),
                headers: HashMap::new(),
            },
            MockFile {
                file_path: "-f=src/services/test_mock_data.json".to_string(),
            },
        );

        match ret {
            TypeOr::Buffer(_, _) => panic!("Should not return binary data"),
            TypeOr::Json(ret) => {
                assert_eq!(ret["$.body"]["name"], "John Doe");
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_exception_when_input_wrong_file_path() {
        execute(
            Http {
                path: "/register",
                method: "POST",
                request_body: "".to_string(),
                query_params: HashMap::new(),
                headers: HashMap::new(),
            },
            MockFile {
                file_path: "file_that_not_exist.json".to_string(),
            },
        );
    }

    #[test]
    fn test_read_from_default_file_and_return_mock_request_body_match() {
        let ret = execute(
            Http {
                path: "/register",
                method: "POST",
                request_body: Null.to_string(),
                query_params: HashMap::new(),
                headers: HashMap::new(),
            },
            MockFile {
                file_path: "./src/services/test_mock_data.json".to_string(),
            },
        );

        match ret {
            TypeOr::Buffer(_, _) => panic!("Should not return binary data"),
            TypeOr::Json(ret) => {
                assert_eq!(ret["$.body"]["name"], "John Doe");
            }
        }
    }

    #[test]
    fn test_list_all_mock_routes() {
        let ret = execute(
            Http {
                path: "/_cat/routes",
                method: "GET",
                request_body: "".to_string(),
                query_params: HashMap::new(),
                headers: HashMap::new(),
            },
            MockFile {
                file_path: "./src/services/test_mock_data.json".to_string(),
            },
        );

        match ret {
            TypeOr::Buffer(_, _) => panic!("Should not return binary data"),
            TypeOr::Json(ret) => {
                assert_eq!(
                    ret["$.body"],
                    json!({
                      "/register": {
                        "POST": {
                          "$.response": {
                            "$.status": 201,
                            "$.body": {
                              "name": "John Doe",
                              "age": 30,
                              "address": {
                                "street": "123 Main St",
                                "city": "Anytown",
                                "state": "CA",
                                "zip": "12345"
                              }
                            }
                          }
                        }
                      }
                    })
                );
            }
        }
    }
}
