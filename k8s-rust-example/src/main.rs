use std::io::Write;

mod server;

#[path = "./controllers/mock_controller.rs"]
mod mock_controller;

fn main() {
    let listener = server::start();

    println!("Server started on port {}", server::port());

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                std::thread::spawn(|| {
                    //let mut stream = stream.unwrap();

                    let http_fields = server::request(&stream);

                    if http_fields.original_url == "/healthcheck"
                        || http_fields.original_url == "/"
                        || http_fields.original_url == "/favicon.ico"
                    {
                        let content: String = String::from("{\"status\": \"Ok\"}");
                        let length = format!("Content-Length: {}", content.len());
                        stream.write_all(format!("HTTP/1.1 200 OK\r\n{length}\r\nContent-Type: application/json\r\n\r\n{content}").as_bytes()).unwrap();
                    } else {
                        mock_controller::mock(stream, http_fields);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
