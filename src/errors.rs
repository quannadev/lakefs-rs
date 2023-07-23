use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("setup lakefs error: {0}")]
    Init(String),
    #[error("request error")]
    Request(#[from] reqwest::Error),
    #[error("validation error: {0}")]
    Validation(String),
    #[error("serialize error")]
    Serialization(serde_json::Error),
    #[error("request error: {0}")]
    RequestFail(String),
}
