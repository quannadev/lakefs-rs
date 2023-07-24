use crate::api::client_core::ClientCore;
use crate::api::core_request::{CoreRequest, Response};
use crate::api::sub_api::repository_data::{
    BranchInfo, RepositoryInfo, RepositoryMetadata, RevertInfo,
};
use crate::LakeApiEndpoint::{Branches, Repository};
use crate::ResultData;
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

    /// # Create repository
    ///
    /// # Arguments
    ///
    /// * `name`: example: `test`
    /// * `s3_url`: example: `s3://test`
    /// * `branch_name`: example: `main`
    ///
    /// returns: Result<RepositoryInfo, ClientError>
    ///
    pub async fn create_repository(
        &self,
        name: String,
        s3_url: String,
        branch_name: String,
    ) -> Response<RepositoryInfo> {
        let body = json!({
            "name": name,
            "storage_namespace": s3_url,
            "default_branch": branch_name,
            "sample_data": false
        });
        info!("{:?}", body);
        self.client
            .post(self.client.get_url(Repository(None)), body)
            .await
    }

    pub async fn get_repositories(&self) -> Response<ResultData<Vec<RepositoryInfo>>> {
        let endpoint = self.client.get_url(Repository(None));
        self.client
            .get::<ResultData<Vec<RepositoryInfo>>>(endpoint, vec![])
            .await
    }

    /// # Get Repository Info
    ///
    /// # Arguments
    ///
    /// * `repo_name`: example: `test`
    ///
    /// returns: Result<RepositoryInfo, ClientError>
    ///
    pub async fn get_repository(&self, repo_name: String) -> Response<RepositoryInfo> {
        let endpoint = self.client.get_url(Repository(Some(repo_name)));
        self.client.get::<RepositoryInfo>(endpoint, vec![]).await
    }

    /// # Delete Repository
    ///
    /// # Arguments
    ///
    /// * `repo_name`: example: `test`
    ///
    /// returns: Result<(), ClientError>
    pub async fn del_repository(&self, repo_name: String) -> Response<()> {
        let url = self.client.get_url(Repository(Some(repo_name)));
        let _ = self.client.delete::<Value>(url).await?;
        Ok(())
    }

    pub async fn get_metadata(&self, repo_name: String) -> Response<RepositoryMetadata> {
        let url = format!(
            "{}/metadata",
            self.client.get_url(Repository(Some(repo_name)))
        );
        self.client.get::<RepositoryMetadata>(url, vec![]).await
    }

    pub async fn create_branch(
        &self,
        repo_name: String,
        name: String,
        source: String,
    ) -> Response<()> {
        let url = String::from(Branches((repo_name, None)));
        let body = json!({
            "name": name,
            "source": source
        });
        self.client.post(url, body).await?;
        Ok(())
    }

    pub async fn get_branches(
        &self,
        repo_name: String,
        queries: Vec<String>,
    ) -> Response<ResultData<Vec<BranchInfo>>> {
        let url = String::from(Branches((repo_name, None)));
        self.client
            .get::<ResultData<Vec<BranchInfo>>>(url, queries)
            .await
    }

    pub async fn get_branch(&self, repo_name: String, name: String) -> Response<BranchInfo> {
        let url = String::from(Branches((repo_name, Some(name))));
        self.client.get::<BranchInfo>(url, vec![]).await
    }

    pub async fn del_branch(&self, repo_name: String, name: String) -> Response<()> {
        let url = String::from(Branches((repo_name, Some(name))));
        self.client.delete(url).await?;
        Ok(())
    }

    pub async fn revert_branch(
        &self,
        repo_name: String,
        name: String,
        revert: RevertInfo,
    ) -> Response<()> {
        let mut url = String::from(Branches((repo_name, Some(name))));
        url.push_str("/revert");
        let body = serde_json::to_value(revert)?;
        self.client.post(url, body).await?;
        Ok(())
    }

    pub async fn cherry_pick_branch(
        &self,
        repo_name: String,
        name: String,
        revert: RevertInfo,
    ) -> Response<()> {
        let mut url = String::from(Branches((repo_name, Some(name))));
        url.push_str("/cherry-pick");
        let body = serde_json::to_value(revert)?;
        self.client.post(url, body).await?;
        Ok(())
    }
}
