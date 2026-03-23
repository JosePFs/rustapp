use std::sync::Arc;

use domain::error::Result;
use domain::repositories::DeleteWorkoutWrite;
use domain::vos::id::Id;

#[derive(Clone)]
pub struct DeleteWorkoutArgs {
    pub workout_id: String,
}

pub struct DeleteWorkoutUseCase<W: DeleteWorkoutWrite> {
    catalog_write: Arc<W>,
}

impl<W: DeleteWorkoutWrite> DeleteWorkoutUseCase<W> {
    pub fn new(catalog_write: Arc<W>) -> Self {
        Self { catalog_write }
    }

    pub async fn execute(&self, args: DeleteWorkoutArgs) -> Result<()> {
        let workout_id = Id::try_from(args.workout_id)?;
        self.catalog_write.delete_workout(&workout_id).await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    use domain::error::DomainError;

    const WID: &str = "550e8400-e29b-41d4-a716-446655440001";

    #[tokio::test]
    async fn delete_workout_invalid_workout_id() {
        let fake = MockDeleteWorkoutWrite::new_ok();
        let uc = DeleteWorkoutUseCase::new(Arc::new(fake));

        let err = uc
            .execute(DeleteWorkoutArgs {
                workout_id: "not-a-uuid".to_string(),
            })
            .await
            .unwrap_err();

        assert!(matches!(err, DomainError::InvalidParameter(_, _)));
    }

    #[tokio::test]
    async fn delete_workout_happy_path_records_id() {
        let fake = MockDeleteWorkoutWrite::new_ok();
        let wid = Id::try_from(WID).unwrap();
        let uc = DeleteWorkoutUseCase::new(Arc::new(fake.clone()));

        uc.execute(DeleteWorkoutArgs {
            workout_id: WID.to_string(),
        })
        .await
        .unwrap();
        let got = fake.last_workout_id.lock().unwrap().clone().unwrap();

        assert_eq!(got, wid);
    }

    #[tokio::test]
    async fn delete_workout_propagates_repo_error() {
        let fake = MockDeleteWorkoutWrite::new_err(DomainError::Api("boom".into()));
        let uc = DeleteWorkoutUseCase::new(Arc::new(fake));

        let err = uc
            .execute(DeleteWorkoutArgs {
                workout_id: WID.to_string(),
            })
            .await
            .unwrap_err();

        assert_eq!(err, DomainError::Api("boom".into()));
    }

    #[derive(Clone)]
    struct MockDeleteWorkoutWrite {
        last_workout_id: Arc<Mutex<Option<Id>>>,
        outcome: Arc<Mutex<Result<()>>>,
    }

    impl MockDeleteWorkoutWrite {
        fn new_ok() -> Self {
            Self {
                last_workout_id: Arc::new(Mutex::new(None)),
                outcome: Arc::new(Mutex::new(Ok(()))),
            }
        }

        fn new_err(e: domain::error::DomainError) -> Self {
            Self {
                last_workout_id: Arc::new(Mutex::new(None)),
                outcome: Arc::new(Mutex::new(Err(e))),
            }
        }
    }

    #[common::async_trait_platform]
    impl DeleteWorkoutWrite for MockDeleteWorkoutWrite {
        async fn delete_workout(&self, workout_id: &Id) -> Result<()> {
            *self.last_workout_id.lock().unwrap() = Some(workout_id.clone());
            self.outcome.lock().unwrap().clone()
        }
    }
}
