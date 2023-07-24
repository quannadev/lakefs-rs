use crate::errors::ClientError;
use serde::Deserialize;
use std::env;
use validator::Validate;

#[derive(Deserialize, Validate, Clone, Debug)]
pub struct Config {
    #[validate(url(message = "invalid endpoint"))]
    pub lakefs_endpoint: String,
    pub lakefs_access_key: String,
    pub lakefs_secret_key: String,
    pub lakefs_api_version: String,
}

impl Config {
    pub fn new(
        endpoint: String,
        access_key: String,
        secret_key: String,
        api_version: Option<String>,
    ) -> Self {
        Self {
            lakefs_endpoint: endpoint,
            lakefs_access_key: access_key,
            lakefs_secret_key: secret_key,
            lakefs_api_version: api_version.map_or("v1".to_string(), |v| v),
        }
    }
    pub fn from_env() -> Result<Self, ClientError> {
        if env::var("LAKEFS_API_VERSION").is_err() {
            env::set_var("LAKEFS_API_VERSION", "v1");
        }
        let cfg = envy::from_env::<Config>().map_err(|e| ClientError::Init(e.to_string()))?;
        match cfg.validate() {
            Ok(_) => Ok(cfg),
            Err(e) => Err(ClientError::Validation(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_config_from_envar() {
        env::set_var("LAKEFS_ENDPOINT", "localhost:8000");
        env::set_var("LAKEFS_ACCESS_KEY", "AKIAJZ6FICUOBFLVMQJQ");
        env::set_var(
            "LAKEFS_SECRET_KEY",
            "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",
        );
        env::set_var("LAKEFS_API_VERSION", "v1");
        let cfg = Config::from_env().expect("get config from env error");

        assert_eq!(cfg.lakefs_endpoint, "localhost:8000");
        assert_eq!(cfg.lakefs_access_key, "AKIAJZ6FICUOBFLVMQJQ");
        assert_eq!(
            cfg.lakefs_secret_key,
            "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"
        );
        assert_eq!(cfg.lakefs_api_version, "v1")
    }
}
