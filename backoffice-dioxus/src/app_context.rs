use std::sync::Arc;

use dioxus::signals::Signal;

use application::facade::BackofficeFacade;
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
use infrastructure::supabase::auth::SupabaseAuth;
use infrastructure::supabase::repositories::{
    SupabaseRestRepository, SupabaseRestRepositoryBuilder,
};

#[derive(Clone)]
pub struct AppContext {
    session: Signal<Option<Session>>,
    backoffice: BackofficeFacadeHandle,
}

impl AppContext {
    pub fn new(session: Option<Session>, backoffice: BackofficeFacadeHandle) -> Self {
        Self {
            session: Signal::new(session),
            backoffice,
        }
    }

    pub fn session(&self) -> Signal<Option<Session>> {
        self.session
    }

    pub fn backoffice_facade(&self) -> BackofficeFacadeHandle {
        self.backoffice.clone()
    }
}

pub fn build_app_context() -> Result<AppContext> {
    let auth = Arc::new(SupabaseAuth::builder().build());
    let repository = Arc::new(SupabaseRestRepositoryBuilder::new().build());

    let add_specialist_patient_use_case = Arc::new(AddSpecialistPatientUseCase::<
        SupabaseRestRepository,
    >::new(repository.clone()));
    let add_exercise_to_workout_use_case = Arc::new(AddExerciseToWorkoutUseCase::<
        SupabaseRestRepository,
    >::new(repository.clone()));
    let assign_program_to_patient_use_case =
        Arc::new(AssignProgramToPatientUseCase::<SupabaseRestRepository>::new(repository.clone()));
    let create_exercise_use_case = Arc::new(CreateExerciseUseCase::<SupabaseRestRepository>::new(
        repository.clone(),
    ));
    let create_program_use_case = Arc::new(CreateProgramUseCase::<SupabaseRestRepository>::new(
        repository.clone(),
    ));
    let create_program_schedule_item_use_case = Arc::new(CreateProgramScheduleItemUseCase::<
        SupabaseRestRepository,
    >::new(repository.clone()));
    let create_workout_use_case = Arc::new(CreateWorkoutUseCase::<SupabaseRestRepository>::new(
        repository.clone(),
    ));
    let delete_program_schedule_item_use_case = Arc::new(DeleteProgramScheduleItemUseCase::<
        SupabaseRestRepository,
    >::new(repository.clone()));
    let delete_workout_use_case = Arc::new(DeleteWorkoutUseCase::<SupabaseRestRepository>::new(
        repository.clone(),
    ));
    let login_use_case = Arc::new(LoginUseCase::<SupabaseRestRepository, SupabaseAuth>::new(
        repository.clone(),
        auth.clone(),
    ));
    let get_specialist_patients_with_profiles_use_case =
        Arc::new(GetSpecialistPatientsWithProfilesUseCase::<
            SupabaseRestRepository,
        >::new(repository.clone()));
    let specialist_programs_data_use_case =
        Arc::new(SpecialistProgramsDataUseCase::<SupabaseRestRepository>::new(repository.clone()));
    let list_exercise_library_use_case = Arc::new(ListExerciseLibraryUseCase::<
        SupabaseRestRepository,
    >::new(repository.clone()));
    let list_program_schedule_use_case = Arc::new(ListProgramScheduleUseCase::<
        SupabaseRestRepository,
    >::new(repository.clone()));
    let list_workout_library_use_case = Arc::new(
        ListWorkoutLibraryUseCase::<SupabaseRestRepository>::new(repository.clone()),
    );
    let patient_progress_use_case = Arc::new(
        PatientProgressUseCase::<SupabaseRestRepository>::new(repository.clone()),
    );
    let remove_exercise_from_workout_use_case = Arc::new(RemoveExerciseFromWorkoutUseCase::<
        SupabaseRestRepository,
    >::new(repository.clone()));
    let restore_exercise_use_case = Arc::new(
        RestoreExerciseUseCase::<SupabaseRestRepository>::new(repository.clone()),
    );
    let soft_delete_exercise_use_case = Arc::new(
        SoftDeleteExerciseUseCase::<SupabaseRestRepository>::new(repository.clone()),
    );
    let update_workout_use_case = Arc::new(UpdateWorkoutUseCase::<SupabaseRestRepository>::new(
        repository.clone(),
    ));
    let update_exercise_use_case = Arc::new(UpdateExerciseUseCase::<SupabaseRestRepository>::new(
        repository.clone(),
    ));
    let update_workout_exercise_use_case = Arc::new(UpdateWorkoutExerciseUseCase::<
        SupabaseRestRepository,
    >::new(repository.clone()));
    let workout_editor_data_use_case = Arc::new(
        WorkoutEditorDataUseCase::<SupabaseRestRepository>::new(repository.clone()),
    );

    let backoffice_facade = Arc::new(BackofficeFacade {
        login_uc: login_use_case,
        add_specialist_patient_uc: add_specialist_patient_use_case,
        add_exercise_to_workout_uc: add_exercise_to_workout_use_case,
        assign_program_to_patient_uc: assign_program_to_patient_use_case,
        create_exercise_uc: create_exercise_use_case,
        create_program_uc: create_program_use_case,
        create_program_schedule_item_uc: create_program_schedule_item_use_case,
        create_workout_uc: create_workout_use_case,
        delete_program_schedule_item_uc: delete_program_schedule_item_use_case,
        delete_workout_uc: delete_workout_use_case,
        get_specialist_patients_with_profiles_uc: get_specialist_patients_with_profiles_use_case,
        specialist_programs_data_uc: specialist_programs_data_use_case,
        list_exercise_library_uc: list_exercise_library_use_case,
        list_program_schedule_uc: list_program_schedule_use_case,
        list_workout_library_uc: list_workout_library_use_case,
        patient_progress_uc: patient_progress_use_case,
        remove_exercise_from_workout_uc: remove_exercise_from_workout_use_case,
        restore_exercise_uc: restore_exercise_use_case,
        soft_delete_exercise_uc: soft_delete_exercise_use_case,
        update_exercise_uc: update_exercise_use_case,
        update_workout_uc: update_workout_use_case,
        update_workout_exercise_uc: update_workout_exercise_use_case,
        workout_editor_data_uc: workout_editor_data_use_case,
    });

    Ok(AppContext::new(None, backoffice_facade))
}

pub type BackofficeFacadeHandle = Arc<BackofficeFacade<SupabaseRestRepository, SupabaseAuth>>;
