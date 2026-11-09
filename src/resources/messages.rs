use reqwest::multipart::{Form, Part};
use reqwest::Method;
use std::sync::Arc;

use crate::errors::Result;
use crate::http::HttpClient;
use crate::models::common::ApiResponse;
use crate::models::messages::{
    AttachmentFile, EmailLogs, SendMessageRequest, SendResult, SpamCheckRequest, SpamCheckResult,
    TrackingDetail, TrackingList, UploadedFile,
};

/// Resource for transactional email sending, deliverability, and tracking.
#[derive(Clone)]
pub struct MessagesResource {
    http: Arc<HttpClient>,
}

impl MessagesResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Send a transactional email.
    pub async fn send(&self, request: SendMessageRequest) -> Result<ApiResponse<SendResult>> {
        self.send_with_idempotency(request, None).await
    }

    /// Send a transactional email with an optional idempotency key.
    pub async fn send_with_idempotency(
        &self,
        request: SendMessageRequest,
        idempotency_key: Option<&str>,
    ) -> Result<ApiResponse<SendResult>> {
        let mut builder = self
            .http
            .request(Method::POST, "/sending/messages")
            .json(&request);
        if let Some(key) = idempotency_key {
            builder = builder.header("Idempotency-Key", key);
        }
        self.http.execute(builder).await
    }

    /// Send a message with direct binary attachments via multipart form data.
    pub async fn send_with_attachments(
        &self,
        request: SendMessageRequest,
        files: Vec<AttachmentFile>,
    ) -> Result<ApiResponse<SendResult>> {
        let mut form = Form::new();
        let message_json = serde_json::to_string(&request)?;
        form = form.text("message", message_json);

        for file in files {
            let part = Part::bytes(file.content)
                .file_name(file.file_name)
                .mime_str(&file.content_type)
                .unwrap_or_else(|_| Part::bytes(vec![]));
            form = form.part("files", part);
        }

        let builder = self
            .http
            .request(Method::POST, "/sending/messages/with-attachments")
            .multipart(form);
        self.http.execute(builder).await
    }

    /// Upload a single attachment to receive a persistent public URL.
    pub async fn upload_attachment(
        &self,
        file: AttachmentFile,
    ) -> Result<ApiResponse<UploadedFile>> {
        let part = Part::bytes(file.content)
            .file_name(file.file_name)
            .mime_str(&file.content_type)
            .unwrap_or_else(|_| Part::bytes(vec![]));
        let form = Form::new().part("file", part);

        let builder = self
            .http
            .request(Method::POST, "/sending/attachments/upload")
            .multipart(form);
        self.http.execute(builder).await
    }

    /// Run deliverability and spam analysis on an email.
    pub async fn spam_check(
        &self,
        request: SpamCheckRequest,
    ) -> Result<ApiResponse<SpamCheckResult>> {
        let builder = self
            .http
            .request(Method::POST, "/sending/spam-check")
            .json(&request);
        self.http.execute(builder).await
    }

    /// List tracked delivery messages.
    pub async fn list_tracking(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
        status: Option<&str>,
    ) -> Result<ApiResponse<TrackingList>> {
        let mut builder = self.http.request(Method::GET, "/sending/tracking/messages");
        if let Some(l) = limit {
            builder = builder.query(&[("limit", l)]);
        }
        if let Some(o) = offset {
            builder = builder.query(&[("offset", o)]);
        }
        if let Some(s) = status {
            builder = builder.query(&[("status", s)]);
        }
        self.http.execute(builder).await
    }

    /// List tracked messages (alias for list_tracking).
    pub async fn list_tracked(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
        status: Option<&str>,
    ) -> Result<ApiResponse<TrackingList>> {
        self.list_tracking(limit, offset, status).await
    }

    /// Read detailed delivery-event timeline for a tracked message.
    pub async fn get_tracking(&self, tracking_id: &str) -> Result<ApiResponse<TrackingDetail>> {
        let path = format!("/sending/tracking/messages/{tracking_id}");
        let builder = self.http.request(Method::GET, &path);
        self.http.execute(builder).await
    }

    /// Read detailed delivery-event timeline (alias for get_tracking).
    pub async fn get_tracked(&self, tracking_id: &str) -> Result<ApiResponse<TrackingDetail>> {
        self.get_tracking(tracking_id).await
    }

    /// List customer-visible email delivery logs.
    pub async fn list_logs(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<ApiResponse<EmailLogs>> {
        let mut builder = self.http.request(Method::GET, "/account/email-logs");
        if let Some(l) = limit {
            builder = builder.query(&[("limit", l)]);
        }
        if let Some(o) = offset {
            builder = builder.query(&[("offset", o)]);
        }
        self.http.execute(builder).await
    }
}
