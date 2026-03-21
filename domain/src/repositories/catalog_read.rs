//! Narrow read ports for the specialist catalog (interface segregation).
//! The aggregate [`crate::repositories::SpecialistCatalogReadRepository`] is their sum.

use async_trait::async_trait;

use crate::aggregates::{
    PatientProgramFull, ProgramWithAgenda, SpecialistDashboard, WorkoutWithExercises,
};
use crate::entities::{
    Exercise, PatientProgram, Program, ProgramScheduleItem, SessionExerciseFeedback,
    SpecialistPatient, Workout, WorkoutExercise, WorkoutSession,
};
use crate::error::Result;
use crate::vos::email::Email;
use crate::vos::id::Id;
use crate::vos::library_name_filter::LibraryNameFilter;
use crate::vos::profile::Profile;
use crate::vos::AccessToken;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait GetProfilesByIdsRead: Send + Sync {
    async fn get_profiles_by_ids(
        &self,
        ids: &[Id],
        access_token: &AccessToken,
    ) -> Result<Vec<Profile>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait GetPatientIdByEmailRead: Send + Sync {
    async fn get_patient_id_by_email(
        &self,
        access_token: &AccessToken,
        email: &Email,
    ) -> Result<Option<Id>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ListSpecialistPatientsRead: Send + Sync {
    async fn list_specialist_patients(
        &self,
        access_token: &AccessToken,
    ) -> Result<Vec<SpecialistPatient>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ListProgramsRead: Send + Sync {
    async fn list_programs(&self, access_token: &AccessToken) -> Result<Vec<Program>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait GetProgramRead: Send + Sync {
    async fn get_program(
        &self,
        access_token: &AccessToken,
        program_id: &Id,
    ) -> Result<Option<Program>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ListWorkoutLibraryRead: Send + Sync {
    async fn list_workout_library(
        &self,
        access_token: &AccessToken,
        specialist_id: &Id,
        name_filter: Option<&LibraryNameFilter>,
    ) -> Result<Vec<Workout>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait GetWorkoutsByIdsRead: Send + Sync {
    async fn get_workouts_by_ids(
        &self,
        access_token: &AccessToken,
        ids: &[Id],
    ) -> Result<Vec<Workout>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ListWorkoutsForProgramRead: Send + Sync {
    async fn list_workouts_for_program(
        &self,
        access_token: &AccessToken,
        program_id: &Id,
    ) -> Result<Vec<Workout>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ListProgramScheduleRead: Send + Sync {
    async fn list_program_schedule(
        &self,
        access_token: &AccessToken,
        program_id: &Id,
    ) -> Result<Vec<ProgramScheduleItem>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ListExercisesForWorkoutRead: Send + Sync {
    async fn list_exercises_for_workout(
        &self,
        access_token: &AccessToken,
        workout_id: &Id,
    ) -> Result<Vec<WorkoutExercise>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ListExerciseLibraryRead: Send + Sync {
    async fn list_exercise_library(
        &self,
        access_token: &AccessToken,
        specialist_id: &Id,
        name_filter: Option<&LibraryNameFilter>,
    ) -> Result<Vec<Exercise>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ListPatientProgramsForSpecialistRead: Send + Sync {
    async fn list_patient_programs_for_specialist(
        &self,
        access_token: &AccessToken,
    ) -> Result<Vec<PatientProgram>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait GetPatientProgramByIdRead: Send + Sync {
    async fn get_patient_program_by_id(
        &self,
        access_token: &AccessToken,
        id: &Id,
    ) -> Result<Option<PatientProgram>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ListWorkoutSessionsRead: Send + Sync {
    async fn list_workout_sessions(
        &self,
        access_token: &AccessToken,
        patient_program_id: &Id,
    ) -> Result<Vec<WorkoutSession>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ListSessionExerciseFeedbackRead: Send + Sync {
    async fn list_session_exercise_feedback(
        &self,
        access_token: &AccessToken,
        workout_session_id: &Id,
    ) -> Result<Vec<SessionExerciseFeedback>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ListSessionExerciseFeedbackForProgramRead: Send + Sync {
    async fn list_session_exercise_feedback_for_program(
        &self,
        access_token: &AccessToken,
        patient_program_id: &Id,
    ) -> Result<Vec<SessionExerciseFeedback>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ListActivePatientProgramsRead: Send + Sync {
    async fn list_active_patient_programs(
        &self,
        access_token: &AccessToken,
    ) -> Result<Vec<PatientProgram>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait GetWorkoutWithExercisesRead: Send + Sync {
    async fn get_workout_with_exercises(
        &self,
        access_token: &AccessToken,
        workout_id: &Id,
    ) -> Result<Option<WorkoutWithExercises>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait GetProgramWithAgendaRead: Send + Sync {
    async fn get_program_with_agenda(
        &self,
        access_token: &AccessToken,
        program_id: &Id,
    ) -> Result<Option<ProgramWithAgenda>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait GetPatientProgramFullRead: Send + Sync {
    async fn get_patient_program_full(
        &self,
        access_token: &AccessToken,
        patient_program_id: &Id,
    ) -> Result<Option<PatientProgramFull>>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait GetSpecialistDashboardRead: Send + Sync {
    async fn get_specialist_dashboard(
        &self,
        access_token: &AccessToken,
        specialist_id: &Id,
    ) -> Result<SpecialistDashboard>;
}
