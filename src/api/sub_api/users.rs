use crate::api::client_core::ClientCore;

#[derive(Clone, Debug)]
pub struct UserApi {
    client: ClientCore
}

impl UserApi {
    pub fn new(client: ClientCore) -> Self {
        Self {client}
    }
}