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

#[common::async_trait_platform]
pub trait GetProfilesByIdsRead: Send + Sync {
    async fn get_profiles_by_ids(&self, ids: &[Id]) -> Result<Vec<Profile>>;
}

#[common::async_trait_platform]
pub trait GetPatientIdByEmailRead: Send + Sync {
    async fn get_patient_id_by_email(&self, email: &Email) -> Result<Option<Id>>;
}

#[common::async_trait_platform]
pub trait ListSpecialistPatientsRead: Send + Sync {
    async fn list_specialist_patients(&self) -> Result<Vec<SpecialistPatient>>;
}

#[common::async_trait_platform]
pub trait ListProgramsRead: Send + Sync {
    async fn list_programs(&self) -> Result<Vec<Program>>;
}

#[common::async_trait_platform]
pub trait GetProgramRead: Send + Sync {
    async fn get_program(&self, program_id: &Id) -> Result<Option<Program>>;
}

#[common::async_trait_platform]
pub trait ListWorkoutLibrary: Send + Sync {
    async fn list_workout_library(
        &self,
        name_filter: Option<&LibraryNameFilter>,
    ) -> Result<Vec<Workout>>;
}

#[common::async_trait_platform]
pub trait GetWorkoutsByIdsRead: Send + Sync {
    async fn get_workouts_by_ids(&self, ids: &[Id]) -> Result<Vec<Workout>>;
}

#[common::async_trait_platform]
pub trait ListWorkoutsForProgramRead: Send + Sync {
    async fn list_workouts_for_program(&self, program_id: &Id) -> Result<Vec<Workout>>;
}

#[common::async_trait_platform]
pub trait ListProgramScheduleRead: Send + Sync {
    async fn list_program_schedule(&self, program_id: &Id) -> Result<Vec<ProgramScheduleItem>>;
}

#[common::async_trait_platform]
pub trait ListExercisesForWorkoutRead: Send + Sync {
    async fn list_exercises_for_workout(&self, workout_id: &Id) -> Result<Vec<WorkoutExercise>>;
}

#[common::async_trait_platform]
pub trait ListExerciseLibrary: Send + Sync {
    async fn list_exercise_library(
        &self,
        name_filter: Option<&LibraryNameFilter>,
    ) -> Result<Vec<Exercise>>;
}

#[common::async_trait_platform]
pub trait ListPatientProgramsForSpecialistRead: Send + Sync {
    async fn list_patient_programs_for_specialist(&self) -> Result<Vec<PatientProgram>>;
}

#[common::async_trait_platform]
pub trait GetPatientProgramByIdRead: Send + Sync {
    async fn get_patient_program_by_id(&self, id: &Id) -> Result<Option<PatientProgram>>;
}

#[common::async_trait_platform]
pub trait ListWorkoutSessionsRead: Send + Sync {
    async fn list_workout_sessions(&self, patient_program_id: &Id) -> Result<Vec<WorkoutSession>>;
}

#[common::async_trait_platform]
pub trait ListSessionExerciseFeedbackRead: Send + Sync {
    async fn list_session_exercise_feedback(
        &self,
        workout_session_id: &Id,
    ) -> Result<Vec<SessionExerciseFeedback>>;
}

#[common::async_trait_platform]
pub trait ListSessionExerciseFeedbackForProgramRead: Send + Sync {
    async fn list_session_exercise_feedback_for_program(
        &self,
        patient_program_id: &Id,
    ) -> Result<Vec<SessionExerciseFeedback>>;
}

#[common::async_trait_platform]
pub trait ListActivePatientProgramsRead: Send + Sync {
    async fn list_active_patient_programs(&self) -> Result<Vec<PatientProgram>>;
}

#[common::async_trait_platform]
pub trait GetWorkoutWithExercisesRead: Send + Sync {
    async fn get_workout_with_exercises(
        &self,
        workout_id: &Id,
    ) -> Result<Option<WorkoutWithExercises>>;
}

#[common::async_trait_platform]
pub trait GetProgramWithAgendaRead: Send + Sync {
    async fn get_program_with_agenda(&self, program_id: &Id) -> Result<Option<ProgramWithAgenda>>;
}

#[common::async_trait_platform]
pub trait GetPatientProgramFullRead: Send + Sync {
    async fn get_patient_program_full(
        &self,
        patient_program_id: &Id,
    ) -> Result<Option<PatientProgramFull>>;
}

#[common::async_trait_platform]
pub trait GetSpecialistDashboardRead: Send + Sync {
    async fn get_specialist_dashboard(&self) -> Result<SpecialistDashboard>;
}
