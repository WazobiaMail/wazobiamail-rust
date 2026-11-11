use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::common::Page;

/// Customer invoice entity (`GET /billing/invoices` items).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub id: String,
    #[serde(default)]
    pub account_id: i64,
    #[serde(default)]
    pub amount: f64,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub status: String,
    /// `subscription`, `email_units`, `whatsapp_units`, ...
    #[serde(default)]
    pub purpose: String,
    #[serde(default)]
    pub metadata: Value,
    #[serde(default)]
    pub due_date: Option<String>,
    #[serde(default)]
    pub paid_at: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Paginated invoice collection (`{"items": [...], "pagination": {...}}`).
pub type InvoicePage = Page<Invoice>;

/// `GET /billing/invoices/{id}` payload: `{"invoice": {...}}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceEnvelope {
    pub invoice: Invoice,
}

/// Plan summary embedded in the current subscription record.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionPlanSummary {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub usd_price: f64,
    #[serde(default)]
    pub naira_price: f64,
    #[serde(default)]
    pub monthly_quota: i64,
}

/// Active subscription returned by `GET /subscriptions/current`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub account_id: i64,
    #[serde(default)]
    pub plan_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub auto_renew: bool,
    #[serde(default)]
    pub billing_interval: String,
    #[serde(default)]
    pub plan: Option<SubscriptionPlanSummary>,
}

/// `GET /subscriptions/current` payload: `{"subscription": {...} | null}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionEnvelope {
    #[serde(default)]
    pub subscription: Option<Subscription>,
}

/// Prepaid email unit balance summary (`GET /units/summary`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitsSummary {
    #[serde(default)]
    pub included_allowance: i64,
    #[serde(default)]
    pub included_used: i64,
    #[serde(default)]
    pub included_remaining: i64,
    #[serde(default)]
    pub purchased_balance: i64,
    #[serde(default)]
    pub unlimited: bool,
    #[serde(default)]
    pub can_buy_credits: bool,
}

/// Unit pack catalog item (`GET /units/packs` items).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitPack {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub units: i64,
    #[serde(default)]
    pub price_ngn_minor: i64,
    #[serde(default)]
    pub price_usd_minor: i64,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default)]
    pub sort_order: i32,
}

/// Currency accepted by the units and subscription endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "NGN")]
    Ngn,
    #[serde(rename = "USD")]
    Usd,
}

impl Currency {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ngn => "NGN",
            Self::Usd => "USD",
        }
    }
}

/// Body for `POST /units/purchases`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseUnitsRequest {
    pub pack_id: String,
    pub currency: Currency,
}

impl PurchaseUnitsRequest {
    pub fn new(pack_id: impl Into<String>, currency: Currency) -> Self {
        Self {
            pack_id: pack_id.into(),
            currency,
        }
    }
}

/// Response from `POST /units/purchases`: the created invoice/payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitPurchase {
    #[serde(default)]
    pub invoice_id: String,
    #[serde(default)]
    pub units: i64,
    #[serde(default)]
    pub amount_minor: i64,
    #[serde(default)]
    pub credit_amount_minor: i64,
    #[serde(default)]
    pub service_amount_minor: i64,
    #[serde(default)]
    pub currency: String,
    /// For example `pending_payment`.
    #[serde(default)]
    pub status: String,
}

/// A single `GET /units/ledger` row.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitLedgerEntry {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub account_id: Option<i64>,
    #[serde(default)]
    pub email: Option<String>,
    /// Signed change applied to the purchased balance.
    #[serde(default)]
    pub quantity: i64,
    #[serde(default)]
    pub balance_after: i64,
    /// `purchase`, `charge`, `grant`, ...
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub reference: String,
    #[serde(default)]
    pub actor_id: Option<i64>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Paginated unit ledger (`GET /units/ledger`).
pub type UnitLedgerPage = Page<UnitLedgerEntry>;

/// Subscription plan catalog item (`GET /plans` items).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub usd_price: f64,
    #[serde(default)]
    pub naira_price: f64,
    #[serde(default)]
    pub yearly_usd_price: Option<f64>,
    #[serde(default)]
    pub yearly_naira_price: Option<f64>,
    #[serde(default)]
    pub interval_prices: Value,
    #[serde(default)]
    pub hourly_quota: i32,
    #[serde(default)]
    pub daily_quota: i32,
    #[serde(default)]
    pub monthly_quota: i32,
    #[serde(default)]
    pub send_rate_limit_per_minute: i32,
    #[serde(default)]
    pub data_retention_days: i32,
    #[serde(default)]
    pub features: Value,
    #[serde(default)]
    pub limits: Value,
    #[serde(default)]
    pub overage_usd_per_1000: f64,
    #[serde(default)]
    pub dedicated_ip_addon_usd: f64,
    #[serde(default)]
    pub status: String,
}

/// Paginated plan catalog (`GET /plans`).
pub type PlanPage = Page<Plan>;

/// Effective resource limits for the account (`GET /plans/effective`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectiveLimits {
    #[serde(default)]
    pub account_id: i64,
    #[serde(default)]
    pub plan_id: String,
    #[serde(default)]
    pub send_rate_limit_per_minute: i32,
    #[serde(default)]
    pub mailbox_ops_per_minute: i32,
    /// `-1` means unlimited.
    #[serde(default)]
    pub mailbox_limit: i32,
    #[serde(default)]
    pub base_mailbox_limit: i32,
    #[serde(default)]
    pub additional_mailbox_limit: i32,
    #[serde(default)]
    pub custom_domain_sending_enabled: bool,
    #[serde(default)]
    pub domain_limit: i32,
    #[serde(default)]
    pub receiving_enabled: bool,
    #[serde(default)]
    pub mailbox_storage_gb: i64,
}

/// `GET /plans/effective` payload: `{"limits": {...}}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EffectiveLimitsEnvelope {
    pub limits: EffectiveLimits,
}

/// Plan block in `GET /status`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPlanStatus {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub billing_interval: String,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub auto_renew: bool,
}

/// Unit block in `GET /status`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountUnitStatus {
    #[serde(default)]
    pub monthly_allocation: i64,
    #[serde(default)]
    pub subscription_used: i64,
    #[serde(default)]
    pub subscription_remaining: i64,
    #[serde(default)]
    pub purchased_balance: i64,
    #[serde(default)]
    pub total_remaining: i64,
    #[serde(default)]
    pub unlimited: bool,
}

/// Account and plan health returned by `GET /status`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountStatus {
    pub plan: AccountPlanStatus,
    pub units: AccountUnitStatus,
}
