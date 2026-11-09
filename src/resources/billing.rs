use reqwest::Method;
use std::sync::Arc;

use crate::errors::Result;
use crate::http::HttpClient;
use crate::models::billing::{
    EffectiveLimits, Invoice, Plan, Subscription, UnitPack, UnitsSummary,
};
use crate::models::common::ApiResponse;

/// Resource for billing, subscriptions, and invoices.
#[derive(Clone)]
pub struct BillingResource {
    http: Arc<HttpClient>,
}

impl BillingResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List past and current invoices.
    pub async fn list_invoices(&self) -> Result<ApiResponse<Vec<Invoice>>> {
        let builder = self.http.request(Method::GET, "/billing/invoices");
        self.http.execute(builder).await
    }

    /// Retrieve an invoice by ID.
    pub async fn get_invoice(&self, invoice_id: &str) -> Result<ApiResponse<Invoice>> {
        let path = format!("/billing/invoices/{invoice_id}");
        let builder = self.http.request(Method::GET, &path);
        self.http.execute(builder).await
    }

    /// Download invoice PDF document bytes.
    pub async fn download_invoice(&self, invoice_id: &str) -> Result<Vec<u8>> {
        let path = format!("/billing/invoices/{invoice_id}/download");
        let builder = self.http.request(Method::GET, &path);
        self.http.execute_bytes(builder).await
    }

    /// Retrieve current subscription details.
    pub async fn get_subscription(&self) -> Result<ApiResponse<Subscription>> {
        let builder = self.http.request(Method::GET, "/billing/subscription");
        self.http.execute(builder).await
    }
}

/// Resource for prepaid email sending units.
#[derive(Clone)]
pub struct UnitsResource {
    http: Arc<HttpClient>,
}

impl UnitsResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Retrieve current unit balances and allocations.
    pub async fn get_summary(&self) -> Result<ApiResponse<UnitsSummary>> {
        let builder = self.http.request(Method::GET, "/billing/units");
        self.http.execute(builder).await
    }

    /// List available unit purchase packs.
    pub async fn list_packs(&self) -> Result<ApiResponse<Vec<UnitPack>>> {
        let builder = self.http.request(Method::GET, "/billing/units/packs");
        self.http.execute(builder).await
    }

    /// Purchase a unit pack.
    pub async fn purchase(&self, pack_id: &str) -> Result<ApiResponse<UnitsSummary>> {
        let payload = serde_json::json!({ "packId": pack_id });
        let builder = self
            .http
            .request(Method::POST, "/billing/units/purchase")
            .json(&payload);
        self.http.execute(builder).await
    }
}

/// Resource for reading plans catalog and effective account limits.
#[derive(Clone)]
pub struct PlansResource {
    http: Arc<HttpClient>,
}

impl PlansResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List available subscription plans.
    pub async fn list(&self) -> Result<ApiResponse<Vec<Plan>>> {
        let builder = self.http.request(Method::GET, "/plans");
        self.http.execute(builder).await
    }

    /// Read the account's effective tier limits.
    pub async fn get_effective_limits(&self) -> Result<ApiResponse<EffectiveLimits>> {
        let builder = self.http.request(Method::GET, "/plans/effective");
        self.http.execute(builder).await
    }
}
