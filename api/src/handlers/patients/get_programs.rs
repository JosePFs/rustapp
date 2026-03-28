use std::sync::Arc;

use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Result},
    router::api_response::APIResponse,
    state::AppState,
};

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

pub async fn get_programs(
    State(state): State<Arc<AppState>>,
) -> Result<APIResponse<Vec<PatientProgramResponse>>> {
    let result = state
        .facade()
        .get_patient_programs()
        .await
        .map_err(Error::from)?;

    let programs = result
        .patient_programs
        .into_iter()
        .map(|program| {
            let days = program
                .days
                .into_iter()
                .map(|day| ProgramDayResponse {
                    day_index: day.day_index,
                    day_number: day.day_number,
                    session_id: day.session_id,
                    workout_name: day.workout_name,
                    workout_description: day.workout_description,
                    is_rest_day: day.is_rest_day,
                    session_date: day.session_date,
                    completed_at: day.completed_at,
                    exercises: day
                        .exercises
                        .into_iter()
                        .map(|e| ExerciseInstructionResponse {
                            exercise_id: e.exercise_id,
                            name: e.name,
                            description: e.description,
                            video_url: e.video_url,
                            sets: e.sets,
                            reps: e.reps,
                            effort: e.effort,
                            pain: e.pain,
                            comment: e.comment,
                        })
                        .collect(),
                })
                .collect();

            PatientProgramResponse {
                patient_program_id: program.patient_program_id,
                program_id: program.program_id,
                program_name: program.program_name,
                program_description: program.program_description,
                days,
                progress_percent: program.progress_percent,
                average_effort: program.average_effort,
                average_pain: program.average_pain,
            }
        })
        .collect();

    Ok(APIResponse::ok(programs))
}
