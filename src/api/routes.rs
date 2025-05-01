use crate::storage::request_log::{CapturedRequest, RequestLog};
use axum::{Json, Router, routing::get};
use std::sync::Arc;

pub fn api_routes(log: Arc<RequestLog>) -> Router {
    Router::new().route("/requests", get(move || get_requests(log.clone())))
}

async fn get_requests(log: Arc<RequestLog>) -> Json<Vec<CapturedRequest>> {
    Json(log.get_requests())
}
