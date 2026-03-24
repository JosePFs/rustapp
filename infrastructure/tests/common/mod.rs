pub mod config;
pub mod logger;

pub async fn setup() -> (reqwest::Client, config::TestConfig) {
    config::load_env();
    logger::init_test_logger();
    let (client, config) = config::client_for_test(config::TestConfig::from_env())
        .await
        .expect("Failed to create client");
    (client, config)
}
