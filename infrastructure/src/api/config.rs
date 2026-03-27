use std::env;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub base_url: String,
}

impl ApiConfig {
    pub fn from_env() -> Self {
        let host = env::var("API_HOST").unwrap_or("127.0.0.1".to_string());
        let port = env::var("API_PORT").unwrap_or("3000".to_string());
        Self {
            base_url: format!("http://{}:{}", host, port),
        }
    }
}
