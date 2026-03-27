use std::{env, net::IpAddr};

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            server: ServerConfig::from_env(),
        }
    }

    pub fn host(&self) -> IpAddr {
        self.server
            .host
            .parse()
            .unwrap_or("127.0.0.1".parse().unwrap())
    }

    pub fn port(&self) -> u16 {
        self.server.port
    }

    pub fn cors_allowed_origins(&self) -> String {
        self.server.cors_allowed_origins.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub log_folder: String,
    pub environment: String,
    pub cors_allowed_origins: String,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        Self {
            host: env::var("API_HOST").unwrap_or("127.0.0.1".to_string()),
            port: env::var("API_PORT")
                .unwrap_or("3000".to_string())
                .parse()
                .unwrap_or(3000),
            log_level: env::var("API_LOG_LEVEL").unwrap_or("debug".to_string()),
            log_folder: env::var("API_LOG_FOLDER").unwrap_or("logs".to_string()),
            environment: env::var("API_ENVIRONMENT").unwrap_or("development".to_string()),
            cors_allowed_origins: env::var("API_CORS_ALLOWED_ORIGINS")
                .unwrap_or("".to_string())
                .split(",")
                .map(|s| s.to_string())
                .collect(),
        }
    }
}
