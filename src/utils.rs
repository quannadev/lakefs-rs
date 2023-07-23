use crate::errors::ClientError;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::env;

pub fn set_evnvar() {
    env::set_var("RUST_LOG", "INFO");
    env::set_var("LAKEFS_ENDPOINT", "http://localhost:8000");
    env::set_var("LAKEFS_ACCESS_KEY", "AKIAJWOBNKM7IZS6DVTQ");
    env::set_var(
        "LAKEFS_SECRET_KEY",
        "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",
    );
    env::set_var("LAKEFS_API_VERSION", "v1");
    env_logger::try_init().unwrap_or_default();
}

pub fn get_response<T>(value: Value) -> Result<T, ClientError>
where
    T: DeserializeOwned,
{
    match serde_json::from_value(value.clone()) {
        Ok(res) => Ok(res),
        Err(e) => {
            let message = value
                .get("message")
                .map_or(e.to_string(), |m| m.to_string());
            Err(ClientError::RequestFail(message))
        }
    }
}
