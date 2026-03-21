use std::sync::Arc;

use futures::stream::{self, StreamExt};
use futures::try_join;

use crate::ports::Backend;
use domain::aggregates::PatientProgramFull;
use domain::entities::{
    PatientProgram, Program, ProgramScheduleItem, SessionExerciseFeedback, Workout, WorkoutSession,
};
use domain::error::Result;
use domain::vos::profile::Profile;

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

impl From<PatientProgramFull> for ProgramWithSessions {
    fn from(full: PatientProgramFull) -> Self {
        ProgramWithSessions {
            program: full.program,
            assignment: full.patient_program,
            sessions: full.sessions,
            program_feedback: full.feedback,
            schedule: full.schedule,
            workouts: full.workouts.into_iter().map(|w| w.workout).collect(),
        }
    }
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
                    let full = backend.get_patient_program_full(&token, &ass.id).await?;

                    Ok(full.map(ProgramWithSessions::from))
                }
            })
            .buffer_unordered(Self::MAX_CONCURRENT_PROGRAM_REQUESTS)
            .collect::<Vec<Result<Option<ProgramWithSessions>>>>()
            .await
            .into_iter()
            .collect::<Result<Vec<Option<ProgramWithSessions>>>>()?
            .into_iter()
            .flatten()
            .collect();

        Ok(PatientProgressResult {
            profile,
            programs_with_sessions,
        })
    }
}
