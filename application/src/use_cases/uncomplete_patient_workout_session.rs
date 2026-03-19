use std::sync::Arc;

use crate::ports::MobileBackend;
use domain::error::Result;

#[derive(Clone)]
pub struct UncompletePatientWorkoutSessionArgs {
    pub token: String,
    pub workout_session_id: String,
}

pub struct UncompletePatientWorkoutSessionUseCase<B: MobileBackend> {
    backend: Arc<B>,
}

impl<B: MobileBackend> UncompletePatientWorkoutSessionUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: UncompletePatientWorkoutSessionArgs) -> Result<()> {
        self.backend
            .uncomplete_session(&args.token, &args.workout_session_id)
            .await
    }
}
