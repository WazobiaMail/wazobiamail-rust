use reqwest::Method;
use std::sync::Arc;

use crate::errors::Result;
use crate::http::HttpClient;
use crate::models::common::{ApiResponse, Page};
use crate::models::mailboxes::{
    CreateMailboxRequest, CreatedMailbox, InboxMessageDetail, InboxMessageSummary, Mailbox,
    MailboxCredential, MailboxStorage,
};

/// Resource for hosted mailboxes, credentials, storage, and headless inboxes.
#[derive(Clone)]
pub struct MailboxesResource {
    http: Arc<HttpClient>,
}

impl MailboxesResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List all hosted mailboxes, optionally filtered by domain ID.
    pub async fn list(&self, domain_id: Option<&str>) -> Result<ApiResponse<Vec<Mailbox>>> {
        let mut builder = self.http.request(Method::GET, "/mailboxes");
        if let Some(did) = domain_id {
            builder = builder.query(&[("domainId", did)]);
        }
        self.http.execute(builder).await
    }

    /// Provision a new mailbox.
    pub async fn create(
        &self,
        domain_id: &str,
        local_part: &str,
        display_name: Option<&str>,
    ) -> Result<ApiResponse<CreatedMailbox>> {
        let payload = CreateMailboxRequest {
            domain_id: domain_id.to_string(),
            local_part: local_part.to_string(),
            display_name: display_name.map(|s| s.to_string()),
            password: None,
        };
        let builder = self.http.request(Method::POST, "/mailboxes").json(&payload);
        self.http.execute(builder).await
    }

    /// Retrieve mailbox details.
    pub async fn get(&self, mailbox_id: &str) -> Result<ApiResponse<Mailbox>> {
        let path = format!("/mailboxes/{mailbox_id}");
        let builder = self.http.request(Method::GET, &path);
        self.http.execute(builder).await
    }

    /// Update mailbox display name.
    pub async fn update(
        &self,
        mailbox_id: &str,
        display_name: Option<&str>,
    ) -> Result<ApiResponse<Mailbox>> {
        let path = format!("/mailboxes/{mailbox_id}");
        let payload = serde_json::json!({ "displayName": display_name });
        let builder = self.http.request(Method::PATCH, &path).json(&payload);
        self.http.execute(builder).await
    }

    /// Delete a hosted mailbox.
    pub async fn delete(&self, mailbox_id: &str) -> Result<ApiResponse<()>> {
        let path = format!("/mailboxes/{mailbox_id}");
        let builder = self.http.request(Method::DELETE, &path);
        self.http.execute(builder).await
    }

    /// Retrieve account-wide mailbox storage quotas and usage metrics.
    pub async fn get_storage(&self) -> Result<ApiResponse<MailboxStorage>> {
        let builder = self.http.request(Method::GET, "/mailboxes/storage");
        self.http.execute(builder).await
    }

    /// Generate or rotate password credentials for a mailbox.
    pub async fn create_credential(
        &self,
        mailbox_id: &str,
        password: Option<&str>,
    ) -> Result<ApiResponse<MailboxCredential>> {
        let path = format!("/mailboxes/{mailbox_id}/credentials");
        let payload = serde_json::json!({ "password": password });
        let builder = self.http.request(Method::POST, &path).json(&payload);
        self.http.execute(builder).await
    }

    /// Revoke a mailbox credential.
    pub async fn revoke_credential(
        &self,
        mailbox_id: &str,
        credential_id: &str,
    ) -> Result<ApiResponse<()>> {
        let path = format!("/mailboxes/{mailbox_id}/credentials/{credential_id}");
        let builder = self.http.request(Method::DELETE, &path);
        self.http.execute(builder).await
    }

    /// Access enterprise headless inbox messages on this mailbox.
    pub fn messages(&self) -> InboxMessagesSubResource {
        InboxMessagesSubResource {
            http: Arc::clone(&self.http),
        }
    }
}

/// Enterprise headless inbox messages sub-resource.
#[derive(Clone)]
pub struct InboxMessagesSubResource {
    http: Arc<HttpClient>,
}

impl InboxMessagesSubResource {
    /// List messages in an enterprise inbox.
    pub async fn list(
        &self,
        mailbox_id: &str,
        unread_only: Option<bool>,
    ) -> Result<ApiResponse<Page<InboxMessageSummary>>> {
        self.list_with_params(mailbox_id, unread_only, None, None)
            .await
    }

    /// List messages in an enterprise inbox with pagination parameters.
    pub async fn list_with_params(
        &self,
        mailbox_id: &str,
        unread_only: Option<bool>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<ApiResponse<Page<InboxMessageSummary>>> {
        let path = format!("/mailboxes/{mailbox_id}/messages");
        let mut builder = self.http.request(Method::GET, &path);
        if let Some(unread) = unread_only {
            builder = builder.query(&[("unreadOnly", unread)]);
        }
        if let Some(l) = limit {
            builder = builder.query(&[("limit", l)]);
        }
        if let Some(o) = offset {
            builder = builder.query(&[("offset", o)]);
        }
        self.http.execute(builder).await
    }

    /// Read full message details including HTML/text body and attachments.
    pub async fn get(
        &self,
        mailbox_id: &str,
        message_id: &str,
    ) -> Result<ApiResponse<InboxMessageDetail>> {
        let path = format!("/mailboxes/{mailbox_id}/messages/{message_id}");
        let builder = self.http.request(Method::GET, &path);
        self.http.execute(builder).await
    }

    /// Delete an inbox message.
    pub async fn delete(&self, mailbox_id: &str, message_id: &str) -> Result<ApiResponse<()>> {
        let path = format!("/mailboxes/{mailbox_id}/messages/{message_id}");
        let builder = self.http.request(Method::DELETE, &path);
        self.http.execute(builder).await
    }
}
