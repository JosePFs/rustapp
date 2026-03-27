use std::env;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
}

impl ApiConfig {
    pub fn from_env() -> Self {
        Self {
            host: env::var("API_HOST").unwrap_or("127.0.0.1".to_string()),
            port: env::var("API_PORT")
                .unwrap_or("3000".to_string())
                .parse()
                .unwrap(),
        }
    }
}
