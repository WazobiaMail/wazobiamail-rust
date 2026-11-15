use reqwest::multipart::{Form, Part};
use reqwest::Method;
use std::sync::Arc;

use crate::errors::{Result, WazobiaError};
use crate::http::{encode_path_segment, HttpClient};
use crate::models::common::ApiResponse;
use crate::models::messages::{
    AttachmentFile, EmailLogs, SendEnvelope, SendMessageRequest, SpamCheckRequest, SpamCheckResult,
    TrackingDetailEnvelope, TrackingList, UploadedFileEnvelope,
};

/// Resource for transactional email sending, deliverability, and tracking.
#[derive(Clone)]
pub struct MessagesResource {
    http: Arc<HttpClient>,
}

fn sanitize_file_name(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| {
            if matches!(c, '\r' | '\n' | '\0' | '"' | '\\' | '/') {
                '_'
            } else {
                c
            }
        })
        .collect();
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        "attachment.dat".to_string()
    } else {
        trimmed.to_string()
    }
}

fn sanitize_mime(mime: &str) -> String {
    let cleaned: String = mime
        .chars()
        .filter(|c| !matches!(c, '\r' | '\n' | '\0'))
        .collect();
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        "application/octet-stream".to_string()
    } else {
        trimmed.to_string()
    }
}

fn build_file_part(file: AttachmentFile) -> Result<Part> {
    if file.content.is_empty() {
        return Err(WazobiaError::Validation {
            message: "Attachment file content must not be empty".to_string(),
            errors: Default::default(),
        });
    }

    let safe_name = sanitize_file_name(&file.file_name);
    let candidate_mime = sanitize_mime(&file.content_type);
    let safe_mime = if Part::bytes(&[][..]).mime_str(&candidate_mime).is_ok() {
        candidate_mime
    } else {
        "application/octet-stream".to_string()
    };

    Ok(Part::bytes(file.content)
        .file_name(safe_name)
        .mime_str(&safe_mime)?)
}

impl MessagesResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Send a transactional email.
    pub async fn send(&self, request: SendMessageRequest) -> Result<ApiResponse<SendEnvelope>> {
        self.send_with_idempotency(request, None).await
    }

    /// Send a transactional email with an optional idempotency key.
    pub async fn send_with_idempotency(
        &self,
        request: SendMessageRequest,
        idempotency_key: Option<&str>,
    ) -> Result<ApiResponse<SendEnvelope>> {
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
    ) -> Result<ApiResponse<SendEnvelope>> {
        if files.is_empty() {
            return Err(WazobiaError::Validation {
                message: "At least one file attachment is required".to_string(),
                errors: Default::default(),
            });
        }

        let mut form = Form::new();
        let message_json = serde_json::to_string(&request)?;
        form = form.text("message", message_json);

        for file in files {
            form = form.part("files", build_file_part(file)?);
        }

        let builder = self
            .http
            .request(Method::POST, "/sending/messages/with-attachments")
            .multipart(form);
        self.http.execute(builder).await
    }

    /// Upload a single attachment to receive a persistent public URL.
    ///
    /// The backend returns `data: {"file": {...}}`.
    pub async fn upload_attachment(
        &self,
        file: AttachmentFile,
    ) -> Result<ApiResponse<UploadedFileEnvelope>> {
        let form = Form::new().part("file", build_file_part(file)?);

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

    /// List tracked delivery messages (`GET /sending/tracking/messages`).
    ///
    /// The backend returns `{"messages": [...], "summary": {...}}`.
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

    /// List tracked messages (alias for `list_tracking`).
    pub async fn list_tracked(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
        status: Option<&str>,
    ) -> Result<ApiResponse<TrackingList>> {
        self.list_tracking(limit, offset, status).await
    }

    /// Read detailed delivery-event timeline for a tracked message.
    ///
    /// The backend returns `{"message": {"message": {...}, "events": [...]}}`.
    pub async fn get_tracking(
        &self,
        tracking_id: &str,
    ) -> Result<ApiResponse<TrackingDetailEnvelope>> {
        let tracking_id = encode_path_segment(tracking_id);
        let path = format!("/sending/tracking/messages/{tracking_id}");
        let builder = self.http.request(Method::GET, &path);
        self.http.execute(builder).await
    }

    /// Read detailed delivery-event timeline (alias for `get_tracking`).
    pub async fn get_tracked(
        &self,
        tracking_id: &str,
    ) -> Result<ApiResponse<TrackingDetailEnvelope>> {
        self.get_tracking(tracking_id).await
    }

    /// List customer-visible email delivery logs (`GET /account/email-logs`).
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
