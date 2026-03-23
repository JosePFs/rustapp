use std::sync::Arc;

use serde::Deserialize;

use crate::supabase::config::SupabaseConfig;
use application::ports::{
    auth::AuthService,
    error::{ApplicationError, Result},
    HttpRestClient,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ResponseStatus(pub u16);

impl ResponseStatus {
    pub const DEFAULT_ERROR_STATUS: u16 = 400;

    pub fn is_auth_error(&self) -> bool {
        self.0 == 401 || self.0 == 403
    }

    pub fn is_success(&self) -> bool {
        self.0 >= 200 && self.0 < 300
    }

    pub fn is_client_error(&self) -> bool {
        self.0 >= 400 && self.0 < 500
    }

    pub fn is_server_error(&self) -> bool {
        self.0 >= 500 && self.0 < 600
    }
}

pub struct SupabaseClient {
    config: SupabaseConfig,
    auth_service: Arc<dyn AuthService>,
}

impl HttpRestClient for SupabaseClient {
    async fn get(&self, path: &str) -> Result<Vec<u8>> {
        let token = self.get_valid_token().await?;
        self.rest_request_with_retry(Some(&token), "GET", path, None, None)
            .await
    }

    async fn post(&self, path: &str, body: &str) -> Result<Vec<u8>> {
        let token = self.get_valid_token().await?;
        self.rest_request_with_retry(Some(&token), "POST", path, Some(body.as_bytes()), None)
            .await
    }

    async fn patch(&self, path: &str, body: &str) -> Result<Vec<u8>> {
        let token = self.get_valid_token().await?;
        self.rest_request_with_retry(Some(&token), "PATCH", path, Some(body.as_bytes()), None)
            .await
    }

    async fn upsert(&self, path: &str, body: &str) -> Result<Vec<u8>> {
        let token = self.get_valid_token().await?;
        self.rest_request_with_retry(
            Some(&token),
            "POST",
            path,
            Some(body.as_bytes()),
            Some("resolution=merge-duplicates,return=representation"),
        )
        .await
    }

    async fn delete(&self, path: &str) -> Result<Vec<u8>> {
        self.rest_request_with_retry(None, "DELETE", path, None, None)
            .await
    }
}

impl SupabaseClient {
    pub fn new(config: SupabaseConfig, auth_service: Arc<dyn AuthService>) -> Self {
        Self {
            config,
            auth_service,
        }
    }

    async fn get_valid_token(&self) -> Result<String> {
        let session = self
            .auth_service
            .get_session()
            .ok_or(ApplicationError::NoSession)?;
        if session.should_refresh() {
            let refresh_token = session
                .refresh_token()
                .ok_or(ApplicationError::NoRefreshToken)?;
            let refreshed = self
                .auth_service
                .refresh_session(refresh_token)
                .await
                .or(Err(ApplicationError::RefreshFailed))?;
            return Ok(refreshed.access_token().to_string());
        }
        Ok(session.access_token().to_string())
    }

    async fn rest_request_with_retry(
        &self,
        access_token: Option<&str>,
        method: &str,
        path: &str,
        body: Option<&[u8]>,
        prefer: Option<&str>,
    ) -> Result<Vec<u8>> {
        let result = self
            .rest_request(access_token, method, path, body, prefer)
            .await;
        match result {
            Ok(response) => Ok(response),
            Err(error) => {
                if error.is_auth_error() {
                    let token = self.get_valid_token().await?;
                    return self
                        .rest_request(Some(&token), method, path, body, prefer)
                        .await;
                }
                Err(error)
            }
        }
    }

    pub async fn rest_request(
        &self,
        access_token: Option<&str>,
        method: &str,
        path: &str,
        body: Option<&[u8]>,
        prefer: Option<&str>,
    ) -> Result<Vec<u8>> {
        let url = format!("{}{}", self.config.rest_url().trim_end_matches('/'), path);
        let response = rest_request_platform(
            method,
            &url,
            &self.config.anon_key,
            access_token,
            body,
            prefer,
        )
        .await?;
        if ResponseStatus(response.status).is_success() {
            Ok(response.body)
        } else {
            Err(ApplicationError::api(
                response.status,
                String::from_utf8_lossy(&response.body).to_string(),
            ))
        }
    }
}

#[derive(Deserialize)]
struct HttpResponse {
    status: u16,
    body: Vec<u8>,
}

#[cfg(not(target_arch = "wasm32"))]
async fn rest_request_platform(
    method: &str,
    url: &str,
    apikey: &str,
    bearer: Option<&str>,
    body: Option<&[u8]>,
    prefer: Option<&str>,
) -> Result<HttpResponse> {
    let client = &*crate::supabase::SHARED_REQWEST_CLIENT;

    let mut req = match method {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PATCH" => client.patch(url),
        "DELETE" => client.delete(url),
        _ => {
            return Err(ApplicationError::api(
                500,
                "Unsupported rest request method".to_string(),
            ))
        }
    };
    req = req
        .header("apikey", apikey)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Prefer", prefer.unwrap_or("return=representation"));
    if let Some(t) = bearer {
        req = req.header("Authorization", format!("Bearer {}", t));
    }
    if let Some(b) = body {
        req = req.body(b.to_vec());
    }
    let response = req.send().await.map_err(|e| {
        ApplicationError::api(
            e.status()
                .map_or(ResponseStatus::DEFAULT_ERROR_STATUS, |s| s.as_u16()),
            e.to_string(),
        )
    })?;
    let status = response.status().as_u16();
    let body = response
        .bytes()
        .await
        .map_err(|e| ApplicationError::api(ResponseStatus::DEFAULT_ERROR_STATUS, e.to_string()))?
        .to_vec();

    Ok(HttpResponse { status, body })
}

#[cfg(target_arch = "wasm32")]
async fn rest_request_platform(
    method: &str,
    url: &str,
    apikey: &str,
    bearer: Option<&str>,
    body: Option<&[u8]>,
    prefer: Option<&str>,
) -> Result<HttpResponse> {
    use gloo_net::http::Request;
    use js_sys::Uint8Array;
    use wasm_bindgen::JsValue;
    let mut req = match method {
        "GET" => Request::get(url),
        "POST" => Request::post(url),
        "PATCH" => Request::patch(url),
        "DELETE" => Request::delete(url),
        _ => {
            return Err(ApplicationError::api(
                ResponseStatus::DEFAULT_ERROR_STATUS,
                "Unsupported rest request method".to_string(),
            ))
        }
    };
    req = req
        .header("apikey", apikey)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Prefer", prefer.unwrap_or("return=representation"));
    if let Some(t) = bearer {
        req = req.header("Authorization", &format!("Bearer {}", t));
    }
    let response = if let Some(b) = body {
        let js_body: JsValue = Uint8Array::from(b).into();
        req.body(js_body)
            .map_err(|e| {
                ApplicationError::api(ResponseStatus::DEFAULT_ERROR_STATUS, e.to_string())
            })?
            .send()
            .await
            .map_err(|e| {
                ApplicationError::api(ResponseStatus::DEFAULT_ERROR_STATUS, e.to_string())
            })?
    } else {
        req.send().await.map_err(|e| {
            ApplicationError::api(ResponseStatus::DEFAULT_ERROR_STATUS, e.to_string())
        })?
    };
    let status = response.status();
    let body = response
        .binary()
        .await
        .map_err(|e| ApplicationError::api(ResponseStatus::DEFAULT_ERROR_STATUS, e.to_string()))?;
    Ok(HttpResponse {
        status,
        body: body.to_vec(),
    })
}
