use reqwest::Method;
use std::sync::Arc;

use crate::errors::Result;
use crate::http::HttpClient;
use crate::models::common::{ApiResponse, Empty, Items};
use crate::models::domains::{
    CloudflareConnect, CloudflareStatus, CloudflareSyncRequest, CloudflareSyncResult,
    CreateDomainRequest, Domain, DomainDeleted, DomainDnsEnvelope, DomainEnvelope,
    DomainVerificationEnvelope, UpdateDomainRequest, VerifyDomainRequest,
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

    /// List all registered sending domains. The payload is `{"items": [...]}`.
    pub async fn list(&self) -> Result<ApiResponse<Items<Domain>>> {
        let builder = self.http.request(Method::GET, "/sending/domains");
        self.http.execute(builder).await
    }

    /// Register a new sending domain with backend defaults.
    pub async fn create(&self, domain: &str) -> Result<ApiResponse<DomainEnvelope>> {
        self.create_with_request(CreateDomainRequest::new(domain))
            .await
    }

    /// Register a new sending domain with explicit options.
    pub async fn create_with_request(
        &self,
        request: CreateDomainRequest,
    ) -> Result<ApiResponse<DomainEnvelope>> {
        let builder = self
            .http
            .request(Method::POST, "/sending/domains")
            .json(&request);
        self.http.execute(builder).await
    }

    /// Retrieve domain details by ID.
    pub async fn get(&self, domain_id: &str) -> Result<ApiResponse<DomainEnvelope>> {
        let path = format!("/sending/domains/{domain_id}");
        let builder = self.http.request(Method::GET, &path);
        self.http.execute(builder).await
    }

    /// Update a domain (`PATCH /sending/domains/{id}`).
    pub async fn update(
        &self,
        domain_id: &str,
        request: UpdateDomainRequest,
    ) -> Result<ApiResponse<DomainEnvelope>> {
        let path = format!("/sending/domains/{domain_id}");
        let builder = self.http.request(Method::PATCH, &path).json(&request);
        self.http.execute(builder).await
    }

    /// Mark a domain as (or stop it being) the account's default sender.
    pub async fn set_default(
        &self,
        domain_id: &str,
        is_default: bool,
    ) -> Result<ApiResponse<DomainEnvelope>> {
        self.update(
            domain_id,
            UpdateDomainRequest {
                is_default: Some(is_default),
                ..UpdateDomainRequest::default()
            },
        )
        .await
    }

    /// Remove a registered domain.
    pub async fn delete(&self, domain_id: &str) -> Result<ApiResponse<DomainDeleted>> {
        let path = format!("/sending/domains/{domain_id}");
        let builder = self.http.request(Method::DELETE, &path);
        self.http.execute(builder).await
    }

    /// Trigger verification of domain DNS records (DKIM, SPF, MX).
    pub async fn verify(&self, domain_id: &str) -> Result<ApiResponse<DomainVerificationEnvelope>> {
        self.verify_with_force(domain_id, None).await
    }

    /// Trigger verification, optionally forcing a fresh DNS lookup.
    pub async fn verify_with_force(
        &self,
        domain_id: &str,
        force: Option<bool>,
    ) -> Result<ApiResponse<DomainVerificationEnvelope>> {
        let path = format!("/sending/domains/{domain_id}/verify");
        let payload = VerifyDomainRequest { force };
        let builder = self.http.request(Method::POST, &path).json(&payload);
        self.http.execute(builder).await
    }

    /// Retrieve DNS records required for sending and receiving
    /// (`GET /sending/domains/{id}/dns-records`, `{"domain": {...}}`).
    pub async fn get_dns_records(&self, domain_id: &str) -> Result<ApiResponse<DomainDnsEnvelope>> {
        let path = format!("/sending/domains/{domain_id}/dns-records");
        let builder = self.http.request(Method::GET, &path);
        self.http.execute(builder).await
    }

    /// Account-level Cloudflare DNS automation sub-resource.
    pub fn cloudflare(&self) -> CloudflareSubResource {
        CloudflareSubResource {
            http: Arc::clone(&self.http),
        }
    }
}

/// Cloudflare DNS automation.
///
/// These routes are account-level, not per-domain:
///
/// - `GET    /sending/domains/cloudflare/status`
/// - `GET    /sending/domains/cloudflare/connect`
/// - `POST   /sending/domains/cloudflare/sync`
/// - `DELETE /sending/domains/cloudflare/disconnect`
#[derive(Clone)]
pub struct CloudflareSubResource {
    http: Arc<HttpClient>,
}

impl CloudflareSubResource {
    /// Read Cloudflare connection status, optionally scoped to one domain.
    ///
    /// The backend deserializes the camelCase `domainId` query key.
    pub async fn status(&self, domain_id: Option<&str>) -> Result<ApiResponse<CloudflareStatus>> {
        let builder = self
            .http
            .request(Method::GET, "/sending/domains/cloudflare/status");
        let builder = match domain_id {
            Some(id) => builder.query(&[("domainId", id)]),
            None => builder,
        };
        self.http.execute(builder).await
    }

    /// Begin the Cloudflare OAuth flow and return the authorization URL.
    ///
    /// The backend deserializes the camelCase `domainId` query key.
    pub async fn connect(&self, domain_id: Option<&str>) -> Result<ApiResponse<CloudflareConnect>> {
        let builder = self
            .http
            .request(Method::GET, "/sending/domains/cloudflare/connect");
        let builder = match domain_id {
            Some(id) => builder.query(&[("domainId", id)]),
            None => builder,
        };
        self.http.execute(builder).await
    }

    /// Synchronize the domain's required DNS records into the connected
    /// Cloudflare zone (`POST /sending/domains/cloudflare/sync`).
    pub async fn sync(&self, domain_id: &str) -> Result<ApiResponse<CloudflareSyncResult>> {
        let payload = CloudflareSyncRequest {
            domain_id: domain_id.to_string(),
        };
        let builder = self
            .http
            .request(Method::POST, "/sending/domains/cloudflare/sync")
            .json(&payload);
        self.http.execute(builder).await
    }

    /// Disconnect the account's Cloudflare integration.
    ///
    /// The disconnect handler deserializes the camelCase `domainId` query key.
    pub async fn disconnect(&self, domain_id: Option<&str>) -> Result<ApiResponse<Empty>> {
        let builder = self
            .http
            .request(Method::DELETE, "/sending/domains/cloudflare/disconnect");
        let builder = match domain_id {
            Some(id) => builder.query(&[("domainId", id)]),
            None => builder,
        };
        self.http.execute(builder).await
    }
}
