use std::collections::HashMap;
use std::io::{BufWriter, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

use learning::http::{HttpMethod, HttpStatus};

/*
 * $ cargo run --bin http11
 * $ curl -v http://localhost:8080
 */
fn main() {
    let listener = TcpListener::bind(("0.0.0.0", 8080)).unwrap();

    loop {
        let socket = listener.accept().expect("Failed to accept connection");

        thread::spawn(move || {
            handle_client(socket);
        });
    }
}

#[derive(Debug)]
struct HttpRequest {
    http_method: HttpMethod,
    http_version: String,

    path: String,
    headers: HashMap<String, String>,
}

fn handle_client(socket: (TcpStream, SocketAddr)) {
    let (stream, addr) = socket;
    let request = read_request(&stream);

    println!("-- START REQUEST (Address: {}:{})", addr.ip(), addr.port());
    println!("path: {}", request.path);
    println!("method: {}", request.http_method.name());
    println!("version: {}", request.http_version);
    println!("headers:");
    for (key, value) in request.headers {
        println!(" - {} => {}", key, value);
    }
    println!("-- END REQUEST");

    send_response(
        &stream,
        HttpStatus::OK,
        Some("Hello World, from Rust!\r\n"),
        None,
    );
}

fn send_response(
    stream: &TcpStream,
    status: HttpStatus,
    content: Option<&str>,
    headers: Option<HashMap<&str, String>>,
) {
    let mut buffer = BufWriter::new(stream);

    buffer
        .write(format!("HTTP/1.1 {} {}\r\n", status.code(), status.name()).as_bytes())
        .expect("Failed to write buffer");

    let content_length = match content {
        Some(body) => body.len(),
        None => 0,
    };

    let mut response_headers: HashMap<&str, String> = HashMap::new();
    response_headers.insert("Content-Type", "text/plain; charset=utf-8".to_string());
    response_headers.insert("Content-Length", content_length.to_string());

    if let Some(additional_headers) = headers {
        for (key, value) in additional_headers {
            response_headers.insert(key, value);
        }
    }

    for (key, value) in response_headers {
        buffer
            .write(format!("{}: {}\r\n", key, value).as_bytes())
            .expect("Failed to write buffer");
    }

    buffer.write(b"\r\n").expect("Failed to write buffer");

    if let Some(body) = content {
        buffer
            .write(body.as_bytes())
            .expect("Failed to write buffer");
    }

    buffer.flush().expect("Failed to flush buffer");
}

fn read_request(mut stream: &TcpStream) -> HttpRequest {
    let mut buffer = [0; 1024];

    let bytes_read = stream.read(&mut buffer).expect("failed to read buffer");
    let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);

    let mut raw_request: Vec<&str> = request_str.split("\r\n").collect();
    let first_line: Vec<&str> = raw_request.remove(0).split(" ").collect();

    let mut headers: HashMap<String, String> = HashMap::new();

    for header in raw_request {
        if header.is_empty() {
            break;
        }

        if let Some((key, value)) = header.split_once(":") {
            headers.insert(key.to_string(), value.trim_start().to_string());
        }
    }

    HttpRequest {
        http_method: HttpMethod::from(first_line[0]),
        http_version: http_version_from_string(first_line[2]),

        path: first_line[1].to_string(),
        headers,
    }
}

fn http_version_from_string(value: &str) -> String {
    value.split('/').last().unwrap_or("1.1").to_string()
}
