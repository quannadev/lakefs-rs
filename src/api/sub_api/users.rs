use crate::api::api_requests::LakeApiEndpoint::Users;
use crate::api::client_core::ClientCore;
use crate::api::core_request::{CoreRequest, Response};
use crate::{AuthInfo, QueryData, ResultData, UserAuthInfo, UserLakefs};
use serde_json::json;

#[derive(Clone, Debug)]
pub struct UserApi {
    client: ClientCore,
}

impl UserApi {
    pub fn new(client: ClientCore) -> Self {
        Self { client }
    }

    pub async fn create_user(&self, user_name: String, is_invite: bool) -> Response<UserLakefs> {
        let url = self.client.get_url(Users(None));
        let body = json!({
          "id": user_name,
          "invite_user": is_invite
        });
        self.client.post(url, body).await
    }

    pub async fn get_users(&self, queries: QueryData) -> Response<ResultData<Vec<UserLakefs>>> {
        let url = self.client.get_url(Users(None));
        self.client.get(url, Some(queries)).await
    }

    pub async fn get_user(&self, id: String) -> Response<UserLakefs> {
        let url = self.client.get_url(Users(Some(id)));
        self.client.get(url, None).await
    }

    pub async fn del_user(&self, id: String) -> Response<()> {
        let url = self.client.get_url(Users(Some(id)));
        self.client.delete(url).await?;
        Ok(())
    }

    pub async fn get_user_credentials(
        &self,
        id: String,
        queries: Option<QueryData>,
    ) -> Response<ResultData<UserAuthInfo>> {
        let mut url = self.client.get_url(Users(Some(id)));
        url.push_str("/credentials");
        self.client.get(url, queries).await
    }

    pub async fn create_user_credential(&self, id: String) -> Response<AuthInfo> {
        let mut url = self.client.get_url(Users(Some(id)));
        url.push_str("/credentials");
        self.client.post(url, serde_json::Value::Null).await
    }

    pub async fn del_user_credential(&self, id: String, access_key: String) -> Response<()> {
        let mut url = self.client.get_url(Users(Some(id)));
        url.push_str("/credentials/");
        url.push_str(&access_key);
        self.client.delete(url).await?;
        Ok(())
    }

    pub async fn get_credential(&self, id: String, access_key: String) -> Response<UserAuthInfo> {
        let mut url = self.client.get_url(Users(Some(id)));
        url.push_str("/credentials/");
        url.push_str(&access_key);
        self.client.get(url, None).await
    }
}
