use std::sync::Arc;

use futures::stream::{self, StreamExt};
use futures::try_join;

use crate::use_cases::agenda_schedule::{
    AgendaSessionFeedback, AgendaWorkoutSession, ProgramScheduleRow, WorkoutSummaryRow,
};
use domain::aggregates::PatientProgramFull;
use domain::entities::PatientProgram;
use domain::error::DomainError;
use domain::error::Result;
use domain::repositories::{
    GetPatientProgramFullRead, GetProfilesByIdsRead, ListPatientProgramsForSpecialistRead,
};
use domain::vos::id::Id;
use domain::vos::profile::Profile;
use domain::vos::AccessToken;

#[derive(Clone)]
pub struct PatientProgressArgs {
    pub token: String,
    pub patient_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatientProgressProfile {
    pub full_name: String,
    pub email: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatientProgressProgramBlock {
    pub program_name: String,
    pub program_description: Option<String>,
    pub assignment_status: String,
    pub sessions: Vec<AgendaWorkoutSession>,
    pub program_feedback: Vec<AgendaSessionFeedback>,
    pub schedule: Vec<ProgramScheduleRow>,
    pub workouts: Vec<WorkoutSummaryRow>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatientProgressResult {
    pub profile: PatientProgressProfile,
    pub programs_with_sessions: Vec<PatientProgressProgramBlock>,
}

pub struct PatientProgressUseCase<
    R: GetProfilesByIdsRead + ListPatientProgramsForSpecialistRead + GetPatientProgramFullRead,
> {
    catalog_read: Arc<R>,
}

impl<
        R: GetProfilesByIdsRead + ListPatientProgramsForSpecialistRead + GetPatientProgramFullRead,
    > PatientProgressUseCase<R>
{
    const MAX_CONCURRENT_PROGRAM_REQUESTS: usize = 4;

    pub fn new(catalog_read: Arc<R>) -> Self {
        Self { catalog_read }
    }

    pub async fn execute(&self, args: PatientProgressArgs) -> Result<PatientProgressResult> {
        let access = AccessToken::try_from(args.token)?;
        let patient_id = Id::try_from(args.patient_id)?;
        let profile_ids: [Id; 1] = [patient_id.clone()];

        let (profiles, all_assignments) = try_join!(
            self.catalog_read.get_profiles_by_ids(&profile_ids, &access),
            self.catalog_read
                .list_patient_programs_for_specialist(&access),
        )?;

        let profile_domain = profiles
            .into_iter()
            .next()
            .ok_or_else(|| DomainError::Api("patient_not_found".into()))?;

        let profile = map_profile(&profile_domain);

        let assignments: Vec<PatientProgram> = all_assignments
            .into_iter()
            .filter(|a| a.patient_id == patient_id)
            .collect();

        let programs_with_sessions: Vec<PatientProgressProgramBlock> = stream::iter(assignments)
            .map(|ass| {
                let catalog_read = self.catalog_read.clone();
                let access = access.clone();

                async move {
                    let full = catalog_read
                        .get_patient_program_full(&access, &ass.id)
                        .await?;

                    Ok(full.map(map_program_block))
                }
            })
            .buffer_unordered(Self::MAX_CONCURRENT_PROGRAM_REQUESTS)
            .collect::<Vec<Result<Option<PatientProgressProgramBlock>>>>()
            .await
            .into_iter()
            .collect::<Result<Vec<Option<PatientProgressProgramBlock>>>>()?
            .into_iter()
            .flatten()
            .collect();

        Ok(PatientProgressResult {
            profile,
            programs_with_sessions,
        })
    }
}

fn map_profile(p: &Profile) -> PatientProgressProfile {
    PatientProgressProfile {
        full_name: p.full_name().value().to_string(),
        email: p.email().to_string(),
    }
}

fn map_program_block(full: PatientProgramFull) -> PatientProgressProgramBlock {
    PatientProgressProgramBlock {
        program_name: full.program.name,
        program_description: full.program.description,
        assignment_status: full.patient_program.status,
        sessions: full
            .sessions
            .into_iter()
            .map(|s| AgendaWorkoutSession {
                id: s.id.to_string(),
                day_index: s.day_index,
                session_date: s.session_date,
                completed_at: s.completed_at,
            })
            .collect(),
        program_feedback: full
            .feedback
            .into_iter()
            .map(|f| AgendaSessionFeedback {
                workout_session_id: f.workout_session_id.to_string(),
                exercise_id: f.exercise_id.to_string(),
                effort: f.effort,
                pain: f.pain,
                comment: f.comment,
            })
            .collect(),
        schedule: full
            .schedule
            .into_iter()
            .map(|item| ProgramScheduleRow {
                workout_id: item.workout_id.map(|id| id.to_string()),
                days_count: item.days_count,
            })
            .collect(),
        workouts: full
            .workouts
            .into_iter()
            .map(|w| WorkoutSummaryRow {
                id: w.workout.id.to_string(),
                name: w.workout.name,
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::vos::email::Email;
    use domain::vos::fullname::FullName;
    use domain::vos::id::Id;
    use domain::vos::role::Role;

    #[test]
    fn map_profile_maps_full_name_and_email_to_strings() {
        let id = Id::try_from("00000000-0000-0000-0000-000000000001").unwrap();
        let email = Email::try_from("patient@example.com").unwrap();
        let full_name = FullName::try_from("Ada Lovelace").unwrap();
        let role = Role::try_from("patient").unwrap();
        let profile = Profile::new(id, email, full_name, role);

        let dto = map_profile(&profile);

        assert_eq!(dto.full_name, "Ada Lovelace");
        assert_eq!(dto.email, "patient@example.com");
    }

    #[tokio::test]
    async fn execute_rejects_empty_access_token() {
        use std::sync::Arc;

        use domain::error::DomainError;

        let catalog = crate::test_mocks::FakePatientProgressCatalog::default();
        let uc = PatientProgressUseCase::new(Arc::new(catalog));

        let err = uc
            .execute(PatientProgressArgs {
                token: "   ".to_string(),
                patient_id: "550e8400-e29b-41d4-a716-446655440110".to_string(),
            })
            .await
            .unwrap_err();

        assert!(matches!(err, DomainError::InvalidParameter(_, _)));
    }
}
