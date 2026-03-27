pub const API_BASE_PATH: &str = "/api/v1";

pub const PATIENTS_BASE_PATH: &str = "/patients";

pub fn api_path() -> String {
    API_BASE_PATH.to_string()
}

pub fn patients_path(path: Option<String>) -> String {
    if let Some(path) = path {
        format!("{}/{}", PATIENTS_BASE_PATH, path)
    } else {
        PATIENTS_BASE_PATH.to_string()
    }
}
