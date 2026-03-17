use std::sync::Arc;

use crate::application::Backend;
use crate::domain::error::Result;

#[derive(Clone)]
pub struct SoftDeleteExerciseArgs {
    pub token: String,
    pub exercise_id: String,
}

pub struct SoftDeleteExerciseUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> SoftDeleteExerciseUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: SoftDeleteExerciseArgs) -> Result<()> {
        self.backend
            .soft_delete_exercise(&args.token, &args.exercise_id)
            .await
    }
}
