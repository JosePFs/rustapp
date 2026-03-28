#[cfg(not(target_arch = "wasm32"))]
use std::sync::LazyLock;

#[cfg(not(target_arch = "wasm32"))]
static REQWEST_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

#[derive(Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

#[derive(Clone)]
pub struct PlatformsHttpClient;

impl PlatformsHttpClient {
    pub async fn get(&self, url: &str) -> Result<HttpResponse, String> {
        rest_request_platform("GET", url, None).await
    }

    pub async fn post(&self, url: &str, body: Option<&[u8]>) -> Result<HttpResponse, String> {
        rest_request_platform("POST", url, body).await
    }

    pub async fn patch(&self, url: &str, body: &[u8]) -> Result<HttpResponse, String> {
        rest_request_platform("PATCH", url, Some(body)).await
    }

    pub async fn delete(&self, url: &str) -> Result<HttpResponse, String> {
        rest_request_platform("DELETE", url, None).await
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn rest_request_platform(
    method: &str,
    url: &str,
    body: Option<&[u8]>,
) -> Result<HttpResponse, String> {
    let client = &*REQWEST_CLIENT;

    let mut req = match method {
        "POST" => client.post(url),
        "GET" => client.get(url),
        "PATCH" => client.patch(url),
        "DELETE" => client.delete(url),
        _ => {
            return Err("Unsupported method".to_string());
        }
    };

    req = req
        .header("Content-Type", "application/json")
        .header("Accept", "application/json");

    let response = req
        .body(body.unwrap_or(&[]).to_vec())
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status().as_u16();
    let body = response.bytes().await.map_err(|e| e.to_string())?.to_vec();

    Ok(HttpResponse { status, body })
}

#[cfg(target_arch = "wasm32")]
async fn rest_request_platform(
    method: &str,
    url: &str,
    body: Option<&[u8]>,
) -> Result<HttpResponse, String> {
    use gloo_net::http::Request;
    use js_sys::Uint8Array;
    use wasm_bindgen::JsValue;

    let request_builder = match method {
        "POST" => Request::post(url),
        "PATCH" => Request::patch(url),
        "PUT" => Request::put(url),
        "GET" => Request::get(url),
        "DELETE" => Request::delete(url),
        _ => {
            return Err("Unsupported method".to_string());
        }
    }
    .header("Content-Type", "application/json")
    .header("Accept", "application/json");

    if matches!(method, "POST" | "PATCH" | "PUT") {
        let js_body: JsValue = Uint8Array::from(body.unwrap_or(&[])).into();

        let request = request_builder.body(js_body).map_err(|e| {
            log::error!("Body error: {:?}", e);
            e.to_string()
        })?;
        let response = request.send().await.map_err(|e| e.to_string())?;
        let status = response.status();
        let body: Vec<u8> = response.binary().await.map_err(|e| e.to_string())?;

        return Ok(HttpResponse { status, body });
    } else {
        let response = request_builder.send().await.map_err(|e| e.to_string())?;
        let status = response.status();
        let body: Vec<u8> = response.binary().await.map_err(|e| e.to_string())?;

        return Ok(HttpResponse { status, body });
    }
}
