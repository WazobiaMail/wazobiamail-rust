use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Hosted mailbox entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mailbox {
    pub id: String,
    #[serde(rename = "domainId", alias = "domain_id", default)]
    pub domain_id: String,
    #[serde(default, alias = "address")]
    pub email: String,
    #[serde(rename = "localPart", alias = "local_part", default)]
    pub local_part: String,
    #[serde(rename = "displayName", alias = "display_name", default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(rename = "storageUsageBytes", alias = "storage_usage_bytes", default)]
    pub storage_usage_bytes: u64,
    #[serde(rename = "createdAt", alias = "created_at", default)]
    pub created_at: Option<String>,
}

/// Request to provision a new mailbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMailboxRequest {
    #[serde(rename = "domainId")]
    pub domain_id: String,
    #[serde(rename = "localPart")]
    pub local_part: String,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// Mailbox authentication credential.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailboxCredential {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(rename = "createdAt", alias = "created_at", default)]
    pub created_at: Option<String>,
}

/// Response returned when a mailbox is created.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedMailbox {
    pub mailbox: Mailbox,
    pub credential: Option<MailboxCredential>,
}

/// Storage metrics for account mailboxes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailboxStorage {
    #[serde(rename = "totalBytes", alias = "total_bytes", default)]
    pub total_bytes: u64,
    #[serde(rename = "quotaBytes", alias = "quota_bytes", default)]
    pub quota_bytes: u64,
    #[serde(rename = "percentUsed", alias = "percent_used", default)]
    pub percent_used: f64,
    #[serde(rename = "mailboxesCount", alias = "mailboxes_count", default)]
    pub mailboxes_count: u32,
}

/// Email address representation for inbox messages.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InboxEmailAddress {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub email: String,
}

/// Summary of an enterprise headless inbox message.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InboxMessageSummary {
    pub id: String,
    #[serde(rename = "messageId", alias = "message_id", default)]
    pub message_id: String,
    #[serde(default)]
    pub subject: String,
    pub from: InboxEmailAddress,
    #[serde(default)]
    pub to: Vec<String>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(rename = "isRead", alias = "is_read", default)]
    pub is_read: bool,
    #[serde(rename = "hasAttachments", alias = "has_attachments", default)]
    pub has_attachments: bool,
    #[serde(rename = "sizeBytes", alias = "size_bytes", default)]
    pub size_bytes: u64,
}

/// Attachment on an inbox message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxAttachment {
    pub id: String,
    #[serde(rename = "fileName", alias = "file_name")]
    pub file_name: String,
    #[serde(rename = "contentType", alias = "content_type")]
    pub content_type: String,
    #[serde(rename = "sizeBytes", alias = "size_bytes", default)]
    pub size_bytes: u64,
}

/// Detailed enterprise headless inbox message with body and headers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxMessageDetail {
    pub id: String,
    #[serde(rename = "messageId", alias = "message_id", default)]
    pub message_id: String,
    #[serde(default)]
    pub subject: String,
    pub from: InboxEmailAddress,
    #[serde(default)]
    pub to: Vec<String>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(rename = "isRead", alias = "is_read", default)]
    pub is_read: bool,
    #[serde(rename = "hasAttachments", alias = "has_attachments", default)]
    pub has_attachments: bool,
    #[serde(rename = "sizeBytes", alias = "size_bytes", default)]
    pub size_bytes: u64,
    #[serde(rename = "htmlBody", alias = "html_body", default)]
    pub html_body: Option<String>,
    #[serde(rename = "textBody", alias = "text_body", default)]
    pub text_body: Option<String>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub attachments: Vec<InboxAttachment>,
}
