use std::sync::Arc;

use crate::ports::Backend;
use domain::error::Result;

#[derive(Clone)]
pub struct RestoreExerciseArgs {
    pub token: String,
    pub exercise_id: String,
}

pub struct RestoreExerciseUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> RestoreExerciseUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: RestoreExerciseArgs) -> Result<()> {
        self.backend
            .restore_exercise(&args.token, &args.exercise_id)
            .await
    }
}
