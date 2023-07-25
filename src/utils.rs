#![allow(dead_code)]
use crate::errors::ClientError;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::env;

pub fn set_evnvar() {
    env::set_var("RUST_LOG", "INFO");
    env::set_var("LAKEFS_ENDPOINT", "http://localhost:8000");
    env::set_var("LAKEFS_ACCESS_KEY", "AKIAJUVSF66ADRQ5X5TQ");
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

pub fn parse_str_to_timestamp(date_time_str: &str) -> i64 {
    let format_str = "%a, %d %b %Y %H:%M:%S %Z";
    let naive_date_time = NaiveDateTime::parse_from_str(date_time_str, format_str)
        .expect("Failed to parse date and time");

    // Convert the NaiveDateTime into a DateTime with UTC timezone
    let date_time_utc: DateTime<Utc> = Utc.from_utc_datetime(&naive_date_time);

    // Get the timestamp (UNIX timestamp)
    date_time_utc.timestamp()
}
