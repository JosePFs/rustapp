use std::sync::Arc;

use domain::repositories::SpecialistRepository;

use crate::error::Result;
use crate::ports::auth::AuthService;
use crate::ports::backoffice_api::{
    AddSpecialistPatientArgs, AddSpecialistPatientResult, AddExerciseToWorkoutArgs,
    AssignProgramToPatientArgs, AssignProgramToPatientResult, CreateExerciseArgs,
    CreateExerciseResult, CreateProgramArgs, CreateProgramResult,
    CreateProgramScheduleItemArgs, CreateProgramScheduleItemResult, CreateWorkoutArgs,
    CreateWorkoutResult, DeleteProgramScheduleItemArgs, DeleteWorkoutArgs,
    GetSpecialistPatientsWithProfilesArgs, GetSpecialistPatientsWithProfilesResult,
    ListExerciseLibraryArgs, ListExerciseLibraryResult, ListProgramScheduleArgs,
    ListProgramScheduleResult, ListUnassignedPatientsArgs, ListUnassignedPatientsResult,
    ListWorkoutLibraryArgs, ListWorkoutLibraryResult, LoginArgs, LoginResult,
    PatientProgressArgs, PatientProgressResult, RemoveExerciseFromWorkoutArgs,
    RestoreExerciseArgs, SoftDeleteExerciseArgs, SpecialistProgramsDataResult,
    UpdateExerciseArgs, UpdateWorkoutArgs, UpdateWorkoutExerciseArgs,
    WorkoutEditorDataArgs, WorkoutEditorDataResult, BackofficeApi,
};
use crate::use_cases::add_exercise_to_workout::AddExerciseToWorkoutUseCase;
use crate::use_cases::add_specialist_patient::AddSpecialistPatientUseCase;
use crate::use_cases::assign_program_to_patient::AssignProgramToPatientUseCase;
use crate::use_cases::create_exercise::CreateExerciseUseCase;
use crate::use_cases::create_program::CreateProgramUseCase;
use crate::use_cases::create_program_schedule_item::CreateProgramScheduleItemUseCase;
use crate::use_cases::create_workout::CreateWorkoutUseCase;
use crate::use_cases::delete_program_schedule_item::DeleteProgramScheduleItemUseCase;
use crate::use_cases::delete_workout::DeleteWorkoutUseCase;
use crate::use_cases::get_specialist_patients_with_profiles::GetSpecialistPatientsWithProfilesUseCase;
use crate::use_cases::list_exercise_library::ListExerciseLibraryUseCase;
use crate::use_cases::list_program_schedule::ListProgramScheduleUseCase;
use crate::use_cases::list_unassigned_patients::ListUnassignedPatientsUseCase;
use crate::use_cases::list_workout_library::ListWorkoutLibraryUseCase;
use crate::use_cases::login::{LoginUseCase, LoginUseCaseArgs};
use crate::use_cases::patient_progress::PatientProgressUseCase;
use crate::use_cases::remove_exercise_from_workout::RemoveExerciseFromWorkoutUseCase;
use crate::use_cases::restore_exercise::RestoreExerciseUseCase;
use crate::use_cases::soft_delete_exercise::SoftDeleteExerciseUseCase;
use crate::use_cases::specialist_programs_data::SpecialistProgramsDataUseCase;
use crate::use_cases::update_exercise::UpdateExerciseUseCase;
use crate::use_cases::update_workout::UpdateWorkoutUseCase;
use crate::use_cases::update_workout_exercise::UpdateWorkoutExerciseUseCase;
use crate::use_cases::workout_editor_data::WorkoutEditorDataUseCase;

pub struct BackofficeFacade<D, A>
where
    D: SpecialistRepository,
    A: AuthService,
{
    login_uc: LoginUseCase<D, A>,
    add_specialist_patient_uc: AddSpecialistPatientUseCase<D>,
    add_exercise_to_workout_uc: AddExerciseToWorkoutUseCase<D>,
    assign_program_to_patient_uc: AssignProgramToPatientUseCase<D>,
    create_exercise_uc: CreateExerciseUseCase<D>,
    create_program_uc: CreateProgramUseCase<D>,
    create_program_schedule_item_uc: CreateProgramScheduleItemUseCase<D>,
    create_workout_uc: CreateWorkoutUseCase<D>,
    delete_program_schedule_item_uc: DeleteProgramScheduleItemUseCase<D>,
    delete_workout_uc: DeleteWorkoutUseCase<D>,
    get_specialist_patients_with_profiles_uc: GetSpecialistPatientsWithProfilesUseCase<D>,
    specialist_programs_data_uc: SpecialistProgramsDataUseCase<D>,
    list_exercise_library_uc: ListExerciseLibraryUseCase<D>,
    list_program_schedule_uc: ListProgramScheduleUseCase<D>,
    list_unassigned_patients_uc: ListUnassignedPatientsUseCase<D>,
    list_workout_library_uc: ListWorkoutLibraryUseCase<D>,
    patient_progress_uc: PatientProgressUseCase<D>,
    remove_exercise_from_workout_uc: RemoveExerciseFromWorkoutUseCase<D>,
    restore_exercise_uc: RestoreExerciseUseCase<D>,
    soft_delete_exercise_uc: SoftDeleteExerciseUseCase<D>,
    update_exercise_uc: UpdateExerciseUseCase<D>,
    update_workout_uc: UpdateWorkoutUseCase<D>,
    update_workout_exercise_uc: UpdateWorkoutExerciseUseCase<D>,
    workout_editor_data_uc: WorkoutEditorDataUseCase<D>,
}

impl<D, A> BackofficeFacade<D, A>
where
    D: SpecialistRepository,
    A: AuthService,
{
    pub fn builder(repository: Arc<D>, auth: Arc<A>) -> BackofficeFacadeBuilder<D, A> {
        BackofficeFacadeBuilder::new(repository, auth)
    }
}

#[common::async_trait_platform]
impl<D, A> BackofficeApi for BackofficeFacade<D, A>
where
    D: SpecialistRepository,
    A: AuthService,
{
    async fn login(&self, args: LoginArgs) -> Result<LoginResult> {
        let uc_args = LoginUseCaseArgs::from(&args.email, &args.password);
        let result = self.login_uc.execute(uc_args).await?;
        Ok(LoginResult {
            access_token: result.session.access_token().to_string(),
            refresh_token: result.session.refresh_token().map(|s| s.to_string()),
            user_id: result.session.user_id().to_string(),
            expires_at: result.session.expires_at().copied(),
            user_profile_type: result.user_profile_type.to_string(),
        })
    }

    async fn add_specialist_patient(
        &self,
        args: AddSpecialistPatientArgs,
    ) -> Result<AddSpecialistPatientResult> {
        let entity = self.add_specialist_patient_uc.execute(args.into()).await?;
        Ok(AddSpecialistPatientResult {
            id: entity.id.to_string(),
            specialist_id: entity.specialist_id.to_string(),
            patient_id: entity.patient_id.to_string(),
            created_at: entity.created_at,
        })
    }

    async fn add_exercise_to_workout(&self, args: AddExerciseToWorkoutArgs) -> Result<()> {
        self.add_exercise_to_workout_uc.execute(args.into()).await
    }

    async fn assign_program_to_patient(
        &self,
        args: AssignProgramToPatientArgs,
    ) -> Result<AssignProgramToPatientResult> {
        let entity = self.assign_program_to_patient_uc.execute(args.into()).await?;
        Ok(AssignProgramToPatientResult {
            id: entity.id.to_string(),
            patient_id: entity.patient_id.to_string(),
            program_id: entity.program_id.to_string(),
            status: entity.status,
        })
    }

    async fn create_exercise(&self, args: CreateExerciseArgs) -> Result<CreateExerciseResult> {
        let entity = self.create_exercise_uc.execute(args.into()).await?;
        Ok(CreateExerciseResult {
            id: entity.id.to_string(),
            name: entity.name,
            description: entity.description,
            order_index: entity.order_index.value(),
            video_url: entity.video_url.map(|v| v.value().to_string()),
            deleted_at: entity.deleted_at,
        })
    }

    async fn create_program(&self, args: CreateProgramArgs) -> Result<CreateProgramResult> {
        let entity = self.create_program_uc.execute(args.into()).await?;
        Ok(CreateProgramResult {
            id: entity.id.to_string(),
            name: entity.name,
            description: entity.description,
        })
    }

    async fn create_program_schedule_item(
        &self,
        args: CreateProgramScheduleItemArgs,
    ) -> Result<CreateProgramScheduleItemResult> {
        let entity = self.create_program_schedule_item_uc.execute(args.into()).await?;
        Ok(CreateProgramScheduleItemResult {
            id: entity.id.to_string(),
            program_id: entity.program_id.to_string(),
            order_index: entity.order_index.value(),
            workout_id: entity.workout_id.map(|w| w.to_string()),
            days_count: entity.days_count.value(),
        })
    }

    async fn create_workout(&self, args: CreateWorkoutArgs) -> Result<CreateWorkoutResult> {
        let entity = self.create_workout_uc.execute(args.into()).await?;
        Ok(CreateWorkoutResult {
            id: entity.id.to_string(),
            name: entity.name,
            description: entity.description,
            order_index: entity.order_index.value(),
        })
    }

    async fn delete_program_schedule_item(
        &self,
        args: DeleteProgramScheduleItemArgs,
    ) -> Result<()> {
        self.delete_program_schedule_item_uc.execute(args.into()).await
    }

    async fn delete_workout(&self, args: DeleteWorkoutArgs) -> Result<()> {
        self.delete_workout_uc.execute(args.into()).await
    }

    async fn get_specialist_patients_with_profiles(
        &self,
        args: GetSpecialistPatientsWithProfilesArgs,
    ) -> Result<GetSpecialistPatientsWithProfilesResult> {
        let result = self.get_specialist_patients_with_profiles_uc
            .execute(args.into())
            .await?;
        Ok(result.into())
    }

    async fn specialist_programs_data(&self) -> Result<SpecialistProgramsDataResult> {
        let result = self.specialist_programs_data_uc.execute().await?;
        Ok(result.into())
    }

    async fn list_exercise_library(
        &self,
        args: ListExerciseLibraryArgs,
    ) -> Result<ListExerciseLibraryResult> {
        let items = self.list_exercise_library_uc.execute(args.into()).await?;
        Ok(items.into())
    }

    async fn list_program_schedule(
        &self,
        args: ListProgramScheduleArgs,
    ) -> Result<ListProgramScheduleResult> {
        let data = self.list_program_schedule_uc.execute(args.into()).await?;
        Ok(data.into())
    }

    async fn list_workout_library(
        &self,
        args: ListWorkoutLibraryArgs,
    ) -> Result<ListWorkoutLibraryResult> {
        let items = self.list_workout_library_uc.execute(args.into()).await?;
        Ok(items.into())
    }

    async fn list_unassigned_patients(
        &self,
        args: ListUnassignedPatientsArgs,
    ) -> Result<ListUnassignedPatientsResult> {
        let result = self.list_unassigned_patients_uc.execute(args.into()).await?;
        Ok(result.into())
    }

    async fn patient_progress(&self, args: PatientProgressArgs) -> Result<PatientProgressResult> {
        let result = self.patient_progress_uc.execute(args.into()).await?;
        Ok(result.into())
    }

    async fn remove_exercise_from_workout(
        &self,
        args: RemoveExerciseFromWorkoutArgs,
    ) -> Result<()> {
        self.remove_exercise_from_workout_uc.execute(args.into()).await
    }

    async fn restore_exercise(&self, args: RestoreExerciseArgs) -> Result<()> {
        self.restore_exercise_uc.execute(args.into()).await
    }

    async fn soft_delete_exercise(&self, args: SoftDeleteExerciseArgs) -> Result<()> {
        self.soft_delete_exercise_uc.execute(args.into()).await
    }

    async fn update_exercise(&self, args: UpdateExerciseArgs) -> Result<()> {
        self.update_exercise_uc.execute(args.into()).await
    }

    async fn update_workout(&self, args: UpdateWorkoutArgs) -> Result<()> {
        self.update_workout_uc.execute(args.into()).await
    }

    async fn update_workout_exercise(&self, args: UpdateWorkoutExerciseArgs) -> Result<()> {
        self.update_workout_exercise_uc.execute(args.into()).await
    }

    async fn workout_editor_data(
        &self,
        args: WorkoutEditorDataArgs,
    ) -> Result<WorkoutEditorDataResult> {
        let result = self.workout_editor_data_uc.execute(args.into()).await?;
        Ok(result.into())
    }
}

pub struct BackofficeFacadeBuilder<D, A>
where
    D: SpecialistRepository,
    A: AuthService,
{
    repository: Arc<D>,
    auth: Arc<A>,
}

impl<D, A> BackofficeFacadeBuilder<D, A>
where
    D: SpecialistRepository,
    A: AuthService,
{
    pub fn new(repository: Arc<D>, auth: Arc<A>) -> Self {
        Self { repository, auth }
    }
}

impl<D, A> BackofficeFacadeBuilder<D, A>
where
    D: SpecialistRepository,
    A: AuthService,
{
    pub fn build(self) -> BackofficeFacade<D, A> {
        BackofficeFacade {
            login_uc: LoginUseCase::new(self.repository.clone(), self.auth.clone()),
            add_specialist_patient_uc: AddSpecialistPatientUseCase::new(self.repository.clone()),
            add_exercise_to_workout_uc: AddExerciseToWorkoutUseCase::new(self.repository.clone()),
            assign_program_to_patient_uc: AssignProgramToPatientUseCase::new(
                self.repository.clone(),
            ),
            create_exercise_uc: CreateExerciseUseCase::new(self.repository.clone()),
            create_program_uc: CreateProgramUseCase::new(self.repository.clone()),
            create_program_schedule_item_uc: CreateProgramScheduleItemUseCase::new(
                self.repository.clone(),
            ),
            create_workout_uc: CreateWorkoutUseCase::new(self.repository.clone()),
            delete_program_schedule_item_uc: DeleteProgramScheduleItemUseCase::new(
                self.repository.clone(),
            ),
            delete_workout_uc: DeleteWorkoutUseCase::new(self.repository.clone()),
            get_specialist_patients_with_profiles_uc: GetSpecialistPatientsWithProfilesUseCase::new(
                self.repository.clone(),
            ),
            specialist_programs_data_uc: SpecialistProgramsDataUseCase::new(
                self.repository.clone(),
            ),
            list_exercise_library_uc: ListExerciseLibraryUseCase::new(self.repository.clone()),
            list_program_schedule_uc: ListProgramScheduleUseCase::new(self.repository.clone()),
            list_unassigned_patients_uc: ListUnassignedPatientsUseCase::new(
                self.repository.clone(),
            ),
            list_workout_library_uc: ListWorkoutLibraryUseCase::new(self.repository.clone()),
            patient_progress_uc: PatientProgressUseCase::new(self.repository.clone()),
            remove_exercise_from_workout_uc: RemoveExerciseFromWorkoutUseCase::new(
                self.repository.clone(),
            ),
            restore_exercise_uc: RestoreExerciseUseCase::new(self.repository.clone()),
            soft_delete_exercise_uc: SoftDeleteExerciseUseCase::new(self.repository.clone()),
            update_exercise_uc: UpdateExerciseUseCase::new(self.repository.clone()),
            update_workout_uc: UpdateWorkoutUseCase::new(self.repository.clone()),
            update_workout_exercise_uc: UpdateWorkoutExerciseUseCase::new(self.repository.clone()),
            workout_editor_data_uc: WorkoutEditorDataUseCase::new(self.repository.clone()),
        }
    }
}
