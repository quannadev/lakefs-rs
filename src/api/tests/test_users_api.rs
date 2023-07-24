use crate::utils::set_evnvar;
use crate::{Config, LakeFsClient, QueryData};
use log::info;

#[tokio::test]
async fn test_get_list_users() {
    set_evnvar();
    let cfg = Config::from_env().unwrap();
    let client = LakeFsClient::new(cfg);
    let queries = QueryData::default();
    let user = client.user_api.get_users(queries).await.unwrap();
    assert_eq!(user.results.len(), 1)
}

#[tokio::test]
async fn test_get_user() {
    set_evnvar();
    let cfg = Config::from_env().unwrap();
    let client = LakeFsClient::new(cfg);
    let user = client.user_api.get_user("admin".to_string()).await.unwrap();
    info!("get user admin {:?}", user);
    assert_eq!(user.id, "admin")
}

#[tokio::test]
async fn test_create_user() {
    set_evnvar();
    let cfg = Config::from_env().unwrap();
    let client = LakeFsClient::new(cfg);
    let user = client
        .user_api
        .create_user("test".to_string(), false)
        .await
        .unwrap();
    info!("create user test {:?}", user);
    assert_eq!(user.id, "test")
}

#[tokio::test]
async fn test_credential_user() {
    set_evnvar();
    let cfg = Config::from_env().unwrap();
    let user = "test".to_string();
    let client = LakeFsClient::new(cfg);
    let info = client
        .user_api
        .create_user_credential(user.clone())
        .await
        .unwrap();
    info!("create user test {:?}", info);
    let cert = client
        .user_api
        .get_credential(user, info.access_key_id.clone())
        .await
        .unwrap();
    assert_eq!(cert.access_key_id, info.access_key_id)
}

#[tokio::test]
async fn test_add_user_to_groups() {
    set_evnvar();
    let cfg = Config::from_env().unwrap();
    let user = "test".to_string();
    let group_admin = "Admins".to_string();
    let client = LakeFsClient::new(cfg);
    let result = client
        .user_group_api
        .add_member_to_group(user.clone(), group_admin.clone())
        .await
        .unwrap();
    info!("result: {:?}", result);
    assert!(result);
    let group = client
        .user_group_api
        .group_members(group_admin, QueryData::default())
        .await
        .unwrap();
    info!("groups items: {:?}", group);

    assert_eq!(group.results.len(), 2)
}

#[tokio::test]
async fn test_remove_user_to_groups() {
    set_evnvar();
    let cfg = Config::from_env().unwrap();
    let user = "test".to_string();
    let group_admin = "Admins".to_string();
    let client = LakeFsClient::new(cfg);
    let result = client
        .user_group_api
        .remove_member_to_group(user.clone(), group_admin.clone())
        .await;
    info!("result: {:?}", result);
    assert!(result.is_ok());
    let group = client
        .user_group_api
        .group_members(group_admin, QueryData::default())
        .await
        .unwrap();
    info!("groups items: {:?}", group);

    assert_eq!(group.results.len(), 1)
}

#[tokio::test]
async fn test_create_delete_groups() {
    set_evnvar();
    let cfg = Config::from_env().unwrap();
    let new_group = "test_group".to_string();
    let client = LakeFsClient::new(cfg);
    let item = client
        .user_group_api
        .create_group(new_group.to_string())
        .await
        .unwrap();
    info!("created item: {:?}", item);
    assert_eq!(item.id, new_group);
    let groups = client
        .user_group_api
        .get_groups(QueryData::default())
        .await
        .unwrap();
    info!("groups item: {:?}", groups);
    assert_eq!(groups.results.len(), 5);
    let del = client.user_group_api.delete_group(new_group).await;
    info!("{:?}", del);
    assert!(del.is_ok());
    let groups = client
        .user_group_api
        .get_groups(QueryData::default())
        .await
        .unwrap();
    info!("groups item: {:?}", groups);
    assert_eq!(groups.results.len(), 4);
}
