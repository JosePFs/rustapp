use std::sync::Arc;

use crate::application::Backend;
use crate::domain::entities::Workout;
use crate::domain::error::Result;

#[derive(Clone)]
pub struct CreateWorkoutArgs {
    pub token: String,
    pub specialist_id: String,
    pub name: String,
    pub description: Option<String>,
}

pub struct CreateWorkoutUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> CreateWorkoutUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: CreateWorkoutArgs) -> Result<Workout> {
        self.backend
            .create_workout(
                &args.token,
                &args.specialist_id,
                &args.name,
                args.description.as_deref(),
            )
            .await
    }
}
