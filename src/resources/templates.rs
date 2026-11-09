use reqwest::Method;
use std::sync::Arc;

use crate::errors::Result;
use crate::http::HttpClient;
use crate::models::common::ApiResponse;
use crate::models::templates::{
    CreateTemplateRequest, CreateWebhookRequest, Template, TemplatePreview, Webhook,
};

/// Resource for managing reusable email templates.
#[derive(Clone)]
pub struct TemplatesResource {
    http: Arc<HttpClient>,
}

impl TemplatesResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List all email templates.
    pub async fn list(&self) -> Result<ApiResponse<Vec<Template>>> {
        let builder = self.http.request(Method::GET, "/templates");
        self.http.execute(builder).await
    }

    /// Create a new template.
    pub async fn create(&self, request: CreateTemplateRequest) -> Result<ApiResponse<Template>> {
        let builder = self.http.request(Method::POST, "/templates").json(&request);
        self.http.execute(builder).await
    }

    /// Retrieve a template by ID.
    pub async fn get(&self, template_id: &str) -> Result<ApiResponse<Template>> {
        let path = format!("/templates/{template_id}");
        let builder = self.http.request(Method::GET, &path);
        self.http.execute(builder).await
    }

    /// Update a template.
    pub async fn update(
        &self,
        template_id: &str,
        request: CreateTemplateRequest,
    ) -> Result<ApiResponse<Template>> {
        let path = format!("/templates/{template_id}");
        let builder = self.http.request(Method::PUT, &path).json(&request);
        self.http.execute(builder).await
    }

    /// Preview rendered template with sample variable substitutions.
    pub async fn preview(
        &self,
        template_id: &str,
        variables: serde_json::Value,
    ) -> Result<ApiResponse<TemplatePreview>> {
        let path = format!("/templates/{template_id}/preview");
        let builder = self.http.request(Method::POST, &path).json(&variables);
        self.http.execute(builder).await
    }

    /// Delete a template.
    pub async fn delete(&self, template_id: &str) -> Result<ApiResponse<()>> {
        let path = format!("/templates/{template_id}");
        let builder = self.http.request(Method::DELETE, &path);
        self.http.execute(builder).await
    }
}

/// Resource for configuring webhook event listeners.
#[derive(Clone)]
pub struct WebhooksResource {
    http: Arc<HttpClient>,
}

impl WebhooksResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List all configured webhooks.
    pub async fn list(&self) -> Result<ApiResponse<Vec<Webhook>>> {
        let builder = self.http.request(Method::GET, "/webhooks");
        self.http.execute(builder).await
    }

    /// Register a new webhook endpoint.
    pub async fn create(&self, request: CreateWebhookRequest) -> Result<ApiResponse<Webhook>> {
        let builder = self.http.request(Method::POST, "/webhooks").json(&request);
        self.http.execute(builder).await
    }

    /// Delete a webhook endpoint.
    pub async fn delete(&self, webhook_id: &str) -> Result<ApiResponse<()>> {
        let path = format!("/webhooks/{webhook_id}");
        let builder = self.http.request(Method::DELETE, &path);
        self.http.execute(builder).await
    }
}
