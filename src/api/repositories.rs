use crate::api::core_request::CoreRequest;
use crate::errors::ClientError;
use crate::utils::get_response;
use crate::LakeApiEndpoint::Repository;
use crate::{Config, LakeApiEndpoint, Repositories};
use async_trait::async_trait;
use log::info;
use reqwest::{Client, Method, RequestBuilder};
use serde_json::{json, Value};

#[derive(Clone, Debug)]
pub struct RepositoriesApi {
    client: Client,
    auth: (String, String),
    domain: String,
    version: String,
}

impl RepositoriesApi {
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
        self.post(self.get_url(Repository), body).await
    }

    pub async fn get_repositories(
        &self,
        name: Option<String>,
    ) -> Result<Vec<Repositories>, ClientError> {
        let endpoint = match name {
            Some(id) => format!("{}/{}", self.get_url(Repository), id),
            _ => self.get_url(Repository),
        };
        let result = self.get::<Value>(endpoint, vec![]).await?;
        if result.is_array() {
            match result.get("results") {
                None => {
                    let mess = result.get("message").unwrap().to_string();
                    Err(ClientError::RequestFail(mess))
                }
                Some(arr) => get_response::<Vec<Repositories>>(arr.clone()),
            }
        } else {
            let rs = get_response::<Repositories>(result)?;
            Ok(vec![rs])
        }
    }
}

#[async_trait]
impl CoreRequest for RepositoriesApi {
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
