use std::sync::Arc;

use futures::stream::{self, StreamExt};
use futures::try_join;

use dioxus::prelude::*;

use crate::application::ports::Backend;
use crate::domain::entities::{
    ProgramScheduleItem, SessionExerciseFeedback, Workout, WorkoutSession,
};
use crate::domain::error::{DomainError, Result};
use crate::domain::session::Session;

const MAX_CONCURRENT_PROGRAM_REQUESTS: usize = 4;

#[derive(Clone, PartialEq)]
pub struct PatientProgramData {
    pub patient_program_id: String,
    pub program_id: String,
    pub program_name: String,
    pub program_description: Option<String>,
    pub schedule: Vec<ProgramScheduleItem>,
    pub workouts: Vec<Workout>,
    pub sessions: Vec<WorkoutSession>,
    pub program_feedback: Vec<SessionExerciseFeedback>,
}

pub fn use_patient_programs(
    app_session: Option<Session>,
    backend: Arc<dyn Backend>,
) -> Resource<Result<Vec<PatientProgramData>>> {
    use_resource(move || {
        let app_session = app_session.clone();
        let backend = backend.clone();

        async move {
            let sess = app_session.ok_or(DomainError::SessionNotFound)?;
            let token = sess.access_token();

            let patient_programs = backend.list_active_patient_programs(token).await?;

            let patient_programs_data = stream::iter(patient_programs.into_iter())
                .map(|ass| {
                    let backend = backend.clone();

                    async move {
                        let (prog, workouts, schedule, sessions, feedback) = try_join!(
                            backend.get_program(token, &ass.program_id),
                            backend.list_workouts_for_program(token, &ass.program_id),
                            backend.list_program_schedule(token, &ass.program_id),
                            backend.list_workout_sessions(token, &ass.id),
                            backend.list_session_exercise_feedback_for_program(token, &ass.id),
                        )?;

                        let prog = match prog {
                            Some(p) => p,
                            None => return Ok(None),
                        };

                        Ok(Some(PatientProgramData {
                            patient_program_id: ass.id.clone(),
                            program_id: ass.program_id.clone(),
                            program_name: prog.name,
                            program_description: prog.description,
                            schedule,
                            workouts,
                            sessions,
                            program_feedback: feedback,
                        }))
                    }
                })
                .buffer_unordered(MAX_CONCURRENT_PROGRAM_REQUESTS)
                .collect::<Vec<Result<Option<PatientProgramData>>>>()
                .await;

            let patient_programs_data: Vec<Option<PatientProgramData>> = patient_programs_data
                .into_iter()
                .filter_map(|r| match r {
                    Ok(v) => Some(v),
                    Err(_) => None,
                })
                .collect();

            Ok(patient_programs_data.into_iter().flatten().collect())
        }
    })
}
