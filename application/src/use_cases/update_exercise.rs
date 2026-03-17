use std::sync::Arc;

use crate::application::Backend;
use crate::domain::error::Result;

#[derive(Clone)]
pub struct UpdateExerciseArgs {
    pub token: String,
    pub exercise_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub order_index: Option<i32>,
    pub video_url: Option<String>,
}

pub struct UpdateExerciseUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> UpdateExerciseUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: UpdateExerciseArgs) -> Result<()> {
        self.backend
            .update_exercise(
                &args.token,
                &args.exercise_id,
                args.name.as_deref(),
                args.description.as_ref().map(|s| s.as_str()),
                args.order_index,
                args.video_url.as_ref().map(|s| Some(s.as_str())),
            )
            .await
    }
}
