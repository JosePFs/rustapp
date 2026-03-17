use std::sync::Arc;

use crate::application::Backend;
use crate::domain::error::Result;

#[derive(Clone)]
pub struct AddExerciseToWorkoutArgs {
    pub token: String,
    pub workout_id: String,
    pub exercise_id: String,
    pub order_index: i32,
    pub sets: i32,
    pub reps: i32,
}

pub struct AddExerciseToWorkoutUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> AddExerciseToWorkoutUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: AddExerciseToWorkoutArgs) -> Result<()> {
        self.backend
            .add_exercise_to_workout(
                &args.token,
                &args.workout_id,
                &args.exercise_id,
                args.order_index,
                args.sets,
                args.reps,
            )
            .await
    }
}
