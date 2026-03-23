use application::ports::error::ApplicationError;
use dioxus::prelude::*;

use crate::hooks::app_context::use_app_context;
use application::ports::error::Result;
use application::ports::BackofficeApi;
use application::use_cases::list_workout_library::{ListWorkoutLibraryArgs, WorkoutLibraryItem};

#[derive(Clone)]
pub struct UseWorkoutLibraryData {
    pub resource: Resource<Result<Vec<WorkoutLibraryItem>>>,
}

pub fn use_workout_library_data() -> UseWorkoutLibraryData {
    let app_context = use_app_context();
    let facade = app_context.backoffice_facade();
    let session_signal = app_context.session();

    let facade_clone = facade.clone();
    let session_clone = session_signal.clone();

    let resource = use_resource(move || {
        let facade = facade_clone.clone();
        let session = session_clone.clone();
        async move {
            let sess_opt = session.read().clone();
            let Some(sess) = sess_opt else {
                return Err(ApplicationError::NoSession);
            };

            let specialist_id = sess.user_id().to_string();

            let args = ListWorkoutLibraryArgs {
                specialist_id,
                name_filter: None,
            };

            facade.list_workout_library(args).await
        }
    });

    UseWorkoutLibraryData { resource }
}
