use std::sync::Arc;

use crate::error::{ApplicationError, Result};
use domain::repositories::UpdateWorkoutExerciseWrite;
use domain::vos::id::Id;
use domain::vos::{Reps, ScheduleOrderIndex, Sets};

#[derive(Clone)]
pub struct UpdateWorkoutExerciseArgs {
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
        let workout_id = Id::try_from(args.workout_id)?;
        let exercise_id = Id::try_from(args.exercise_id)?;
        let sets = Sets::try_from(args.sets)?;
        let reps = Reps::try_from(args.reps)?;
        let order_index = args
            .order_index
            .map(ScheduleOrderIndex::try_from)
            .transpose()?;
        self.catalog_write
            .update_workout_exercise(&workout_id, &exercise_id, sets, reps, order_index)
            .await
            .map_err(ApplicationError::from)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    use domain::error::Result;
    use domain::repositories::UpdateWorkoutExerciseWrite;

    const W: &str = "550e8400-e29b-41d4-a716-446655440300";
    const E: &str = "550e8400-e29b-41d4-a716-446655440301";

    #[tokio::test]
    async fn update_workout_exercise_forwards_ids_and_sets_reps() {
        let fake = MockUpdateWorkoutExerciseWrite::new_ok();
        let uc = UpdateWorkoutExerciseUseCase::new(Arc::new(fake.clone()));

        uc.execute(UpdateWorkoutExerciseArgs {
            workout_id: W.to_string(),
            exercise_id: E.to_string(),
            sets: 4,
            reps: 12,
            order_index: None,
        })
        .await
        .unwrap();

        let got = fake.last_call.lock().unwrap().clone().unwrap();
        assert_eq!(got.0.to_string(), W);
        assert_eq!(got.1.to_string(), E);
        assert_eq!(got.2, 4);
        assert_eq!(got.3, 12);
        assert!(got.4.is_none());
    }

    #[derive(Clone)]
    struct MockUpdateWorkoutExerciseWrite {
        last_call: Arc<Mutex<Option<(Id, Id, i32, i32, Option<ScheduleOrderIndex>)>>>,
        outcome: Arc<Mutex<Result<()>>>,
    }

    impl MockUpdateWorkoutExerciseWrite {
        fn new_ok() -> Self {
            Self {
                last_call: Arc::new(Mutex::new(None)),
                outcome: Arc::new(Mutex::new(Ok(()))),
            }
        }
    }

    #[common::async_trait_platform]
    impl UpdateWorkoutExerciseWrite for MockUpdateWorkoutExerciseWrite {
        async fn update_workout_exercise(
            &self,
            workout_id: &Id,
            exercise_id: &Id,
            sets: Sets,
            reps: Reps,
            order_index: Option<ScheduleOrderIndex>,
        ) -> Result<()> {
            *self.last_call.lock().unwrap() = Some((
                workout_id.clone(),
                exercise_id.clone(),
                sets.value(),
                reps.value(),
                order_index,
            ));
            self.outcome.lock().unwrap().clone()
        }
    }
}
