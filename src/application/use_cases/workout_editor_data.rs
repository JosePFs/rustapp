use std::sync::Arc;

use futures::try_join;

use crate::application::Backend;
use crate::domain::entities::{Exercise, Workout, WorkoutExercise};

#[derive(Clone)]
pub struct WorkoutEditorDataArgs {
    pub token: String,
    pub specialist_id: String,
    pub workout_id: String,
}

#[derive(Clone, Debug)]
pub struct WorkoutEditorDataResult {
    pub workout: Option<Workout>,
    pub exercises: Vec<WorkoutExercise>,
    pub library: Vec<Exercise>,
}

pub struct WorkoutEditorDataUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> WorkoutEditorDataUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(
        &self,
        args: WorkoutEditorDataArgs,
    ) -> Result<WorkoutEditorDataResult, crate::domain::error::DomainError> {
        let token = args.token;
        let specialist_id = args.specialist_id;
        let workout_id = args.workout_id.clone();
        let workout_ids = [workout_id.clone()];

        let (workouts, exercises, library) = try_join!(
            self.backend.get_workouts_by_ids(&token, &workout_ids),
            self.backend.list_exercises_for_workout(&token, &workout_id),
            self.backend.list_exercise_library(&token, &specialist_id, None),
        )?;

        let workout = workouts.into_iter().next();
        Ok(WorkoutEditorDataResult {
            workout,
            exercises,
            library,
        })
    }
}
