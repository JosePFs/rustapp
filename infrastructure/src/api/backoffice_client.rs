use serde::{Deserialize, Serialize};

use crate::api::{config::ApiConfig, platforms_http_client::PlatformsHttpClient};
use application::error::{ApplicationError, Result};
use application::ports::backoffice_api::{
    AddExerciseToWorkoutArgs, AddSpecialistPatientArgs, AddSpecialistPatientResult,
    AssignProgramToPatientArgs, AssignProgramToPatientResult, CreateExerciseArgs,
    CreateExerciseResult, CreateProgramArgs, CreateProgramResult,
    CreateProgramScheduleItemArgs, CreateProgramScheduleItemResult, CreateWorkoutArgs,
    CreateWorkoutResult, DeleteProgramScheduleItemArgs, DeleteWorkoutArgs,
    ExerciseLibraryItem, GetSpecialistPatientsWithProfilesArgs,
    GetSpecialistPatientsWithProfilesResult, ListExerciseLibraryArgs,
    ListExerciseLibraryResult, ListProgramScheduleArgs, ListProgramScheduleResult,
    ListUnassignedPatientsArgs, ListUnassignedPatientsResult, ListWorkoutLibraryArgs,
    ListWorkoutLibraryResult, LoginArgs, LoginResult, PatientProgramAssignment,
    PatientProgressArgs, PatientProgressProfile, PatientProgressResult,
    PatientProfileSummary, ProgramSummary, ProgramScheduleEntry, RemoveExerciseFromWorkoutArgs,
    RestoreExerciseArgs, SpecialistPatientLink, SpecialistProgramsDataResult,
    SoftDeleteExerciseArgs, UnassignedPatient, UpdateExerciseArgs, UpdateWorkoutArgs,
    UpdateWorkoutExerciseArgs, WorkoutEditorDataArgs, WorkoutEditorDataResult,
    WorkoutEditorExerciseItem, WorkoutEditorLine, WorkoutEditorWorkout, WorkoutList,
    WorkoutLibraryItem, BackofficeApi,
};

const API_BASE_PATH: &str = "/api/v1/specialists";

#[derive(Debug, Clone, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddSpecialistPatientRequest {
    pub patient_email: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddExerciseToWorkoutRequest {
    pub workout_id: String,
    pub exercise_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AssignProgramToPatientRequest {
    pub patient_email: String,
    pub program_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateExerciseRequest {
    pub name: String,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub sets: i32,
    pub reps: i32,
    pub target_body_part: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateProgramRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateProgramScheduleItemRequest {
    pub program_id: String,
    pub day_index: i32,
    pub workout_id: Option<String>,
    pub is_rest_day: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateWorkoutRequest {
    pub name: String,
    pub description: Option<String>,
    pub exercise_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteProgramScheduleItemRequest {
    pub program_id: String,
    pub day_index: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteWorkoutRequest {
    pub workout_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListExerciseLibraryRequest {
    pub search: Option<String>,
    pub body_part: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListProgramScheduleRequest {
    pub program_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListWorkoutLibraryRequest {
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatientProgressRequest {
    pub patient_id: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RemoveExerciseFromWorkoutRequest {
    pub workout_id: String,
    pub exercise_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RestoreExerciseRequest {
    pub exercise_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SoftDeleteExerciseRequest {
    pub exercise_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateExerciseRequest {
    pub exercise_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub sets: Option<i32>,
    pub reps: Option<i32>,
    pub target_body_part: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateWorkoutRequest {
    pub workout_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateWorkoutExerciseRequest {
    pub workout_id: String,
    pub exercise_id: String,
    pub sets: Option<i32>,
    pub reps: Option<i32>,
    pub order_index: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkoutEditorDataRequest {
    pub workout_id: String,
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

impl LoginResponse {
    pub fn is_login_as_specialist(&self) -> bool {
        self.user_profile_type == "specialist"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialistPatientResponse {
    pub patient_id: String,
    pub email: String,
    pub full_name: String,
    pub assigned_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub sets: i32,
    pub reps: i32,
    pub target_body_part: Option<String>,
    pub deleted_at: Option<String>,
    pub exercise: ExerciseBasicResponse,
    pub order_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseBasicResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramScheduleItemResponse {
    pub program_id: String,
    pub day_index: i32,
    pub workout_id: Option<String>,
    pub is_rest_day: bool,
    pub id: String,
    pub days_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub exercises: Vec<ExerciseResponse>,
    pub workout: Option<WorkoutBasicResponse>,
    pub library: Vec<ExerciseLibraryItemResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutBasicResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

impl Default for WorkoutResponse {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: None,
            exercises: vec![],
            workout: None,
            library: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientWithProfileResponse {
    pub patient_id: String,
    pub email: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub program_name: Option<String>,
    pub progress_percent: Option<i32>,
    pub links: Vec<PatientLinkResponse>,
    pub profiles: Vec<PatientProfileResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientLinkResponse {
    pub link_id: String,
    pub patient_id: String,
    pub assigned_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientProfileResponse {
    pub patient_id: String,
    pub profile_id: String,
    pub full_name: Option<String>,
    pub email: Option<String>,
}

impl Default for PatientProfileResponse {
    fn default() -> Self {
        Self {
            patient_id: String::new(),
            profile_id: String::new(),
            full_name: None,
            email: None,
        }
    }
}

impl Default for PatientWithProfileResponse {
    fn default() -> Self {
        Self {
            patient_id: String::new(),
            email: String::new(),
            full_name: None,
            phone: None,
            program_name: None,
            progress_percent: None,
            links: vec![],
            profiles: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialistPatientLinkResponse {
    pub link_id: String,
    pub patient_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientProfileSummaryResponse {
    pub patient_id: String,
    pub full_name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialistProgramsDataResponse {
    pub programs: Vec<ProgramResponse>,
    pub patients_count: i32,
    pub links: Vec<SpecialistPatientLinkResponse>,
    pub profiles: Vec<PatientProfileSummaryResponse>,
    pub assignments: Vec<ProgramAssignmentResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramAssignmentResponse {
    pub program_id: String,
    pub patient_email: String,
    pub patient_id: Option<String>,
}

impl Default for SpecialistProgramsDataResponse {
    fn default() -> Self {
        Self {
            programs: vec![],
            patients_count: 0,
            links: vec![],
            profiles: vec![],
            assignments: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseLibraryItemResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub target_body_part: Option<String>,
    pub deleted_at: Option<String>,
}

impl Default for ExerciseLibraryItemResponse {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: None,
            video_url: None,
            target_body_part: None,
            deleted_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramScheduleDataResponse {
    pub program_id: String,
    pub items: Vec<ProgramScheduleItemResponse>,
    pub schedule: Vec<ProgramScheduleItemResponse>,
    pub workouts: Vec<WorkoutListResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutListResponse {
    pub id: String,
    pub name: String,
}

impl Default for ProgramScheduleDataResponse {
    fn default() -> Self {
        Self {
            program_id: String::new(),
            items: vec![],
            schedule: vec![],
            workouts: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnassignedPatientResponse {
    pub email: String,
    pub full_name: Option<String>,
}

impl Default for UnassignedPatientResponse {
    fn default() -> Self {
        Self {
            email: String::new(),
            full_name: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnassignedPatientsResponse {
    pub patients: Vec<UnassignedPatientResponse>,
}

impl Default for UnassignedPatientsResponse {
    fn default() -> Self {
        Self { patients: vec![] }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutLibraryItemResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub exercises_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientProgressResponse {
    pub full_name: String,
    pub email: String,
    pub programs: Vec<PatientProgressProgramResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientProgressProgramResponse {
    pub program_name: String,
    pub program_description: Option<String>,
    pub assignment_status: String,
    pub sessions_count: i32,
}

impl Default for PatientProgressResponse {
    fn default() -> Self {
        Self {
            full_name: String::new(),
            email: String::new(),
            programs: vec![],
        }
    }
}

#[derive(Clone)]
pub struct BackofficeClient {
    config: ApiConfig,
    http_client: PlatformsHttpClient,
}

impl BackofficeClient {
    pub fn new(config: ApiConfig, http_client: PlatformsHttpClient) -> Self {
        Self {
            config,
            http_client,
        }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<LoginResponse> {
        let url = format!("{}{}/login", self.config.base_url, API_BASE_PATH);
        let request = LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        };
        let body =
            serde_json::to_vec(&request).map_err(|e| ApplicationError::internal(e.to_string()))?;
        let response = self
            .http_client
            .post(&url, Some(&body))
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        let api_response: ApiResponse<LoginResponse> = serde_json::from_slice(&response.body)
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        Ok(api_response.data)
    }

    pub async fn add_specialist_patient(
        &self,
        patient_email: &str,
    ) -> Result<SpecialistPatientResponse> {
        let url = format!(
            "{}{}/add-specialist-patient",
            self.config.base_url, API_BASE_PATH
        );
        let request = AddSpecialistPatientRequest {
            patient_email: patient_email.to_string(),
        };
        let body =
            serde_json::to_vec(&request).map_err(|e| ApplicationError::internal(e.to_string()))?;
        let response = self
            .http_client
            .post(&url, Some(&body))
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        let api_response: ApiResponse<SpecialistPatientResponse> =
            serde_json::from_slice(&response.body)
                .map_err(|e| ApplicationError::internal(e.to_string()))?;
        Ok(api_response.data)
    }

    pub async fn add_exercise_to_workout(&self, workout_id: &str, exercise_id: &str) -> Result<()> {
        let url = format!(
            "{}{}/add-exercise-to-workout",
            self.config.base_url, API_BASE_PATH
        );
        let request = AddExerciseToWorkoutRequest {
            workout_id: workout_id.to_string(),
            exercise_id: exercise_id.to_string(),
        };
        let body =
            serde_json::to_vec(&request).map_err(|e| ApplicationError::internal(e.to_string()))?;
        let response = self
            .http_client
            .post(&url, Some(&body))
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        Ok(())
    }

    pub async fn assign_program_to_patient(
        &self,
        patient_email: &str,
        program_id: &str,
    ) -> Result<()> {
        let url = format!(
            "{}{}/assign-program-to-patient",
            self.config.base_url, API_BASE_PATH
        );
        let request = AssignProgramToPatientRequest {
            patient_email: patient_email.to_string(),
            program_id: program_id.to_string(),
        };
        let body =
            serde_json::to_vec(&request).map_err(|e| ApplicationError::internal(e.to_string()))?;
        let response = self
            .http_client
            .post(&url, Some(&body))
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        Ok(())
    }

    pub async fn create_exercise(
        &self,
        name: &str,
        description: Option<&str>,
        video_url: Option<&str>,
        sets: i32,
        reps: i32,
        target_body_part: Option<&str>,
    ) -> Result<ExerciseResponse> {
        let url = format!("{}{}/create-exercise", self.config.base_url, API_BASE_PATH);
        let request = CreateExerciseRequest {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            video_url: video_url.map(|s| s.to_string()),
            sets,
            reps,
            target_body_part: target_body_part.map(|s| s.to_string()),
        };
        let body =
            serde_json::to_vec(&request).map_err(|e| ApplicationError::internal(e.to_string()))?;
        let response = self
            .http_client
            .post(&url, Some(&body))
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        let api_response: ApiResponse<ExerciseResponse> = serde_json::from_slice(&response.body)
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        Ok(api_response.data)
    }

    pub async fn create_program(
        &self,
        name: &str,
        description: Option<&str>,
    ) -> Result<ProgramResponse> {
        let url = format!("{}{}/create-program", self.config.base_url, API_BASE_PATH);
        let request = CreateProgramRequest {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
        };
        let body =
            serde_json::to_vec(&request).map_err(|e| ApplicationError::internal(e.to_string()))?;
        let response = self
            .http_client
            .post(&url, Some(&body))
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        let api_response: ApiResponse<ProgramResponse> = serde_json::from_slice(&response.body)
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        Ok(api_response.data)
    }

    pub async fn create_program_schedule_item(
        &self,
        program_id: &str,
        day_index: i32,
        workout_id: Option<&str>,
        is_rest_day: bool,
    ) -> Result<ProgramScheduleItemResponse> {
        let url = format!(
            "{}{}/create-program-schedule-item",
            self.config.base_url, API_BASE_PATH
        );
        let request = CreateProgramScheduleItemRequest {
            program_id: program_id.to_string(),
            day_index,
            workout_id: workout_id.map(|s| s.to_string()),
            is_rest_day,
        };
        let body =
            serde_json::to_vec(&request).map_err(|e| ApplicationError::internal(e.to_string()))?;
        let response = self
            .http_client
            .post(&url, Some(&body))
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        let api_response: ApiResponse<ProgramScheduleItemResponse> =
            serde_json::from_slice(&response.body)
                .map_err(|e| ApplicationError::internal(e.to_string()))?;
        Ok(api_response.data)
    }

    pub async fn create_workout(
        &self,
        name: &str,
        description: Option<&str>,
        exercise_ids: Vec<String>,
    ) -> Result<WorkoutResponse> {
        let url = format!("{}{}/create-workout", self.config.base_url, API_BASE_PATH);
        let request = CreateWorkoutRequest {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            exercise_ids,
        };
        let body =
            serde_json::to_vec(&request).map_err(|e| ApplicationError::internal(e.to_string()))?;
        let response = self
            .http_client
            .post(&url, Some(&body))
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        let api_response: ApiResponse<WorkoutResponse> = serde_json::from_slice(&response.body)
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        Ok(api_response.data)
    }

    pub async fn delete_program_schedule_item(
        &self,
        program_id: &str,
        day_index: i32,
    ) -> Result<()> {
        let url = format!(
            "{}{}/delete-program-schedule-item",
            self.config.base_url, API_BASE_PATH
        );
        let request = DeleteProgramScheduleItemRequest {
            program_id: program_id.to_string(),
            day_index,
        };
        let body =
            serde_json::to_vec(&request).map_err(|e| ApplicationError::internal(e.to_string()))?;
        let response = self
            .http_client
            .post(&url, Some(&body))
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        Ok(())
    }

    pub async fn delete_workout(&self, workout_id: &str) -> Result<()> {
        let url = format!("{}{}/delete-workout", self.config.base_url, API_BASE_PATH);
        let request = DeleteWorkoutRequest {
            workout_id: workout_id.to_string(),
        };
        let body =
            serde_json::to_vec(&request).map_err(|e| ApplicationError::internal(e.to_string()))?;
        let response = self
            .http_client
            .post(&url, Some(&body))
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        Ok(())
    }

    pub async fn get_specialist_patients_with_profiles(
        &self,
    ) -> Result<Vec<PatientWithProfileResponse>> {
        let url = format!(
            "{}{}/get-specialist-patients-with-profiles",
            self.config.base_url, API_BASE_PATH
        );

        let response = self
            .http_client
            .get(&url)
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        let api_response: ApiResponse<Vec<PatientWithProfileResponse>> =
            serde_json::from_slice(&response.body)
                .map_err(|e| ApplicationError::internal(e.to_string()))?;
        Ok(api_response.data)
    }

    pub async fn specialist_programs_data(&self) -> Result<SpecialistProgramsDataResponse> {
        let url = format!(
            "{}{}/specialist-programs-data",
            self.config.base_url, API_BASE_PATH
        );
        let response = self
            .http_client
            .get(&url)
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        let api_response: ApiResponse<SpecialistProgramsDataResponse> =
            serde_json::from_slice(&response.body)
                .map_err(|e| ApplicationError::internal(e.to_string()))?;
        Ok(api_response.data)
    }

    pub async fn list_exercise_library(
        &self,
        search: Option<&str>,
    ) -> Result<Vec<ExerciseLibraryItemResponse>> {
        let url = format!(
            "{}{}/list-exercise-library?name_filter={}",
            self.config.base_url,
            API_BASE_PATH,
            search.unwrap_or("")
        );
        let response = self
            .http_client
            .get(&url)
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        let api_response: ApiResponse<Vec<ExerciseLibraryItemResponse>> =
            serde_json::from_slice(&response.body)
                .map_err(|e| ApplicationError::internal(e.to_string()))?;
        Ok(api_response.data)
    }

    pub async fn list_program_schedule(
        &self,
        program_id: &str,
    ) -> Result<ProgramScheduleDataResponse> {
        let url = format!(
            "{}{}/list-program-schedule?program_id={}",
            self.config.base_url, API_BASE_PATH, program_id
        );
        let response = self
            .http_client
            .get(&url)
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        let api_response: ApiResponse<ProgramScheduleDataResponse> =
            serde_json::from_slice(&response.body)
                .map_err(|e| ApplicationError::internal(e.to_string()))?;
        Ok(api_response.data)
    }

    pub async fn list_unassigned_patients(&self) -> Result<UnassignedPatientsResponse> {
        let url = format!(
            "{}{}/list-unassigned-patients",
            self.config.base_url, API_BASE_PATH
        );
        let response = self
            .http_client
            .get(&url)
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        let api_response: ApiResponse<UnassignedPatientsResponse> =
            serde_json::from_slice(&response.body)
                .map_err(|e| ApplicationError::internal(e.to_string()))?;
        Ok(api_response.data)
    }

    pub async fn list_workout_library(
        &self,
        search: Option<&str>,
    ) -> Result<Vec<WorkoutLibraryItemResponse>> {
        let url = format!(
            "{}{}/list-workout-library?name_filter={}",
            self.config.base_url,
            API_BASE_PATH,
            search.unwrap_or("")
        );
        let response = self
            .http_client
            .get(&url)
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        let api_response: ApiResponse<Vec<WorkoutLibraryItemResponse>> =
            serde_json::from_slice(&response.body)
                .map_err(|e| ApplicationError::internal(e.to_string()))?;
        Ok(api_response.data)
    }

    pub async fn patient_progress(
        &self,
        patient_id: &str,
        start_date: &str,
        end_date: &str,
    ) -> Result<PatientProgressResponse> {
        let url = format!(
            "{}{}/patient-progress?patient_id={}&start_date={}&end_date={}",
            self.config.base_url, API_BASE_PATH, patient_id, start_date, end_date
        );
        let response = self
            .http_client
            .get(&url)
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        let api_response: ApiResponse<PatientProgressResponse> =
            serde_json::from_slice(&response.body)
                .map_err(|e| ApplicationError::internal(e.to_string()))?;
        Ok(api_response.data)
    }

    pub async fn remove_exercise_from_workout(
        &self,
        workout_id: &str,
        exercise_id: &str,
    ) -> Result<()> {
        let url = format!(
            "{}{}/remove-exercise-from-workout",
            self.config.base_url, API_BASE_PATH
        );
        let request = RemoveExerciseFromWorkoutRequest {
            workout_id: workout_id.to_string(),
            exercise_id: exercise_id.to_string(),
        };
        let body =
            serde_json::to_vec(&request).map_err(|e| ApplicationError::internal(e.to_string()))?;
        let response = self
            .http_client
            .post(&url, Some(&body))
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        Ok(())
    }

    pub async fn restore_exercise(&self, exercise_id: &str) -> Result<()> {
        let url = format!("{}{}/restore-exercise", self.config.base_url, API_BASE_PATH);
        let request = RestoreExerciseRequest {
            exercise_id: exercise_id.to_string(),
        };
        let body =
            serde_json::to_vec(&request).map_err(|e| ApplicationError::internal(e.to_string()))?;
        let response = self
            .http_client
            .post(&url, Some(&body))
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        Ok(())
    }

    pub async fn soft_delete_exercise(&self, exercise_id: &str) -> Result<()> {
        let url = format!(
            "{}{}/soft-delete-exercise",
            self.config.base_url, API_BASE_PATH
        );
        let request = SoftDeleteExerciseRequest {
            exercise_id: exercise_id.to_string(),
        };
        let body =
            serde_json::to_vec(&request).map_err(|e| ApplicationError::internal(e.to_string()))?;
        let response = self
            .http_client
            .post(&url, Some(&body))
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        Ok(())
    }

    pub async fn update_exercise(
        &self,
        exercise_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        video_url: Option<&str>,
        sets: Option<i32>,
        reps: Option<i32>,
        target_body_part: Option<&str>,
    ) -> Result<()> {
        let url = format!("{}{}/update-exercise", self.config.base_url, API_BASE_PATH);
        let request = UpdateExerciseRequest {
            exercise_id: exercise_id.to_string(),
            name: name.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
            video_url: video_url.map(|s| s.to_string()),
            sets,
            reps,
            target_body_part: target_body_part.map(|s| s.to_string()),
        };
        let body =
            serde_json::to_vec(&request).map_err(|e| ApplicationError::internal(e.to_string()))?;
        let response = self
            .http_client
            .patch(&url, &body)
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        Ok(())
    }

    pub async fn update_workout(
        &self,
        workout_id: &str,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Result<()> {
        let url = format!("{}{}/update-workout", self.config.base_url, API_BASE_PATH);
        let request = UpdateWorkoutRequest {
            workout_id: workout_id.to_string(),
            name: name.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
        };
        let body =
            serde_json::to_vec(&request).map_err(|e| ApplicationError::internal(e.to_string()))?;
        let response = self
            .http_client
            .patch(&url, &body)
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        Ok(())
    }

    pub async fn update_workout_exercise(
        &self,
        workout_id: &str,
        exercise_id: &str,
        sets: Option<i32>,
        reps: Option<i32>,
        order_index: Option<i32>,
    ) -> Result<()> {
        let url = format!(
            "{}{}/update-workout-exercise",
            self.config.base_url, API_BASE_PATH
        );
        let request = UpdateWorkoutExerciseRequest {
            workout_id: workout_id.to_string(),
            exercise_id: exercise_id.to_string(),
            sets,
            reps,
            order_index,
        };
        let body =
            serde_json::to_vec(&request).map_err(|e| ApplicationError::internal(e.to_string()))?;
        let response = self
            .http_client
            .patch(&url, &body)
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        Ok(())
    }

    pub async fn workout_editor_data(&self, workout_id: &str) -> Result<WorkoutResponse> {
        let url = format!(
            "{}{}/workout-editor-data?workout_id={}",
            self.config.base_url, API_BASE_PATH, workout_id
        );
        let response = self
            .http_client
            .get(&url)
            .await
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        if response.status < 200 || response.status >= 300 {
            let body_str = String::from_utf8_lossy(&response.body);
            return Err(ApplicationError::api(format!(
                "API error {}: {}",
                response.status, body_str
            )));
        }
        let api_response: ApiResponse<WorkoutResponse> = serde_json::from_slice(&response.body)
            .map_err(|e| ApplicationError::internal(e.to_string()))?;
        Ok(api_response.data)
    }
}

#[common::async_trait_platform]
impl BackofficeApi for BackofficeClient {
    async fn login(&self, args: LoginArgs) -> Result<LoginResult> {
        let response = self.login(&args.email, &args.password).await?;
        Ok(LoginResult {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            user_id: response.user_id,
            expires_at: None,
            user_profile_type: response.user_profile_type,
        })
    }

    async fn add_specialist_patient(
        &self,
        args: AddSpecialistPatientArgs,
    ) -> Result<AddSpecialistPatientResult> {
        let response = self.add_specialist_patient(&args.patient_email).await?;
        Ok(AddSpecialistPatientResult {
            id: response.patient_id.clone(),
            specialist_id: String::new(),
            patient_id: response.patient_id,
            created_at: response.assigned_at,
        })
    }

    async fn add_exercise_to_workout(&self, args: AddExerciseToWorkoutArgs) -> Result<()> {
        self.add_exercise_to_workout(&args.workout_id, &args.exercise_id)
            .await
    }

    async fn assign_program_to_patient(
        &self,
        args: AssignProgramToPatientArgs,
    ) -> Result<AssignProgramToPatientResult> {
        self.assign_program_to_patient(&args.patient_id, &args.program_id)
            .await?;
        Ok(AssignProgramToPatientResult {
            id: String::new(),
            patient_id: args.patient_id,
            program_id: args.program_id,
            status: "active".to_string(),
        })
    }

    async fn create_exercise(&self, args: CreateExerciseArgs) -> Result<CreateExerciseResult> {
        let response = self
            .create_exercise(
                &args.name,
                args.description.as_deref(),
                args.video_url.as_deref(),
                args.order_index,
                0,
                None,
            )
            .await?;
        Ok(CreateExerciseResult {
            id: response.id,
            name: response.name,
            description: response.description,
            order_index: response.order_index,
            video_url: response.video_url,
            deleted_at: response.deleted_at,
        })
    }

    async fn create_program(&self, args: CreateProgramArgs) -> Result<CreateProgramResult> {
        let response = self
            .create_program(&args.name, args.description.as_deref())
            .await?;
        Ok(CreateProgramResult {
            id: response.id,
            name: response.name,
            description: response.description,
        })
    }

    async fn create_program_schedule_item(
        &self,
        args: CreateProgramScheduleItemArgs,
    ) -> Result<CreateProgramScheduleItemResult> {
        let response = self
            .create_program_schedule_item(
                &args.program_id,
                args.order_index,
                args.workout_id.as_deref(),
                false,
            )
            .await?;
        Ok(CreateProgramScheduleItemResult {
            id: response.id,
            program_id: response.program_id,
            order_index: response.day_index,
            workout_id: response.workout_id,
            days_count: response.days_count,
        })
    }

    async fn create_workout(&self, args: CreateWorkoutArgs) -> Result<CreateWorkoutResult> {
        let response = self
            .create_workout(&args.name, args.description.as_deref(), vec![])
            .await?;
        Ok(CreateWorkoutResult {
            id: response.id,
            name: response.name,
            description: response.description,
            order_index: 0,
        })
    }

    async fn delete_program_schedule_item(
        &self,
        args: DeleteProgramScheduleItemArgs,
    ) -> Result<()> {
        self.delete_program_schedule_item(&args.schedule_item_id, 0)
            .await
    }

    async fn delete_workout(&self, args: DeleteWorkoutArgs) -> Result<()> {
        self.delete_workout(&args.workout_id).await
    }

    async fn get_specialist_patients_with_profiles(
        &self,
        _args: GetSpecialistPatientsWithProfilesArgs,
    ) -> Result<GetSpecialistPatientsWithProfilesResult> {
        let response = self.get_specialist_patients_with_profiles().await?;
        let links: Vec<SpecialistPatientLink> = response
            .iter()
            .flat_map(|p| {
                p.links.iter().map(|l| SpecialistPatientLink {
                    link_id: l.link_id.clone(),
                    patient_id: l.patient_id.clone(),
                })
            })
            .collect();
        let profiles: Vec<PatientProfileSummary> = response
            .iter()
            .flat_map(|p| {
                p.profiles.iter().map(|pr| PatientProfileSummary {
                    patient_id: pr.patient_id.clone(),
                    full_name: pr.full_name.clone().unwrap_or_default(),
                    email: pr.email.clone().unwrap_or_default(),
                })
            })
            .collect();
        Ok(GetSpecialistPatientsWithProfilesResult { links, profiles })
    }

    async fn specialist_programs_data(&self) -> Result<SpecialistProgramsDataResult> {
        let response = self.specialist_programs_data().await?;
        let links: Vec<SpecialistPatientLink> = response
            .links
            .iter()
            .map(|l| SpecialistPatientLink {
                link_id: l.link_id.clone(),
                patient_id: l.patient_id.clone(),
            })
            .collect();
        let profiles: Vec<PatientProfileSummary> = response
            .profiles
            .iter()
            .map(|p| PatientProfileSummary {
                patient_id: p.patient_id.clone(),
                full_name: p.full_name.clone(),
                email: p.email.clone(),
            })
            .collect();
        let programs: Vec<ProgramSummary> = response
            .programs
            .iter()
            .map(|p| ProgramSummary {
                id: p.id.clone(),
                name: p.name.clone(),
                description: p.description.clone(),
            })
            .collect();
        let assignments: Vec<PatientProgramAssignment> = response
            .assignments
            .iter()
            .map(|a| PatientProgramAssignment {
                id: a.patient_id.clone().unwrap_or_default(),
                patient_id: a.patient_id.clone().unwrap_or_default(),
                program_id: a.program_id.clone(),
                status: "active".to_string(),
            })
            .collect();
        Ok(SpecialistProgramsDataResult {
            links,
            profiles,
            programs,
            assignments,
        })
    }

    async fn list_exercise_library(
        &self,
        args: ListExerciseLibraryArgs,
    ) -> Result<ListExerciseLibraryResult> {
        let response = self
            .list_exercise_library(args.name_filter.as_deref())
            .await?;
        let items: Vec<ExerciseLibraryItem> = response
            .iter()
            .map(|e| ExerciseLibraryItem {
                id: e.id.clone(),
                name: e.name.clone(),
                description: e.description.clone(),
                order_index: 0,
                video_url: e.video_url.clone(),
                deleted_at: e.deleted_at.clone(),
            })
            .collect();
        Ok(ListExerciseLibraryResult { items })
    }

    async fn list_program_schedule(
        &self,
        args: ListProgramScheduleArgs,
    ) -> Result<ListProgramScheduleResult> {
        let response = self.list_program_schedule(&args.program_id).await?;
        let schedule: Vec<ProgramScheduleEntry> = response
            .schedule
            .iter()
            .map(|s| ProgramScheduleEntry {
                id: s.id.clone(),
                order_index: s.day_index,
                workout_id: s.workout_id.clone(),
                days_count: s.days_count,
            })
            .collect();
        let workouts: Vec<WorkoutList> = response
            .workouts
            .iter()
            .map(|w| WorkoutList {
                id: w.id.clone(),
                name: w.name.clone(),
            })
            .collect();
        Ok(ListProgramScheduleResult { schedule, workouts })
    }

    async fn list_workout_library(
        &self,
        args: ListWorkoutLibraryArgs,
    ) -> Result<ListWorkoutLibraryResult> {
        let response = self
            .list_workout_library(args.name_filter.as_deref())
            .await?;
        let items: Vec<WorkoutLibraryItem> = response
            .iter()
            .map(|w| WorkoutLibraryItem {
                id: w.id.clone(),
                name: w.name.clone(),
                description: w.description.clone(),
                order_index: w.exercises_count,
            })
            .collect();
        Ok(ListWorkoutLibraryResult { items })
    }

    async fn list_unassigned_patients(
        &self,
        _args: ListUnassignedPatientsArgs,
    ) -> Result<ListUnassignedPatientsResult> {
        let response = self.list_unassigned_patients().await?;
        let patients: Vec<UnassignedPatient> = response
            .patients
            .iter()
            .map(|p| UnassignedPatient {
                patient_id: p.email.clone(),
                email: p.email.clone(),
                full_name: p.full_name.clone().unwrap_or_default(),
            })
            .collect();
        Ok(ListUnassignedPatientsResult { patients })
    }

    async fn patient_progress(&self, args: PatientProgressArgs) -> Result<PatientProgressResult> {
        let response = self
            .patient_progress(&args.patient_id, "", "")
            .await?;
        Ok(PatientProgressResult {
            profile: PatientProgressProfile {
                full_name: response.full_name,
                email: response.email,
            },
            programs_with_sessions: vec![],
        })
    }

    async fn remove_exercise_from_workout(
        &self,
        args: RemoveExerciseFromWorkoutArgs,
    ) -> Result<()> {
        self.remove_exercise_from_workout(&args.workout_id, &args.exercise_id)
            .await
    }

    async fn restore_exercise(&self, args: RestoreExerciseArgs) -> Result<()> {
        self.restore_exercise(&args.exercise_id).await
    }

    async fn soft_delete_exercise(&self, args: SoftDeleteExerciseArgs) -> Result<()> {
        self.soft_delete_exercise(&args.exercise_id).await
    }

    async fn update_exercise(&self, args: UpdateExerciseArgs) -> Result<()> {
        self.update_exercise(
            &args.exercise_id,
            args.name.as_deref(),
            args.description.as_deref(),
            args.video_url.as_deref(),
            None,
            None,
            None,
        )
        .await
    }

    async fn update_workout(&self, args: UpdateWorkoutArgs) -> Result<()> {
        self.update_workout(
            &args.workout_id,
            args.name.as_deref(),
            args.description.as_deref(),
        )
        .await
    }

    async fn update_workout_exercise(&self, args: UpdateWorkoutExerciseArgs) -> Result<()> {
        self.update_workout_exercise(
            &args.workout_id,
            &args.exercise_id,
            Some(args.sets),
            Some(args.reps),
            args.order_index,
        )
        .await
    }

    async fn workout_editor_data(
        &self,
        args: WorkoutEditorDataArgs,
    ) -> Result<WorkoutEditorDataResult> {
        let response = self.workout_editor_data(&args.workout_id).await?;
        let workout = response.workout.map(|w| WorkoutEditorWorkout {
            id: w.id,
            name: w.name,
            description: w.description,
            order_index: 0,
        });
        let exercises: Vec<WorkoutEditorLine> = response
            .exercises
            .iter()
            .map(|e| WorkoutEditorLine {
                exercise: WorkoutEditorExerciseItem {
                    id: e.exercise.id.clone(),
                    name: e.exercise.name.clone(),
                    description: e.exercise.description.clone(),
                    order_index: e.order_index,
                    video_url: e.video_url.clone(),
                    deleted_at: e.exercise.deleted_at.clone(),
                },
                order_index: e.order_index,
                sets: e.sets,
                reps: e.reps,
            })
            .collect();
        let library: Vec<WorkoutEditorExerciseItem> = response
            .library
            .iter()
            .map(|l| WorkoutEditorExerciseItem {
                id: l.id.clone(),
                name: l.name.clone(),
                description: l.description.clone(),
                order_index: 0,
                video_url: l.video_url.clone(),
                deleted_at: l.deleted_at.clone(),
            })
            .collect();
        Ok(WorkoutEditorDataResult {
            workout,
            exercises,
            library,
        })
    }
}
