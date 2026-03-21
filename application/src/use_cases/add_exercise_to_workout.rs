use std::sync::Arc;

use domain::error::Result;
use domain::repositories::AddExerciseToWorkoutWrite;
use domain::vos::id::Id;
use domain::vos::{AccessToken, Reps, ScheduleOrderIndex, Sets};

#[derive(Clone)]
pub struct AddExerciseToWorkoutArgs {
    pub token: String,
    pub workout_id: String,
    pub exercise_id: String,
    pub order_index: i32,
    pub sets: i32,
    pub reps: i32,
}

pub struct AddExerciseToWorkoutUseCase<W: AddExerciseToWorkoutWrite> {
    catalog_write: Arc<W>,
}

impl<W: AddExerciseToWorkoutWrite> AddExerciseToWorkoutUseCase<W> {
    pub fn new(catalog_write: Arc<W>) -> Self {
        Self { catalog_write }
    }

    pub async fn execute(&self, args: AddExerciseToWorkoutArgs) -> Result<()> {
        let access = AccessToken::try_from(args.token)?;
        let workout_id = Id::try_from(args.workout_id)?;
        let exercise_id = Id::try_from(args.exercise_id)?;
        let order_index = ScheduleOrderIndex::try_from(args.order_index)?;
        let sets = Sets::try_from(args.sets)?;
        let reps = Reps::try_from(args.reps)?;
        self.catalog_write
            .add_exercise_to_workout(&access, &workout_id, &exercise_id, order_index, sets, reps)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::test_mocks::FakeAddExerciseToWorkout;

    const W: &str = "550e8400-e29b-41d4-a716-446655440140";
    const E: &str = "550e8400-e29b-41d4-a716-446655440141";

    #[tokio::test]
    async fn add_exercise_forwards_ids() {
        let fake = FakeAddExerciseToWorkout::new_ok();
        let uc = AddExerciseToWorkoutUseCase::new(Arc::new(fake.clone()));

        uc.execute(AddExerciseToWorkoutArgs {
            token: "t".to_string(),
            workout_id: W.to_string(),
            exercise_id: E.to_string(),
            order_index: 0,
            sets: 3,
            reps: 10,
        })
        .await
        .unwrap();
        let key = fake.last_key.lock().unwrap().clone().unwrap();

        assert_eq!(key.0.to_string(), W);
        assert_eq!(key.1.to_string(), E);
    }
}
