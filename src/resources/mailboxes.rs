use reqwest::Method;
use std::sync::Arc;

use crate::errors::Result;
use crate::http::HttpClient;
use crate::models::common::ApiResponse;
use crate::models::mailboxes::{
    CreateMailboxCredentialRequest, CreateMailboxRequest, CreatedMailbox, InboxMessageDetail,
    Mailbox, MailboxCredentialEnvelope, MailboxCredentialRevoked, MailboxDeleted, MailboxEnvelope,
    MailboxMessageDeleted, MailboxMessagePage, MailboxStorage,
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

    /// List all hosted mailboxes.
    ///
    /// The backend returns a bare array in `data`.
    pub async fn list(&self) -> Result<ApiResponse<Vec<Mailbox>>> {
        let builder = self.http.request(Method::GET, "/mailboxes");
        self.http.execute(builder).await
    }

    /// Provision a new mailbox with a generated password.
    pub async fn create(
        &self,
        domain_id: &str,
        local_part: &str,
        display_name: Option<&str>,
    ) -> Result<ApiResponse<CreatedMailbox>> {
        self.create_with_request(CreateMailboxRequest {
            domain_id: domain_id.to_string(),
            local_part: local_part.to_string(),
            display_name: display_name.map(|s| s.to_string()),
            quota_mb: None,
            password: None,
            role: None,
            status: None,
        })
        .await
    }

    /// Provision a new mailbox with explicit options.
    pub async fn create_with_request(
        &self,
        request: CreateMailboxRequest,
    ) -> Result<ApiResponse<CreatedMailbox>> {
        let builder = self.http.request(Method::POST, "/mailboxes").json(&request);
        self.http.execute(builder).await
    }

    /// Retrieve mailbox details (`{"mailbox": {...}}`).
    pub async fn get(&self, mailbox_id: &str) -> Result<ApiResponse<MailboxEnvelope>> {
        let path = format!("/mailboxes/{mailbox_id}");
        let builder = self.http.request(Method::GET, &path);
        self.http.execute(builder).await
    }

    /// Update a mailbox display name or password (`PATCH /mailboxes/{id}`).
    pub async fn update(
        &self,
        mailbox_id: &str,
        display_name: Option<&str>,
    ) -> Result<ApiResponse<MailboxEnvelope>> {
        let path = format!("/mailboxes/{mailbox_id}");
        let payload = serde_json::json!({ "displayName": display_name });
        let builder = self.http.request(Method::PATCH, &path).json(&payload);
        self.http.execute(builder).await
    }

    /// Delete a hosted mailbox.
    pub async fn delete(&self, mailbox_id: &str) -> Result<ApiResponse<MailboxDeleted>> {
        let path = format!("/mailboxes/{mailbox_id}");
        let builder = self.http.request(Method::DELETE, &path);
        self.http.execute(builder).await
    }

    /// Retrieve account-wide mailbox storage quotas and usage.
    pub async fn get_storage(&self) -> Result<ApiResponse<MailboxStorage>> {
        let builder = self.http.request(Method::GET, "/mailboxes/storage");
        self.http.execute(builder).await
    }

    /// Generate a mailbox password credential.
    ///
    /// The backend currently rejects `expiresAt`, so this sends an empty body.
    pub async fn create_credential(
        &self,
        mailbox_id: &str,
    ) -> Result<ApiResponse<MailboxCredentialEnvelope>> {
        self.create_credential_with_expiry(mailbox_id, None).await
    }

    /// Generate a mailbox credential with an optional expiry timestamp.
    ///
    /// `expiresAt` is forwarded when provided, but the current backend rejects
    /// expiring inbox passwords.
    pub async fn create_credential_with_expiry(
        &self,
        mailbox_id: &str,
        expires_at: Option<&str>,
    ) -> Result<ApiResponse<MailboxCredentialEnvelope>> {
        let path = format!("/mailboxes/{mailbox_id}/credentials");
        let payload = CreateMailboxCredentialRequest {
            expires_at: expires_at.map(|s| s.to_string()),
        };
        let builder = self.http.request(Method::POST, &path).json(&payload);
        self.http.execute(builder).await
    }

    /// Revoke a mailbox credential
    /// (`POST /mailboxes/{id}/credentials/{credentialId}/revoke`).
    pub async fn revoke_credential(
        &self,
        mailbox_id: &str,
        credential_id: &str,
    ) -> Result<ApiResponse<MailboxCredentialRevoked>> {
        let path = format!("/mailboxes/{mailbox_id}/credentials/{credential_id}/revoke");
        let builder = self.http.request(Method::POST, &path);
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
    ) -> Result<ApiResponse<MailboxMessagePage>> {
        self.list_with_params(mailbox_id, unread_only, None, None, None)
            .await
    }

    /// List messages with pagination (`page`, `limit`), unread filter, and search.
    pub async fn list_with_params(
        &self,
        mailbox_id: &str,
        unread_only: Option<bool>,
        page: Option<u32>,
        limit: Option<u32>,
        search: Option<&str>,
    ) -> Result<ApiResponse<MailboxMessagePage>> {
        let path = format!("/mailboxes/{mailbox_id}/messages");
        let mut builder = self.http.request(Method::GET, &path);
        if let Some(unread) = unread_only {
            builder = builder.query(&[("unreadOnly", unread)]);
        }
        if let Some(page) = page {
            builder = builder.query(&[("page", page)]);
        }
        if let Some(limit) = limit {
            builder = builder.query(&[("limit", limit)]);
        }
        if let Some(search) = search {
            builder = builder.query(&[("search", search)]);
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
    pub async fn delete(
        &self,
        mailbox_id: &str,
        message_id: &str,
    ) -> Result<ApiResponse<MailboxMessageDeleted>> {
        let path = format!("/mailboxes/{mailbox_id}/messages/{message_id}");
        let builder = self.http.request(Method::DELETE, &path);
        self.http.execute(builder).await
    }
}
