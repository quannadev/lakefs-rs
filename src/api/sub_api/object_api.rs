use crate::api::client_core::ClientCore;
use crate::api::core_request::{CoreRequest, Response};
use crate::api::sub_api::object_data::{FileHeadInfo, ObjectItem};
use crate::errors::ClientError;
use crate::LakeApiEndpoint::{Branches, RefsObj};
use crate::{QueryData, ResultData};

#[derive(Clone, Debug)]
pub struct ObjectApi {
    client: ClientCore,
}

impl ObjectApi {
    pub fn new(client: ClientCore) -> Self {
        Self { client }
    }

    pub async fn ls_objects(
        &self,
        repo_name: String,
        branch_name: String,
        queries: QueryData,
    ) -> Response<ResultData<Vec<ObjectItem>>> {
        let url = self
            .client
            .get_url(RefsObj(repo_name, branch_name, Some("ls".to_string())));
        self.client.get(url, Some(queries)).await
    }

    pub async fn get_stat(
        &self,
        repo_name: String,
        branch_name: String,
        queries: QueryData,
    ) -> Response<ObjectItem> {
        if queries.file_name.is_empty() {
            return Err(ClientError::RequestFail(
                "path in queries required".to_string(),
            ));
        }
        let url = self
            .client
            .get_url(RefsObj(repo_name, branch_name, Some("stat".to_string())));

        self.client.get(url, Some(queries)).await
    }

    pub async fn get_file_obj(
        &self,
        repo_name: String,
        branch_name: String,
        queries: QueryData,
    ) -> Response<ObjectItem> {
        if queries.file_name.is_empty() {
            return Err(ClientError::RequestFail(
                "path in queries required".to_string(),
            ));
        }
        let url = self.client.get_url(RefsObj(repo_name, branch_name, None));

        self.client.get(url, Some(queries)).await
    }

    pub async fn check_file_obj(
        &self,
        repo_name: String,
        branch_name: String,
        queries: QueryData,
    ) -> Response<FileHeadInfo> {
        if queries.file_name.is_empty() {
            return Err(ClientError::RequestFail(
                "path in queries required".to_string(),
            ));
        }
        let url = self.client.get_url(RefsObj(repo_name, branch_name, None));
        self.client.get_file_head_info(url, queries).await
    }

    pub async fn delete_file_obj(
        &self,
        repo_name: String,
        branch_name: String,
        queries: QueryData,
    ) -> Response<bool> {
        if queries.file_name.is_empty() {
            return Err(ClientError::RequestFail(
                "path in queries required".to_string(),
            ));
        }
        let mut url = self
            .client
            .get_url(Branches((repo_name, Some(branch_name))));
        url.push_str("/objects");

        self.client.delete_with_query(url, queries).await
    }
}
