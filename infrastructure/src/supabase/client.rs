use serde::{Deserialize, Serialize};

use crate::supabase::config::SupabaseConfig;
use domain::vos::credentials::Credentials;

#[derive(Clone)]
pub struct SupabaseClient {
    config: SupabaseConfig,
}

impl SupabaseClient {
    pub fn new(config: SupabaseConfig) -> Self {
        Self { config }
    }

    pub async fn sign_in(&self, credentials: &Credentials) -> Result<AuthSession, String> {
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
            return Err(format!("Auth failed: status {}", response.status));
        }
        let session: AuthSession =
            serde_json::from_slice(&response.body).map_err(|e| format!("Parse auth: {}", e))?;
        Ok(session)
    }

    pub async fn refresh_session(&self, refresh_token: &str) -> Result<AuthSession, String> {
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
            return Err(format!("Auth refresh failed: status {}", response.status));
        }
        let session: AuthSession =
            serde_json::from_slice(&response.body).map_err(|e| format!("Parse auth: {}", e))?;
        Ok(session)
    }

    pub async fn rest_request(
        &self,
        access_token: Option<&str>,
        method: &str,
        path: &str,
        body: Option<&[u8]>,
    ) -> Result<Vec<u8>, String> {
        let url = format!("{}{}", self.config.rest_url().trim_end_matches('/'), path);
        let response = rest_request_inner(
            method,
            &url,
            &self.config.anon_key,
            access_token,
            body,
            None,
        )
        .await?;
        if response.status >= 200 && response.status < 300 {
            Ok(response.body)
        } else {
            Err(format!("REST {}: status {}", method, response.status))
        }
    }

    pub async fn rest_get(
        &self,
        access_token: Option<&str>,
        path: &str,
    ) -> Result<Vec<u8>, String> {
        self.rest_request(access_token, "GET", path, None).await
    }

    pub async fn rest_post<T: Serialize>(
        &self,
        access_token: Option<&str>,
        path: &str,
        payload: &T,
    ) -> Result<Vec<u8>, String> {
        let body = serde_json::to_vec(payload).map_err(|e| e.to_string())?;
        self.rest_request(access_token, "POST", path, Some(&body))
            .await
    }

    pub async fn rest_patch<T: Serialize>(
        &self,
        access_token: Option<&str>,
        path: &str,
        payload: &T,
    ) -> Result<Vec<u8>, String> {
        let body = serde_json::to_vec(payload).map_err(|e| e.to_string())?;
        self.rest_request(access_token, "PATCH", path, Some(&body))
            .await
    }

    pub async fn rest_delete(
        &self,
        access_token: Option<&str>,
        path: &str,
    ) -> Result<Vec<u8>, String> {
        self.rest_request(access_token, "DELETE", path, None).await
    }

    pub async fn rest_upsert<T: Serialize>(
        &self,
        access_token: Option<&str>,
        path: &str,
        payload: &T,
    ) -> Result<Vec<u8>, String> {
        let body = serde_json::to_vec(payload).map_err(|e| e.to_string())?;
        let url = format!("{}{}", self.config.rest_url().trim_end_matches('/'), path);
        let response = rest_request_inner(
            "POST",
            &url,
            &self.config.anon_key,
            access_token,
            Some(&body),
            Some("resolution=merge-duplicates,return=representation"),
        )
        .await?;
        if response.status >= 200 && response.status < 300 {
            Ok(response.body)
        } else {
            Err(format!("REST UPSERT: status {}", response.status))
        }
    }
}

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

#[cfg(not(target_arch = "wasm32"))]
async fn rest_request_inner(
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
async fn rest_request_inner(
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
