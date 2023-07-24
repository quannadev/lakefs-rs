use crate::utils::set_evnvar;
use crate::{Config, LakeFsClient, QueryData};
use log::info;

#[tokio::test]
async fn test_create_delete_branch() {
    set_evnvar();
    let cfg = Config::from_env().unwrap();
    let new_branch = "test_branch".to_string();
    let repo_name = "test".to_string();
    let main_branch = "main".to_string();
    let client = LakeFsClient::new(cfg);
    let result = client
        .repositories_api
        .create_branch(repo_name.clone(), new_branch.clone(), main_branch.clone())
        .await
        .unwrap();
    info!("{:?}", result);
    assert!(result);
    client
        .repositories_api
        .del_branch(repo_name.clone(), new_branch.clone())
        .await
        .unwrap();
    let branches = client
        .repositories_api
        .get_branches(repo_name.clone(), QueryData::default())
        .await
        .unwrap();
    assert_eq!(branches.results.len(), 1);
}
