use crate::utils::set_evnvar;
use crate::{Config, LakeFsClient, QueryData};

#[tokio::test]
async fn test_setup_admin() {
    set_evnvar();
    let client = LakeFsClient::new_with_setup(
        "http://localhost:8000".to_string(),
        "admin@test.com".to_string(),
        "admin".to_string(),
    )
    .await;
    log::info!("{:?}", client);
    assert!(client.is_ok());
    let (_, info) = client.unwrap();
    log::info!("{:?}", info)
}

#[tokio::test]
async fn test_create_repository() {
    set_evnvar();
    let cfg = Config::from_env().unwrap();
    let client = LakeFsClient::new(cfg);
    let result = client
        .repositories_api
        .create_repository(
            "test".to_string(),
            "s3://test".to_string(),
            "main".to_string(),
        )
        .await;
    log::info!("{:?}", result);
    assert!(result.is_ok());
    let repository = result.unwrap();
    assert_eq!(repository.storage_namespace, "s3://test");
    assert_eq!(repository.default_branch, "main");
}

#[tokio::test]
async fn test_get_repository() {
    set_evnvar();
    let cfg = Config::from_env().unwrap();
    let client = LakeFsClient::new(cfg);
    let result = client
        .repositories_api
        .get_repositories(QueryData::default())
        .await;
    log::info!("{:?}", result);
    assert!(result.is_ok());
    let repository = result.unwrap();
    assert_eq!(repository.results.len(), 1);
    let first = client
        .repositories_api
        .get_repository("test".to_string())
        .await
        .unwrap();
    assert_eq!(first.id, "test");
    assert_eq!(first.storage_namespace, "s3://test");
}
