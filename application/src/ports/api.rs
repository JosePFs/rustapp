use async_trait::async_trait;

use crate::use_cases::add_exercise_to_workout::AddExerciseToWorkoutArgs;
use crate::use_cases::add_specialist_patient::AddSpecialistPatientArgs;
use crate::use_cases::assign_program_to_patient::AssignProgramToPatientArgs;
use crate::use_cases::create_exercise::CreateExerciseArgs;
use crate::use_cases::create_program::CreateProgramArgs;
use crate::use_cases::create_program_schedule_item::CreateProgramScheduleItemArgs;
use crate::use_cases::create_workout::CreateWorkoutArgs;
use crate::use_cases::delete_program_schedule_item::DeleteProgramScheduleItemArgs;
use crate::use_cases::delete_workout::DeleteWorkoutArgs;
use crate::use_cases::get_patient_programs::{
    GetPatientProgramsUseCaseArgs, GetPatientProgramsUseCaseResult,
};
use crate::use_cases::get_specialist_patients_with_profiles::{
    GetSpecialistPatientsWithProfilesArgs, GetSpecialistPatientsWithProfilesResult,
};
use crate::use_cases::list_exercise_library::{ExerciseLibraryItem, ListExerciseLibraryArgs};
use crate::use_cases::list_program_schedule::{ListProgramScheduleArgs, ProgramScheduleData};
use crate::use_cases::list_workout_library::{ListWorkoutLibraryArgs, WorkoutLibraryItem};
use crate::use_cases::login::{LoginUseCaseArgs, LoginUseCaseResult};
use crate::use_cases::patient_progress::{PatientProgressArgs, PatientProgressResult};
use crate::use_cases::refresh_session::RefreshSessionArgs;
use crate::use_cases::remove_exercise_from_workout::RemoveExerciseFromWorkoutArgs;
use crate::use_cases::restore_exercise::RestoreExerciseArgs;
use crate::use_cases::soft_delete_exercise::SoftDeleteExerciseArgs;
use crate::use_cases::specialist_programs_data::{
    SpecialistProgramsDataArgs, SpecialistProgramsDataResult,
};
use crate::use_cases::submit_patient_workout_feedback::SubmitPatientWorkoutFeedbackArgs;
use crate::use_cases::uncomplete_patient_workout_session::UncompletePatientWorkoutSessionArgs;
use crate::use_cases::update_exercise::UpdateExerciseArgs;
use crate::use_cases::update_workout::UpdateWorkoutArgs;
use crate::use_cases::update_workout_exercise::UpdateWorkoutExerciseArgs;
use crate::use_cases::workout_editor_data::{WorkoutEditorDataArgs, WorkoutEditorDataResult};
use domain::entities::ProgramScheduleItem;
use domain::entities::Workout;
use domain::entities::{Exercise, PatientProgram, Program, SpecialistPatient};
use domain::error::Result;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait BackofficeApi {
    async fn login(&self, args: LoginUseCaseArgs) -> Result<LoginUseCaseResult>;

    async fn add_specialist_patient(
        &self,
        args: AddSpecialistPatientArgs,
    ) -> Result<SpecialistPatient>;

    async fn add_exercise_to_workout(&self, args: AddExerciseToWorkoutArgs) -> Result<()>;

    async fn assign_program_to_patient(
        &self,
        args: AssignProgramToPatientArgs,
    ) -> Result<PatientProgram>;

    async fn create_exercise(&self, args: CreateExerciseArgs) -> Result<Exercise>;

    async fn create_program(&self, args: CreateProgramArgs) -> Result<Program>;

    async fn create_program_schedule_item(
        &self,
        args: CreateProgramScheduleItemArgs,
    ) -> Result<ProgramScheduleItem>;

    async fn create_workout(&self, args: CreateWorkoutArgs) -> Result<Workout>;

    async fn delete_program_schedule_item(&self, args: DeleteProgramScheduleItemArgs)
        -> Result<()>;

    async fn delete_workout(&self, args: DeleteWorkoutArgs) -> Result<()>;

    async fn get_specialist_patients_with_profiles(
        &self,
        args: GetSpecialistPatientsWithProfilesArgs,
    ) -> Result<GetSpecialistPatientsWithProfilesResult>;

    async fn specialist_programs_data(
        &self,
        args: SpecialistProgramsDataArgs,
    ) -> Result<SpecialistProgramsDataResult>;

    async fn list_exercise_library(
        &self,
        args: ListExerciseLibraryArgs,
    ) -> Result<Vec<ExerciseLibraryItem>>;

    async fn list_program_schedule(
        &self,
        args: ListProgramScheduleArgs,
    ) -> Result<ProgramScheduleData>;

    async fn list_workout_library(
        &self,
        args: ListWorkoutLibraryArgs,
    ) -> Result<Vec<WorkoutLibraryItem>>;

    async fn patient_progress(&self, args: PatientProgressArgs) -> Result<PatientProgressResult>;

    async fn remove_exercise_from_workout(&self, args: RemoveExerciseFromWorkoutArgs)
        -> Result<()>;

    async fn restore_exercise(&self, args: RestoreExerciseArgs) -> Result<()>;

    async fn soft_delete_exercise(&self, args: SoftDeleteExerciseArgs) -> Result<()>;

    async fn update_exercise(&self, args: UpdateExerciseArgs) -> Result<()>;

    async fn update_workout(&self, args: UpdateWorkoutArgs) -> Result<()>;

    async fn update_workout_exercise(&self, args: UpdateWorkoutExerciseArgs) -> Result<()>;

    async fn workout_editor_data(
        &self,
        args: WorkoutEditorDataArgs,
    ) -> Result<WorkoutEditorDataResult>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait MobileApi {
    async fn login(&self, args: LoginUseCaseArgs) -> Result<LoginUseCaseResult>;

    async fn refresh_session(&self, args: RefreshSessionArgs) -> Result<LoginUseCaseResult>;

    async fn get_patient_programs(
        &self,
        args: GetPatientProgramsUseCaseArgs,
    ) -> Result<GetPatientProgramsUseCaseResult>;

    async fn submit_patient_workout_feedback(
        &self,
        args: SubmitPatientWorkoutFeedbackArgs,
    ) -> Result<()>;

    async fn uncomplete_patient_workout_session(
        &self,
        args: UncompletePatientWorkoutSessionArgs,
    ) -> Result<()>;
}
