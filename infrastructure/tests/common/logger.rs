use std::sync::LazyLock;

static LOGGER: LazyLock<()> = LazyLock::new(|| {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()))
        .with_test_writer()
        .init();
});

pub fn init_test_logger() {
    let _ = *LOGGER;
}
