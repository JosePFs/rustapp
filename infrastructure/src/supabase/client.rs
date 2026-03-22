use std::sync::Arc;

use serde::Deserialize;

use crate::supabase::config::SupabaseConfig;
use application::ports::{auth::AuthService, HttpRestClient};

#[derive(Clone)]
pub struct SupabaseClient {
    config: SupabaseConfig,
    auth_service: Arc<dyn AuthService>,
}

impl HttpRestClient for SupabaseClient {
    async fn get(&self, path: &str) -> Result<Vec<u8>, String> {
        let session = self
            .auth_service
            .get_session()
            .map(|s| s.access_token().to_string());
        self.rest_request(session.as_deref(), "GET", path, None, None)
            .await
    }

    async fn post(&self, path: &str, body: &str) -> Result<Vec<u8>, String> {
        let session = self
            .auth_service
            .get_session()
            .map(|s| s.access_token().to_string());
        self.rest_request(
            session.as_deref(),
            "POST",
            path,
            Some(body.as_bytes()),
            None,
        )
        .await
    }

    async fn patch(&self, path: &str, body: &str) -> Result<Vec<u8>, String> {
        let session = self
            .auth_service
            .get_session()
            .map(|s| s.access_token().to_string());
        self.rest_request(
            session.as_deref(),
            "PATCH",
            path,
            Some(body.as_bytes()),
            None,
        )
        .await
    }

    async fn upsert(&self, path: &str, body: &str) -> Result<Vec<u8>, String> {
        let session = self
            .auth_service
            .get_session()
            .map(|s| s.access_token().to_string());
        self.rest_request(
            session.as_deref(),
            "POST",
            path,
            Some(body.as_bytes()),
            Some("resolution=merge-duplicates,return=representation"),
        )
        .await
    }

    async fn delete(&self, path: &str) -> Result<Vec<u8>, String> {
        self.rest_request(None, "DELETE", path, None, None).await
    }
}

impl SupabaseClient {
    pub fn new(config: SupabaseConfig, auth_service: Arc<dyn AuthService>) -> Self {
        Self {
            config,
            auth_service,
        }
    }

    pub async fn rest_request(
        &self,
        access_token: Option<&str>,
        method: &str,
        path: &str,
        body: Option<&[u8]>,
        prefer: Option<&str>,
    ) -> Result<Vec<u8>, String> {
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
        if response.status >= 200 && response.status < 300 {
            Ok(response.body)
        } else {
            Err(format!("REST {}: status {}", method, response.status))
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
) -> Result<HttpResponse, String> {
    use reqwest::Client;
    let client = Client::new();
    let mut req = match method {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PATCH" => client.patch(url),
        "DELETE" => client.delete(url),
        _ => return Err("Unsupported method".to_string()),
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
    let response = req.send().await.map_err(|e| e.to_string())?;
    let status = response.status().as_u16();
    let body = response.bytes().await.map_err(|e| e.to_string())?.to_vec();
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
) -> Result<HttpResponse, String> {
    use gloo_net::http::Request;
    use js_sys::Uint8Array;
    use wasm_bindgen::JsValue;
    let mut req = match method {
        "GET" => Request::get(url),
        "POST" => Request::post(url),
        "PATCH" => Request::patch(url),
        "DELETE" => Request::delete(url),
        _ => return Err("Unsupported method".to_string()),
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
