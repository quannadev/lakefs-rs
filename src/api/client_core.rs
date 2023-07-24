use crate::api::core_request::CoreRequest;
use crate::Config;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use reqwest::Client;

#[derive(Clone, Debug)]
pub struct ClientCore {
    client: Client,
    auth: (String, String),
    domain: String,
    version: String,
}

#[async_trait]
impl CoreRequest for ClientCore {
    fn setup(cfg: &Config) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        let client = Client::builder().default_headers(headers).build().unwrap();
        Self {
            client,
            auth: (cfg.lakefs_access_key.clone(), cfg.lakefs_secret_key.clone()),
            domain: cfg.lakefs_endpoint.clone(),
            version: cfg.lakefs_api_version.clone(),
        }
    }

    fn get_client(&self) -> &Client {
        &self.client
    }

    fn get_auth(&self) -> (String, String) {
        self.auth.clone()
    }

    fn get_domain(&self) -> String {
        self.domain.clone()
    }

    fn get_version(&self) -> String {
        self.version.clone()
    }
}
