use std::sync::Arc;

use futures::try_join;

use crate::ports::Backend;
use domain::{
    entities::{Exercise, Workout, WorkoutExercise},
    error::Result,
};

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

    pub async fn execute(&self, args: WorkoutEditorDataArgs) -> Result<WorkoutEditorDataResult> {
        let token = args.token;
        let specialist_id = args.specialist_id;
        let workout_id = args.workout_id;

        let (workout_with_exercises, library) = try_join!(
            self.backend.get_workout_with_exercises(&token, &workout_id),
            self.backend
                .list_exercise_library(&token, &specialist_id, None),
        )?;

        let (workout, exercises) = workout_with_exercises
            .map(|w| (Some(w.workout), w.exercises))
            .unwrap_or((None, vec![]));

        Ok(WorkoutEditorDataResult {
            workout,
            exercises,
            library,
        })
    }
}
