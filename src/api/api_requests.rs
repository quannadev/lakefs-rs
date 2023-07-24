#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub enum LakeApiEndpoint {
    PreSetup,
    SetupAdmin,
    Repository(Option<String>),
    Auth(String),
    Branches((String, Option<String>)),
    Config(Option<String>),
    Users(Option<String>),
    Tags((String, Option<String>)),
    UserGroup(Option<String>),
}

impl From<LakeApiEndpoint> for String {
    fn from(value: LakeApiEndpoint) -> Self {
        match value {
            LakeApiEndpoint::PreSetup => "setup_comm_prefs".to_string(),
            LakeApiEndpoint::SetupAdmin => "setup_lakefs".to_string(),
            LakeApiEndpoint::Repository(path) => path.map_or("repositories".to_string(), |p| {
                format!("repositories/{}", p)
            }),
            LakeApiEndpoint::Config(path) => {
                path.map_or("config".to_string(), |p| format!("config/{}", p))
            }
            LakeApiEndpoint::Auth(path) => format!("auth/{}", path),
            LakeApiEndpoint::Branches((repo_name, Some(branch_name))) => format!(
                "{}/branches/{}",
                String::from(LakeApiEndpoint::Repository(Some(repo_name))),
                branch_name
            ),
            LakeApiEndpoint::Branches((repo_name, None)) => format!(
                "{}/branches",
                String::from(LakeApiEndpoint::Repository(Some(repo_name)))
            ),
            LakeApiEndpoint::Users(path) => path.map_or(
                format!(
                    "{}",
                    String::from(LakeApiEndpoint::Auth("users".to_string()))
                ),
                |p| {
                    format!(
                        "{}/{}",
                        String::from(LakeApiEndpoint::Auth("users".to_string())),
                        p
                    )
                },
            ),
            LakeApiEndpoint::UserGroup(path) => path.map_or(
                format!(
                    "{}",
                    String::from(LakeApiEndpoint::Auth("groups".to_string()))
                ),
                |p| {
                    format!(
                        "{}/{}",
                        String::from(LakeApiEndpoint::Auth("groups".to_string())),
                        p
                    )
                },
            ),
            LakeApiEndpoint::Tags((repo_name, Some(path))) => format!(
                "{}/tags/{}",
                String::from(LakeApiEndpoint::Repository(Some(repo_name))),
                path
            ),
            LakeApiEndpoint::Tags((repo_name, None)) => format!(
                "{}/tags",
                String::from(LakeApiEndpoint::Repository(Some(repo_name)))
            ),
        }
    }
}

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

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct QueryData {
    pub after: Option<String>,
    pub amount: Option<u64>,
    pub prefix: Option<String>,
    pub delimiter: Option<String>,
}
