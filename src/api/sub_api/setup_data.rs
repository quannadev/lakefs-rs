#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub creation_date: u64,
}

#[derive(Debug, Deserialize)]
pub struct LakefsVersion {
    version: String,
    latest_version: String,
    upgrade_recommended: bool,
    upgrade_url: String,
}

#[derive(Debug, Deserialize)]
pub struct BlockstoreConfig {
    blockstore_type: String,
    blockstore_namespace_example: String,
    blockstore_namespace_validity_regex: String,
    default_namespace_prefix: String,
    pre_sign_support: bool,
    pre_sign_support_ui: bool,
    import_support: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        let json_data = r#"
        {
            "version": "1.0.0",
            "latest_version": "2.0.0",
            "upgrade_recommended": true,
            "upgrade_url": "https://example.com/upgrade"
        }
    "#;
        let version: LakefsVersion = serde_json::from_str(json_data).unwrap();
        assert_eq!(version.version, "1.0.0");
        assert_eq!(version.latest_version, "2.0.0");
        assert_eq!(version.upgrade_url, "https://example.com/upgrade");
        assert!(version.upgrade_recommended);
    }

    #[test]
    fn test_blockstore_config_deserialization() {
        // JSON data representing BlockstoreConfig
        let json_data = r#"{
            "blockstore_type": "some_type",
            "blockstore_namespace_example": "example_namespace",
            "blockstore_namespace_validity_regex": "validity_regex",
            "default_namespace_prefix": "default_prefix",
            "pre_sign_support": true,
            "pre_sign_support_ui": true,
            "import_support": true
        }"#;

        // Deserialize JSON data into BlockstoreConfig
        let blockstore_config: BlockstoreConfig = serde_json::from_str(json_data).unwrap();

        // Test individual fields of the deserialized config
        assert_eq!(blockstore_config.blockstore_type, "some_type");
        assert_eq!(
            blockstore_config.blockstore_namespace_example,
            "example_namespace"
        );
        assert_eq!(
            blockstore_config.blockstore_namespace_validity_regex,
            "validity_regex"
        );
        assert_eq!(blockstore_config.default_namespace_prefix, "default_prefix");
        assert!(blockstore_config.pre_sign_support);
        assert!(blockstore_config.pre_sign_support_ui);
        assert!(blockstore_config.import_support);
    }
}
