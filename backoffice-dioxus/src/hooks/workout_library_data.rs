use dioxus::prelude::*;

use crate::hooks::app_context::use_app_context;
use application::use_cases::list_workout_library::{ListWorkoutLibraryArgs, ListWorkoutLibraryUseCase};
use domain::error::DomainError;
use domain::entities::Workout;

#[derive(Clone)]
pub struct UseWorkoutLibraryData {
    pub resource: Resource<Result<Vec<Workout>, DomainError>>,
}

pub fn use_workout_library_data() -> UseWorkoutLibraryData {
    let app_context = use_app_context();
    let use_case: std::sync::Arc<
        ListWorkoutLibraryUseCase<infrastructure::supabase::api::Api>,
    > = app_context.list_workout_library_use_case();
    let session_signal = app_context.session();

    let use_case_clone = use_case.clone();
    let session_clone = session_signal.clone();

    let resource = use_resource(move || {
        let use_case = use_case_clone.clone();
        let session = session_clone.clone();
        async move {
            let sess_opt = session.read().clone();
            let Some(sess) = sess_opt else {
                return Err(DomainError::SessionNotFound);
            };

            let specialist_id = sess.user_id().to_string();

            let args = ListWorkoutLibraryArgs {
                token: sess.access_token().to_string(),
                specialist_id,
                name_filter: None,
            };

            use_case.execute(args).await
        }
    });

    UseWorkoutLibraryData { resource }
}
