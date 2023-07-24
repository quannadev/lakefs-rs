use crate::api::core_request::CoreRequest;
use crate::errors::ClientError;
use crate::LakeApiEndpoint::{PreSetup, SetupAdmin};
use crate::{AuthInfo, Config, LakeApiEndpoint};
use async_trait::async_trait;
use log::info;
use reqwest::Client;
use serde_json::{json, Value};

#[derive(Clone, Debug)]
pub struct SetupApi {
    client: Client,
    auth: (String, String),
    domain: String,
    version: String,
}

impl SetupApi {
    pub async fn setup_admin(
        &self,
        email: String,
        username: String,
    ) -> Result<AuthInfo, ClientError> {
        let next_step = self.pre_setup(email).await?;
        info!("setup: {}", next_step);
        if next_step {
            let info = self.setup_user(username).await?;
            return Ok(info);
        }
        Err(ClientError::Init("Setup initialized".to_string()))
    }

    pub async fn pre_setup(&self, email: String) -> Result<bool, ClientError> {
        let check_status = self.check_setup().await?;
        if check_status == "comm_prefs_done" {
            return Ok(true);
        }
        let body = json!({ "email": email, "featureUpdates": false, "securityUpdates": false });
        let endpoint = self.get_url(PreSetup);
        let setup = self.post::<Value>(endpoint, body).await?;
        info!("pre setup {}", setup);
        Ok(setup.get("nextStep").is_some())
    }

    async fn check_setup(&self) -> Result<String, ClientError> {
        let endpoint = self.get_url(SetupAdmin);
        let check = self.get::<Value>(endpoint, vec![]).await?;
        let status = check.get("state").unwrap().as_str().unwrap().to_string();
        info!("check status: {}", status);
        Ok(status)
    }

    pub async fn setup_user(&self, username: String) -> Result<AuthInfo, ClientError> {
        let check_status = self.check_setup().await?;
        if check_status == "initialized" {
            return Err(ClientError::Init("Lakefs initialized".to_string()));
        }
        let endpoint = self.get_url(SetupAdmin);
        let body = json!({ "username": username });
        self.post::<AuthInfo>(endpoint, body).await
    }
}

#[async_trait]
impl CoreRequest for SetupApi {
    fn setup(cfg: &Config, client: Client) -> Self {
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

    fn get_url(&self, api: LakeApiEndpoint) -> String {
        api.to_endpoint(self.domain.clone(), self.version.clone())
    }

    fn get_auth(&self) -> (String, String) {
        self.auth.clone()
    }
}
