use serde::{Deserialize, Serialize};

/// Customer invoice entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub number: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    #[serde(rename = "createdAt", alias = "created_at", default)]
    pub created_at: Option<String>,
    #[serde(rename = "paidAt", alias = "paid_at", default)]
    pub paid_at: Option<String>,
    #[serde(rename = "pdfUrl", alias = "pdf_url", default)]
    pub pdf_url: Option<String>,
}

/// Active subscription details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    #[serde(rename = "planId", alias = "plan_id", default)]
    pub plan_id: String,
    pub status: String,
    #[serde(rename = "billingCycle", alias = "billing_cycle", default)]
    pub billing_cycle: Option<String>,
    #[serde(rename = "renewsAt", alias = "renews_at", default)]
    pub renews_at: Option<String>,
}

/// Prepaid units balance summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitsSummary {
    #[serde(default)]
    pub balance: u64,
    #[serde(default)]
    pub reserved: u64,
    #[serde(default)]
    pub currency: String,
}

/// Unit pack catalog item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitPack {
    pub id: String,
    pub units: u64,
    pub price: f64,
    pub currency: String,
}

/// Subscription plan catalog item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub price: f64,
    #[serde(default)]
    pub currency: String,
}

/// Account effective resource limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectiveLimits {
    #[serde(rename = "planName", alias = "plan_name", default)]
    pub plan_name: String,
    #[serde(rename = "emailLimit", alias = "email_limit", default)]
    pub email_limit: u64,
    #[serde(rename = "domainLimit", alias = "domain_limit", default)]
    pub domain_limit: u32,
    #[serde(rename = "mailboxLimit", alias = "mailbox_limit", default)]
    pub mailbox_limit: u32,
}

/// System and account status health check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountStatus {
    #[serde(default)]
    pub status: String,
    #[serde(rename = "accountId", alias = "account_id", default)]
    pub account_id: Option<String>,
}
