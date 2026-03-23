use std::sync::Arc;

use crate::ports::error::{ApplicationError, Result};
use domain::repositories::AddExerciseToWorkoutWrite;
use domain::vos::id::Id;
use domain::vos::{Reps, ScheduleOrderIndex, Sets};

#[derive(Clone)]
pub struct AddExerciseToWorkoutArgs {
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
        let workout_id = Id::try_from(args.workout_id)?;
        let exercise_id = Id::try_from(args.exercise_id)?;
        let order_index = ScheduleOrderIndex::try_from(args.order_index)?;
        let sets = Sets::try_from(args.sets)?;
        let reps = Reps::try_from(args.reps)?;
        self.catalog_write
            .add_exercise_to_workout(&workout_id, &exercise_id, order_index, sets, reps)
            .await
            .map_err(ApplicationError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Mutex;

    use domain::error::Result;

    const W: &str = "550e8400-e29b-41d4-a716-446655440140";
    const E: &str = "550e8400-e29b-41d4-a716-446655440141";

    #[tokio::test]
    async fn add_exercise_forwards_ids() {
        let fake = MockAddExerciseToWorkoutWrite::new_ok();
        let uc = AddExerciseToWorkoutUseCase::new(Arc::new(fake.clone()));

        uc.execute(AddExerciseToWorkoutArgs {
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

    #[derive(Clone)]
    struct MockAddExerciseToWorkoutWrite {
        last_key: Arc<Mutex<Option<(Id, Id)>>>,
        outcome: Arc<Mutex<Result<()>>>,
    }

    impl MockAddExerciseToWorkoutWrite {
        fn new_ok() -> Self {
            Self {
                last_key: Arc::new(Mutex::new(None)),
                outcome: Arc::new(Mutex::new(Ok(()))),
            }
        }
    }

    #[common::async_trait_platform]
    impl AddExerciseToWorkoutWrite for MockAddExerciseToWorkoutWrite {
        async fn add_exercise_to_workout(
            &self,
            workout_id: &Id,
            exercise_id: &Id,
            _order_index: ScheduleOrderIndex,
            _sets: Sets,
            _reps: Reps,
        ) -> Result<()> {
            *self.last_key.lock().unwrap() = Some((workout_id.clone(), exercise_id.clone()));
            self.outcome.lock().unwrap().clone()
        }
    }
}
