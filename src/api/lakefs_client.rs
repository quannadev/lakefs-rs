use crate::api::client_core::ClientCore;
use crate::api::core_request::{CoreRequest, Response};
use crate::api::sub_api::group_api::GroupApi;
use crate::api::sub_api::object_api::ObjectApi;
use crate::api::sub_api::repositories::RepositoriesApi;
use crate::api::sub_api::setup::SetupApi;
use crate::api::sub_api::users::UserApi;
use crate::api::sub_api::AuthInfo;
use crate::Config;

#[derive(Clone, Debug)]
pub struct LakeFsClient {
    pub setup_api: SetupApi,
    pub repositories_api: RepositoriesApi,
    pub user_api: UserApi,
    pub user_group_api: GroupApi,
    pub object_api: ObjectApi,
}

impl LakeFsClient {
    pub fn new(cfg: Config) -> Self {
        let client = ClientCore::setup(&cfg);
        Self {
            setup_api: SetupApi::new(client.clone()),
            repositories_api: RepositoriesApi::new(client.clone()),
            user_api: UserApi::new(client.clone()),
            user_group_api: GroupApi::new(client.clone()),
            object_api: ObjectApi::new(client),
        }
    }

    pub async fn new_with_setup(
        endpoint: String,
        admin_email: String,
        username: String,
    ) -> Response<(Self, AuthInfo)> {
        let mut cfg = Config::new(endpoint, "".to_string(), "".to_string(), None);
        let client = ClientCore::setup(&cfg);
        let setup_api = SetupApi::new(client.clone());
        let info = setup_api.setup_admin(admin_email, username).await?;
        cfg.lakefs_secret_key = info.secret_access_key.clone();
        cfg.lakefs_access_key = info.access_key_id.clone();
        let lakefs = LakeFsClient::new(cfg);
        Ok((lakefs, info))
    }
}
