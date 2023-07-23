use crate::errors::ClientError;
use crate::utils::get_response;
use crate::LakeApiEndpoint::{PreSetup, Repository, SetupAdmin};
use crate::{AuthInfo, Config, LakeApiEndpoint, Repositories};
use log::info;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

#[derive(Clone, Debug)]
pub struct LakeFsClient {
    cfg: Config,
    client: Client,
}

impl LakeFsClient {
    pub fn new(cfg: Config) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        let client = Client::builder().default_headers(headers).build().unwrap();
        Self { cfg, client }
    }

    pub async fn new_with_setup(
        endpoint: String,
        admin_email: String,
        username: String,
    ) -> Result<(Self, AuthInfo), ClientError> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        let client = Client::builder().default_headers(headers).build().unwrap();
        let cfg = Config::new(endpoint, "".to_string(), "".to_string(), None);
        let mut lakefs = LakeFsClient { cfg, client };
        let next_step = lakefs.pre_setup(admin_email).await?;
        info!("setup: {}", next_step);
        if next_step {
            let info = lakefs.setup_user(username).await?;
            lakefs.cfg.lakefs_secret_key = info.secret_access_key.clone();
            lakefs.cfg.lakefs_access_key = info.access_key_id.clone();
            return Ok((lakefs, info));
        }
        Err(ClientError::Init("setup admin error".to_string()))
    }

    async fn make_get_request(&self, api: LakeApiEndpoint) -> Result<Value, ClientError> {
        let result = self
            .client
            .get(self.cfg.get_api_endpoint(api))
            .basic_auth(
                &self.cfg.lakefs_access_key,
                Some(&self.cfg.lakefs_secret_key),
            )
            .send()
            .await?
            .json::<Value>()
            .await?;
        Ok(result)
    }

    async fn make_post_request<T>(
        &self,
        api: LakeApiEndpoint,
        body: Option<Value>,
    ) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        let result = self
            .client
            .post(self.cfg.get_api_endpoint(api))
            .basic_auth(
                &self.cfg.lakefs_access_key,
                Some(&self.cfg.lakefs_secret_key),
            )
            .json(&body)
            .send()
            .await?
            .json::<Value>()
            .await?;
        get_response(result)
    }

    pub async fn pre_setup(&self, email: String) -> Result<bool, ClientError> {
        let check_status = self.check_setup().await?;
        if check_status == "comm_prefs_done" {
            return Ok(true);
        }
        let body = json!({ "email": email, "featureUpdates": false, "securityUpdates": false });
        let setup = self
            .make_post_request::<Value>(PreSetup, Some(body))
            .await?;
        info!("pre setup {}", setup);
        Ok(setup.get("nextStep").is_some())
    }

    async fn check_setup(&self) -> Result<String, ClientError> {
        let check = self.make_get_request(SetupAdmin).await?;
        let status = check.get("state").unwrap().as_str().unwrap().to_string();
        info!("check status: {}", status);
        Ok(status)
    }

    pub async fn setup_user(&self, username: String) -> Result<AuthInfo, ClientError> {
        let check_status = self.check_setup().await?;
        if check_status == "initialized" {
            return Err(ClientError::Init("Lakefs initialized".to_string()));
        }
        let body = json!({ "username": username });
        self.make_post_request::<AuthInfo>(SetupAdmin, Some(body))
            .await
    }

    pub async fn create_repository(
        &self,
        name: String,
        s3_url: String,
        branch_name: String,
    ) -> Result<Repositories, ClientError> {
        let body = json!({
            "name": name,
            "storage_namespace": s3_url,
            "default_branch": branch_name,
            "sample_data": false
        });
        info!("{:?}", body);
        self.make_post_request(Repository, Some(body)).await
    }

    pub async fn get_repositories(&self) -> Result<Vec<Repositories>, ClientError> {
        let result = self.make_get_request(Repository).await?;
        let arr = result.get("results").unwrap();
        get_response::<Vec<Repositories>>(arr.clone())
    }
}
