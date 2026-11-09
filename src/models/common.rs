use serde::{Deserialize, Serialize};

/// Standard API response envelope returned by all WazobiaMail endpoints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: T,
}

/// Generic paginated collection.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(bound(deserialize = "T: serde::de::DeserializeOwned"))]
pub struct Page<T> {
    #[serde(default)]
    pub items: Vec<T>,
    #[serde(default)]
    pub pagination: Option<Pagination>,
}

/// Pagination metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    #[serde(default, rename = "currentPage", alias = "current_page")]
    pub current_page: u32,
    #[serde(default, rename = "pageSize", alias = "page_size")]
    pub page_size: u32,
    #[serde(default, rename = "totalItems", alias = "total_items")]
    pub total_items: u64,
    #[serde(default, rename = "totalPages", alias = "total_pages")]
    pub total_pages: u32,
    #[serde(default, rename = "hasNext", alias = "has_next")]
    pub has_next: bool,
    #[serde(default, rename = "hasPrev", alias = "has_prev")]
    pub has_prev: bool,
}
