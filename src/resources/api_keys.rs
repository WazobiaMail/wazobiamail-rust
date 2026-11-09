use reqwest::Method;
use std::sync::Arc;

use crate::errors::Result;
use crate::http::HttpClient;
use crate::models::api_keys::{ApiKey, CreateApiKeyRequest, CreatedApiKey};
use crate::models::common::ApiResponse;

/// Resource for managing scoped API keys.
#[derive(Clone)]
pub struct ApiKeysResource {
    http: Arc<HttpClient>,
}

impl ApiKeysResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List all API keys for the current account.
    pub async fn list(&self) -> Result<ApiResponse<Vec<ApiKey>>> {
        let builder = self.http.request(Method::GET, "/api-keys");
        self.http.execute(builder).await
    }

    /// Create a new scoped API key.
    pub async fn create(&self, request: CreateApiKeyRequest) -> Result<ApiResponse<CreatedApiKey>> {
        let builder = self.http.request(Method::POST, "/api-keys").json(&request);
        self.http.execute(builder).await
    }

    /// Regenerate an existing API key's secret.
    pub async fn regenerate(&self, key_id: &str) -> Result<ApiResponse<CreatedApiKey>> {
        let path = format!("/api-keys/{key_id}/regenerate");
        let builder = self.http.request(Method::POST, &path);
        self.http.execute(builder).await
    }

    /// Revoke an API key.
    pub async fn revoke(&self, key_id: &str) -> Result<ApiResponse<()>> {
        let path = format!("/api-keys/{key_id}");
        let builder = self.http.request(Method::DELETE, &path);
        self.http.execute(builder).await
    }
}
