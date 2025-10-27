// src/utils.rs
use serde::Deserialize;

// A struct to handle pagination query parameters, e.g., /api/klien?page=1&limit=20
#[derive(Debug, Deserialize)]
pub struct Pagination {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 { 1 }
fn default_limit() -> i64 { 20 }