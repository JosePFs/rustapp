use std::sync::Arc;

use crate::application::Backend;
use crate::domain::error::Result;

#[derive(Clone)]
pub struct DeleteWorkoutArgs {
    pub token: String,
    pub workout_id: String,
}

pub struct DeleteWorkoutUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> DeleteWorkoutUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: DeleteWorkoutArgs) -> Result<()> {
        self.backend
            .delete_workout(&args.token, &args.workout_id)
            .await
    }
}
