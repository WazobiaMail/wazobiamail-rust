use reqwest::Method;
use std::sync::Arc;

use crate::errors::Result;
use crate::http::HttpClient;
use crate::models::billing::{
    Currency, EffectiveLimitsEnvelope, Invoice, InvoiceEnvelope, InvoicePage, PlanPage,
    PurchaseUnitsRequest, SubscriptionEnvelope, UnitLedgerPage, UnitPack, UnitPurchase,
    UnitsSummary,
};
use crate::models::common::{ApiResponse, Items};

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
    ///
    /// The backend returns a `CollectionPage`: `data: {"items", "pagination"}`.
    pub async fn list_invoices(&self) -> Result<ApiResponse<InvoicePage>> {
        let builder = self.http.request(Method::GET, "/billing/invoices");
        self.http.execute(builder).await
    }

    /// Retrieve an invoice by ID. The payload is `{"invoice": {...}}`.
    pub async fn get_invoice(&self, invoice_id: &str) -> Result<ApiResponse<InvoiceEnvelope>> {
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

    /// Retrieve the current product subscription (`GET /subscriptions/current`).
    ///
    /// `data.subscription` is `null` when the account has no active subscription.
    pub async fn get_subscription(&self) -> Result<ApiResponse<SubscriptionEnvelope>> {
        let builder = self.http.request(Method::GET, "/subscriptions/current");
        self.http.execute(builder).await
    }
}

/// Convenience accessor for the invoice inside its envelope.
impl InvoiceEnvelope {
    pub fn into_invoice(self) -> Invoice {
        self.invoice
    }
}

/// Resource for prepaid email sending units (`/units`).
#[derive(Clone)]
pub struct UnitsResource {
    http: Arc<HttpClient>,
}

impl UnitsResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Retrieve current unit balances and allocations (`GET /units/summary`).
    pub async fn get_summary(&self) -> Result<ApiResponse<UnitsSummary>> {
        let builder = self.http.request(Method::GET, "/units/summary");
        self.http.execute(builder).await
    }

    /// List available unit purchase packs (`GET /units/packs`, `{"items": [...]}`).
    pub async fn list_packs(&self) -> Result<ApiResponse<Items<UnitPack>>> {
        let builder = self.http.request(Method::GET, "/units/packs");
        self.http.execute(builder).await
    }

    /// Purchase a unit pack (`POST /units/purchases`).
    ///
    /// The backend creates a pending invoice and returns its identifiers; it does
    /// not credit the account until payment is verified.
    pub async fn purchase(
        &self,
        pack_id: &str,
        currency: Currency,
    ) -> Result<ApiResponse<UnitPurchase>> {
        let payload = PurchaseUnitsRequest::new(pack_id, currency);
        let builder = self
            .http
            .request(Method::POST, "/units/purchases")
            .json(&payload);
        self.http.execute(builder).await
    }

    /// Read the unit purchase/charge ledger (`GET /units/ledger`).
    pub async fn ledger(&self) -> Result<ApiResponse<UnitLedgerPage>> {
        let builder = self.http.request(Method::GET, "/units/ledger");
        self.http.execute(builder).await
    }

    /// Read a specific ledger page (`page` and `per_page` query parameters).
    pub async fn ledger_page(
        &self,
        page: u32,
        per_page: u32,
    ) -> Result<ApiResponse<UnitLedgerPage>> {
        let builder = self
            .http
            .request(Method::GET, "/units/ledger")
            .query(&[("page", page), ("per_page", per_page)]);
        self.http.execute(builder).await
    }
}

/// Resource for reading the plans catalog and effective account limits.
#[derive(Clone)]
pub struct PlansResource {
    http: Arc<HttpClient>,
}

impl PlansResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List available subscription plans (`GET /plans`, a `CollectionPage`).
    pub async fn list(&self) -> Result<ApiResponse<PlanPage>> {
        let builder = self.http.request(Method::GET, "/plans");
        self.http.execute(builder).await
    }

    /// Read the account's effective tier limits (`GET /plans/effective`).
    ///
    /// The payload is `{"limits": {...}}`.
    pub async fn get_effective_limits(&self) -> Result<ApiResponse<EffectiveLimitsEnvelope>> {
        let builder = self.http.request(Method::GET, "/plans/effective");
        self.http.execute(builder).await
    }
}
