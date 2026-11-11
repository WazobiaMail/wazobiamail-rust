use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Email/social template entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub id: String,
    #[serde(default)]
    pub account_id: Option<i64>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub slug: String,
    /// `email`, `sms`, or `whatsapp`.
    #[serde(default)]
    pub channel: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub rejection_reason: Option<String>,
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub text_body: String,
    #[serde(default)]
    pub meta_template_id: Option<String>,
    #[serde(default)]
    pub meta_template_name: Option<String>,
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub components: Value,
    #[serde(default)]
    pub variables: Value,
    #[serde(default)]
    pub design: Option<Value>,
    #[serde(default)]
    pub preview_image_url: Option<String>,
    #[serde(default)]
    pub is_system: bool,
    #[serde(default)]
    pub revision: i32,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Request body for `POST /templates`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTemplateRequest {
    pub name: String,
    #[serde(default = "default_channel")]
    pub channel: String,
    #[serde(default = "default_category")]
    pub category: String,
    #[serde(default)]
    pub subject: String,
    pub body: String,
    #[serde(default)]
    pub text_body: String,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub design: Option<Value>,
    #[serde(default)]
    pub auto_submit_meta: bool,
    #[serde(default)]
    pub is_system: bool,
}

fn default_channel() -> String {
    "email".to_string()
}
fn default_category() -> String {
    "marketing".to_string()
}
fn default_language() -> String {
    "en_US".to_string()
}

impl CreateTemplateRequest {
    pub fn new(name: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            channel: default_channel(),
            category: default_category(),
            subject: String::new(),
            body: body.into(),
            text_body: String::new(),
            language: default_language(),
            components: None,
            variables: None,
            design: None,
            auto_submit_meta: false,
            is_system: false,
        }
    }
}

/// Request body for `PUT /templates/{id}`. Only provided fields change.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTemplateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub design: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Query parameters accepted by `GET /templates`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateListQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

/// `GET /templates` payload: `{"items", "total", "limit", "offset"}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplatePage {
    #[serde(default)]
    pub items: Vec<Template>,
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

/// Request body for `POST /templates/{id}/preview`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewTemplateRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_variables: Option<Value>,
}

/// Rendered template preview.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplatePreview {
    #[serde(default)]
    pub channel: String,
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub rendered_body: String,
    #[serde(default)]
    pub rendered_text: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub can_send: bool,
}
