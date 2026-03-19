use std::sync::Arc;

use crate::ports::Backend;
use domain::error::Result;

#[derive(Clone)]
pub struct RemoveExerciseFromWorkoutArgs {
    pub token: String,
    pub workout_id: String,
    pub exercise_id: String,
}

pub struct RemoveExerciseFromWorkoutUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> RemoveExerciseFromWorkoutUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: RemoveExerciseFromWorkoutArgs) -> Result<()> {
        self.backend
            .remove_exercise_from_workout(&args.token, &args.workout_id, &args.exercise_id)
            .await
    }
}
