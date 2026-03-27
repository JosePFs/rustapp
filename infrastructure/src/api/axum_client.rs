use serde::{Deserialize, Serialize};

use crate::api::config::ApiConfig;

const API_BASE_PATH: &str = "/api/v1/patients";

#[derive(Debug, Clone, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RefreshSessionRequest {
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExerciseFeedbackRequest {
    pub exercise_id: String,
    pub effort: i32,
    pub pain: i32,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MarkDayAsCompletedRequest {
    pub patient_program_id: String,
    pub day_index: i32,
    pub session_date: String,
    pub feedback: Vec<ExerciseFeedbackRequest>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MarkDayAsUncompletedRequest {
    pub workout_session_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub user_id: String,
    pub user_profile_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseInstructionResponse {
    pub exercise_id: String,
    pub name: String,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub sets: i32,
    pub reps: i32,
    pub effort: Option<i32>,
    pub pain: Option<i32>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramDayResponse {
    pub day_index: i32,
    pub day_number: i32,
    pub session_id: Option<String>,
    pub workout_name: Option<String>,
    pub workout_description: Option<String>,
    pub is_rest_day: bool,
    pub session_date: Option<String>,
    pub completed_at: Option<String>,
    pub exercises: Vec<ExerciseInstructionResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientProgramResponse {
    pub patient_program_id: String,
    pub program_id: String,
    pub program_name: String,
    pub program_description: Option<String>,
    pub days: Vec<ProgramDayResponse>,
    pub progress_percent: i32,
    pub average_effort: Option<f32>,
    pub average_pain: Option<f32>,
}

pub struct AxumApiClient {
    config: ApiConfig,
}

impl AxumApiClient {
    pub fn new(config: ApiConfig) -> Self {
        Self { config }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<LoginResponse, String> {
        let url = format!("{}{}/login", self.config.base_url, API_BASE_PATH);

        let request = LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let body = serde_json::to_vec(&request).map_err(|e| e.to_string())?;

        let response = rest_request_platform("POST", &url, &body).await?;

        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(format!("API error {}: {}", response.status, body_str));
        }

        let api_response: ApiResponse<LoginResponse> =
            serde_json::from_slice(&response.body).map_err(|e| e.to_string())?;

        Ok(api_response.data)
    }

    pub async fn refresh_session(&self, refresh_token: &str) -> Result<LoginResponse, String> {
        let url = format!("{}{}/refresh-session", self.config.base_url, API_BASE_PATH);

        let request = RefreshSessionRequest {
            refresh_token: refresh_token.to_string(),
        };

        let body = serde_json::to_vec(&request).map_err(|e| e.to_string())?;

        let response = rest_request_platform("POST", &url, &body).await?;

        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(format!("API error {}: {}", response.status, body_str));
        }

        let api_response: ApiResponse<LoginResponse> =
            serde_json::from_slice(&response.body).map_err(|e| e.to_string())?;

        Ok(api_response.data)
    }

    pub async fn get_programs(&self) -> Result<Vec<PatientProgramResponse>, String> {
        let url = format!("{}{}/get-programs", self.config.base_url, API_BASE_PATH);

        let response = rest_request_platform("GET", &url, &[]).await?;

        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(format!("API error {}: {}", response.status, body_str));
        }

        let api_response: ApiResponse<Vec<PatientProgramResponse>> =
            serde_json::from_slice(&response.body).map_err(|e| e.to_string())?;

        Ok(api_response.data)
    }

    pub async fn mark_day_as_completed(
        &self,
        patient_program_id: &str,
        day_index: i32,
        session_date: &str,
        feedback: Vec<(String, i32, i32, String)>,
    ) -> Result<(), String> {
        let url = format!("{}{}/mark-day-as-completed", self.config.base_url, API_BASE_PATH);

        let feedback_requests: Vec<ExerciseFeedbackRequest> = feedback
            .into_iter()
            .map(|(exercise_id, effort, pain, comment)| ExerciseFeedbackRequest {
                exercise_id,
                effort,
                pain,
                comment: if comment.is_empty() { None } else { Some(comment) },
            })
            .collect();

        let request = MarkDayAsCompletedRequest {
            patient_program_id: patient_program_id.to_string(),
            day_index,
            session_date: session_date.to_string(),
            feedback: feedback_requests,
        };

        let body = serde_json::to_vec(&request).map_err(|e| e.to_string())?;

        let response = rest_request_platform("POST", &url, &body).await?;

        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(format!("API error {}: {}", response.status, body_str));
        }

        Ok(())
    }

    pub async fn mark_day_as_uncompleted(&self, workout_session_id: &str) -> Result<(), String> {
        let url = format!("{}{}/mark-day-as-uncompleted", self.config.base_url, API_BASE_PATH);

        let request = MarkDayAsUncompletedRequest {
            workout_session_id: workout_session_id.to_string(),
        };

        let body = serde_json::to_vec(&request).map_err(|e| e.to_string())?;

        let response = rest_request_platform("POST", &url, &body).await?;

        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(format!("API error {}: {}", response.status, body_str));
        }

        Ok(())
    }
}

struct HttpResponse {
    status: u16,
    body: Vec<u8>,
}

#[cfg(not(target_arch = "wasm32"))]
async fn rest_request_platform(method: &str, url: &str, body: &[u8]) -> Result<HttpResponse, String> {
    let client = &*crate::api::SHARED_REQWEST_CLIENT;

    let mut req = match method {
        "POST" => client.post(url),
        "GET" => client.get(url),
        "PATCH" => client.patch(url),
        "DELETE" => client.delete(url),
        _ => return Err(format!("Unsupported method: {}", method)),
    };

    req = req
        .header("Content-Type", "application/json")
        .header("Accept", "application/json");

    let response = req
        .body(body.to_vec())
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status().as_u16();
    let body = response
        .bytes()
        .await
        .map_err(|e| e.to_string())?
        .to_vec();

    Ok(HttpResponse { status, body })
}

#[cfg(target_arch = "wasm32")]
async fn rest_request_platform(method: &str, url: &str, body: &[u8]) -> Result<HttpResponse, String> {
    use gloo_net::http::Request;
    use js_sys::Uint8Array;
    use wasm_bindgen::JsValue;

    let req = match method {
        "POST" => Request::post(url),
        "GET" => Request::get(url),
        "PATCH" => Request::patch(url),
        "DELETE" => Request::delete(url),
        _ => return Err(format!("Unsupported method: {}", method)),
    };

    let js_body: JsValue = Uint8Array::from(body).into();

    let response = req
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(js_body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();
    let body = response
        .binary()
        .await
        .map_err(|e| e.to_string())?
        .to_vec();

    Ok(HttpResponse { status, body })
}