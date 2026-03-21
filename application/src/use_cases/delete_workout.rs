use std::sync::Arc;

use domain::error::Result;
use domain::repositories::DeleteWorkoutWrite;
use domain::vos::id::Id;
use domain::vos::AccessToken;

#[derive(Clone)]
pub struct DeleteWorkoutArgs {
    pub token: String,
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
        let access = AccessToken::try_from(args.token)?;
        let workout_id = Id::try_from(args.workout_id)?;
        self.catalog_write
            .delete_workout(&access, &workout_id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::test_mocks::FakeDeleteWorkout;
    use domain::error::DomainError;

    const TOKEN: &str = "valid-token";
    const WID: &str = "550e8400-e29b-41d4-a716-446655440001";

    #[tokio::test]
    async fn delete_workout_invalid_token() {
        let fake = FakeDeleteWorkout::new_ok();
        let uc = DeleteWorkoutUseCase::new(Arc::new(fake));

        let err = uc
            .execute(DeleteWorkoutArgs {
                token: "  ".to_string(),
                workout_id: WID.to_string(),
            })
            .await
            .unwrap_err();

        assert!(matches!(err, DomainError::InvalidParameter(_, _)));
    }

    #[tokio::test]
    async fn delete_workout_invalid_workout_id() {
        let fake = FakeDeleteWorkout::new_ok();
        let uc = DeleteWorkoutUseCase::new(Arc::new(fake));

        let err = uc
            .execute(DeleteWorkoutArgs {
                token: TOKEN.to_string(),
                workout_id: "not-a-uuid".to_string(),
            })
            .await
            .unwrap_err();

        assert!(matches!(err, DomainError::InvalidParameter(_, _)));
    }

    #[tokio::test]
    async fn delete_workout_happy_path_records_id() {
        let fake = FakeDeleteWorkout::new_ok();
        let wid = Id::try_from(WID).unwrap();
        let uc = DeleteWorkoutUseCase::new(Arc::new(fake.clone()));

        uc.execute(DeleteWorkoutArgs {
            token: TOKEN.to_string(),
            workout_id: WID.to_string(),
        })
        .await
        .unwrap();
        let got = fake.last_workout_id.lock().unwrap().clone().unwrap();

        assert_eq!(got, wid);
    }

    #[tokio::test]
    async fn delete_workout_propagates_repo_error() {
        let fake = FakeDeleteWorkout::new_err(DomainError::Api("boom".into()));
        let uc = DeleteWorkoutUseCase::new(Arc::new(fake));

        let err = uc
            .execute(DeleteWorkoutArgs {
                token: TOKEN.to_string(),
                workout_id: WID.to_string(),
            })
            .await
            .unwrap_err();

        assert_eq!(err, DomainError::Api("boom".into()));
    }
}
