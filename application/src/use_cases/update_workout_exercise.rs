use std::sync::Arc;

use domain::error::Result;
use domain::repositories::UpdateWorkoutExerciseWrite;
use domain::vos::id::Id;
use domain::vos::{AccessToken, Reps, ScheduleOrderIndex, Sets};

#[derive(Clone)]
pub struct UpdateWorkoutExerciseArgs {
    pub token: String,
    pub workout_id: String,
    pub exercise_id: String,
    pub sets: i32,
    pub reps: i32,
    pub order_index: Option<i32>,
}

pub struct UpdateWorkoutExerciseUseCase<W: UpdateWorkoutExerciseWrite> {
    catalog_write: Arc<W>,
}

impl<W: UpdateWorkoutExerciseWrite> UpdateWorkoutExerciseUseCase<W> {
    pub fn new(catalog_write: Arc<W>) -> Self {
        Self { catalog_write }
    }

    pub async fn execute(&self, args: UpdateWorkoutExerciseArgs) -> Result<()> {
        let access = AccessToken::try_from(args.token)?;
        let workout_id = Id::try_from(args.workout_id)?;
        let exercise_id = Id::try_from(args.exercise_id)?;
        let sets = Sets::try_from(args.sets)?;
        let reps = Reps::try_from(args.reps)?;
        let order_index = args
            .order_index
            .map(ScheduleOrderIndex::try_from)
            .transpose()?;
        self.catalog_write
            .update_workout_exercise(&access, &workout_id, &exercise_id, sets, reps, order_index)
            .await
    }
}
