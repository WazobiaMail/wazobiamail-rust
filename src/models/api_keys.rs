use serde::{Deserialize, Serialize};

/// API key metadata as returned by `GET /sending/api-keys`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKey {
    pub id: String,
    pub name: String,
    /// `general` or `domain`.
    #[serde(default)]
    pub scope: String,
    #[serde(default)]
    pub domain_id: Option<String>,
    /// Domain name resolved from `domainId` by the backend (read-only).
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub key_prefix: String,
    #[serde(default)]
    pub last_used_at: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub revoked_at: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Created (or regenerated) API key including the one-time plaintext secret.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedApiKey {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub scope: String,
    #[serde(default)]
    pub domain_id: Option<String>,
    #[serde(default)]
    pub key_prefix: String,
    /// Full `wzbm_...` secret. Returned only on create/regenerate.
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub expires_at: Option<String>,
}

/// Request to create a new sending API key.
///
/// `POST /sending/api-keys` accepts `name`, optional `scope`
/// (`general`/`domain`), optional `domainId`, and optional `expiresAt`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// ISO-8601 date-time (for example `2027-01-01T00:00:00`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl CreateApiKeyRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }
}

/// Envelope used by create and regenerate: `{"apiKey": {...}}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyEnvelope {
    pub api_key: CreatedApiKey,
}

/// Envelope returned by revoke: `{"keyId": "..."}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyRevoked {
    pub key_id: String,
}
