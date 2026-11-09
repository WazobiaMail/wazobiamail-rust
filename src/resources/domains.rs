use reqwest::Method;
use std::sync::Arc;

use crate::errors::Result;
use crate::http::HttpClient;
use crate::models::common::ApiResponse;
use crate::models::domains::{
    CloudflareConnectRequest, CloudflareStatus, CreateDomainRequest, DnsRecord, Domain,
    DomainVerification,
};

/// Resource for custom sending domains and DNS verification.
#[derive(Clone)]
pub struct DomainsResource {
    http: Arc<HttpClient>,
}

impl DomainsResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List all registered sending domains.
    pub async fn list(&self) -> Result<ApiResponse<Vec<Domain>>> {
        let builder = self.http.request(Method::GET, "/sending/domains");
        self.http.execute(builder).await
    }

    /// Register a new sending domain.
    pub async fn create(&self, domain: &str) -> Result<ApiResponse<Domain>> {
        let payload = CreateDomainRequest {
            domain: domain.to_string(),
        };
        let builder = self
            .http
            .request(Method::POST, "/sending/domains")
            .json(&payload);
        self.http.execute(builder).await
    }

    /// Retrieve domain details by ID.
    pub async fn get(&self, domain_id: &str) -> Result<ApiResponse<Domain>> {
        let path = format!("/sending/domains/{domain_id}");
        let builder = self.http.request(Method::GET, &path);
        self.http.execute(builder).await
    }

    /// Update a domain's default status.
    pub async fn update(&self, domain_id: &str, is_default: bool) -> Result<ApiResponse<Domain>> {
        let path = format!("/sending/domains/{domain_id}");
        let payload = serde_json::json!({ "isDefault": is_default });
        let builder = self.http.request(Method::PATCH, &path).json(&payload);
        self.http.execute(builder).await
    }

    /// Remove a registered domain.
    pub async fn delete(&self, domain_id: &str) -> Result<ApiResponse<()>> {
        let path = format!("/sending/domains/{domain_id}");
        let builder = self.http.request(Method::DELETE, &path);
        self.http.execute(builder).await
    }

    /// Trigger verification of domain DNS records (DKIM, SPF, MX).
    pub async fn verify(&self, domain_id: &str) -> Result<ApiResponse<DomainVerification>> {
        let path = format!("/sending/domains/{domain_id}/verify");
        let builder = self.http.request(Method::POST, &path);
        self.http.execute(builder).await
    }

    /// Retrieve DNS records required for sending and receiving.
    pub async fn get_dns_records(&self, domain_id: &str) -> Result<ApiResponse<Vec<DnsRecord>>> {
        let path = format!("/sending/domains/{domain_id}/dns");
        let builder = self.http.request(Method::GET, &path);
        self.http.execute(builder).await
    }

    /// Sub-resource for Cloudflare integration on this domain.
    pub fn cloudflare(&self, domain_id: &str) -> CloudflareSubResource {
        CloudflareSubResource {
            http: Arc::clone(&self.http),
            domain_id: domain_id.to_string(),
        }
    }
}

/// Cloudflare DNS synchronization sub-resource.
#[derive(Clone)]
pub struct CloudflareSubResource {
    http: Arc<HttpClient>,
    domain_id: String,
}

impl CloudflareSubResource {
    /// Read Cloudflare connection status.
    pub async fn get_status(&self) -> Result<ApiResponse<CloudflareStatus>> {
        let path = format!("/sending/domains/{}/cloudflare", self.domain_id);
        let builder = self.http.request(Method::GET, &path);
        self.http.execute(builder).await
    }

    /// Connect domain to Cloudflare with API token and zone ID.
    pub async fn connect(
        &self,
        api_token: &str,
        zone_id: &str,
    ) -> Result<ApiResponse<CloudflareStatus>> {
        let path = format!("/sending/domains/{}/cloudflare", self.domain_id);
        let payload = CloudflareConnectRequest {
            api_token: api_token.to_string(),
            zone_id: zone_id.to_string(),
        };
        let builder = self.http.request(Method::POST, &path).json(&payload);
        self.http.execute(builder).await
    }

    /// Manually trigger DNS record sync with Cloudflare.
    pub async fn sync(&self) -> Result<ApiResponse<CloudflareStatus>> {
        let path = format!("/sending/domains/{}/cloudflare/sync", self.domain_id);
        let builder = self.http.request(Method::POST, &path);
        self.http.execute(builder).await
    }

    /// Disconnect domain from Cloudflare.
    pub async fn disconnect(&self) -> Result<ApiResponse<()>> {
        let path = format!("/sending/domains/{}/cloudflare", self.domain_id);
        let builder = self.http.request(Method::DELETE, &path);
        self.http.execute(builder).await
    }
}
