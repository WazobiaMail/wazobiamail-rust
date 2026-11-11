use serde::{Deserialize, Serialize};

/// External SMTP relay configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmtpRelay {
    pub id: String,
    #[serde(default)]
    pub account_id: i64,
    pub name: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    /// `none`, `starttls`, or `ssl_tls`.
    #[serde(default)]
    pub smtp_security: String,
    /// `active` or `inactive`.
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub is_default: bool,
    /// The backend never returns the password; it reports whether one is stored.
    #[serde(default)]
    pub has_password: bool,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Request body for `POST /smtp-relays`.
///
/// Unlike most of the API, the relay request DTO uses snake_case wire keys
/// (`smtp_host`, `smtp_password`, `is_default`, ...). Responses remain camelCase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSmtpRelayRequest {
    pub name: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    /// `none`, `starttls`, or `ssl_tls`.
    pub smtp_security: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

/// Request body for `PATCH /smtp-relays/{id}`. Only provided fields change.
///
/// Uses snake_case wire keys to match `UpdateSmtpRelayRequest` on the backend.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSmtpRelayRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_port: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_security: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

/// `GET /smtp-relays` payload: `{"smtpRelays": [...]}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmtpRelayList {
    #[serde(default)]
    pub smtp_relays: Vec<SmtpRelay>,
}

/// Single relay payload (`get`/`create`/`update`/`activate`/`deactivate`):
/// `{"smtpRelay": {...}}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmtpRelayEnvelope {
    pub smtp_relay: SmtpRelay,
}

/// `DELETE /smtp-relays/{id}` payload: `{"relayId": "..."}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmtpRelayDeleted {
    pub relay_id: String,
}

/// Customer SMTP submission settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmtpAccessConnection {
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub security: String,
    #[serde(default)]
    pub usernames: Vec<String>,
}

/// `GET /sending/smtp-access` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmtpAccess {
    /// The account has a paid plan that includes SMTP submission.
    #[serde(default)]
    pub paid: bool,
    /// The account is entitled to enable SMTP submission.
    #[serde(default)]
    pub eligible: bool,
    /// The stored customer preference.
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub persona: Option<String>,
    /// Server-side SMTP submission is configured.
    #[serde(default)]
    pub configured: bool,
    /// Present only when access is both enabled and configured.
    #[serde(default)]
    pub connection: Option<SmtpAccessConnection>,
}

/// `PUT /sending/smtp-access` payload: `{"enabled": bool}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSmtpAccessRequest {
    pub enabled: bool,
}

/// `PUT /sending/smtp-access` response payload: `{"enabled": bool}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmtpAccessPreference {
    pub enabled: bool,
}
