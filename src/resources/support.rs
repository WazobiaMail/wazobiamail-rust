use reqwest::Method;
use std::sync::Arc;

use crate::errors::Result;
use crate::http::HttpClient;
use crate::models::common::{ApiResponse, Empty};
use crate::models::support::{
    CreateSupportTicketRequest, ReplySupportRequest, SupportTicket, SupportTicketCreated,
    SupportTicketPage,
};

/// Resource for customer support tickets and thread messages.
#[derive(Clone)]
pub struct SupportResource {
    http: Arc<HttpClient>,
}

impl SupportResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List the current account's support tickets (`GET /support/my`).
    pub async fn list(&self) -> Result<ApiResponse<SupportTicketPage>> {
        let builder = self.http.request(Method::GET, "/support/my");
        self.http.execute(builder).await
    }

    /// List support tickets with filters (`GET /support/my`).
    pub async fn list_with_params(
        &self,
        status: Option<&str>,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<ApiResponse<SupportTicketPage>> {
        let mut builder = self.http.request(Method::GET, "/support/my");
        if let Some(status) = status {
            builder = builder.query(&[("status", status)]);
        }
        if let Some(page) = page {
            builder = builder.query(&[("page", page)]);
        }
        if let Some(limit) = limit {
            builder = builder.query(&[("limit", limit)]);
        }
        self.http.execute(builder).await
    }

    /// Create a new support ticket (`POST /support`).
    ///
    /// The payload only contains the newly created identifiers; read the ticket
    /// with [`SupportResource::get`] for the full thread.
    pub async fn create(
        &self,
        request: CreateSupportTicketRequest,
    ) -> Result<ApiResponse<SupportTicketCreated>> {
        let builder = self.http.request(Method::POST, "/support").json(&request);
        self.http.execute(builder).await
    }

    /// Retrieve a support ticket and its messages by ID.
    pub async fn get(&self, ticket_id: &str) -> Result<ApiResponse<SupportTicket>> {
        let path = format!("/support/tickets/{ticket_id}");
        let builder = self.http.request(Method::GET, &path);
        self.http.execute(builder).await
    }

    /// Post a plain-text reply to an open support ticket.
    ///
    /// The backend responds with an empty object.
    pub async fn reply(&self, ticket_id: &str, message: &str) -> Result<ApiResponse<Empty>> {
        self.reply_with(ticket_id, ReplySupportRequest::text(message))
            .await
    }

    /// Post a reply with an explicit content type and attachments.
    pub async fn reply_with(
        &self,
        ticket_id: &str,
        request: ReplySupportRequest,
    ) -> Result<ApiResponse<Empty>> {
        let path = format!("/support/tickets/{ticket_id}/messages");
        let builder = self.http.request(Method::POST, &path).json(&request);
        self.http.execute(builder).await
    }
}
