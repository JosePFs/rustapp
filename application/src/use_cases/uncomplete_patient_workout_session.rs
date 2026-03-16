use std::sync::Arc;

use crate::application::Backend;
use crate::domain::error::Result;

#[derive(Clone)]
pub struct UncompletePatientWorkoutSessionArgs {
    pub token: String,
    pub session_id: String,
}

pub struct UncompletePatientWorkoutSessionUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> UncompletePatientWorkoutSessionUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: UncompletePatientWorkoutSessionArgs) -> Result<()> {
        self.backend
            .uncomplete_session(&args.token, &args.session_id)
            .await
    }
}
