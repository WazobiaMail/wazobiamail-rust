use reqwest::Method;
use std::sync::Arc;

use crate::errors::Result;
use crate::http::HttpClient;
use crate::models::common::ApiResponse;
use crate::models::templates::{
    CreateTemplateRequest, PreviewTemplateRequest, Template, TemplateListQuery, TemplatePage,
    TemplatePreview, UpdateTemplateRequest,
};

/// Resource for managing reusable email templates (`/templates`).
#[derive(Clone)]
pub struct TemplatesResource {
    http: Arc<HttpClient>,
}

impl TemplatesResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// List email templates with the default page (first 50).
    ///
    /// The backend returns `{"items", "total", "limit", "offset"}`.
    pub async fn list(&self) -> Result<ApiResponse<TemplatePage>> {
        let builder = self.http.request(Method::GET, "/templates");
        self.http.execute(builder).await
    }

    /// List email templates with filters and pagination.
    pub async fn list_with_query(
        &self,
        query: TemplateListQuery,
    ) -> Result<ApiResponse<TemplatePage>> {
        let mut builder = self.http.request(Method::GET, "/templates");
        if let Some(channel) = &query.channel {
            builder = builder.query(&[("channel", channel)]);
        }
        if let Some(category) = &query.category {
            builder = builder.query(&[("category", category)]);
        }
        if let Some(status) = &query.status {
            builder = builder.query(&[("status", status)]);
        }
        if let Some(search) = &query.search {
            builder = builder.query(&[("search", search)]);
        }
        if let Some(limit) = query.limit {
            builder = builder.query(&[("limit", limit)]);
        }
        if let Some(offset) = query.offset {
            builder = builder.query(&[("offset", offset)]);
        }
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

    /// Update a template (`PUT /templates/{id}`).
    pub async fn update(
        &self,
        template_id: &str,
        request: UpdateTemplateRequest,
    ) -> Result<ApiResponse<Template>> {
        let path = format!("/templates/{template_id}");
        let builder = self.http.request(Method::PUT, &path).json(&request);
        self.http.execute(builder).await
    }

    /// Preview a rendered template with optional sample variables.
    ///
    /// Sends `{"sampleVariables": {...}}` to match
    /// `PreviewTemplateRequest` on the backend.
    pub async fn preview(
        &self,
        template_id: &str,
        sample_variables: Option<serde_json::Value>,
    ) -> Result<ApiResponse<TemplatePreview>> {
        let path = format!("/templates/{template_id}/preview");
        let payload = PreviewTemplateRequest { sample_variables };
        let builder = self.http.request(Method::POST, &path).json(&payload);
        self.http.execute(builder).await
    }

    /// Delete a template.
    pub async fn delete(&self, template_id: &str) -> Result<ApiResponse<()>> {
        let path = format!("/templates/{template_id}");
        let builder = self.http.request(Method::DELETE, &path);
        self.http.execute(builder).await
    }
}
