use std::sync::Arc;

use domain::error::Result;
use domain::repositories::RemoveExerciseFromWorkoutWrite;
use domain::vos::id::Id;
use domain::vos::AccessToken;

#[derive(Clone)]
pub struct RemoveExerciseFromWorkoutArgs {
    pub token: String,
    pub workout_id: String,
    pub exercise_id: String,
}

pub struct RemoveExerciseFromWorkoutUseCase<W: RemoveExerciseFromWorkoutWrite> {
    catalog_write: Arc<W>,
}

impl<W: RemoveExerciseFromWorkoutWrite> RemoveExerciseFromWorkoutUseCase<W> {
    pub fn new(catalog_write: Arc<W>) -> Self {
        Self { catalog_write }
    }

    pub async fn execute(&self, args: RemoveExerciseFromWorkoutArgs) -> Result<()> {
        let access = AccessToken::try_from(args.token)?;
        let workout_id = Id::try_from(args.workout_id)?;
        let exercise_id = Id::try_from(args.exercise_id)?;
        self.catalog_write
            .remove_exercise_from_workout(&access, &workout_id, &exercise_id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use domain::error::Result;
    use domain::repositories::RemoveExerciseFromWorkoutWrite;
    use domain::vos::AccessToken;

    const W: &str = "550e8400-e29b-41d4-a716-446655440130";
    const E: &str = "550e8400-e29b-41d4-a716-446655440131";

    #[tokio::test]
    async fn remove_forwards_pair() {
        let fake = MockRemoveExerciseFromWorkoutWrite::new_ok();
        let uc = RemoveExerciseFromWorkoutUseCase::new(Arc::new(fake.clone()));

        uc.execute(RemoveExerciseFromWorkoutArgs {
            token: "t".to_string(),
            workout_id: W.to_string(),
            exercise_id: E.to_string(),
        })
        .await
        .unwrap();
        let pair = fake.last_pair.lock().unwrap().clone().unwrap();

        assert_eq!(pair.0.to_string(), W);
        assert_eq!(pair.1.to_string(), E);
    }

    #[derive(Clone)]
    struct MockRemoveExerciseFromWorkoutWrite {
        last_pair: Arc<Mutex<Option<(Id, Id)>>>,
        outcome: Arc<Mutex<Result<()>>>,
    }

    impl MockRemoveExerciseFromWorkoutWrite {
        fn new_ok() -> Self {
            Self {
                last_pair: Arc::new(Mutex::new(None)),
                outcome: Arc::new(Mutex::new(Ok(()))),
            }
        }
    }

    #[common::async_trait_platform]
    impl RemoveExerciseFromWorkoutWrite for MockRemoveExerciseFromWorkoutWrite {
        async fn remove_exercise_from_workout(
            &self,
            _access_token: &AccessToken,
            workout_id: &Id,
            exercise_id: &Id,
        ) -> Result<()> {
            *self.last_pair.lock().unwrap() = Some((workout_id.clone(), exercise_id.clone()));
            self.outcome.lock().unwrap().clone()
        }
    }
}
