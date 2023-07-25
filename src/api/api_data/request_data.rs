#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub has_more: bool,
    pub next_offset: String,
    pub results: u64,
    pub max_per_page: u64,
}

#[derive(Debug, Deserialize)]
pub struct ResultData<T> {
    pub pagination: Pagination,
    pub results: T,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QueryData {
    pub after: String,
    pub amount: u64,
    pub prefix: String,
    pub delimiter: String,
    pub user_metadata: bool,
    pub presign: bool,
    #[serde(rename = "path")]
    pub file_name: String,
}

impl Default for QueryData {
    fn default() -> Self {
        Self {
            after: String::new(),
            amount: 100,
            prefix: String::new(),
            delimiter: String::new(),
            user_metadata: false,
            presign: false,
            file_name: String::new(),
        }
    }
}
