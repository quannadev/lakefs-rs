#![allow(dead_code)]

use crate::errors::ClientError;
use crate::utils::parse_str_to_timestamp;
use crate::RepositoryMetadata;
use reqwest::header::HeaderMap;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ObjectItem {
    pub path: String,
    pub path_type: String,
    pub physical_address: String,
    pub checksum: String,
    pub size_bytes: u64,
    pub mtime: u64,
    pub metadata: Option<RepositoryMetadata>,
    pub content_type: String,
}

#[derive(Debug, Deserialize)]
pub struct FileHeadInfo {
    #[serde(rename = "content-length")]
    pub content_length: u64,
    #[serde(rename = "content-type")]
    pub content_type: String,
    pub etag: String,
    #[serde(rename = "last-modified")]
    pub last_modified: u64, //timestamp
}

impl TryFrom<HeaderMap> for FileHeadInfo {
    type Error = ClientError;

    fn try_from(value: HeaderMap) -> Result<Self, Self::Error> {
        let content_length = match value.get("content-length") {
            Some(v) => v
                .to_str()
                .map_err(|e| ClientError::Parse(e.to_string()))?
                .parse::<u64>()
                .map_err(|p| ClientError::Parse(p.to_string())),
            _ => Err(ClientError::Parse("Invalid field".to_string())),
        }?;
        let content_type = match value.get("content-type") {
            Some(v) => v.to_str().map_err(|e| ClientError::Parse(e.to_string())),
            _ => Err(ClientError::Parse("Invalid field".to_string())),
        }?
        .to_string();
        let etag = match value.get("etag") {
            Some(v) => v.to_str().map_err(|e| ClientError::Parse(e.to_string())),
            _ => Err(ClientError::Parse("Invalid field".to_string())),
        }?
        .replace('\"', "");
        let last_modified = match value.get("last-modified") {
            Some(v) => {
                let value = v.to_str().map_err(|e| ClientError::Parse(e.to_string()))?;
                Ok(parse_str_to_timestamp(value) as u64)
            }
            _ => Err(ClientError::Parse("Invalid field".to_string())),
        }?;
        Ok(Self {
            content_length,
            content_type,
            etag,
            last_modified,
        })
    }
}
