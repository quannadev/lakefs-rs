#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLakefs {
    id: String,
    creation_date: u64,
    friendly_name: String,
    email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserAuthInfo {
    pub access_key_id: String,
    pub secret_access_key: Option<String>,
    pub creation_date: u64,
}
