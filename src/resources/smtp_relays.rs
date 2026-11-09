use reqwest::Method;
use std::sync::Arc;

use crate::errors::Result;
use crate::http::HttpClient;
use crate::models::api_keys::{CreateSmtpRelayRequest, SmtpAccess, SmtpRelay};
use crate::models::common::ApiResponse;

/// Resource for configuring external SMTP relay credentials.
#[derive(Clone)]
pub struct SmtpRelaysResource {
    http: Arc<HttpClient>,
}

impl SmtpRelaysResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List all configured SMTP relays.
    pub async fn list(&self) -> Result<ApiResponse<Vec<SmtpRelay>>> {
        let builder = self.http.request(Method::GET, "/sending/smtp-relays");
        self.http.execute(builder).await
    }

    /// Create a new SMTP relay.
    pub async fn create(&self, request: CreateSmtpRelayRequest) -> Result<ApiResponse<SmtpRelay>> {
        let builder = self
            .http
            .request(Method::POST, "/sending/smtp-relays")
            .json(&request);
        self.http.execute(builder).await
    }

    /// Retrieve an SMTP relay by ID.
    pub async fn get(&self, relay_id: &str) -> Result<ApiResponse<SmtpRelay>> {
        let path = format!("/sending/smtp-relays/{relay_id}");
        let builder = self.http.request(Method::GET, &path);
        self.http.execute(builder).await
    }

    /// Update an SMTP relay.
    pub async fn update(
        &self,
        relay_id: &str,
        request: CreateSmtpRelayRequest,
    ) -> Result<ApiResponse<SmtpRelay>> {
        let path = format!("/sending/smtp-relays/{relay_id}");
        let builder = self.http.request(Method::PUT, &path).json(&request);
        self.http.execute(builder).await
    }

    /// Delete an SMTP relay.
    pub async fn delete(&self, relay_id: &str) -> Result<ApiResponse<()>> {
        let path = format!("/sending/smtp-relays/{relay_id}");
        let builder = self.http.request(Method::DELETE, &path);
        self.http.execute(builder).await
    }
}

/// Resource for toggling account-level SMTP sending access.
#[derive(Clone)]
pub struct SmtpAccessResource {
    http: Arc<HttpClient>,
}

impl SmtpAccessResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Read current SMTP access entitlement and status.
    pub async fn get(&self) -> Result<ApiResponse<SmtpAccess>> {
        let builder = self.http.request(Method::GET, "/account/smtp-access");
        self.http.execute(builder).await
    }

    /// Toggle or update account SMTP access.
    pub async fn update(&self, enabled: bool) -> Result<ApiResponse<SmtpAccess>> {
        let payload = serde_json::json!({ "enabled": enabled });
        let builder = self
            .http
            .request(Method::PUT, "/account/smtp-access")
            .json(&payload);
        self.http.execute(builder).await
    }
}
