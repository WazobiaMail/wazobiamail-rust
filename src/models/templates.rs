use serde::{Deserialize, Serialize};

/// Email template entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub html: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
}

/// Request to create or update an email template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Rendered template preview.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatePreview {
    #[serde(rename = "renderedHtml", alias = "rendered_html", default)]
    pub rendered_html: Option<String>,
    #[serde(rename = "renderedText", alias = "rendered_text", default)]
    pub rendered_text: Option<String>,
}

/// Webhook endpoint configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    pub id: String,
    pub url: String,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(default)]
    pub secret: Option<String>,
}

/// Request to create a webhook endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebhookRequest {
    pub url: String,
    pub events: Vec<String>,
}
