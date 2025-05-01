use crate::{proxy::parser::parse_http_request, storage::request_log::RequestLog};
use std::sync::Arc;
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
            if let Some(request) = parse_http_request(&buffer[..n]) {
                log.add_request(request);
                let _ = stream
                    .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n")
                    .await;
            }
        }
        Err(e) => eprintln!("Failed to read from stream: {}", e),
    }
}
