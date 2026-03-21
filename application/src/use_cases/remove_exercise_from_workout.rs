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
    use std::sync::Arc;

    use super::*;
    use crate::test_mocks::FakeRemoveExerciseFromWorkout;

    const W: &str = "550e8400-e29b-41d4-a716-446655440130";
    const E: &str = "550e8400-e29b-41d4-a716-446655440131";

    #[tokio::test]
    async fn remove_forwards_pair() {
        let fake = FakeRemoveExerciseFromWorkout::new_ok();
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
}
