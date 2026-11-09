use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::errors::{Result, WazobiaError};

/// URL-referenced attachment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlAttachment {
    pub url: String,
    #[serde(rename = "fileName", alias = "file_name")]
    pub file_name: String,
}

impl UrlAttachment {
    pub fn new(url: impl Into<String>, file_name: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            file_name: file_name.into(),
        }
    }
}

/// Binary file attachment for multipart sending or uploading.
#[derive(Debug, Clone)]
pub struct AttachmentFile {
    pub file_name: String,
    pub content: Vec<u8>,
    pub content_type: String,
}

impl AttachmentFile {
    pub fn from_bytes(
        file_name: impl Into<String>,
        content: impl Into<Vec<u8>>,
        content_type: impl Into<String>,
    ) -> Self {
        Self {
            file_name: file_name.into(),
            content: content.into(),
            content_type: content_type.into(),
        }
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("attachment")
            .to_string();
        let content = std::fs::read(path)?;
        let content_type = "application/octet-stream".to_string();
        Ok(Self {
            file_name,
            content,
            content_type,
        })
    }
}

/// Inner content block for transactional message requests.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendMessageContent {
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "htmlBody", skip_serializing_if = "Option::is_none")]
    pub html_body: Option<String>,
    #[serde(rename = "textBody", skip_serializing_if = "Option::is_none")]
    pub text_body: Option<String>,
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
}

/// Top-level transactional send payload.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SendMessageRequest {
    pub to: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cc: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bcc: Vec<String>,
    pub content: SendMessageContent,
    #[serde(rename = "envelopeSender", skip_serializing_if = "Option::is_none")]
    pub envelope_sender: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<UrlAttachment>,
}

impl SendMessageRequest {
    pub fn builder() -> SendMessageRequestBuilder {
        SendMessageRequestBuilder::default()
    }
}

/// Fluent builder for transactional email sending requests.
#[derive(Debug, Default)]
pub struct SendMessageRequestBuilder {
    to: Vec<String>,
    cc: Vec<String>,
    bcc: Vec<String>,
    from: Option<String>,
    subject: Option<String>,
    html: Option<String>,
    text: Option<String>,
    reply_to: Option<String>,
    envelope_sender: Option<String>,
    attachments: Vec<UrlAttachment>,
}

impl SendMessageRequestBuilder {
    pub fn to(mut self, recipient: impl Into<String>) -> Self {
        self.to.push(recipient.into());
        self
    }

    pub fn to_many<I, S>(mut self, recipients: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for r in recipients {
            self.to.push(r.into());
        }
        self
    }

    pub fn cc(mut self, recipient: impl Into<String>) -> Self {
        self.cc.push(recipient.into());
        self
    }

    pub fn bcc(mut self, recipient: impl Into<String>) -> Self {
        self.bcc.push(recipient.into());
        self
    }

    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    pub fn html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn reply_to(mut self, reply_to: impl Into<String>) -> Self {
        self.reply_to = Some(reply_to.into());
        self
    }

    pub fn envelope_sender(mut self, envelope: impl Into<String>) -> Self {
        self.envelope_sender = Some(envelope.into());
        self
    }

    pub fn attachment(mut self, attachment: UrlAttachment) -> Self {
        self.attachments.push(attachment);
        self
    }

    pub fn build(self) -> Result<SendMessageRequest> {
        if self.to.is_empty() {
            return Err(WazobiaError::InvalidConfiguration(
                "At least one 'to' recipient is required".to_string(),
            ));
        }
        let subject = self.subject.unwrap_or_default();
        if self.html.is_none() && self.text.is_none() {
            return Err(WazobiaError::InvalidConfiguration(
                "Either html or text body must be provided".to_string(),
            ));
        }

        Ok(SendMessageRequest {
            to: self.to,
            cc: self.cc,
            bcc: self.bcc,
            content: SendMessageContent {
                subject,
                from: self.from,
                html_body: self.html,
                text_body: self.text,
                reply_to: self.reply_to,
            },
            envelope_sender: self.envelope_sender,
            attachments: self.attachments,
        })
    }
}

/// Send receipt response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendResult {
    #[serde(default)]
    pub status: String,
    #[serde(rename = "trackingId", alias = "tracking_id", default)]
    pub tracking_id: String,
    #[serde(default)]
    pub recipients: u32,
}

/// Result of uploading an attachment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadedFile {
    #[serde(rename = "fileName", alias = "file_name")]
    pub file_name: String,
    #[serde(rename = "contentType", alias = "content_type")]
    pub content_type: String,
    #[serde(rename = "sizeBytes", alias = "size_bytes")]
    pub size_bytes: u64,
    #[serde(rename = "publicUrl", alias = "public_url")]
    pub public_url: String,
}

/// Spam check request payload.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpamCheckRequest {
    #[serde(default)]
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "replyTo", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
}

impl SpamCheckRequest {
    pub fn builder() -> SpamCheckRequestBuilder {
        SpamCheckRequestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct SpamCheckRequestBuilder {
    subject: Option<String>,
    html: Option<String>,
    text: Option<String>,
    from: Option<String>,
    reply_to: Option<String>,
}

impl SpamCheckRequestBuilder {
    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }
    pub fn html(mut self, html: impl Into<String>) -> Self {
        self.html = Some(html.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }
    pub fn reply_to(mut self, reply_to: impl Into<String>) -> Self {
        self.reply_to = Some(reply_to.into());
        self
    }
    pub fn build(self) -> Result<SpamCheckRequest> {
        Ok(SpamCheckRequest {
            subject: self.subject.unwrap_or_default(),
            html: self.html,
            text: self.text,
            from: self.from,
            reply_to: self.reply_to,
        })
    }
}

/// Deliverability finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpamFinding {
    pub rule: String,
    pub score: f64,
    pub description: String,
}

/// Deliverability spam check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpamCheckResult {
    pub score: f64,
    pub threshold: f64,
    #[serde(
        rename = "deliverabilityScore",
        alias = "deliverability_score",
        default
    )]
    pub deliverability_score: u32,
    #[serde(default)]
    pub findings: Vec<SpamFinding>,
    #[serde(default)]
    pub recommendations: Vec<String>,
}

/// Tracked message metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingMessage {
    #[serde(rename = "trackingId", alias = "tracking_id")]
    pub tracking_id: String,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(rename = "recipientsCount", alias = "recipients_count", default)]
    pub recipients_count: u32,
    #[serde(rename = "opensCount", alias = "opens_count", default)]
    pub opens_count: u32,
    #[serde(rename = "clicksCount", alias = "clicks_count", default)]
    pub clicks_count: u32,
    #[serde(default)]
    pub status: String,
    #[serde(rename = "sentAt", alias = "sent_at", default)]
    pub sent_at: Option<String>,
}

/// Tracked event in a delivery timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingEvent {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "eventType", alias = "event_type")]
    pub event_type: String,
    #[serde(rename = "recipientEmail", alias = "recipient_email", default)]
    pub recipient_email: Option<String>,
    #[serde(rename = "clickUrl", alias = "click_url", default)]
    pub click_url: Option<String>,
    #[serde(rename = "occurredAt", alias = "occurred_at", default)]
    pub occurred_at: Option<String>,
}

/// Tracked message detail with event timeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingDetail {
    pub message: Option<TrackingMessage>,
    #[serde(default)]
    pub events: Vec<TrackingEvent>,
}

/// List of tracked messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingList {
    #[serde(default)]
    pub items: Vec<TrackingMessage>,
    #[serde(default)]
    pub total: u64,
}

/// Customer delivery audit log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailLog {
    #[serde(rename = "trackingId", alias = "tracking_id", default)]
    pub tracking_id: String,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub to: Vec<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(rename = "receivedAt", alias = "received_at", default)]
    pub received_at: Option<String>,
}

/// List of customer delivery audit logs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailLogs {
    #[serde(default)]
    pub messages: Vec<EmailLog>,
    #[serde(rename = "hasMore", alias = "has_more", default)]
    pub has_more: bool,
}
