use std::sync::Arc;

use crate::ports::Backend;
use domain::entities::Exercise;
use domain::error::Result;

#[derive(Clone)]
pub struct ListExerciseLibraryArgs {
    pub token: String,
    pub specialist_id: String,
    pub name_filter: Option<String>,
}

pub struct ListExerciseLibraryUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> ListExerciseLibraryUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: ListExerciseLibraryArgs) -> Result<Vec<Exercise>> {
        let filter = args.name_filter.as_deref().filter(|s| !s.is_empty());
        self.backend
            .list_exercise_library(&args.token, &args.specialist_id, filter)
            .await
    }
}
