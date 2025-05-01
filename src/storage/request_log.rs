use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedRequest {
    pub id: String,
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub authorization: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl CapturedRequest {
    pub fn new(
        method: &str,
        url: &str,
        headers: HashMap<String, String>,
        body: String,
        authorization: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            method: method.to_string(),
            url: url.to_string(),
            headers,
            body,
            authorization,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Default)]
pub struct RequestLog {
    requests: Mutex<Vec<CapturedRequest>>,
}

impl RequestLog {
    pub fn new() -> Self {
        Self {
            requests: Mutex::new(Vec::new()),
        }
    }

    pub fn add_request(&self, req: CapturedRequest) {
        let mut lock = self.requests.lock().unwrap();
        lock.push(req);
    }

    pub fn get_requests(&self) -> Vec<CapturedRequest> {
        let lock = self.requests.lock().unwrap();
        lock.clone()
    }
}
