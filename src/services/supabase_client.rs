//! Supabase client: Auth + PostgREST.
//! Abstraction over HTTP so backend can be swapped for Axum later.

use serde::{Deserialize, Serialize};

/// Supabase project config (from env or app config).
#[derive(Clone)]
pub struct SupabaseConfig {
    pub url: String,
    pub anon_key: String,
}

impl SupabaseConfig {
    /// Load from env: SUPABASE_URL, SUPABASE_ANON_KEY.
    /// Values come from .env at build time (build.rs) so they work in WASM and native.
    pub fn from_env() -> Option<Self> {
        let url: String = option_env!("SUPABASE_URL")
            .map(str::to_string)
            .or_else(|| std::env::var("SUPABASE_URL").ok())?;
        let anon_key: String = option_env!("SUPABASE_ANON_KEY")
            .map(str::to_string)
            .or_else(|| std::env::var("SUPABASE_ANON_KEY").ok())?;
        Some(Self { url, anon_key })
    }

    pub fn auth_url(&self) -> String {
        format!("{}/auth/v1", self.url.trim_end_matches('/'))
    }

    pub fn rest_url(&self) -> String {
        format!("{}/rest/v1", self.url.trim_end_matches('/'))
    }
}

// -----------------------------------------------------------------------------
// Auth
// -----------------------------------------------------------------------------

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

/// Sign in with email and password.
/// Returns session on success.
pub async fn sign_in(
    config: &SupabaseConfig,
    email: &str,
    password: &str,
) -> Result<AuthSession, String> {
    let url = format!("{}/token?grant_type=password", config.auth_url());
    let body = SignInBody {
        grant_type: "password".to_string(),
        email: email.to_string(),
        password: password.to_string(),
    };
    let body_bytes = serde_json::to_vec(&body).map_err(|e| e.to_string())?;

    let response = http_request(
        "POST",
        &url,
        &[
            ("apikey", config.anon_key.as_str()),
            ("Content-Type", "application/json"),
        ],
        Some(&body_bytes),
    )
    .await?;

    if response.status < 200 || response.status >= 300 {
        return Err(format!("Auth failed: status {}", response.status));
    }
    let session: AuthSession =
        serde_json::from_slice(&response.body).map_err(|e| format!("Parse auth: {}", e))?;
    Ok(session)
}

// -----------------------------------------------------------------------------
// REST (PostgREST) helpers
// -----------------------------------------------------------------------------

struct HttpResponse {
    status: u16,
    body: Vec<u8>,
}

#[cfg(not(target_arch = "wasm32"))]
async fn http_request(
    method: &str,
    url: &str,
    headers: &[(&str, &str)],
    body: Option<&[u8]>,
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
    for (k, v) in headers {
        req = req.header(*k, *v);
    }
    let response = if let Some(b) = body {
        let js_body: JsValue = Uint8Array::from(b).into();
        req.body(js_body).map_err(|e| e.to_string())?.send().await.map_err(|e| e.to_string())?
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

/// REST request with optional Bearer token (for RLS).
pub async fn rest_request(
    config: &SupabaseConfig,
    access_token: Option<&str>,
    method: &str,
    path: &str,
    body: Option<&[u8]>,
) -> Result<Vec<u8>, String> {
    let url = format!("{}{}", config.rest_url().trim_end_matches('/'), path);
    let response = rest_request_inner(
        method,
        &url,
        &config.anon_key,
        access_token,
        body,
    )
    .await?;
    if response.status >= 200 && response.status < 300 {
        Ok(response.body)
    } else {
        Err(format!("REST {}: status {}", method, response.status))
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn rest_request_inner(
    method: &str,
    url: &str,
    apikey: &str,
    bearer: Option<&str>,
    body: Option<&[u8]>,
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
        .header("Prefer", "return=representation");
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
async fn rest_request_inner(
    method: &str,
    url: &str,
    apikey: &str,
    bearer: Option<&str>,
    body: Option<&[u8]>,
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
        .header("Prefer", "return=representation");
    if let Some(t) = bearer {
        req = req.header("Authorization", &format!("Bearer {}", t));
    }
    let response = if let Some(b) = body {
        let js_body: JsValue = Uint8Array::from(b).into();
        req.body(js_body).map_err(|e| e.to_string())?.send().await.map_err(|e| e.to_string())?
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

/// GET table or path. Returns JSON bytes.
pub async fn rest_get(
    config: &SupabaseConfig,
    access_token: Option<&str>,
    path: &str,
) -> Result<Vec<u8>, String> {
    rest_request(config, access_token, "GET", path, None).await
}

/// POST to path with JSON body.
pub async fn rest_post<T: Serialize>(
    config: &SupabaseConfig,
    access_token: Option<&str>,
    path: &str,
    payload: &T,
) -> Result<Vec<u8>, String> {
    let body = serde_json::to_vec(payload).map_err(|e| e.to_string())?;
    rest_request(config, access_token, "POST", path, Some(&body)).await
}

/// PATCH path with JSON body.
pub async fn rest_patch<T: Serialize>(
    config: &SupabaseConfig,
    access_token: Option<&str>,
    path: &str,
    payload: &T,
) -> Result<Vec<u8>, String> {
    let body = serde_json::to_vec(payload).map_err(|e| e.to_string())?;
    rest_request(config, access_token, "PATCH", path, Some(&body)).await
}
