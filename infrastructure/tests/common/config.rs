use std::env;

pub fn load_env() {
    let _ = dotenvy::from_filename(".env.test");
}

pub struct TestConfig {
    pub supabase_url: String,
    pub auth_url: String,
    pub anon_key: String,
}

impl TestConfig {
    pub fn from_env() -> Self {
        Self {
            supabase_url: env::var("SUPABASE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:54321".into()),
            auth_url: env::var("TEST_SUPABASE_AUTH_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:54321/auth/v1".into()),
            anon_key: env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY no definida"),
        }
    }
}

pub async fn client_for_test(
    config: TestConfig,
) -> Result<(reqwest::Client, TestConfig), reqwest::Error> {
    let client = reqwest::Client::new();
    for _ in 0..30 {
        if client.get(&config.supabase_url).send().await.is_ok() {
            return Ok((client, config));
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    panic!("PostgREST not available at {}", config.supabase_url);
}
