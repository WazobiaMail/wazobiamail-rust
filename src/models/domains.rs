use serde::{Deserialize, Serialize};

/// Sending domain entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    pub id: String,
    #[serde(default)]
    pub account_id: i64,
    pub domain: String,
    /// The hostname the customer asked for when it differs from `domain`.
    #[serde(default)]
    pub requested_domain: Option<String>,
    /// `pending`, `verified`, `failed`, ...
    #[serde(default)]
    pub verification_status: String,
    #[serde(default)]
    pub is_default: bool,
    /// `primary` (hosts inboxes) or `sending` (dedicated `mta.` host).
    #[serde(default)]
    pub domain_role: String,
    #[serde(default)]
    pub parent_domain_id: Option<String>,
    #[serde(default)]
    pub create_default_inboxes: bool,
    /// `hosted` or `sending_only`.
    #[serde(default)]
    pub mail_mode: String,
    #[serde(default)]
    pub created_at: Option<String>,
}

impl Domain {
    /// Convenience check for the backend's `verificationStatus`.
    pub fn is_verified(&self) -> bool {
        self.verification_status.eq_ignore_ascii_case("verified")
    }
}

/// Request payload to create a new domain.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDomainRequest {
    pub domain: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Send from the apex as typed instead of the dedicated sending subdomain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_root_domain: Option<bool>,
    /// Host inboxes on the typed domain (default) or on the sending host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_inboxes_on_main_domain: Option<bool>,
    /// Label for the sending host, for example `mta`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sending_subdomain: Option<String>,
    /// `hosted` or `sending_only`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail_mode: Option<String>,
}

impl CreateDomainRequest {
    pub fn new(domain: impl Into<String>) -> Self {
        Self {
            domain: domain.into(),
            ..Self::default()
        }
    }
}

/// Request payload to update a domain (`PATCH /sending/domains/{id}`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDomainRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_root_domain: Option<bool>,
    /// `hosted` or `sending_only`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mail_mode: Option<String>,
}

/// DNS record required for domain verification (DKIM, SPF, MX, tracking).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsRecord {
    #[serde(default)]
    pub id: String,
    pub record_type: String,
    pub host: String,
    pub value: String,
    #[serde(default)]
    pub ttl: i32,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub last_checked_at: Option<String>,
}

/// Domain DNS records collection returned by
/// `GET /sending/domains/{id}/dns-records`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainDns {
    #[serde(default)]
    pub provisioning: bool,
    #[serde(default)]
    pub domain_id: String,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub verification_status: String,
    #[serde(default)]
    pub records: Vec<DnsRecord>,
    #[serde(default)]
    pub copy_block: String,
}

/// `GET /sending/domains/{id}/dns-records` payload: `{"domain": {...}}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainDnsEnvelope {
    pub domain: DomainDns,
}

/// Single domain payload (`get`/`create`/`update`): `{"domain": {...}}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainEnvelope {
    pub domain: Domain,
}

/// `DELETE /sending/domains/{id}` payload: `{"domainId": "..."}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainDeleted {
    pub domain_id: String,
}

/// Individual DNS verification check.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainVerificationCheck {
    #[serde(default)]
    pub record_type: String,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub expected_value: String,
    #[serde(default)]
    pub matched: bool,
    #[serde(default)]
    pub observed_values: Vec<String>,
}

/// Result of `POST /sending/domains/{id}/verify`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainVerification {
    #[serde(default)]
    pub domain_id: String,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub verification_status: String,
    #[serde(default)]
    pub verified_count: u32,
    #[serde(default)]
    pub pending_count: u32,
    #[serde(default)]
    pub checks: Vec<DomainVerificationCheck>,
}

/// `POST /sending/domains/{id}/verify` payload: `{"verification": {...}}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainVerificationEnvelope {
    pub verification: DomainVerification,
}

/// Request body for `POST /sending/domains/{id}/verify`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyDomainRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

/// Cloudflare connection status (`GET /sending/domains/cloudflare/status`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudflareStatus {
    #[serde(default)]
    pub configured: bool,
    #[serde(default)]
    pub managed_eligible: bool,
    #[serde(default)]
    pub oauth_configured: bool,
    #[serde(default)]
    pub domain_connect_available: bool,
    /// `customerOAuth` or `platformToken`.
    #[serde(default)]
    pub connection_source: Option<String>,
    #[serde(default)]
    pub connected: bool,
    #[serde(default)]
    pub needs_reconnect: bool,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub scopes: String,
}

/// `GET /sending/domains/cloudflare/connect` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudflareConnect {
    #[serde(default)]
    pub authorization_url: String,
}

/// `POST /sending/domains/cloudflare/sync` request body.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudflareSyncRequest {
    pub domain_id: String,
}

/// Result of `POST /sending/domains/cloudflare/sync`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudflareSyncResult {
    #[serde(default)]
    pub zone: String,
    #[serde(default)]
    pub added: u32,
    #[serde(default)]
    pub updated: u32,
    #[serde(default)]
    pub records: u32,
    #[serde(default)]
    pub verification_status: String,
    #[serde(default)]
    pub receiving_ready: Option<bool>,
    #[serde(default)]
    pub dmarc_ready: Option<bool>,
    #[serde(default)]
    pub verified_count: Option<u32>,
    #[serde(default)]
    pub pending_count: Option<u32>,
    /// Present when records were written but verification could not finish.
    #[serde(default)]
    pub note: Option<String>,
}
