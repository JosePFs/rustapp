use std::sync::Arc;

use futures::stream::{self, StreamExt};
use futures::try_join;

use crate::application::Backend;
use crate::domain::entities::{
    ProgramScheduleItem, SessionExerciseFeedback, Workout, WorkoutSession,
};
use crate::domain::error::Result;

pub struct GetPatientProgramsUseCaseArgs {
    pub token: String,
}

#[derive(Clone, PartialEq)]
pub struct PatientPrograms {
    pub patient_program_id: String,
    pub program_id: String,
    pub program_name: String,
    pub program_description: Option<String>,
    pub schedule: Vec<ProgramScheduleItem>,
    pub workouts: Vec<Workout>,
    pub sessions: Vec<WorkoutSession>,
    pub program_feedback: Vec<SessionExerciseFeedback>,
}

#[derive(Clone, PartialEq)]
pub struct GetPatientProgramsUseCaseResult {
    pub patient_programs: Vec<PatientPrograms>,
}

pub struct GetPatientProgramsUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> GetPatientProgramsUseCase<B> {
    const MAX_CONCURRENT_PROGRAM_REQUESTS: usize = 4;

    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(
        &self,
        args: GetPatientProgramsUseCaseArgs,
    ) -> Result<GetPatientProgramsUseCaseResult> {
        let patient_programs = self
            .backend
            .list_active_patient_programs(&args.token)
            .await?;

        let patient_programs_data = stream::iter(patient_programs.into_iter())
            .map(|ass| {
                let backend = self.backend.clone();
                let token = args.token.clone();

                async move {
                    let (prog, workouts, schedule, sessions, feedback) = try_join!(
                        backend.get_program(&token, &ass.program_id),
                        backend.list_workouts_for_program(&token, &ass.program_id),
                        backend.list_program_schedule(&token, &ass.program_id),
                        backend.list_workout_sessions(&token, &ass.id),
                        backend.list_session_exercise_feedback_for_program(&token, &ass.id),
                    )?;

                    let prog = match prog {
                        Some(p) => p,
                        None => return Ok(None),
                    };

                    Ok(Some(PatientPrograms {
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
            .buffer_unordered(Self::MAX_CONCURRENT_PROGRAM_REQUESTS)
            .collect::<Vec<Result<Option<PatientPrograms>>>>()
            .await;

        let patient_programs_result = GetPatientProgramsUseCaseResult {
            patient_programs: patient_programs_data
                .into_iter()
                .filter_map(|r| match r {
                    Ok(Some(v)) => Some(v),
                    _ => None,
                })
                .collect(),
        };

        Ok(patient_programs_result)
    }
}
