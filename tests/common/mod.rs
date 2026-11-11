#![allow(dead_code)]

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Notify;

use wazobiamail::WazobiaMail;

type QueuedResponse = (u16, Vec<(String, String)>, Vec<u8>);

#[derive(Debug, Clone)]
pub struct RecordedRequest {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl RecordedRequest {
    pub fn header(&self, name: &str) -> Option<&str> {
        let name_lower = name.to_lowercase();
        self.headers
            .iter()
            .find(|(k, _)| k.to_lowercase() == name_lower)
            .map(|(_, v)| v.as_str())
    }

    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }

    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.body)
    }
}

#[derive(Clone)]
pub struct MockServer {
    addr: SocketAddr,
    requests: Arc<Mutex<Vec<RecordedRequest>>>,
    responses: Arc<Mutex<Vec<QueuedResponse>>>,
    shutdown: Arc<Notify>,
}

impl MockServer {
    pub async fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let responses = Arc::new(Mutex::new(Vec::new()));
        let shutdown = Arc::new(Notify::new());

        let reqs_clone = Arc::clone(&requests);
        let resps_clone = Arc::clone(&responses);
        let shutdown_clone = Arc::clone(&shutdown);

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = shutdown_clone.notified() => {
                        break;
                    }
                    accept_res = listener.accept() => {
                        if let Ok((mut socket, _)) = accept_res {
                            let reqs = Arc::clone(&reqs_clone);
                            let resps = Arc::clone(&resps_clone);
                            tokio::spawn(async move {
                                let mut buf = vec![0u8; 131072]; // 128 KB buffer
                                let mut total_read = 0;

                                // Read HTTP request headers
                                loop {
                                    let n = match socket.read(&mut buf[total_read..]).await {
                                        Ok(0) => break,
                                        Ok(n) => n,
                                        Err(_) => break,
                                    };
                                    total_read += n;
                                    if let Some(pos) = find_header_end(&buf[..total_read]) {
                                        // Check Content-Length to read entire body if present
                                        let header_text = String::from_utf8_lossy(&buf[..pos]);
                                        let content_len = parse_content_length(&header_text);
                                        let body_start = pos + 4;
                                        let body_read = total_read.saturating_sub(body_start);
                                        if body_read >= content_len {
                                            break;
                                        }
                                    }
                                }

                                if total_read > 0 {
                                    if let Some(rec) = parse_raw_request(&buf[..total_read]) {
                                        reqs.lock().unwrap().push(rec);
                                    }

                                    // Pick queued response or default 200 OK
                                    let (status, headers, body) = {
                                        let mut r = resps.lock().unwrap();
                                        if !r.is_empty() {
                                            r.remove(0)
                                        } else {
                                            (200, vec![("Content-Type".to_string(), "application/json".to_string())], b"{\"success\":true,\"message\":\"ok\",\"data\":null}".to_vec())
                                        }
                                    };

                                    let mut response_bytes = format!("HTTP/1.1 {} OK\r\nContent-Length: {}\r\nConnection: close\r\n", status, body.len());
                                    for (k, v) in headers {
                                        response_bytes.push_str(&format!("{k}: {v}\r\n"));
                                    }
                                    response_bytes.push_str("\r\n");

                                    let _ = socket.write_all(response_bytes.as_bytes()).await;
                                    let _ = socket.write_all(&body).await;
                                    let _ = socket.flush().await;
                                }
                            });
                        }
                    }
                }
            }
        });

        Self {
            addr,
            requests,
            responses,
            shutdown,
        }
    }

    pub fn base_url(&self) -> String {
        format!("http://{}/api/v1", self.addr)
    }

    pub fn client(&self) -> WazobiaMail {
        WazobiaMail::builder()
            .api_key("wzbm_mock_key_123456789")
            .base_url(self.base_url())
            .build()
            .unwrap()
    }

    pub fn enqueue_json<T: serde::Serialize>(&self, status: u16, data: T) {
        let envelope = serde_json::json!({
            "success": (200..300).contains(&status),
            "message": "OK",
            "data": data,
        });
        let body = serde_json::to_vec(&envelope).unwrap();
        let headers = vec![("Content-Type".to_string(), "application/json".to_string())];
        self.responses.lock().unwrap().push((status, headers, body));
    }

    pub fn enqueue_error(
        &self,
        status: u16,
        message: &str,
        upgrade_required: bool,
        errors: Option<serde_json::Value>,
    ) {
        // Mirrors `AppError::into_response` on the backend:
        // {"success": false, "statusCode": <code>, "message": <safe message>}
        let mut envelope = serde_json::json!({
            "success": false,
            "statusCode": status,
            "message": message,
        });
        if upgrade_required {
            envelope["upgradeRequired"] = serde_json::json!(true);
        }
        if let Some(errs) = errors {
            envelope["errors"] = errs;
        }
        let body = serde_json::to_vec(&envelope).unwrap();
        let mut headers = vec![("Content-Type".to_string(), "application/json".to_string())];
        if status == 429 {
            headers.push(("Retry-After".to_string(), "30".to_string()));
        }
        self.responses.lock().unwrap().push((status, headers, body));
    }

    pub fn enqueue_bytes(&self, status: u16, content_type: &str, bytes: Vec<u8>) {
        let headers = vec![("Content-Type".to_string(), content_type.to_string())];
        self.responses
            .lock()
            .unwrap()
            .push((status, headers, bytes));
    }

    pub fn last_request(&self) -> RecordedRequest {
        let reqs = self.requests.lock().unwrap();
        reqs.last().cloned().expect("Expected at least one request")
    }

    pub fn requests(&self) -> Vec<RecordedRequest> {
        self.requests.lock().unwrap().clone()
    }
}

fn find_header_end(data: &[u8]) -> Option<usize> {
    data.windows(4).position(|w| w == b"\r\n\r\n")
}

fn parse_content_length(headers: &str) -> usize {
    for line in headers.lines() {
        if let Some((k, v)) = line.split_once(':') {
            if k.trim().eq_ignore_ascii_case("content-length") {
                if let Ok(len) = v.trim().parse::<usize>() {
                    return len;
                }
            }
        }
    }
    0
}

fn parse_raw_request(raw: &[u8]) -> Option<RecordedRequest> {
    let header_end = find_header_end(raw)?;
    let header_str = String::from_utf8_lossy(&raw[..header_end]);
    let mut lines = header_str.lines();
    let request_line = lines.next()?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next()?.to_string();
    let path = parts.next()?.to_string();

    let mut headers = Vec::new();
    for line in lines {
        if let Some((k, v)) = line.split_once(':') {
            headers.push((k.trim().to_string(), v.trim().to_string()));
        }
    }

    let body = raw[header_end + 4..].to_vec();
    Some(RecordedRequest {
        method,
        path,
        headers,
        body,
    })
}
