use std::sync::Arc;

use crate::ports::Backend;
use domain::error::Result;

#[derive(Clone)]
pub struct UpdateWorkoutExerciseArgs {
    pub token: String,
    pub workout_id: String,
    pub exercise_id: String,
    pub sets: i32,
    pub reps: i32,
    pub order_index: Option<i32>,
}

pub struct UpdateWorkoutExerciseUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> UpdateWorkoutExerciseUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: UpdateWorkoutExerciseArgs) -> Result<()> {
        self.backend
            .update_workout_exercise(
                &args.token,
                &args.workout_id,
                &args.exercise_id,
                args.sets,
                args.reps,
                args.order_index,
            )
            .await
    }
}
