use serde::{Deserialize, Serialize};

/// Attachment uploaded with a support ticket or reply.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportUpload {
    pub filename: String,
    /// Base64-encoded bytes (max 5 MiB per file, 10 MiB per message).
    pub data: String,
}

/// Stored support ticket attachment metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportAttachment {
    pub id: String,
    #[serde(default)]
    pub filename: String,
    #[serde(default)]
    pub size_bytes: i64,
}

/// Message on a support ticket thread.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportMessage {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub ticket_id: String,
    #[serde(default)]
    pub sender_name: String,
    #[serde(default)]
    pub sender_email: String,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub is_from_staff: bool,
    /// `text` or `html`.
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub attachments: Vec<SupportAttachment>,
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Full support ticket returned by `GET /support/tickets/{id}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportTicket {
    pub id: String,
    #[serde(default)]
    pub account_id: Option<i64>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub priority: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub messages: Vec<SupportMessage>,
    #[serde(default)]
    pub customer_read_at: Option<String>,
    #[serde(default)]
    pub staff_read_at: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Request body for `POST /support`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSupportTicketRequest {
    pub name: String,
    pub email: String,
    pub subject: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    pub category: String,
    pub priority: String,
    pub message: String,
    /// `text` (default) or `html`.
    #[serde(default = "default_content_type")]
    pub content_type: String,
    #[serde(default)]
    pub attachments: Vec<SupportUpload>,
}

fn default_content_type() -> String {
    "text".to_string()
}

/// Response from `POST /support`: `{"id": "...", "ticketId": "..."}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportTicketCreated {
    pub id: String,
    pub ticket_id: String,
}

/// `GET /support/my` payload: `{"items", "total", "page", "pages"}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportTicketPage {
    #[serde(default)]
    pub items: Vec<SupportTicket>,
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub page: i32,
    #[serde(default)]
    pub pages: i32,
}

/// Request body for `POST /support/tickets/{id}/messages`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplySupportRequest {
    pub message: String,
    #[serde(default = "default_content_type")]
    pub content_type: String,
    #[serde(default)]
    pub attachments: Vec<SupportUpload>,
}

impl ReplySupportRequest {
    pub fn text(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            content_type: default_content_type(),
            attachments: Vec::new(),
        }
    }
}
