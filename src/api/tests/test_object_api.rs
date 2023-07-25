use crate::utils::set_evnvar;
use crate::{Config, LakeFsClient, QueryData};
use log::info;

#[tokio::test]
async fn test_get_stat_obj() {
    set_evnvar();
    let cfg = Config::from_env().unwrap();
    let repo_name = "test".to_string();
    let main_branch = "main".to_string();
    let client = LakeFsClient::new(cfg);
    let mut query = QueryData::default();
    query.file_name = "example.parquet".to_string();
    let file = client
        .object_api
        .get_stat(repo_name, main_branch, query)
        .await
        .unwrap();
    info!("file: {:?}", file);
    // assert_eq!(file.path, "example.parquet")
}

#[tokio::test]
async fn test_get_list_objs() {
    set_evnvar();
    let cfg = Config::from_env().unwrap();
    let repo_name = "test".to_string();
    let main_branch = "main".to_string();
    let client = LakeFsClient::new(cfg);
    let query = QueryData::default();
    let list = client
        .object_api
        .ls_objects(repo_name, main_branch, query)
        .await
        .unwrap();
    assert_eq!(list.results.len(), 1);
}

#[tokio::test]
async fn test_get_file_info() {
    set_evnvar();
    let cfg = Config::from_env().unwrap();
    let repo_name = "test".to_string();
    let main_branch = "main".to_string();
    let client = LakeFsClient::new(cfg);
    let mut query = QueryData::default();
    query.file_name = "example.parquet".to_string();
    let file = client
        .object_api
        .check_file_obj(repo_name, main_branch, query)
        .await
        .unwrap();
    info!("file: {:?}", file);
    // assert_eq!(file.path, "example.parquet")
}

#[tokio::test]
async fn test_del_file() {
    set_evnvar();
    let cfg = Config::from_env().unwrap();
    let repo_name = "test".to_string();
    let main_branch = "main".to_string();
    let client = LakeFsClient::new(cfg);
    let mut query = QueryData::default();
    query.file_name = "example.parquet".to_string();
    let status = client
        .object_api
        .delete_file_obj(repo_name, main_branch, query)
        .await
        .unwrap();
    assert!(status);
}
