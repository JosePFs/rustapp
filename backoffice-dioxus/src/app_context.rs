use std::sync::Arc;

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
    use_cases::get_patient_programs::GetPatientProgramsUseCase,
    use_cases::get_specialist_patients_with_profiles::GetSpecialistPatientsWithProfilesUseCase,
    use_cases::list_exercise_library::ListExerciseLibraryUseCase,
    use_cases::list_program_schedule::ListProgramScheduleUseCase,
    use_cases::list_workout_library::ListWorkoutLibraryUseCase, use_cases::login::LoginUseCase,
    use_cases::patient_progress::PatientProgressUseCase,
    use_cases::patient_workout_session::PatientWorkoutSessionUseCase,
    use_cases::remove_exercise_from_workout::RemoveExerciseFromWorkoutUseCase,
    use_cases::restore_exercise::RestoreExerciseUseCase,
    use_cases::soft_delete_exercise::SoftDeleteExerciseUseCase,
    use_cases::specialist_programs_data::SpecialistProgramsDataUseCase,
    use_cases::update_exercise::UpdateExerciseUseCase,
    use_cases::update_patient_workout_feedback::UpdatePatientWorkoutFeedbackUseCase,
    use_cases::update_workout::UpdateWorkoutUseCase,
    use_cases::update_workout_exercise::UpdateWorkoutExerciseUseCase,
    use_cases::workout_editor_data::WorkoutEditorDataUseCase,
};
use dioxus::signals::Signal;
use domain::{error::Result, session::Session};
use infrastructure::supabase::{api::Api, client::SupabaseClient, config::SupabaseConfig};

#[derive(Clone)]
pub struct AppContext {
    session: Signal<Option<Session>>,
    add_exercise_to_workout_use_case: Arc<AddExerciseToWorkoutUseCase<Api>>,
    add_specialist_patient_use_case: Arc<AddSpecialistPatientUseCase<Api>>,
    assign_program_to_patient_use_case: Arc<AssignProgramToPatientUseCase<Api>>,
    create_exercise_use_case: Arc<CreateExerciseUseCase<Api>>,
    create_program_use_case: Arc<CreateProgramUseCase<Api>>,
    create_program_schedule_item_use_case: Arc<CreateProgramScheduleItemUseCase<Api>>,
    create_workout_use_case: Arc<CreateWorkoutUseCase<Api>>,
    delete_program_schedule_item_use_case: Arc<DeleteProgramScheduleItemUseCase<Api>>,
    delete_workout_use_case: Arc<DeleteWorkoutUseCase<Api>>,
    login_use_case: Arc<LoginUseCase<Api>>,
    get_patient_programs_use_case: Arc<GetPatientProgramsUseCase<Api>>,
    get_specialist_patients_with_profiles_use_case:
        Arc<GetSpecialistPatientsWithProfilesUseCase<Api>>,
    specialist_programs_data_use_case: Arc<SpecialistProgramsDataUseCase<Api>>,
    list_exercise_library_use_case: Arc<ListExerciseLibraryUseCase<Api>>,
    list_program_schedule_use_case: Arc<ListProgramScheduleUseCase<Api>>,
    list_workout_library_use_case: Arc<ListWorkoutLibraryUseCase<Api>>,
    patient_progress_use_case: Arc<PatientProgressUseCase<Api>>,
    patient_workout_session_use_case: Arc<PatientWorkoutSessionUseCase<Api>>,
    remove_exercise_from_workout_use_case: Arc<RemoveExerciseFromWorkoutUseCase<Api>>,
    restore_exercise_use_case: Arc<RestoreExerciseUseCase<Api>>,
    soft_delete_exercise_use_case: Arc<SoftDeleteExerciseUseCase<Api>>,
    submit_patient_workout_feedback_use_case: Arc<UpdatePatientWorkoutFeedbackUseCase<Api>>,
    update_exercise_use_case: Arc<UpdateExerciseUseCase<Api>>,
    update_workout_use_case: Arc<UpdateWorkoutUseCase<Api>>,
    update_workout_exercise_use_case: Arc<UpdateWorkoutExerciseUseCase<Api>>,
    workout_editor_data_use_case: Arc<WorkoutEditorDataUseCase<Api>>,
}

impl AppContext {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        session: Option<Session>,
        add_exercise_to_workout_use_case: Arc<AddExerciseToWorkoutUseCase<Api>>,
        add_specialist_patient_use_case: Arc<AddSpecialistPatientUseCase<Api>>,
        assign_program_to_patient_use_case: Arc<AssignProgramToPatientUseCase<Api>>,
        create_exercise_use_case: Arc<CreateExerciseUseCase<Api>>,
        create_program_use_case: Arc<CreateProgramUseCase<Api>>,
        create_program_schedule_item_use_case: Arc<CreateProgramScheduleItemUseCase<Api>>,
        create_workout_use_case: Arc<CreateWorkoutUseCase<Api>>,
        delete_program_schedule_item_use_case: Arc<DeleteProgramScheduleItemUseCase<Api>>,
        delete_workout_use_case: Arc<DeleteWorkoutUseCase<Api>>,
        login_use_case: Arc<LoginUseCase<Api>>,
        get_patient_programs_use_case: Arc<GetPatientProgramsUseCase<Api>>,
        get_specialist_patients_with_profiles_use_case: Arc<
            GetSpecialistPatientsWithProfilesUseCase<Api>,
        >,
        specialist_programs_data_use_case: Arc<SpecialistProgramsDataUseCase<Api>>,
        list_exercise_library_use_case: Arc<ListExerciseLibraryUseCase<Api>>,
        list_program_schedule_use_case: Arc<ListProgramScheduleUseCase<Api>>,
        list_workout_library_use_case: Arc<ListWorkoutLibraryUseCase<Api>>,
        patient_progress_use_case: Arc<PatientProgressUseCase<Api>>,
        patient_workout_session_use_case: Arc<PatientWorkoutSessionUseCase<Api>>,
        remove_exercise_from_workout_use_case: Arc<RemoveExerciseFromWorkoutUseCase<Api>>,
        restore_exercise_use_case: Arc<RestoreExerciseUseCase<Api>>,
        soft_delete_exercise_use_case: Arc<SoftDeleteExerciseUseCase<Api>>,
        submit_patient_workout_feedback_use_case: Arc<UpdatePatientWorkoutFeedbackUseCase<Api>>,
        update_exercise_use_case: Arc<UpdateExerciseUseCase<Api>>,
        update_workout_use_case: Arc<UpdateWorkoutUseCase<Api>>,
        update_workout_exercise_use_case: Arc<UpdateWorkoutExerciseUseCase<Api>>,
        workout_editor_data_use_case: Arc<WorkoutEditorDataUseCase<Api>>,
    ) -> Self {
        Self {
            session: Signal::new(session),
            add_exercise_to_workout_use_case,
            add_specialist_patient_use_case,
            assign_program_to_patient_use_case,
            create_exercise_use_case,
            create_program_use_case,
            create_program_schedule_item_use_case,
            create_workout_use_case,
            delete_program_schedule_item_use_case,
            delete_workout_use_case,
            login_use_case,
            get_patient_programs_use_case,
            get_specialist_patients_with_profiles_use_case,
            specialist_programs_data_use_case,
            list_exercise_library_use_case,
            list_program_schedule_use_case,
            list_workout_library_use_case,
            patient_progress_use_case,
            patient_workout_session_use_case,
            remove_exercise_from_workout_use_case,
            restore_exercise_use_case,
            soft_delete_exercise_use_case,
            submit_patient_workout_feedback_use_case,
            update_exercise_use_case,
            update_workout_use_case,
            update_workout_exercise_use_case,
            workout_editor_data_use_case,
        }
    }

    pub fn session(&self) -> Signal<Option<Session>> {
        self.session
    }

    pub fn add_specialist_patient_use_case(&self) -> Arc<AddSpecialistPatientUseCase<Api>> {
        self.add_specialist_patient_use_case.clone()
    }

    pub fn add_exercise_to_workout_use_case(&self) -> Arc<AddExerciseToWorkoutUseCase<Api>> {
        self.add_exercise_to_workout_use_case.clone()
    }

    pub fn assign_program_to_patient_use_case(&self) -> Arc<AssignProgramToPatientUseCase<Api>> {
        self.assign_program_to_patient_use_case.clone()
    }

    pub fn create_exercise_use_case(&self) -> Arc<CreateExerciseUseCase<Api>> {
        self.create_exercise_use_case.clone()
    }

    pub fn create_program_use_case(&self) -> Arc<CreateProgramUseCase<Api>> {
        self.create_program_use_case.clone()
    }

    pub fn create_program_schedule_item_use_case(
        &self,
    ) -> Arc<CreateProgramScheduleItemUseCase<Api>> {
        self.create_program_schedule_item_use_case.clone()
    }

    pub fn create_workout_use_case(&self) -> Arc<CreateWorkoutUseCase<Api>> {
        self.create_workout_use_case.clone()
    }

    pub fn delete_workout_use_case(&self) -> Arc<DeleteWorkoutUseCase<Api>> {
        self.delete_workout_use_case.clone()
    }

    pub fn delete_program_schedule_item_use_case(
        &self,
    ) -> Arc<DeleteProgramScheduleItemUseCase<Api>> {
        self.delete_program_schedule_item_use_case.clone()
    }

    pub fn login_use_case(&self) -> Arc<LoginUseCase<Api>> {
        self.login_use_case.clone()
    }

    pub fn get_patient_programs_use_case(&self) -> Arc<GetPatientProgramsUseCase<Api>> {
        self.get_patient_programs_use_case.clone()
    }

    pub fn get_specialist_patients_with_profiles_use_case(
        &self,
    ) -> Arc<GetSpecialistPatientsWithProfilesUseCase<Api>> {
        self.get_specialist_patients_with_profiles_use_case.clone()
    }

    pub fn specialist_programs_data_use_case(&self) -> Arc<SpecialistProgramsDataUseCase<Api>> {
        self.specialist_programs_data_use_case.clone()
    }

    pub fn list_exercise_library_use_case(&self) -> Arc<ListExerciseLibraryUseCase<Api>> {
        self.list_exercise_library_use_case.clone()
    }

    pub fn list_program_schedule_use_case(&self) -> Arc<ListProgramScheduleUseCase<Api>> {
        self.list_program_schedule_use_case.clone()
    }

    pub fn list_workout_library_use_case(&self) -> Arc<ListWorkoutLibraryUseCase<Api>> {
        self.list_workout_library_use_case.clone()
    }

    pub fn patient_progress_use_case(&self) -> Arc<PatientProgressUseCase<Api>> {
        self.patient_progress_use_case.clone()
    }

    pub fn patient_workout_session_use_case(&self) -> Arc<PatientWorkoutSessionUseCase<Api>> {
        self.patient_workout_session_use_case.clone()
    }

    pub fn remove_exercise_from_workout_use_case(
        &self,
    ) -> Arc<RemoveExerciseFromWorkoutUseCase<Api>> {
        self.remove_exercise_from_workout_use_case.clone()
    }

    pub fn restore_exercise_use_case(&self) -> Arc<RestoreExerciseUseCase<Api>> {
        self.restore_exercise_use_case.clone()
    }

    pub fn soft_delete_exercise_use_case(&self) -> Arc<SoftDeleteExerciseUseCase<Api>> {
        self.soft_delete_exercise_use_case.clone()
    }

    pub fn submit_patient_workout_feedback_use_case(
        &self,
    ) -> Arc<UpdatePatientWorkoutFeedbackUseCase<Api>> {
        self.submit_patient_workout_feedback_use_case.clone()
    }

    pub fn update_exercise_use_case(&self) -> Arc<UpdateExerciseUseCase<Api>> {
        self.update_exercise_use_case.clone()
    }

    pub fn update_workout_use_case(&self) -> Arc<UpdateWorkoutUseCase<Api>> {
        self.update_workout_use_case.clone()
    }

    pub fn update_workout_exercise_use_case(&self) -> Arc<UpdateWorkoutExerciseUseCase<Api>> {
        self.update_workout_exercise_use_case.clone()
    }

    pub fn workout_editor_data_use_case(&self) -> Arc<WorkoutEditorDataUseCase<Api>> {
        self.workout_editor_data_use_case.clone()
    }
}

pub fn build_app_context() -> Result<AppContext> {
    let config = SupabaseConfig::from_env()?;

    let api = Api::new(SupabaseClient::new(config));
    let backend = Arc::new(api);

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
    let login_use_case = Arc::new(LoginUseCase::<Api>::new(backend.clone()));
    let get_patient_programs_use_case =
        Arc::new(GetPatientProgramsUseCase::<Api>::new(backend.clone()));
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
    let patient_workout_session_use_case =
        Arc::new(PatientWorkoutSessionUseCase::<Api>::new(backend.clone()));
    let remove_exercise_from_workout_use_case = Arc::new(
        RemoveExerciseFromWorkoutUseCase::<Api>::new(backend.clone()),
    );
    let restore_exercise_use_case = Arc::new(RestoreExerciseUseCase::<Api>::new(backend.clone()));
    let soft_delete_exercise_use_case =
        Arc::new(SoftDeleteExerciseUseCase::<Api>::new(backend.clone()));
    let submit_patient_workout_feedback_use_case = Arc::new(UpdatePatientWorkoutFeedbackUseCase::<
        Api,
    >::new(backend.clone()));
    let update_workout_use_case = Arc::new(UpdateWorkoutUseCase::<Api>::new(backend.clone()));
    let update_exercise_use_case = Arc::new(UpdateExerciseUseCase::<Api>::new(backend.clone()));
    let update_workout_exercise_use_case =
        Arc::new(UpdateWorkoutExerciseUseCase::<Api>::new(backend.clone()));
    let workout_editor_data_use_case =
        Arc::new(WorkoutEditorDataUseCase::<Api>::new(backend.clone()));

    Ok(AppContext::new(
        None,
        add_exercise_to_workout_use_case,
        add_specialist_patient_use_case,
        assign_program_to_patient_use_case,
        create_exercise_use_case,
        create_program_use_case,
        create_program_schedule_item_use_case,
        create_workout_use_case,
        delete_program_schedule_item_use_case,
        delete_workout_use_case,
        login_use_case,
        get_patient_programs_use_case,
        get_specialist_patients_with_profiles_use_case,
        specialist_programs_data_use_case,
        list_exercise_library_use_case,
        list_program_schedule_use_case,
        list_workout_library_use_case,
        patient_progress_use_case,
        patient_workout_session_use_case,
        remove_exercise_from_workout_use_case,
        restore_exercise_use_case,
        soft_delete_exercise_use_case,
        submit_patient_workout_feedback_use_case,
        update_exercise_use_case,
        update_workout_use_case,
        update_workout_exercise_use_case,
        workout_editor_data_use_case,
    ))
}
