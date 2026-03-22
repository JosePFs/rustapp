use std::sync::Arc;

use domain::error::Result;
use domain::repositories::DeleteProgramScheduleItemWrite;
use domain::vos::id::Id;
use domain::vos::AccessToken;

#[derive(Clone)]
pub struct DeleteProgramScheduleItemArgs {
    pub token: String,
    pub schedule_item_id: String,
}

pub struct DeleteProgramScheduleItemUseCase<W: DeleteProgramScheduleItemWrite> {
    catalog_write: Arc<W>,
}

impl<W: DeleteProgramScheduleItemWrite> DeleteProgramScheduleItemUseCase<W> {
    pub fn new(catalog_write: Arc<W>) -> Self {
        Self { catalog_write }
    }

    pub async fn execute(&self, args: DeleteProgramScheduleItemArgs) -> Result<()> {
        let access = AccessToken::try_from(args.token)?;
        let schedule_item_id = Id::try_from(args.schedule_item_id)?;
        self.catalog_write
            .delete_program_schedule_item(&access, &schedule_item_id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use domain::error::DomainError;
    use domain::error::Result;
    use domain::repositories::DeleteProgramScheduleItemWrite;
    use domain::vos::AccessToken;

    const TOKEN: &str = "t";
    const SID: &str = "550e8400-e29b-41d4-a716-446655440002";

    #[tokio::test]
    async fn delete_schedule_item_invalid_token() {
        let fake = MockDeleteProgramScheduleItemWrite::new_ok();
        let uc = DeleteProgramScheduleItemUseCase::new(Arc::new(fake));

        let err = uc
            .execute(DeleteProgramScheduleItemArgs {
                token: "".to_string(),
                schedule_item_id: SID.to_string(),
            })
            .await
            .unwrap_err();

        assert!(matches!(err, DomainError::InvalidParameter(_, _)));
    }

    #[tokio::test]
    async fn delete_schedule_item_happy_path() {
        let fake = MockDeleteProgramScheduleItemWrite::new_ok();
        let sid = Id::try_from(SID).unwrap();
        let uc = DeleteProgramScheduleItemUseCase::new(Arc::new(fake.clone()));

        uc.execute(DeleteProgramScheduleItemArgs {
            token: TOKEN.to_string(),
            schedule_item_id: SID.to_string(),
        })
        .await
        .unwrap();

        assert_eq!(*fake.last_schedule_id.lock().unwrap(), Some(sid));
    }

    #[derive(Clone)]
    struct MockDeleteProgramScheduleItemWrite {
        last_schedule_id: Arc<Mutex<Option<Id>>>,
        outcome: Arc<Mutex<Result<()>>>,
    }

    impl MockDeleteProgramScheduleItemWrite {
        fn new_ok() -> Self {
            Self {
                last_schedule_id: Arc::new(Mutex::new(None)),
                outcome: Arc::new(Mutex::new(Ok(()))),
            }
        }
    }

    #[common::async_trait_platform]
    impl DeleteProgramScheduleItemWrite for MockDeleteProgramScheduleItemWrite {
        async fn delete_program_schedule_item(
            &self,
            _access_token: &AccessToken,
            schedule_id: &Id,
        ) -> Result<()> {
            *self.last_schedule_id.lock().unwrap() = Some(schedule_id.clone());
            self.outcome.lock().unwrap().clone()
        }
    }
}
