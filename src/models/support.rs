use serde::{Deserialize, Serialize};

/// Support ticket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportTicket {
    pub id: String,
    pub subject: String,
    pub status: String,
    pub priority: String,
    #[serde(rename = "createdAt", alias = "created_at", default)]
    pub created_at: Option<String>,
}

/// Request to create a new support ticket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSupportTicketRequest {
    pub subject: String,
    pub message: String,
    #[serde(default)]
    pub priority: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
}

/// Message on a support ticket thread.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportMessage {
    pub id: String,
    #[serde(rename = "ticketId", alias = "ticket_id", default)]
    pub ticket_id: String,
    pub message: String,
    #[serde(rename = "senderType", alias = "sender_type", default)]
    pub sender_type: String,
    #[serde(rename = "createdAt", alias = "created_at", default)]
    pub created_at: Option<String>,
}
