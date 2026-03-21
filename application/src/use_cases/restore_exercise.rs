use std::sync::Arc;

use domain::error::Result;
use domain::repositories::RestoreExerciseWrite;
use domain::vos::id::Id;
use domain::vos::AccessToken;

#[derive(Clone)]
pub struct RestoreExerciseArgs {
    pub token: String,
    pub exercise_id: String,
}

pub struct RestoreExerciseUseCase<W: RestoreExerciseWrite> {
    catalog_write: Arc<W>,
}

impl<W: RestoreExerciseWrite> RestoreExerciseUseCase<W> {
    pub fn new(catalog_write: Arc<W>) -> Self {
        Self { catalog_write }
    }

    pub async fn execute(&self, args: RestoreExerciseArgs) -> Result<()> {
        let access = AccessToken::try_from(args.token)?;
        let exercise_id = Id::try_from(args.exercise_id)?;
        self.catalog_write
            .restore_exercise(&access, &exercise_id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::test_mocks::FakeRestoreExercise;

    const TOKEN: &str = "t";
    const EID: &str = "550e8400-e29b-41d4-a716-446655440120";

    #[tokio::test]
    async fn restore_records_id() {
        let fake = FakeRestoreExercise::new_ok();
        let eid = Id::try_from(EID).unwrap();
        let uc = RestoreExerciseUseCase::new(Arc::new(fake.clone()));

        uc.execute(RestoreExerciseArgs {
            token: TOKEN.to_string(),
            exercise_id: EID.to_string(),
        })
        .await
        .unwrap();

        assert_eq!(*fake.last_exercise_id.lock().unwrap(), Some(eid));
    }
}
