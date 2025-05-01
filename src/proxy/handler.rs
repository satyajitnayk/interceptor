use crate::storage::request_log::{CapturedRequest, RequestLog};
use axum::http::header;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn handle_connection(mut stream: TcpStream, log: Arc<RequestLog>) {
    //todo: implement  TLS wrapping, etc.
    let mut buffer = [0; 8192];
    match stream.read(&mut buffer).await {
        Ok(n) if n == 0 => return,
        Ok(n) => {
            let raw = &buffer[..n];
            let request_str = String::from_utf8_lossy(raw);
            let mut lines = request_str.lines();
            println!("request data: {:?}", lines);
            if let Some(request_line) = lines.next() {
                let parts: Vec<&str> = request_line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let method = parts[0];
                    let url = parts[1];

                    let mut headers = HashMap::new();
                    let mut body = String::new();
                    let mut is_body = false;

                    for line in lines {
                        if line.is_empty() {
                            is_body = true;
                            continue;
                        }
                        if is_body {
                            body.push_str(line);
                            body.push('\n');
                        } else if let Some((key, value)) = line.split_once(":") {
                            headers.insert(key.trim().to_string(), value.trim().to_string());
                        }
                    }

                    let authorization = headers.get("Authorization").cloned();
                    let request = CapturedRequest::new_with_details(
                        method,
                        url,
                        headers,
                        body,
                        authorization,
                    );
                    log.add_request(request);
                    let _ = stream
                        .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n")
                        .await;
                }
            }
        }
        Err(e) => eprintln!("Failed to read from stream: {}", e),
    }
}
