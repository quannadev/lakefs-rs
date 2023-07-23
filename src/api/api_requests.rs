use serde::{Deserialize, Serialize};
pub enum LakeApiEndpoint {
    PreSetup,
    SetupAdmin,
    Repository,
}

impl LakeApiEndpoint {
    pub fn to_endpoint(&self, domain: String, version: String) -> String {
        let url = match self {
            LakeApiEndpoint::PreSetup => "setup_comm_prefs".to_string(),
            LakeApiEndpoint::SetupAdmin => "setup_lakefs".to_string(),
            LakeApiEndpoint::Repository => "repositories".to_string(),
        };
        format!("{domain}/api/{version}/{url}")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub creation_date: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Repositories {
    pub id: String,
    pub default_branch: String,
    pub storage_namespace: String,
    pub creation_date: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseType {
    Ok(),
    Error(),
}
