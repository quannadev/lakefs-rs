#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLakefs {
    pub id: String,
    pub creation_date: u64,
    pub friendly_name: Option<String>,
    pub email: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserAuthInfo {
    pub access_key_id: String,
    pub secret_access_key: Option<String>,
    pub creation_date: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GroupItem {
    pub id: String,
    pub creation_date: u64,
}
