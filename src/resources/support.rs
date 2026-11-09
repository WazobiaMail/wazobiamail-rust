use reqwest::Method;
use std::sync::Arc;

use crate::errors::Result;
use crate::http::HttpClient;
use crate::models::common::ApiResponse;
use crate::models::support::{CreateSupportTicketRequest, SupportMessage, SupportTicket};

/// Resource for customer support tickets and thread messages.
#[derive(Clone)]
pub struct SupportResource {
    http: Arc<HttpClient>,
}

impl SupportResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List all support tickets for the current account.
    pub async fn list(&self) -> Result<ApiResponse<Vec<SupportTicket>>> {
        let builder = self.http.request(Method::GET, "/support/tickets");
        self.http.execute(builder).await
    }

    /// Create a new support ticket.
    pub async fn create(
        &self,
        request: CreateSupportTicketRequest,
    ) -> Result<ApiResponse<SupportTicket>> {
        let builder = self
            .http
            .request(Method::POST, "/support/tickets")
            .json(&request);
        self.http.execute(builder).await
    }

    /// Retrieve a support ticket by ID.
    pub async fn get(&self, ticket_id: &str) -> Result<ApiResponse<SupportTicket>> {
        let path = format!("/support/tickets/{ticket_id}");
        let builder = self.http.request(Method::GET, &path);
        self.http.execute(builder).await
    }

    /// Post a reply message to an open support ticket.
    pub async fn reply(
        &self,
        ticket_id: &str,
        message: &str,
    ) -> Result<ApiResponse<SupportMessage>> {
        let path = format!("/support/tickets/{ticket_id}/messages");
        let payload = serde_json::json!({ "message": message });
        let builder = self.http.request(Method::POST, &path).json(&payload);
        self.http.execute(builder).await
    }
}
