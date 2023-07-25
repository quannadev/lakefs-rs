use crate::api::client_core::ClientCore;
use crate::api::core_request::{CoreRequest, Response};
use crate::api::sub_api::repository_data::{
    BranchInfo, CommitBody, CommitData, DiffItem, RepositoryInfo, RepositoryMetadata, RevertInfo,
};
use crate::LakeApiEndpoint::{Branches, Repository, Tags};
use crate::{QueryData, ResultData};
use log::info;
use serde_json::json;

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

    pub async fn get_repositories(
        &self,
        queries: QueryData,
    ) -> Response<ResultData<Vec<RepositoryInfo>>> {
        let endpoint = self.client.get_url(Repository(None));
        self.client.get(endpoint, Some(queries)).await
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
        self.client.get(endpoint, None).await
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
        self.client.delete(url).await?;
        Ok(())
    }

    pub async fn get_metadata(&self, repo_name: String) -> Response<RepositoryMetadata> {
        let mut url = self.client.get_url(Repository(Some(repo_name)));
        url.push_str("/metadata");
        self.client.get(url, None).await
    }

    pub async fn create_branch(
        &self,
        repo_name: String,
        name: String,
        source: String,
    ) -> Response<bool> {
        let url = self.client.get_url(Branches((repo_name, None)));
        let body = json!({
            "name": name,
            "source": source
        });
        self.client.post_without_parse(url, body).await
    }

    pub async fn get_branches(
        &self,
        repo_name: String,
        queries: QueryData,
    ) -> Response<ResultData<Vec<BranchInfo>>> {
        let url = self.client.get_url(Branches((repo_name, None)));
        self.client.get(url, Some(queries)).await
    }

    pub async fn get_branch(&self, repo_name: String, name: String) -> Response<BranchInfo> {
        let url = self.client.get_url(Branches((repo_name, Some(name))));
        self.client.get(url, None).await
    }

    pub async fn del_branch(&self, repo_name: String, name: String) -> Response<()> {
        let url = self.client.get_url(Branches((repo_name, Some(name))));
        self.client.delete(url).await?;
        Ok(())
    }

    pub async fn revert_branch(
        &self,
        repo_name: String,
        name: String,
        revert: RevertInfo,
    ) -> Response<()> {
        let mut url = self.client.get_url(Branches((repo_name, Some(name))));
        url.push_str("/revert");
        let body = serde_json::to_value(revert)?;
        self.client.post(url, body).await?;
        Ok(())
    }

    pub async fn cherry_pick_branch(
        &self,
        repo_name: String,
        name: String,
        cherry: RevertInfo,
    ) -> Response<()> {
        let mut url = self.client.get_url(Branches((repo_name, Some(name))));
        url.push_str("/cherry-pick");
        let body = serde_json::to_value(cherry)?;
        self.client.post(url, body).await?;
        Ok(())
    }

    pub async fn get_diff_branch(
        &self,
        repo_name: String,
        name: String,
        queries: QueryData,
    ) -> Response<ResultData<Vec<DiffItem>>> {
        let mut url = self.client.get_url(Branches((repo_name, Some(name))));
        url.push_str("/diff");
        self.client.get(url, Some(queries)).await
    }

    pub async fn commit(
        &self,
        repo_name: String,
        name: String,
        commit_body: CommitBody,
    ) -> Response<CommitData> {
        let mut url = self.client.get_url(Branches((repo_name, Some(name))));
        url.push_str("/commits");
        let body = serde_json::to_value(commit_body)?;
        self.client.post(url, body).await
    }

    pub async fn get_commit(&self, repo_name: String, commit_id: String) -> Response<CommitData> {
        let mut url = self.client.get_url(Repository(Some(repo_name)));
        url.push_str("/commits/");
        url.push_str(&commit_id);
        self.client.get(url, None).await
    }

    pub async fn get_tags(
        &self,
        repo_name: String,
        queries: QueryData,
    ) -> Response<ResultData<BranchInfo>> {
        let url = self.client.get_url(Tags((repo_name, None)));
        self.client.get(url, Some(queries)).await
    }

    pub async fn create_tag(
        &self,
        repo_name: String,
        id: String,
        ref_id: String,
    ) -> Response<BranchInfo> {
        let url = self.client.get_url(Tags((repo_name, None)));
        let body = json!({
          "id": id,
          "ref": ref_id
        });
        self.client.post(url, body).await
    }

    pub async fn get_tag(&self, repo_name: String, id: String) -> Response<BranchInfo> {
        let url = self.client.get_url(Tags((repo_name, Some(id))));
        self.client.get(url, None).await
    }

    pub async fn delete_tag(&self, repo_name: String, id: String) -> Response<()> {
        let url = self.client.get_url(Tags((repo_name, Some(id))));
        self.client.delete(url).await?;
        Ok(())
    }
}
