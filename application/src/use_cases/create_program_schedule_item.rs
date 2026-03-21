use std::sync::Arc;

use domain::entities::ProgramScheduleItem;
use domain::error::Result;
use domain::repositories::CreateProgramScheduleItemWrite;
use domain::vos::id::Id;
use domain::vos::{AccessToken, DaysInBlock, ScheduleOrderIndex};

#[derive(Clone)]
pub struct CreateProgramScheduleItemArgs {
    pub token: String,
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
        let access = AccessToken::try_from(args.token)?;
        let workout_id = match args.workout_id {
            Some(w) => Some(Id::try_from(w)?),
            None => None,
        };
        let order_index = ScheduleOrderIndex::try_from(args.order_index)?;
        let days_count = DaysInBlock::try_from(args.days_count)?;
        self.catalog_write
            .create_program_schedule_item(
                &access,
                &program_id,
                order_index,
                workout_id.as_ref(),
                days_count,
            )
            .await
    }
}
