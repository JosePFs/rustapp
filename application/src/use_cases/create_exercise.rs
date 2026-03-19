use std::sync::Arc;

use crate::ports::Backend;
use domain::entities::Exercise;
use domain::error::Result;

#[derive(Clone)]
pub struct CreateExerciseArgs {
    pub token: String,
    pub specialist_id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
    pub video_url: Option<String>,
}

pub struct CreateExerciseUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> CreateExerciseUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: CreateExerciseArgs) -> Result<Exercise> {
        self.backend
            .create_exercise(
                &args.token,
                &args.specialist_id,
                &args.name,
                args.description.as_deref(),
                args.order_index,
                args.video_url.as_deref(),
            )
            .await
    }
}
