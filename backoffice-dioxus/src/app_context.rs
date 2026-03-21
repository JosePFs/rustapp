use std::sync::Arc;

use dioxus::signals::Signal;

use anymap2::{any::Any, AnyMap};

use application::ports::auth::session::Session;
use application::{
    use_cases::add_exercise_to_workout::AddExerciseToWorkoutUseCase,
    use_cases::add_specialist_patient::AddSpecialistPatientUseCase,
    use_cases::assign_program_to_patient::AssignProgramToPatientUseCase,
    use_cases::create_exercise::CreateExerciseUseCase,
    use_cases::create_program::CreateProgramUseCase,
    use_cases::create_program_schedule_item::CreateProgramScheduleItemUseCase,
    use_cases::create_workout::CreateWorkoutUseCase,
    use_cases::delete_program_schedule_item::DeleteProgramScheduleItemUseCase,
    use_cases::delete_workout::DeleteWorkoutUseCase,
    use_cases::get_specialist_patients_with_profiles::GetSpecialistPatientsWithProfilesUseCase,
    use_cases::list_exercise_library::ListExerciseLibraryUseCase,
    use_cases::list_program_schedule::ListProgramScheduleUseCase,
    use_cases::list_workout_library::ListWorkoutLibraryUseCase, use_cases::login::LoginUseCase,
    use_cases::patient_progress::PatientProgressUseCase,
    use_cases::remove_exercise_from_workout::RemoveExerciseFromWorkoutUseCase,
    use_cases::restore_exercise::RestoreExerciseUseCase,
    use_cases::soft_delete_exercise::SoftDeleteExerciseUseCase,
    use_cases::specialist_programs_data::SpecialistProgramsDataUseCase,
    use_cases::update_exercise::UpdateExerciseUseCase,
    use_cases::update_workout::UpdateWorkoutUseCase,
    use_cases::update_workout_exercise::UpdateWorkoutExerciseUseCase,
    use_cases::workout_editor_data::WorkoutEditorDataUseCase,
};
use domain::error::Result;
use infrastructure::supabase::api::{Api, ApiBuilder};
use infrastructure::supabase::auth::SupabaseAuth;

#[derive(Clone)]
pub struct AppContext {
    session: Signal<Option<Session>>,
    use_cases: Arc<AnyMap>,
}

impl AppContext {
    pub fn new(session: Option<Session>, use_cases: AnyMap) -> Self {
        Self {
            session: Signal::new(session),
            use_cases: Arc::new(use_cases),
        }
    }

    pub fn session(&self) -> Signal<Option<Session>> {
        self.session
    }

    pub fn use_case<T: Any + Send + Sync>(&self) -> Arc<T> {
        self.use_cases
            .get::<Arc<T>>()
            .expect("UseCase not found")
            .clone()
    }
}

pub fn build_app_context() -> Result<AppContext> {
    let auth = Arc::new(SupabaseAuth::builder().build());
    let backend = Arc::new(ApiBuilder::new().build());

    let add_specialist_patient_use_case =
        Arc::new(AddSpecialistPatientUseCase::<Api>::new(backend.clone()));
    let add_exercise_to_workout_use_case =
        Arc::new(AddExerciseToWorkoutUseCase::<Api>::new(backend.clone()));
    let assign_program_to_patient_use_case =
        Arc::new(AssignProgramToPatientUseCase::<Api>::new(backend.clone()));
    let create_exercise_use_case = Arc::new(CreateExerciseUseCase::<Api>::new(backend.clone()));
    let create_program_use_case = Arc::new(CreateProgramUseCase::<Api>::new(backend.clone()));
    let create_program_schedule_item_use_case = Arc::new(
        CreateProgramScheduleItemUseCase::<Api>::new(backend.clone()),
    );
    let create_workout_use_case = Arc::new(CreateWorkoutUseCase::<Api>::new(backend.clone()));
    let delete_program_schedule_item_use_case = Arc::new(
        DeleteProgramScheduleItemUseCase::<Api>::new(backend.clone()),
    );
    let delete_workout_use_case = Arc::new(DeleteWorkoutUseCase::<Api>::new(backend.clone()));
    let login_use_case = Arc::new(LoginUseCase::<Api, SupabaseAuth>::new(
        backend.clone(),
        auth.clone(),
    ));
    let get_specialist_patients_with_profiles_use_case = Arc::new(
        GetSpecialistPatientsWithProfilesUseCase::<Api>::new(backend.clone()),
    );
    let specialist_programs_data_use_case =
        Arc::new(SpecialistProgramsDataUseCase::<Api>::new(backend.clone()));
    let list_exercise_library_use_case =
        Arc::new(ListExerciseLibraryUseCase::<Api>::new(backend.clone()));
    let list_program_schedule_use_case =
        Arc::new(ListProgramScheduleUseCase::<Api>::new(backend.clone()));
    let list_workout_library_use_case =
        Arc::new(ListWorkoutLibraryUseCase::<Api>::new(backend.clone()));
    let patient_progress_use_case = Arc::new(PatientProgressUseCase::<Api>::new(backend.clone()));
    let remove_exercise_from_workout_use_case = Arc::new(
        RemoveExerciseFromWorkoutUseCase::<Api>::new(backend.clone()),
    );
    let restore_exercise_use_case = Arc::new(RestoreExerciseUseCase::<Api>::new(backend.clone()));
    let soft_delete_exercise_use_case =
        Arc::new(SoftDeleteExerciseUseCase::<Api>::new(backend.clone()));
    let update_workout_use_case = Arc::new(UpdateWorkoutUseCase::<Api>::new(backend.clone()));
    let update_exercise_use_case = Arc::new(UpdateExerciseUseCase::<Api>::new(backend.clone()));
    let update_workout_exercise_use_case =
        Arc::new(UpdateWorkoutExerciseUseCase::<Api>::new(backend.clone()));
    let workout_editor_data_use_case =
        Arc::new(WorkoutEditorDataUseCase::<Api>::new(backend.clone()));

    let mut use_cases = AnyMap::new();
    use_cases.insert(add_exercise_to_workout_use_case);
    use_cases.insert(add_specialist_patient_use_case);
    use_cases.insert(assign_program_to_patient_use_case);
    use_cases.insert(create_exercise_use_case);
    use_cases.insert(create_program_use_case);
    use_cases.insert(create_program_schedule_item_use_case);
    use_cases.insert(create_workout_use_case);
    use_cases.insert(delete_program_schedule_item_use_case);
    use_cases.insert(delete_workout_use_case);
    use_cases.insert(login_use_case);
    use_cases.insert(get_specialist_patients_with_profiles_use_case);
    use_cases.insert(specialist_programs_data_use_case);
    use_cases.insert(list_exercise_library_use_case);
    use_cases.insert(list_program_schedule_use_case);
    use_cases.insert(list_workout_library_use_case);
    use_cases.insert(patient_progress_use_case);
    use_cases.insert(remove_exercise_from_workout_use_case);
    use_cases.insert(restore_exercise_use_case);
    use_cases.insert(soft_delete_exercise_use_case);
    use_cases.insert(update_exercise_use_case);
    use_cases.insert(update_workout_use_case);
    use_cases.insert(update_workout_exercise_use_case);
    use_cases.insert(workout_editor_data_use_case);

    Ok(AppContext::new(None, use_cases))
}

pub type AddSpecialistPatientUseCaseType = AddSpecialistPatientUseCase<Api>;
pub type AddExerciseToWorkoutUseCaseType = AddExerciseToWorkoutUseCase<Api>;
pub type AssignProgramToPatientUseCaseType = AssignProgramToPatientUseCase<Api>;
pub type CreateExerciseUseCaseType = CreateExerciseUseCase<Api>;
pub type CreateProgramUseCaseType = CreateProgramUseCase<Api>;
pub type CreateProgramScheduleItemUseCaseType = CreateProgramScheduleItemUseCase<Api>;
pub type CreateWorkoutUseCaseType = CreateWorkoutUseCase<Api>;
pub type DeleteProgramScheduleItemUseCaseType = DeleteProgramScheduleItemUseCase<Api>;
pub type DeleteWorkoutUseCaseType = DeleteWorkoutUseCase<Api>;
pub type LoginUseCaseType = LoginUseCase<Api, SupabaseAuth>;
pub type GetSpecialistPatientsWithProfilesUseCaseType =
    GetSpecialistPatientsWithProfilesUseCase<Api>;
pub type SpecialistProgramsDataUseCaseType = SpecialistProgramsDataUseCase<Api>;
pub type ListExerciseLibraryUseCaseType = ListExerciseLibraryUseCase<Api>;
pub type ListProgramScheduleUseCaseType = ListProgramScheduleUseCase<Api>;
pub type ListWorkoutLibraryUseCaseType = ListWorkoutLibraryUseCase<Api>;
pub type PatientProgressUseCaseType = PatientProgressUseCase<Api>;
pub type RemoveExerciseFromWorkoutUseCaseType = RemoveExerciseFromWorkoutUseCase<Api>;
pub type RestoreExerciseUseCaseType = RestoreExerciseUseCase<Api>;
pub type SoftDeleteExerciseUseCaseType = SoftDeleteExerciseUseCase<Api>;
pub type UpdateExerciseUseCaseType = UpdateExerciseUseCase<Api>;
pub type UpdateWorkoutUseCaseType = UpdateWorkoutUseCase<Api>;
pub type UpdateWorkoutExerciseUseCaseType = UpdateWorkoutExerciseUseCase<Api>;
pub type WorkoutEditorDataUseCaseType = WorkoutEditorDataUseCase<Api>;
