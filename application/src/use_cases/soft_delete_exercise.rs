use std::sync::Arc;

use crate::error::{ApplicationError, Result};
use domain::repositories::SoftDeleteExerciseWrite;
use domain::vos::id::Id;

#[derive(Clone)]
pub struct SoftDeleteExerciseArgs {
    pub exercise_id: String,
}

pub struct SoftDeleteExerciseUseCase<W: SoftDeleteExerciseWrite> {
    catalog_write: Arc<W>,
}

impl<W: SoftDeleteExerciseWrite> SoftDeleteExerciseUseCase<W> {
    pub fn new(catalog_write: Arc<W>) -> Self {
        Self { catalog_write }
    }

    pub async fn execute(&self, args: SoftDeleteExerciseArgs) -> Result<()> {
        let exercise_id = Id::try_from(args.exercise_id)?;
        self.catalog_write
            .soft_delete_exercise(&exercise_id)
            .await
            .map_err(ApplicationError::from)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    use domain::error::Result;
    use domain::repositories::SoftDeleteExerciseWrite;

    const EID: &str = "550e8400-e29b-41d4-a716-446655440080";

    #[tokio::test]
    async fn soft_delete_records_exercise_id() {
        let fake = MockSoftDeleteExerciseWrite::new_ok();
        let eid = Id::try_from(EID).unwrap();
        let uc = SoftDeleteExerciseUseCase::new(Arc::new(fake.clone()));

        uc.execute(SoftDeleteExerciseArgs {
            exercise_id: EID.to_string(),
        })
        .await
        .unwrap();

        assert_eq!(*fake.last_exercise_id.lock().unwrap(), Some(eid));
    }

    #[derive(Clone)]
    struct MockSoftDeleteExerciseWrite {
        last_exercise_id: Arc<Mutex<Option<Id>>>,
        outcome: Arc<Mutex<Result<()>>>,
    }

    impl MockSoftDeleteExerciseWrite {
        fn new_ok() -> Self {
            Self {
                last_exercise_id: Arc::new(Mutex::new(None)),
                outcome: Arc::new(Mutex::new(Ok(()))),
            }
        }
    }

    #[common::async_trait_platform]
    impl SoftDeleteExerciseWrite for MockSoftDeleteExerciseWrite {
        async fn soft_delete_exercise(&self, exercise_id: &Id) -> Result<()> {
            *self.last_exercise_id.lock().unwrap() = Some(exercise_id.clone());
            self.outcome.lock().unwrap().clone()
        }
    }
}
