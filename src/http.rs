use std::collections::HashMap;
use std::sync::Arc;

use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::{Method, RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::config::{AuthScheme, ClientConfig};
use crate::errors::{Result, WazobiaError};
use crate::models::common::ApiResponse;

pub struct HttpClient {
    config: Arc<ClientConfig>,
    client: reqwest::Client,
    custom_headers: HeaderMap,
}

impl HttpClient {
    pub fn new(config: ClientConfig, custom_headers: HeaderMap) -> Result<Self> {
        let client = reqwest::Client::builder().timeout(config.timeout).build()?;

        Ok(Self {
            config: Arc::new(config),
            client,
            custom_headers,
        })
    }

    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Build a URL relative to the configured base URL.
    pub fn build_url(&self, path: &str) -> String {
        let base = self.config.base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        format!("{base}/{path}")
    }

    /// Prepare a request builder with authentication and standard headers applied.
    pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = self.build_url(path);
        let mut builder = self.client.request(method, &url);

        // Standard headers
        builder = builder.header(USER_AGENT, &self.config.user_agent);
        builder = builder.header(ACCEPT, "application/json");

        // Custom headers
        for (name, value) in &self.custom_headers {
            builder = builder.header(name.clone(), value.clone());
        }

        // Authentication scheme injection
        match self.config.auth_scheme {
            AuthScheme::Header => {
                if let Some(key) = &self.config.api_key {
                    builder = builder.header("X-API-Key", key);
                } else if let Some(token) = &self.config.access_token {
                    builder = builder.header(AUTHORIZATION, format!("Bearer {token}"));
                }
            }
            AuthScheme::Bearer => {
                if let Some(token) = &self.config.access_token {
                    builder = builder.header(AUTHORIZATION, format!("Bearer {token}"));
                } else if let Some(key) = &self.config.api_key {
                    builder = builder.header(AUTHORIZATION, format!("Bearer {key}"));
                }
            }
            AuthScheme::Both => {
                if let Some(key) = &self.config.api_key {
                    builder = builder.header("X-API-Key", key);
                }
                if let Some(token) = &self.config.access_token {
                    builder = builder.header(AUTHORIZATION, format!("Bearer {token}"));
                }
            }
        }

        builder
    }

    /// Execute a request and deserialize standard ApiResponse<T>.
    pub async fn execute<T: DeserializeOwned>(
        &self,
        builder: RequestBuilder,
    ) -> Result<ApiResponse<T>> {
        let response = builder.send().await?;
        let status = response.status();
        let headers = response.headers().clone();

        if status.is_success() {
            let body_bytes = response.bytes().await?;
            // If body is empty and T is () or Option, handle gracefully
            if body_bytes.is_empty() {
                if let Ok(res) = serde_json::from_str::<ApiResponse<T>>(
                    "{\"success\":true,\"message\":\"ok\",\"data\":null}",
                ) {
                    return Ok(res);
                }
            }
            let api_res: ApiResponse<T> = serde_json::from_slice(&body_bytes)?;
            Ok(api_res)
        } else {
            Err(Self::parse_error_response(status, &headers, response).await)
        }
    }

    /// Execute a request and return raw bytes (useful for PDF invoice downloads).
    pub async fn execute_bytes(&self, builder: RequestBuilder) -> Result<Vec<u8>> {
        let response = builder.send().await?;
        let status = response.status();
        let headers = response.headers().clone();

        if status.is_success() {
            let bytes = response.bytes().await?;
            Ok(bytes.to_vec())
        } else {
            Err(Self::parse_error_response(status, &headers, response).await)
        }
    }

    /// Parse HTTP error response into typed WazobiaError.
    async fn parse_error_response(
        status: StatusCode,
        headers: &HeaderMap,
        response: Response,
    ) -> WazobiaError {
        let status_code = status.as_u16();
        let text = response.text().await.unwrap_or_default();
        let json_val: Option<Value> = serde_json::from_str(&text).ok();

        let message = json_val
            .as_ref()
            .and_then(|v| {
                v.get("message")
                    .and_then(|m| m.as_str())
                    .or_else(|| v.get("error").and_then(|e| e.as_str()))
            })
            .unwrap_or_else(|| {
                if text.is_empty() {
                    status.canonical_reason().unwrap_or("Unknown Error")
                } else {
                    &text
                }
            })
            .to_string();

        let code = json_val.as_ref().and_then(|v| {
            v.get("code")
                .and_then(|c| c.as_str())
                .map(|s| s.to_string())
        });

        match status_code {
            401 => WazobiaError::Authentication(message),
            403 => {
                let upgrade_required = json_val
                    .as_ref()
                    .and_then(|v| {
                        v.get("upgrade_required")
                            .or_else(|| v.get("upgradeRequired"))
                            .and_then(|u| u.as_bool())
                    })
                    .unwrap_or(false);
                WazobiaError::Forbidden {
                    message,
                    upgrade_required,
                }
            }
            404 => WazobiaError::NotFound(message),
            409 => WazobiaError::Conflict(message),
            422 => {
                let mut errors = HashMap::new();
                if let Some(val) = &json_val {
                    if let Some(err_obj) = val.get("errors").and_then(|e| e.as_object()) {
                        for (k, v) in err_obj {
                            if let Some(s) = v.as_str() {
                                errors.insert(k.clone(), s.to_string());
                            } else if let Some(arr) = v.as_array() {
                                let combined = arr
                                    .iter()
                                    .filter_map(|item| item.as_str())
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                errors.insert(k.clone(), combined);
                            }
                        }
                    }
                }
                WazobiaError::Validation { message, errors }
            }
            429 => {
                let retry_after = headers
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse::<u64>().ok());
                WazobiaError::RateLimit {
                    message,
                    retry_after,
                }
            }
            500..=599 => WazobiaError::Server {
                status: status_code,
                message,
            },
            _ => WazobiaError::Api {
                status: status_code,
                message,
                code,
            },
        }
    }
}
