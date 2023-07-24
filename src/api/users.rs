use reqwest::Client;

pub struct UserApi {
    client: Client,
    auth: (String, String),
    domain: String,
    version: String,
}