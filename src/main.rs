mod api;
mod proxy;
mod storage;

use api::routes::api_routes;
use proxy::start_proxy;

use std::sync::Arc;
use storage::request_log::RequestLog;

#[tokio::main]
async fn main() {
    let request_log = Arc::new(RequestLog::new());

    // start proxy in background
    let proxy_log = request_log.clone();
    tokio::spawn(async move {
        start_proxy(proxy_log).await;
    });

    // start API server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    let api = api_routes(request_log);
    println!("API running on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, api).await.unwrap();
}
