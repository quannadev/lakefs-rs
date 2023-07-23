use std::env;

pub fn set_evnvar() {
    env::set_var("RUST_LOG", "INFO");
    env::set_var("LAKEFS_ENDPOINT", "http://localhost:8000");
    env::set_var("LAKEFS_ACCESS_KEY", "AKIAJWOBNKM7IZS6DVTQ");
    env::set_var(
        "LAKEFS_SECRET_KEY",
        "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",
    );
    env::set_var("LAKEFS_API_VERSION", "v1");
    env_logger::init();
}
