use std::sync::Arc;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::supabase::config::SupabaseConfig;
use application::ports::auth::{AuthService, Credentials, Session};
use domain::error::{DomainError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthSession {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub user: AuthUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: Option<String>,
    pub user_metadata: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct SignInBody {
    grant_type: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct RefreshBody {
    grant_type: String,
    refresh_token: String,
}

#[derive(Deserialize)]
struct HttpResponse {
    status: u16,
    body: Vec<u8>,
}

pub struct SupabaseAuth {
    config: SupabaseConfig,
    session: Mutex<Option<Session>>,
}

impl SupabaseAuth {
    fn new(config: SupabaseConfig) -> Self {
        Self {
            config,
            session: Mutex::new(None),
        }
    }

    pub fn builder() -> SupabaseAuthBuilder {
        SupabaseAuthBuilder::new()
    }
}

#[common::async_trait_platform]
impl AuthService for SupabaseAuth {
    async fn sign_in(&self, credentials: &Credentials) -> Result<Session> {
        let url = format!("{}/token?grant_type=password", self.config.auth_url());
        let body = SignInBody {
            grant_type: "password".to_string(),
            email: credentials.email().value().to_string(),
            password: credentials.password().value().to_string(),
        };
        let body_bytes = serde_json::to_vec(&body).map_err(|e| e.to_string())?;
        let response = http_request(
            "POST",
            &url,
            &[
                ("apikey", self.config.anon_key.as_str()),
                ("Content-Type", "application/json"),
            ],
            Some(&body_bytes),
        )
        .await?;
        if response.status < 200 || response.status >= 300 {
            return Err(DomainError::Login(format!(
                "Auth failed: status {}",
                response.status
            )));
        }
        let session: AuthSession = serde_json::from_slice(&response.body)
            .map_err(|e| DomainError::Login(format!("Parse auth: {}", e)))?;
        let session = Session::new(session.access_token, session.refresh_token, session.user.id);
        *self.session.lock().unwrap() = Some(session.clone());
        Ok(session)
    }

    async fn refresh_session(&self, refresh_token: &str) -> Result<Session> {
        let url = format!("{}/token?grant_type=refresh_token", self.config.auth_url());
        let body = RefreshBody {
            grant_type: "refresh_token".to_string(),
            refresh_token: refresh_token.to_string(),
        };
        let body_bytes = serde_json::to_vec(&body).map_err(|e| e.to_string())?;
        let response = http_request(
            "POST",
            &url,
            &[
                ("apikey", self.config.anon_key.as_str()),
                ("Content-Type", "application/json"),
            ],
            Some(&body_bytes),
        )
        .await?;
        if response.status < 200 || response.status >= 300 {
            return Err(DomainError::Login(format!(
                "Auth refresh failed: status {}",
                response.status
            )));
        }
        let session: AuthSession = serde_json::from_slice(&response.body)
            .map_err(|e| DomainError::Login(format!("Parse auth: {}", e)))?;
        let session = Session::new(session.access_token, session.refresh_token, session.user.id);
        *self.session.lock().unwrap() = Some(session.clone());
        Ok(session)
    }

    fn get_session(&self) -> Option<Session> {
        self.session.lock().unwrap().clone()
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn http_request(
    method: &str,
    url: &str,
    headers: &[(&str, &str)],
    body: Option<&[u8]>,
) -> Result<HttpResponse> {
    let client = &*crate::supabase::SHARED_REQWEST_CLIENT;

    let mut req = match method {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PATCH" => client.patch(url),
        "DELETE" => client.delete(url),
        _ => return Err(DomainError::Api("Unsupported method".to_string())),
    };

    for (k, v) in headers {
        req = req.header(*k, *v);
    }
    if let Some(b) = body {
        req = req.body(b.to_vec());
    }

    let response = req.send().await.map_err(|e| e.to_string())?;
    let status = response.status().as_u16();
    let body = response.bytes().await.map_err(|e| e.to_string())?.to_vec();

    Ok(HttpResponse { status, body })
}

#[cfg(target_arch = "wasm32")]
async fn http_request(
    method: &str,
    url: &str,
    headers: &[(&str, &str)],
    body: Option<&[u8]>,
) -> Result<HttpResponse> {
    use gloo_net::http::Request;
    use js_sys::Uint8Array;
    use wasm_bindgen::JsValue;
    let mut req = match method {
        "GET" => Request::get(url),
        "POST" => Request::post(url),
        "PATCH" => Request::patch(url),
        "DELETE" => Request::delete(url),
        _ => return Err(DomainError::Api("Unsupported method".to_string())),
    };
    for (k, v) in headers {
        req = req.header(*k, *v);
    }
    let response = if let Some(b) = body {
        let js_body: JsValue = Uint8Array::from(b).into();
        req.body(js_body)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?
    } else {
        req.send().await.map_err(|e| e.to_string())?
    };
    let status = response.status();
    let body = response.binary().await.map_err(|e| e.to_string())?;
    Ok(HttpResponse {
        status,
        body: body.to_vec(),
    })
}

pub struct SupabaseAuthBuilder {
    config: Option<SupabaseConfig>,
}

impl SupabaseAuthBuilder {
    pub fn new() -> Self {
        Self { config: None }
    }

    pub fn with_config(mut self, config: SupabaseConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> Arc<SupabaseAuth> {
        let config = self
            .config
            .unwrap_or_else(|| SupabaseConfig::from_env().expect("Failed to load Supabase config"));
        Arc::new(SupabaseAuth::new(config))
    }
}
