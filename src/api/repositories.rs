use crate::api::client_core::ClientCore;
use crate::api::core_request::CoreRequest;
use crate::errors::ClientError;
use crate::utils::get_response;
use crate::LakeApiEndpoint::Repository;
use crate::Repositories;
use log::info;
use serde_json::{json, Value};

#[derive(Clone, Debug)]
pub struct RepositoriesApi {
    client: ClientCore,
}

impl RepositoriesApi {
    pub fn new(client: ClientCore) -> Self {
        Self { client }
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
        self.client
            .post(self.client.get_url(Repository), body)
            .await
    }

    pub async fn get_repositories(
        &self,
        name: Option<String>,
    ) -> Result<Vec<Repositories>, ClientError> {
        let endpoint = match name {
            Some(id) => format!("{}/{}", self.client.get_url(Repository), id),
            _ => self.client.get_url(Repository),
        };
        let result = self.client.get::<Value>(endpoint, vec![]).await?;
        if result.is_array() {
            result.get("results").map_or(
                Err(ClientError::RequestFail(
                    "get repositories failed".to_string(),
                )),
                |arr| get_response::<Vec<Repositories>>(arr.clone()),
            )
        } else {
            let rs = get_response::<Repositories>(result)?;
            Ok(vec![rs])
        }
    }
}
