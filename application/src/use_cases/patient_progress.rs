use std::sync::Arc;

use futures::stream::{self, StreamExt};
use futures::try_join;

use crate::ports::Backend;
use domain::entities::{
    PatientProgram, Program, ProgramScheduleItem, SessionExerciseFeedback, Workout, WorkoutSession,
};
use domain::error::Result;
use domain::profile::Profile;

#[derive(Clone)]
pub struct PatientProgressArgs {
    pub token: String,
    pub patient_id: String,
}

#[derive(Clone, Debug)]
pub struct ProgramWithSessions {
    pub program: Program,
    pub assignment: PatientProgram,
    pub sessions: Vec<WorkoutSession>,
    pub program_feedback: Vec<SessionExerciseFeedback>,
    pub schedule: Vec<ProgramScheduleItem>,
    pub workouts: Vec<Workout>,
}

#[derive(Clone, Debug)]
pub struct PatientProgressResult {
    pub profile: Profile,
    pub programs_with_sessions: Vec<ProgramWithSessions>,
}

pub struct PatientProgressUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> PatientProgressUseCase<B> {
    const MAX_CONCURRENT_PROGRAM_REQUESTS: usize = 4;

    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: PatientProgressArgs) -> Result<PatientProgressResult> {
        let token = args.token.clone();
        let patient_id = args.patient_id.clone();
        let profile_ids: [String; 1] = [patient_id.clone()];

        let (profiles, all_assignments) = try_join!(
            self.backend.get_profiles_by_ids(&profile_ids, &token),
            self.backend.list_patient_programs_for_specialist(&token),
        )?;

        let profile = profiles
            .into_iter()
            .next()
            .ok_or_else(|| "patient_not_found".to_string())?;

        let assignments: Vec<PatientProgram> = all_assignments
            .into_iter()
            .filter(|a| a.patient_id == patient_id)
            .collect();

        let programs_with_sessions: Vec<ProgramWithSessions> = stream::iter(assignments)
            .map(|ass| {
                let backend = self.backend.clone();
                let token = token.clone();

                async move {
                    let (prog, sessions, program_feedback, workouts, schedule) = try_join!(
                        backend.get_program(&token, &ass.program_id),
                        backend.list_workout_sessions(&token, &ass.id),
                        backend.list_session_exercise_feedback_for_program(&token, &ass.id),
                        backend.list_workouts_for_program(&token, &ass.program_id),
                        backend.list_program_schedule(&token, &ass.program_id),
                    )?;

                    let program = match prog {
                        Some(p) => p,
                        None => return Ok(None),
                    };

                    Ok(Some(ProgramWithSessions {
                        program,
                        assignment: ass,
                        sessions,
                        program_feedback,
                        schedule,
                        workouts,
                    }))
                }
            })
            .buffer_unordered(Self::MAX_CONCURRENT_PROGRAM_REQUESTS)
            .collect::<Vec<Result<Option<ProgramWithSessions>>>>()
            .await
            .into_iter()
            .collect::<Result<Vec<Option<ProgramWithSessions>>>>()?
            .into_iter()
            .filter_map(|o| o)
            .collect();

        Ok(PatientProgressResult {
            profile,
            programs_with_sessions,
        })
    }
}
