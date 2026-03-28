use std::collections::HashMap;
use std::sync::Arc;

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};
use application::use_cases::submit_patient_workout_feedback::SubmitPatientWorkoutFeedbackArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseFeedbackRequest {
    pub exercise_id: String,
    pub effort: i32,
    pub pain: i32,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkDayAsCompletedRequest {
    pub patient_program_id: String,
    pub day_index: i32,
    pub session_date: String,
    pub feedback: Vec<ExerciseFeedbackRequest>,
}

pub async fn mark_day_as_completed(
    State(state): State<Arc<AppState>>,
    Json(request): Json<MarkDayAsCompletedRequest>,
) -> Result<APIResponse<()>> {
    let feedback_map: HashMap<String, (i32, i32, String)> = request
        .feedback
        .into_iter()
        .map(|f| {
            (
                f.exercise_id,
                (f.effort, f.pain, f.comment.unwrap_or_default()),
            )
        })
        .collect();

    state
        .facade()
        .submit_patient_workout_feedback(SubmitPatientWorkoutFeedbackArgs {
            patient_program_id: request.patient_program_id,
            day_index: request.day_index,
            session_date: request.session_date,
            feedback_map,
            completion_status: None,
        })
        .await
        .map_err(Error::from)?;

    Ok(APIResponse::ok(()))
}
