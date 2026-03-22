use std::sync::Arc;

use domain::entities::{Exercise, PatientProgram, Program, SpecialistPatient};
use domain::entities::{ProgramScheduleItem, Workout};
use domain::error::Result;
use domain::repositories::{SpecialistCatalogReadRepository, SpecialistCatalogWriteRepository};

use crate::ports::api::BackofficeApi;
use crate::ports::auth::AuthService;
use crate::use_cases::add_exercise_to_workout::{
    AddExerciseToWorkoutArgs, AddExerciseToWorkoutUseCase,
};
use crate::use_cases::add_specialist_patient::{
    AddSpecialistPatientArgs, AddSpecialistPatientUseCase,
};
use crate::use_cases::assign_program_to_patient::{
    AssignProgramToPatientArgs, AssignProgramToPatientUseCase,
};
use crate::use_cases::create_exercise::{CreateExerciseArgs, CreateExerciseUseCase};
use crate::use_cases::create_program::{CreateProgramArgs, CreateProgramUseCase};
use crate::use_cases::create_program_schedule_item::{
    CreateProgramScheduleItemArgs, CreateProgramScheduleItemUseCase,
};
use crate::use_cases::create_workout::{CreateWorkoutArgs, CreateWorkoutUseCase};
use crate::use_cases::delete_program_schedule_item::{
    DeleteProgramScheduleItemArgs, DeleteProgramScheduleItemUseCase,
};
use crate::use_cases::delete_workout::{DeleteWorkoutArgs, DeleteWorkoutUseCase};
use crate::use_cases::get_specialist_patients_with_profiles::{
    GetSpecialistPatientsWithProfilesArgs, GetSpecialistPatientsWithProfilesResult,
    GetSpecialistPatientsWithProfilesUseCase,
};
use crate::use_cases::list_exercise_library::{
    ExerciseLibraryItem, ListExerciseLibraryArgs, ListExerciseLibraryUseCase,
};
use crate::use_cases::list_program_schedule::{
    ListProgramScheduleArgs, ListProgramScheduleUseCase, ProgramScheduleData,
};
use crate::use_cases::list_workout_library::{
    ListWorkoutLibraryArgs, ListWorkoutLibraryUseCase, WorkoutLibraryItem,
};
use crate::use_cases::login::{LoginUseCase, LoginUseCaseArgs, LoginUseCaseResult};
use crate::use_cases::patient_progress::{
    PatientProgressArgs, PatientProgressResult, PatientProgressUseCase,
};
use crate::use_cases::remove_exercise_from_workout::{
    RemoveExerciseFromWorkoutArgs, RemoveExerciseFromWorkoutUseCase,
};
use crate::use_cases::restore_exercise::{RestoreExerciseArgs, RestoreExerciseUseCase};
use crate::use_cases::soft_delete_exercise::{SoftDeleteExerciseArgs, SoftDeleteExerciseUseCase};
use crate::use_cases::specialist_programs_data::{
    SpecialistProgramsDataArgs, SpecialistProgramsDataResult, SpecialistProgramsDataUseCase,
};
use crate::use_cases::update_exercise::{UpdateExerciseArgs, UpdateExerciseUseCase};
use crate::use_cases::update_workout::{UpdateWorkoutArgs, UpdateWorkoutUseCase};
use crate::use_cases::update_workout_exercise::{
    UpdateWorkoutExerciseArgs, UpdateWorkoutExerciseUseCase,
};
use crate::use_cases::workout_editor_data::{
    WorkoutEditorDataArgs, WorkoutEditorDataResult, WorkoutEditorDataUseCase,
};

pub struct BackofficeFacade<D, A>
where
    D: SpecialistCatalogReadRepository + SpecialistCatalogWriteRepository + Send + Sync,
    A: AuthService + Send + Sync,
{
    pub login_uc: Arc<LoginUseCase<D, A>>,
    pub add_specialist_patient_uc: Arc<AddSpecialistPatientUseCase<D>>,
    pub add_exercise_to_workout_uc: Arc<AddExerciseToWorkoutUseCase<D>>,
    pub assign_program_to_patient_uc: Arc<AssignProgramToPatientUseCase<D>>,
    pub create_exercise_uc: Arc<CreateExerciseUseCase<D>>,
    pub create_program_uc: Arc<CreateProgramUseCase<D>>,
    pub create_program_schedule_item_uc: Arc<CreateProgramScheduleItemUseCase<D>>,
    pub create_workout_uc: Arc<CreateWorkoutUseCase<D>>,
    pub delete_program_schedule_item_uc: Arc<DeleteProgramScheduleItemUseCase<D>>,
    pub delete_workout_uc: Arc<DeleteWorkoutUseCase<D>>,
    pub get_specialist_patients_with_profiles_uc: Arc<GetSpecialistPatientsWithProfilesUseCase<D>>,
    pub specialist_programs_data_uc: Arc<SpecialistProgramsDataUseCase<D>>,
    pub list_exercise_library_uc: Arc<ListExerciseLibraryUseCase<D>>,
    pub list_program_schedule_uc: Arc<ListProgramScheduleUseCase<D>>,
    pub list_workout_library_uc: Arc<ListWorkoutLibraryUseCase<D>>,
    pub patient_progress_uc: Arc<PatientProgressUseCase<D>>,
    pub remove_exercise_from_workout_uc: Arc<RemoveExerciseFromWorkoutUseCase<D>>,
    pub restore_exercise_uc: Arc<RestoreExerciseUseCase<D>>,
    pub soft_delete_exercise_uc: Arc<SoftDeleteExerciseUseCase<D>>,
    pub update_exercise_uc: Arc<UpdateExerciseUseCase<D>>,
    pub update_workout_uc: Arc<UpdateWorkoutUseCase<D>>,
    pub update_workout_exercise_uc: Arc<UpdateWorkoutExerciseUseCase<D>>,
    pub workout_editor_data_uc: Arc<WorkoutEditorDataUseCase<D>>,
}

#[common::async_trait_platform]
impl<D, A> BackofficeApi for BackofficeFacade<D, A>
where
    D: SpecialistCatalogReadRepository + SpecialistCatalogWriteRepository + Send + Sync,
    A: AuthService + Send + Sync,
{
    async fn login(&self, args: LoginUseCaseArgs) -> Result<LoginUseCaseResult> {
        self.login_uc.execute(args).await
    }

    async fn add_specialist_patient(
        &self,
        args: AddSpecialistPatientArgs,
    ) -> Result<SpecialistPatient> {
        self.add_specialist_patient_uc.execute(args).await
    }

    async fn add_exercise_to_workout(&self, args: AddExerciseToWorkoutArgs) -> Result<()> {
        self.add_exercise_to_workout_uc.execute(args).await
    }

    async fn assign_program_to_patient(
        &self,
        args: AssignProgramToPatientArgs,
    ) -> Result<PatientProgram> {
        self.assign_program_to_patient_uc.execute(args).await
    }

    async fn create_exercise(&self, args: CreateExerciseArgs) -> Result<Exercise> {
        self.create_exercise_uc.execute(args).await
    }

    async fn create_program(&self, args: CreateProgramArgs) -> Result<Program> {
        self.create_program_uc.execute(args).await
    }

    async fn create_program_schedule_item(
        &self,
        args: CreateProgramScheduleItemArgs,
    ) -> Result<ProgramScheduleItem> {
        self.create_program_schedule_item_uc.execute(args).await
    }

    async fn create_workout(&self, args: CreateWorkoutArgs) -> Result<Workout> {
        self.create_workout_uc.execute(args).await
    }

    async fn delete_program_schedule_item(
        &self,
        args: DeleteProgramScheduleItemArgs,
    ) -> Result<()> {
        self.delete_program_schedule_item_uc.execute(args).await
    }

    async fn delete_workout(&self, args: DeleteWorkoutArgs) -> Result<()> {
        self.delete_workout_uc.execute(args).await
    }

    async fn get_specialist_patients_with_profiles(
        &self,
        args: GetSpecialistPatientsWithProfilesArgs,
    ) -> Result<GetSpecialistPatientsWithProfilesResult> {
        self.get_specialist_patients_with_profiles_uc
            .execute(args)
            .await
    }

    async fn specialist_programs_data(
        &self,
        args: SpecialistProgramsDataArgs,
    ) -> Result<SpecialistProgramsDataResult> {
        self.specialist_programs_data_uc.execute(args).await
    }

    async fn list_exercise_library(
        &self,
        args: ListExerciseLibraryArgs,
    ) -> Result<Vec<ExerciseLibraryItem>> {
        self.list_exercise_library_uc.execute(args).await
    }

    async fn list_program_schedule(
        &self,
        args: ListProgramScheduleArgs,
    ) -> Result<ProgramScheduleData> {
        self.list_program_schedule_uc.execute(args).await
    }

    async fn list_workout_library(
        &self,
        args: ListWorkoutLibraryArgs,
    ) -> Result<Vec<WorkoutLibraryItem>> {
        self.list_workout_library_uc.execute(args).await
    }

    async fn patient_progress(&self, args: PatientProgressArgs) -> Result<PatientProgressResult> {
        self.patient_progress_uc.execute(args).await
    }

    async fn remove_exercise_from_workout(
        &self,
        args: RemoveExerciseFromWorkoutArgs,
    ) -> Result<()> {
        self.remove_exercise_from_workout_uc.execute(args).await
    }

    async fn restore_exercise(&self, args: RestoreExerciseArgs) -> Result<()> {
        self.restore_exercise_uc.execute(args).await
    }

    async fn soft_delete_exercise(&self, args: SoftDeleteExerciseArgs) -> Result<()> {
        self.soft_delete_exercise_uc.execute(args).await
    }

    async fn update_exercise(&self, args: UpdateExerciseArgs) -> Result<()> {
        self.update_exercise_uc.execute(args).await
    }

    async fn update_workout(&self, args: UpdateWorkoutArgs) -> Result<()> {
        self.update_workout_uc.execute(args).await
    }

    async fn update_workout_exercise(&self, args: UpdateWorkoutExerciseArgs) -> Result<()> {
        self.update_workout_exercise_uc.execute(args).await
    }

    async fn workout_editor_data(
        &self,
        args: WorkoutEditorDataArgs,
    ) -> Result<WorkoutEditorDataResult> {
        self.workout_editor_data_uc.execute(args).await
    }
}
