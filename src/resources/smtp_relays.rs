use reqwest::Method;
use std::sync::Arc;

use crate::errors::Result;
use crate::http::HttpClient;
use crate::models::common::ApiResponse;
use crate::models::smtp::{
    CreateSmtpRelayRequest, SmtpAccess, SmtpAccessPreference, SmtpRelay, SmtpRelayDeleted,
    SmtpRelayEnvelope, SmtpRelayList, UpdateSmtpAccessRequest, UpdateSmtpRelayRequest,
};

/// Resource for configuring external SMTP relay credentials (`/smtp-relays`).
#[derive(Clone)]
pub struct SmtpRelaysResource {
    http: Arc<HttpClient>,
}

impl SmtpRelaysResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List all configured SMTP relays. The payload is `{"smtpRelays": [...]}`.
    pub async fn list(&self) -> Result<ApiResponse<SmtpRelayList>> {
        let builder = self.http.request(Method::GET, "/smtp-relays");
        self.http.execute(builder).await
    }

    /// Create a new SMTP relay.
    pub async fn create(
        &self,
        request: CreateSmtpRelayRequest,
    ) -> Result<ApiResponse<SmtpRelayEnvelope>> {
        let builder = self
            .http
            .request(Method::POST, "/smtp-relays")
            .json(&request);
        self.http.execute(builder).await
    }

    /// Retrieve an SMTP relay by ID.
    pub async fn get(&self, relay_id: &str) -> Result<ApiResponse<SmtpRelayEnvelope>> {
        let path = format!("/smtp-relays/{relay_id}");
        let builder = self.http.request(Method::GET, &path);
        self.http.execute(builder).await
    }

    /// Update an SMTP relay (`PATCH`). Only provided fields change.
    pub async fn update(
        &self,
        relay_id: &str,
        request: UpdateSmtpRelayRequest,
    ) -> Result<ApiResponse<SmtpRelayEnvelope>> {
        let path = format!("/smtp-relays/{relay_id}");
        let builder = self.http.request(Method::PATCH, &path).json(&request);
        self.http.execute(builder).await
    }

    /// Activate an SMTP relay (`POST /smtp-relays/{id}/activate`).
    pub async fn activate(&self, relay_id: &str) -> Result<ApiResponse<SmtpRelayEnvelope>> {
        let path = format!("/smtp-relays/{relay_id}/activate");
        let builder = self.http.request(Method::POST, &path);
        self.http.execute(builder).await
    }

    /// Deactivate an SMTP relay (`POST /smtp-relays/{id}/deactivate`).
    pub async fn deactivate(&self, relay_id: &str) -> Result<ApiResponse<SmtpRelayEnvelope>> {
        let path = format!("/smtp-relays/{relay_id}/deactivate");
        let builder = self.http.request(Method::POST, &path);
        self.http.execute(builder).await
    }

    /// Delete an SMTP relay.
    pub async fn delete(&self, relay_id: &str) -> Result<ApiResponse<SmtpRelayDeleted>> {
        let path = format!("/smtp-relays/{relay_id}");
        let builder = self.http.request(Method::DELETE, &path);
        self.http.execute(builder).await
    }
}

/// Convenience accessor for the relay inside its envelope.
impl SmtpRelayEnvelope {
    pub fn relay(self) -> SmtpRelay {
        self.smtp_relay
    }
}

/// Resource for the account-level SMTP submission preference
/// (`/sending/smtp-access`).
#[derive(Clone)]
pub struct SmtpAccessResource {
    http: Arc<HttpClient>,
}

impl SmtpAccessResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Read current SMTP entitlement, preference, and client settings.
    pub async fn get(&self) -> Result<ApiResponse<SmtpAccess>> {
        let builder = self.http.request(Method::GET, "/sending/smtp-access");
        self.http.execute(builder).await
    }

    /// Toggle the account SMTP submission preference.
    pub async fn update(&self, enabled: bool) -> Result<ApiResponse<SmtpAccessPreference>> {
        let payload = UpdateSmtpAccessRequest { enabled };
        let builder = self
            .http
            .request(Method::PUT, "/sending/smtp-access")
            .json(&payload);
        self.http.execute(builder).await
    }
}
