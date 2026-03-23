use std::sync::Arc;

use crate::ports::error::{ApplicationError, Result};
use domain::entities::ProgramScheduleItem;
use domain::repositories::CreateProgramScheduleItemWrite;
use domain::vos::id::Id;
use domain::vos::{DaysInBlock, ScheduleOrderIndex};

#[derive(Clone)]
pub struct CreateProgramScheduleItemArgs {
    pub program_id: String,
    pub order_index: i32,
    pub workout_id: Option<String>,
    pub days_count: i32,
}

pub struct CreateProgramScheduleItemUseCase<W: CreateProgramScheduleItemWrite> {
    catalog_write: Arc<W>,
}

impl<W: CreateProgramScheduleItemWrite> CreateProgramScheduleItemUseCase<W> {
    pub fn new(catalog_write: Arc<W>) -> Self {
        Self { catalog_write }
    }

    pub async fn execute(
        &self,
        args: CreateProgramScheduleItemArgs,
    ) -> Result<ProgramScheduleItem> {
        let program_id = Id::try_from(args.program_id)?;
        let workout_id = match args.workout_id {
            Some(w) => Some(Id::try_from(w)?),
            None => None,
        };
        let order_index = ScheduleOrderIndex::try_from(args.order_index)?;
        let days_count = DaysInBlock::try_from(args.days_count)?;
        self.catalog_write
            .create_program_schedule_item(&program_id, order_index, workout_id.as_ref(), days_count)
            .await
            .map_err(ApplicationError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Mutex;

    use domain::error::Result;

    const PRG: &str = "550e8400-e29b-41d4-a716-446655440320";
    const WID: &str = "550e8400-e29b-41d4-a716-446655440321";
    const NEW_ID: &str = "550e8400-e29b-41d4-a716-446655440322";

    #[tokio::test]
    async fn create_schedule_item_returns_created_row() {
        let item = ProgramScheduleItem {
            id: Id::try_from(NEW_ID).unwrap(),
            program_id: Id::try_from(PRG).unwrap(),
            order_index: 1,
            workout_id: Some(Id::try_from(WID).unwrap()),
            days_count: 3,
            created_at: None,
        };
        let fake = MockCreateProgramScheduleItemWrite::new_ok(item.clone());
        let uc = CreateProgramScheduleItemUseCase::new(Arc::new(fake));

        let got = uc
            .execute(CreateProgramScheduleItemArgs {
                program_id: PRG.to_string(),
                order_index: 1,
                workout_id: Some(WID.to_string()),
                days_count: 3,
            })
            .await
            .unwrap();

        assert_eq!(got.id, item.id);
        assert_eq!(got.days_count, 3);
    }

    #[derive(Clone)]
    struct MockCreateProgramScheduleItemWrite {
        outcome: Arc<Mutex<Result<ProgramScheduleItem>>>,
    }

    impl MockCreateProgramScheduleItemWrite {
        fn new_ok(item: ProgramScheduleItem) -> Self {
            Self {
                outcome: Arc::new(Mutex::new(Ok(item))),
            }
        }
    }

    #[common::async_trait_platform]
    impl CreateProgramScheduleItemWrite for MockCreateProgramScheduleItemWrite {
        async fn create_program_schedule_item(
            &self,
            _program_id: &Id,
            _order_index: ScheduleOrderIndex,
            _workout_id: Option<&Id>,
            _days_count: DaysInBlock,
        ) -> Result<ProgramScheduleItem> {
            self.outcome.lock().unwrap().clone()
        }
    }
}
