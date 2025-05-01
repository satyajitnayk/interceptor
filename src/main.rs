mod api;
mod proxy;
mod storage;

use api::routes::api_routes;
use proxy::start_proxy;

use clap::Parser;
use std::sync::Arc;
use storage::request_log::RequestLog;

/// intercept - Local HTTP/HTTPS Traffic Interceptor
#[derive(Parser)]
#[command(
    author,
    version,
    about = "Run a local proxy to capture HTTP/HTTPS requests and expose them via API"
)]
struct Args {
    /// Port to run the proxy server on (default: 8080)
    #[arg(long, default_value = "8080")]
    proxy_port: u16,

    /// Port to run the API server on (default: 3000)
    #[arg(long, default_value = "3000")]
    api_port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let request_log = Arc::new(RequestLog::new());
    // start proxy in background
    let proxy_log = request_log.clone();
    tokio::spawn(async move {
        start_proxy(&args.proxy_port, proxy_log).await;
    });

    // start API server
    let api_address = format!("127.0.0.1:{}", args.api_port);
    let listener = tokio::net::TcpListener::bind(api_address).await.unwrap();
    let api = api_routes(request_log);
    println!("API running on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, api).await.unwrap();
}
