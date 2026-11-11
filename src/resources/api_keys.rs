use reqwest::Method;
use std::sync::Arc;

use crate::errors::Result;
use crate::http::HttpClient;
use crate::models::api_keys::{
    ApiKey, ApiKeyEnvelope, ApiKeyRevoked, CreateApiKeyRequest, CreatedApiKey,
};
use crate::models::common::{ApiResponse, Items};

/// Resource for managing scoped sending API keys (`/sending/api-keys`).
#[derive(Clone)]
pub struct ApiKeysResource {
    http: Arc<HttpClient>,
}

impl ApiKeysResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List all API keys for the current account.
    ///
    /// The backend returns `data: {"items": [...]}`.
    pub async fn list(&self) -> Result<ApiResponse<Items<ApiKey>>> {
        let builder = self.http.request(Method::GET, "/sending/api-keys");
        self.http.execute(builder).await
    }

    /// Create a new scoped sending API key.
    ///
    /// The plaintext secret is returned once inside `data.apiKey.apiKey`.
    pub async fn create(
        &self,
        request: CreateApiKeyRequest,
    ) -> Result<ApiResponse<ApiKeyEnvelope>> {
        let builder = self
            .http
            .request(Method::POST, "/sending/api-keys")
            .json(&request);
        self.http.execute(builder).await
    }

    /// Regenerate an existing API key's secret.
    pub async fn regenerate(&self, key_id: &str) -> Result<ApiResponse<ApiKeyEnvelope>> {
        let path = format!("/sending/api-keys/{key_id}/regenerate");
        let builder = self.http.request(Method::POST, &path);
        self.http.execute(builder).await
    }

    /// Revoke an API key.
    pub async fn revoke(&self, key_id: &str) -> Result<ApiResponse<ApiKeyRevoked>> {
        let path = format!("/sending/api-keys/{key_id}/revoke");
        let builder = self.http.request(Method::POST, &path);
        self.http.execute(builder).await
    }
}

/// Convenience conversion for callers that only need the secret.
impl ApiKeyEnvelope {
    pub fn into_created(self) -> CreatedApiKey {
        self.api_key
    }
}
