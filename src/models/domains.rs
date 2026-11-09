use serde::{Deserialize, Serialize};

/// Sending domain entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    pub id: String,
    pub domain: String,
    #[serde(rename = "isDefault", alias = "is_default", default)]
    pub is_default: bool,
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub status: String,
    #[serde(rename = "createdAt", alias = "created_at", default)]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", alias = "updated_at", default)]
    pub updated_at: Option<String>,
}

/// Request payload to create a new domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDomainRequest {
    pub domain: String,
}

/// DNS record required for domain verification (DKIM, SPF, MX).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "recordType", alias = "record_type", default)]
    pub record_type: String,
    pub host: String,
    pub value: String,
    #[serde(default)]
    pub ttl: u32,
    #[serde(default)]
    pub status: String,
}

/// Domain DNS records collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainDns {
    pub domain: String,
    #[serde(default)]
    pub records: Vec<DnsRecord>,
}

/// Result of domain verification check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainVerification {
    #[serde(default)]
    pub verified: bool,
    #[serde(default)]
    pub records: Vec<DnsRecord>,
}

/// Cloudflare integration status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudflareStatus {
    #[serde(default)]
    pub connected: bool,
    #[serde(rename = "zoneId", alias = "zone_id", default)]
    pub zone_id: Option<String>,
    #[serde(rename = "autoSync", alias = "auto_sync", default)]
    pub auto_sync: bool,
}

/// Cloudflare connect request payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudflareConnectRequest {
    #[serde(rename = "apiToken")]
    pub api_token: String,
    #[serde(rename = "zoneId")]
    pub zone_id: String,
}
