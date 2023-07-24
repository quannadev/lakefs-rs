use crate::errors::ClientError;
use crate::utils::get_response;
use crate::{Config, LakeApiEndpoint};
use async_trait::async_trait;
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde_json::Value;

#[async_trait]
pub trait CoreRequest {
    fn setup(cfg: &Config, client: Client) -> Self;
    async fn get<T>(&self, endpoint: String, queries: Vec<String>) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        let client = self.make_request(endpoint, Method::GET);
        let result = client.query(&queries).send().await?.json::<Value>().await?;
        get_response(result)
    }
    async fn post<T>(&self, endpoint: String, body: Value) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        let client = self.make_request(endpoint, Method::POST);
        let result = client.json(&body).send().await?.json::<Value>().await?;
        get_response(result)
    }
    async fn delete<T>(&self, endpoint: String) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        let client = self.make_request(endpoint, Method::DELETE);
        let result = client.send().await?.json::<Value>().await?;
        get_response(result)
    }
    async fn update<T>(&self, endpoint: String, body: Value) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        let client = self.make_request(endpoint, Method::PUT);
        let result = client.json(&body).send().await?.json::<Value>().await?;
        get_response(result)
    }
    fn get_client(&self) -> &Client;
    fn make_request(&self, endpoint: String, method: Method) -> RequestBuilder {
        let auth = self.get_auth();
        self.get_client()
            .request(method, endpoint)
            .basic_auth(auth.0, Some(auth.1))
    }
    fn get_url(&self, api: LakeApiEndpoint) -> String {
        let url = String::from(api);
        let domain = self.get_domain();
        let version = self.get_version();
        format!("{domain}/api/{version}/{url}")
    }
    fn get_auth(&self) -> (String, String);
    fn get_domain(&self) -> String;
    fn get_version(&self) -> String;
}
