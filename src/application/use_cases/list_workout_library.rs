use std::sync::Arc;

use crate::application::Backend;
use crate::domain::entities::Workout;
use crate::domain::error::Result;

#[derive(Clone)]
pub struct ListWorkoutLibraryArgs {
    pub token: String,
    pub specialist_id: String,
    pub name_filter: Option<String>,
}

pub struct ListWorkoutLibraryUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> ListWorkoutLibraryUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: ListWorkoutLibraryArgs) -> Result<Vec<Workout>> {
        let filter = args
            .name_filter
            .as_deref()
            .filter(|s| !s.is_empty());
        self.backend
            .list_workout_library(&args.token, &args.specialist_id, filter)
            .await
    }
}
