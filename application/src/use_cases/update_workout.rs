use std::sync::Arc;

use crate::ports::Backend;
use domain::error::Result;

#[derive(Clone)]
pub struct UpdateWorkoutArgs {
    pub token: String,
    pub workout_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone)]
pub struct UpdateWorkoutInput {
    pub workout_id: String,
    pub name: String,
    pub description: String,
}

pub struct UpdateWorkoutUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> UpdateWorkoutUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: UpdateWorkoutArgs) -> Result<()> {
        self.backend
            .update_workout(
                &args.token,
                &args.workout_id,
                args.name.as_deref(),
                args.description.as_ref().map(|s| Some(s.as_str())),
                None,
            )
            .await
    }
}
