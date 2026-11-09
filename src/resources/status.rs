use reqwest::Method;
use std::sync::Arc;

use crate::errors::Result;
use crate::http::HttpClient;
use crate::models::billing::AccountStatus;
use crate::models::common::ApiResponse;

/// Resource for system and account health status.
#[derive(Clone)]
pub struct StatusResource {
    http: Arc<HttpClient>,
}

impl StatusResource {
    pub fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Check system and account health.
    pub async fn check(&self) -> Result<ApiResponse<AccountStatus>> {
        let builder = self.http.request(Method::GET, "/status");
        self.http.execute(builder).await
    }
}
