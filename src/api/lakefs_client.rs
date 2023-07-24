use crate::api::core_request::CoreRequest;
use crate::api::repositories::RepositoriesApi;
use crate::api::setup::SetupApi;
use crate::errors::ClientError;
use crate::{AuthInfo, Config};
use log::info;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use reqwest::Client;

#[derive(Clone, Debug)]
pub struct LakeFsClient {
    cfg: Config,
    pub setup_api: SetupApi,
    pub repositories: RepositoriesApi,
}

impl LakeFsClient {
    pub fn new(cfg: Config) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        let client = Client::builder().default_headers(headers).build().unwrap();
        let setup_api = SetupApi::setup(&cfg, client.clone());
        let repositories = RepositoriesApi::setup(&cfg, client.clone());
        Self {
            cfg,
            setup_api,
            repositories,
        }
    }

    pub async fn new_with_setup(
        endpoint: String,
        admin_email: String,
        username: String,
    ) -> Result<(Self, AuthInfo), ClientError> {
        let cfg = Config::new(endpoint, "".to_string(), "".to_string(), None);
        let mut lakefs = LakeFsClient::new(cfg);
        let next_step = lakefs.setup_api.pre_setup(admin_email).await?;
        info!("setup: {}", next_step);
        if next_step {
            let info = lakefs.setup_api.setup_user(username).await?;
            lakefs.cfg.lakefs_secret_key = info.secret_access_key.clone();
            lakefs.cfg.lakefs_access_key = info.access_key_id.clone();
            return Ok((lakefs, info));
        }
        Err(ClientError::Init("setup admin error".to_string()))
    }
}
