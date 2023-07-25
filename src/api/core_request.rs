use crate::api::sub_api::object_data::FileHeadInfo;
use crate::errors::ClientError;
use crate::utils::get_response;
use crate::{Config, LakeApiEndpoint, QueryData};
use async_trait::async_trait;
use log::info;
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub type Response<T> = Result<T, ClientError>;

#[async_trait]
pub trait CoreRequest {
    fn setup(cfg: &Config) -> Self;
    async fn get<T>(&self, endpoint: String, queries: Option<QueryData>) -> Response<T>
    where
        T: DeserializeOwned,
    {
        let client = self.make_request(endpoint, Method::GET);
        let result = match queries {
            None => client.send().await?.json::<Value>().await?,
            Some(q) => client.query(&q).send().await?.json::<Value>().await?,
        };
        get_response(result)
    }
    async fn post<T>(&self, endpoint: String, body: Value) -> Response<T>
    where
        T: DeserializeOwned,
    {
        let client = self.make_request(endpoint, Method::POST);
        let result = client.json(&body).send().await?.json::<Value>().await?;
        get_response(result)
    }

    async fn post_without_parse(&self, endpoint: String, body: Value) -> Response<bool> {
        let client = self.make_request(endpoint, Method::POST);
        let res = client.json(&body).send().await?;
        info!("status {:?}", res.status());
        Ok(res.status().is_success())
    }

    async fn delete(&self, endpoint: String) -> Response<bool> {
        let client = self.make_request(endpoint, Method::DELETE);
        Ok(client.send().await?.status().is_success())
    }
    async fn delete_with_query(&self, endpoint: String, queries: QueryData) -> Response<bool> {
        let client = self.make_request(endpoint, Method::DELETE);
        Ok(client.query(&queries).send().await?.status().is_success())
    }
    async fn put<T>(&self, endpoint: String, body: Value) -> Response<T>
    where
        T: DeserializeOwned,
    {
        let client = self.make_request(endpoint, Method::PUT);
        let result = client.json(&body).send().await?.json::<Value>().await?;
        get_response(result)
    }

    async fn get_file_head_info(
        &self,
        endpoint: String,
        query: QueryData,
    ) -> Response<FileHeadInfo> {
        let client = self.make_request(endpoint, Method::HEAD);
        let res = client.query(&query).send().await?;
        FileHeadInfo::try_from(res.headers().clone())
    }

    async fn put_without_parse_body(&self, endpoint: String, body: Value) -> Response<bool> {
        let client = self.make_request(endpoint, Method::PUT);
        Ok(client.json(&body).send().await?.status().is_success())
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
