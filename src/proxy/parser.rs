use std::collections::HashMap;

use crate::storage::request_log::CapturedRequest;

pub fn parse_http_request(raw: &[u8]) -> Option<CapturedRequest> {
    let request_str = String::from_utf8_lossy(raw);
    let mut lines = request_str.lines();
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
            return Some(CapturedRequest::new(
                method,
                url,
                headers,
                body,
                authorization,
            ));
        }
    }
    None
}
