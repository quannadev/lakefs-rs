# Lakefs Client
lakefs-rs is a high-performance Rust library designed to seamlessly interact with the lakeFS REST API. If you're working with data lakes and need a reliable way to manage versioning, branching, and data operations, this library is your ultimate solution.

[![crates.io](https://img.shields.io/crates/v/lakefs-rs.svg)](https://crates.io/crates/lakefs-rs)
[![Documentation](https://docs.rs/lakefs-rs/badge.svg)](https://docs.rs/lakefs-rs)

## Install

```bash 
cargo add lakefs-rs
```

## Usage

- ENV
```env
LAKEFS_ENDPOINT=http://localhost:8000
LAKEFS_ACCESS_KEY=access_key
LAKEFS_ACCESS_KEY=secret_key
```
- Config from env
```
let cfg = Config::from_env().unwrap();
let client = LakeFsClient::new(cfg);
let test_repo = client.get_repository("test".to_string()).await
```
- Config manual
```
let config = Config::new("http://localhost:8000", "access_key", "secret_key", None);
let client = LakeFsClient::new(cfg);
let test_repo = client.get_repository("test".to_string()).await
```

- API
  - `client.setup_api` api setup lakefs for admin
  - `client.repositories_api` api repositories
  - `client.user_api` api user
  - `client.user_group_api` manager group users

### Todo!

-[ ] Manager [Object](https://docs.lakefs.io/reference/api.html#/objects)
-[ ] Manager [Actions](https://docs.lakefs.io/reference/api.html#/actions)
-[ ] Manager [Retention](https://docs.lakefs.io/reference/api.html#/retention)
-[ ] Manager [MataData](https://docs.lakefs.io/reference/api.html#/metadata)
-[ ] Manager [Import](https://docs.lakefs.io/reference/api.html#/import)
-[ ] Manager [Auth Policies](https://docs.lakefs.io/reference/api.html#/auth)