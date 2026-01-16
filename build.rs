fn main() {
    if std::env::var("CARGO_FEATURE_WITH_SERDE").is_ok() {
        println!("cargo:warning=The `with-serde` feature has been renamed to `serde`. Please update your Cargo.toml to use `serde` instead. The `with-serde` alias will be removed in a future release.");
    }
}
