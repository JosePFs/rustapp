pub const API_BASE_PATH: &str = "/api/v1";

pub const PATIENTS_BASE_PATH: &str = "/patients";
pub const SPECIALISTS_BASE_PATH: &str = "/specialists";

pub fn patients_path() -> String {
    PATIENTS_BASE_PATH.to_string()
}

pub fn specialists_path() -> String {
    SPECIALISTS_BASE_PATH.to_string()
}
