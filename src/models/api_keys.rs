use serde::{Deserialize, Serialize};

/// API Key metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub name: String,
    pub scope: String,
    #[serde(rename = "keyPrefix", alias = "key_prefix", default)]
    pub key_prefix: String,
    #[serde(rename = "createdAt", alias = "created_at", default)]
    pub created_at: Option<String>,
    #[serde(rename = "expiresAt", alias = "expires_at", default)]
    pub expires_at: Option<String>,
}

/// Created API Key response including raw plaintext secret.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedApiKey {
    pub id: String,
    pub name: String,
    pub scope: String,
    #[serde(rename = "keyPrefix", alias = "key_prefix", default)]
    pub key_prefix: String,
    #[serde(rename = "apiKey", alias = "api_key", default)]
    pub api_key: String,
    #[serde(rename = "expiresAt", alias = "expires_at", default)]
    pub expires_at: Option<String>,
}

/// Request to create a new API key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub scope: String,
    #[serde(rename = "domainId", skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    #[serde(rename = "expiresInDays", skip_serializing_if = "Option::is_none")]
    pub expires_in_days: Option<u32>,
}

/// External SMTP relay configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpRelay {
    pub id: String,
    pub name: String,
    #[serde(rename = "smtpHost", alias = "smtp_host", alias = "host")]
    pub smtp_host: String,
    #[serde(rename = "smtpPort", alias = "smtp_port", alias = "port")]
    pub smtp_port: u16,
    pub username: String,
    #[serde(rename = "tlsMode", alias = "tls_mode", default)]
    pub tls_mode: String,
    #[serde(rename = "isActive", alias = "is_active", default)]
    pub is_active: bool,
}

/// Request to create or update an SMTP relay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSmtpRelayRequest {
    pub name: String,
    #[serde(rename = "smtpHost")]
    pub smtp_host: String,
    #[serde(rename = "smtpPort")]
    pub smtp_port: u16,
    pub username: String,
    pub password: Option<String>,
    #[serde(rename = "tlsMode", skip_serializing_if = "Option::is_none")]
    pub tls_mode: Option<String>,
}

/// Account SMTP access configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpAccess {
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "enforcementLevel", alias = "enforcement_level", default)]
    pub enforcement_level: Option<String>,
}
