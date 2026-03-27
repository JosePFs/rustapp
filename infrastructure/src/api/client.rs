use serde::Deserialize;

use crate::api::config::ApiConfig;
use application::ports::{
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

pub struct ApiClient {
    config: ApiConfig,
}

impl ApiClient {
    pub fn new(config: ApiConfig) -> Self {
        Self { config }
    }
}

impl HttpRestClient for ApiClient {
    async fn get(&self, path: &str) -> Result<Vec<u8>> {
        Err(ApplicationError::api(
            ResponseStatus::DEFAULT_ERROR_STATUS,
            "Not implemented".to_string(),
        ))
    }

    async fn post(&self, path: &str, body: &str) -> Result<Vec<u8>> {
        Err(ApplicationError::api(
            ResponseStatus::DEFAULT_ERROR_STATUS,
            "Not implemented".to_string(),
        ))
    }

    async fn patch(&self, path: &str, body: &str) -> Result<Vec<u8>> {
        Err(ApplicationError::api(
            ResponseStatus::DEFAULT_ERROR_STATUS,
            "Not implemented".to_string(),
        ))
    }

    async fn upsert(&self, path: &str, body: &str) -> Result<Vec<u8>> {
        Err(ApplicationError::api(
            ResponseStatus::DEFAULT_ERROR_STATUS,
            "Not implemented".to_string(),
        ))
    }

    async fn delete(&self, path: &str) -> Result<Vec<u8>> {
        Err(ApplicationError::api(
            ResponseStatus::DEFAULT_ERROR_STATUS,
            "Not implemented".to_string(),
        ))
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
