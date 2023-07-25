use crate::api::client_core::ClientCore;
use crate::api::core_request::{CoreRequest, Response};
use crate::api::LakeApiEndpoint::UserGroup;
use crate::{GroupItem, QueryData, ResultData, UserLakefs};
use serde_json::{json, Value};

#[derive(Clone, Debug)]
pub struct GroupApi {
    client: ClientCore,
}

impl GroupApi {
    pub fn new(client: ClientCore) -> Self {
        Self { client }
    }

    pub async fn get_groups(&self, queries: QueryData) -> Response<ResultData<Vec<GroupItem>>> {
        let url = self.client.get_url(UserGroup(None));
        self.client.get(url, Some(queries)).await
    }

    pub async fn get_group(&self, id: String) -> Response<GroupItem> {
        let url = self.client.get_url(UserGroup(Some(id)));
        self.client.get(url, None).await
    }

    pub async fn create_group(&self, name: String) -> Response<GroupItem> {
        let url = self.client.get_url(UserGroup(None));
        let body = json!({ "id": name });
        self.client.post(url, body).await
    }

    pub async fn delete_group(&self, id: String) -> Response<()> {
        let url = self.client.get_url(UserGroup(Some(id)));
        self.client.delete(url).await?;
        Ok(())
    }

    pub async fn group_members(
        &self,
        id: String,
        queries: QueryData,
    ) -> Response<ResultData<Vec<UserLakefs>>> {
        let mut url = self.client.get_url(UserGroup(Some(id)));
        url.push_str("/members");
        self.client.get(url, Some(queries)).await
    }

    pub async fn add_member_to_group(&self, user_id: String, group_id: String) -> Response<bool> {
        let mut url = self.client.get_url(UserGroup(Some(group_id)));
        url.push_str("/members/");
        url.push_str(&user_id);
        self.client.put_without_parse_body(url, Value::Null).await
    }

    pub async fn remove_member_to_group(&self, user_id: String, group_id: String) -> Response<()> {
        let mut url = self.client.get_url(UserGroup(Some(group_id)));
        url.push_str("/members/");
        url.push_str(&user_id);
        self.client.delete(url).await?;
        Ok(())
    }
}
