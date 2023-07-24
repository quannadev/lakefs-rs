use crate::api::client_core::ClientCore;
use crate::api::core_request::CoreRequest;
use crate::api::repositories::RepositoriesApi;
use crate::api::setup::SetupApi;
use crate::errors::ClientError;
use crate::{AuthInfo, Config};

#[derive(Clone, Debug)]
pub struct LakeFsClient {
    pub setup_api: SetupApi,
    pub repositories: RepositoriesApi,
}

impl LakeFsClient {
    pub fn new(cfg: Config) -> Self {
        let client = ClientCore::setup(&cfg);
        let setup_api = SetupApi::new(client.clone());
        let repositories = RepositoriesApi::new(client.clone());
        Self {
            setup_api,
            repositories,
        }
    }

    pub async fn new_with_setup(
        endpoint: String,
        admin_email: String,
        username: String,
    ) -> Result<(Self, AuthInfo), ClientError> {
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
