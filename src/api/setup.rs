use crate::api::client_core::ClientCore;
use crate::api::core_request::CoreRequest;
use crate::errors::ClientError;
use crate::AuthInfo;
use crate::LakeApiEndpoint::{PreSetup, SetupAdmin};
use log::info;
use serde_json::{json, Value};

#[derive(Clone, Debug)]
pub struct SetupApi {
    client: ClientCore,
}

impl SetupApi {
    pub fn new(client: ClientCore) -> Self {
        Self { client }
    }

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
        let endpoint = self.client.get_url(PreSetup);
        let setup = self.client.post::<Value>(endpoint, body).await?;
        info!("pre setup {}", setup);
        Ok(setup.get("nextStep").is_some())
    }

    async fn check_setup(&self) -> Result<String, ClientError> {
        let endpoint = self.client.get_url(SetupAdmin);
        let check = self.client.get::<Value>(endpoint, vec![]).await?;
        let status = check.get("state").unwrap().as_str().unwrap().to_string();
        info!("check status: {}", status);
        Ok(status)
    }

    pub async fn setup_user(&self, username: String) -> Result<AuthInfo, ClientError> {
        let check_status = self.check_setup().await?;
        if check_status == "initialized" {
            return Err(ClientError::Init("Lakefs initialized".to_string()));
        }
        let endpoint = self.client.get_url(SetupAdmin);
        let body = json!({ "username": username });
        self.client.post::<AuthInfo>(endpoint, body).await
    }
}
