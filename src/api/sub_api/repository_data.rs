#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepositoryMetadata {
    #[serde(rename = "additionalProp1")]
    additional_prop1: String,
    #[serde(rename = "additionalProp2")]
    additional_prop2: String,
    #[serde(rename = "additionalProp3")]
    additional_prop3: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RepositoryInfo {
    pub id: String,
    pub default_branch: String,
    pub storage_namespace: String,
    pub creation_date: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BranchInfo {
    pub id: String,
    pub commit_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RevertInfo {
    pub r#ref: String,
    pub parent_number: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiffItem {
    #[serde(rename = "type")]
    item_type: String,
    path: String,
    path_type: String,
    size_bytes: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommitBody {
    pub message: String,
    pub meta: RepositoryMetadata,
    pub date: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CommitData {
    id: String,
    parents: Vec<String>,
    committer: String,
    message: String,
    creation_date: u64,
    meta_range_id: String,
    metadata: RepositoryMetadata,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_metadata() {
        let json_str =
            r#"{"additionalProp1":"value1","additionalProp2":"value2","additionalProp3":"value3"}"#;
        let parsed_data: RepositoryMetadata = serde_json::from_str(json_str).unwrap();
        assert_eq!(parsed_data.additional_prop1, "value1");
        assert_eq!(parsed_data.additional_prop2, "value2");
        assert_eq!(parsed_data.additional_prop3, "value3");
    }
    #[test]
    fn test_repositories_struct_from_json() {
        // JSON data representing a Repositories instance
        let json_data = r#"{
            "id": "repo-123",
            "default_branch": "main",
            "storage_namespace": "user/repo",
            "creation_date": 1630562000
        }"#;

        // Deserialize the JSON data into a Repositories instance
        let repo: RepositoryInfo = serde_json::from_str(json_data).unwrap();

        // Test that the fields are correctly initialized from JSON
        assert_eq!(repo.id, "repo-123");
        assert_eq!(repo.default_branch, "main");
        assert_eq!(repo.storage_namespace, "user/repo");
        assert_eq!(repo.creation_date, 1630562000);
    }
}
