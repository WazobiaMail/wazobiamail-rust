use serde::{Deserialize, Serialize};

/// Hosted mailbox entity returned by `GET /mailboxes` and `GET /mailboxes/{id}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mailbox {
    pub id: String,
    #[serde(default)]
    pub domain_id: String,
    /// Full email address, for example `support@example.com`.
    #[serde(default, alias = "email")]
    pub address: String,
    #[serde(default)]
    pub local_part: String,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub is_platform_subdomain: bool,
    /// Service address (admin@/noreply@) excluded from plan counters.
    #[serde(default)]
    pub auto_provisioned: bool,
    /// `general` or `admin`.
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

impl Mailbox {
    /// Alias for [`Mailbox::address`] kept for readability in inbox code.
    pub fn email(&self) -> &str {
        &self.address
    }
}

/// Request to provision a new mailbox (`POST /mailboxes`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMailboxRequest {
    pub domain_id: String,
    pub local_part: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quota_mb: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// `general` (default) or `admin`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// `active` (default) or `suspended`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Mailbox authentication credential metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxCredential {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub mailbox_id: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub revoked_at: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Credential plus its one-time generated password.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedMailboxCredential {
    pub credential: MailboxCredential,
    pub password: String,
}

/// Response returned when a mailbox is created.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedMailbox {
    pub mailbox: Mailbox,
    pub credential: CreatedMailboxCredential,
}

/// `GET`/`PATCH /mailboxes/{id}` payload: `{"mailbox": {...}}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxEnvelope {
    pub mailbox: Mailbox,
}

/// `DELETE /mailboxes/{id}` payload: `{"mailboxId": "..."}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxDeleted {
    pub mailbox_id: String,
}

/// `POST /mailboxes/{id}/credentials` payload: `{"credential": {...}}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxCredentialEnvelope {
    pub credential: CreatedMailboxCredential,
}

/// `POST /mailboxes/{id}/credentials` request body.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMailboxCredentialRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// `POST /mailboxes/{id}/credentials/{credentialId}/revoke` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxCredentialRevoked {
    pub mailbox_id: String,
    pub credential_id: String,
}

/// Per-mailbox storage usage row.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxStorageUsage {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub account_id: Option<i64>,
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub used_bytes: Option<i64>,
    #[serde(default)]
    pub quota_bytes: Option<i64>,
    #[serde(default)]
    pub remaining_bytes: Option<i64>,
    #[serde(default)]
    pub measured_at: Option<String>,
}

/// Account storage totals row.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxStorageCustomer {
    #[serde(default)]
    pub account_id: Option<i64>,
    #[serde(default)]
    pub used_bytes: Option<i64>,
    #[serde(default)]
    pub allocated_bytes: i64,
    #[serde(default)]
    pub quota_bytes: Option<i64>,
    #[serde(default)]
    pub mailboxes: i64,
    #[serde(default)]
    pub measured_at: Option<String>,
}

/// Per-domain storage totals row.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxStorageDomain {
    #[serde(default)]
    pub domain_id: String,
    #[serde(default)]
    pub mailboxes: i64,
    #[serde(default)]
    pub included_mailboxes: i64,
    #[serde(default)]
    pub used_bytes: Option<i64>,
    #[serde(default)]
    pub allocated_bytes: i64,
    #[serde(default)]
    pub measured_at: Option<String>,
}

/// Plan allocation block in `GET /mailboxes/storage`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxStoragePlan {
    #[serde(default)]
    pub mailbox_limit: i64,
    #[serde(default)]
    pub base_mailbox_limit: i64,
    #[serde(default)]
    pub additional_mailbox_limit: i64,
    #[serde(default)]
    pub mailboxes_used: i64,
    #[serde(default)]
    pub mailbox_remaining: Option<i64>,
    #[serde(default)]
    pub storage_total_bytes: Option<i64>,
    #[serde(default)]
    pub storage_allocated_bytes: i64,
    #[serde(default)]
    pub storage_remaining_bytes: Option<i64>,
}

/// Account mailbox storage report (`GET /mailboxes/storage`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxStorage {
    #[serde(default)]
    pub mailboxes: Vec<MailboxStorageUsage>,
    #[serde(default)]
    pub customers: Vec<MailboxStorageCustomer>,
    #[serde(default)]
    pub domains: Vec<MailboxStorageDomain>,
    #[serde(default)]
    pub measurement: String,
    #[serde(default)]
    pub plan: Option<MailboxStoragePlan>,
}

/// Email address representation for inbox messages.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxEmailAddress {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub email: String,
}

/// Summary of an enterprise headless inbox message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxMessageSummary {
    pub id: String,
    #[serde(default)]
    pub thread_id: Option<String>,
    #[serde(default)]
    pub from: Vec<InboxEmailAddress>,
    #[serde(default)]
    pub to: Vec<InboxEmailAddress>,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub preview: Option<String>,
    #[serde(default)]
    pub received_at: Option<String>,
    #[serde(default)]
    pub is_unread: bool,
    #[serde(default)]
    pub has_attachments: bool,
    #[serde(default)]
    pub size_bytes: i64,
}

/// Attachment metadata on an inbox message.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxAttachment {
    #[serde(default)]
    pub blob_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, rename = "type")]
    pub content_type: String,
    #[serde(default)]
    pub size: i64,
}

/// Detailed enterprise headless inbox message with body and headers.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxMessageDetail {
    pub id: String,
    #[serde(default)]
    pub thread_id: Option<String>,
    #[serde(default)]
    pub from: Vec<InboxEmailAddress>,
    #[serde(default)]
    pub to: Vec<InboxEmailAddress>,
    #[serde(default)]
    pub cc: Vec<InboxEmailAddress>,
    #[serde(default)]
    pub bcc: Vec<InboxEmailAddress>,
    #[serde(default)]
    pub reply_to: Vec<InboxEmailAddress>,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub preview: Option<String>,
    #[serde(default)]
    pub text_body: Option<String>,
    #[serde(default)]
    pub html_body: Option<String>,
    #[serde(default)]
    pub attachments: Vec<InboxAttachment>,
    #[serde(default)]
    pub received_at: Option<String>,
    #[serde(default)]
    pub is_unread: bool,
    #[serde(default)]
    pub size_bytes: i64,
}

/// `GET /mailboxes/{id}/messages` payload. The backend uses `total`/`page`/
/// `limit` rather than the shared `pagination` block.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxMessagePage {
    #[serde(default)]
    pub items: Vec<InboxMessageSummary>,
    #[serde(default)]
    pub total: u64,
    #[serde(default)]
    pub page: u32,
    #[serde(default)]
    pub limit: u32,
}

/// `DELETE /mailboxes/{id}/messages/{messageId}` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxMessageDeleted {
    pub message_id: String,
}
