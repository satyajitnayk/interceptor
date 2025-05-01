pub mod handler;
pub mod tls;

use crate::storage::request_log::RequestLog;
use std::sync::Arc;
use tokio::net::TcpListener;

pub async fn start_proxy(log: Arc<RequestLog>) {
    let proxy_address = "0.0.0.0:8080";
    let listener = TcpListener::bind(proxy_address).await.unwrap();
    println!("Proxy running on {}", proxy_address);

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let log = log.clone();
        tokio::spawn(async move {
            handler::handle_connection(stream, log).await;
        });
    }
}
