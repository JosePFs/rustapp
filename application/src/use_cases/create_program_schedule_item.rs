use std::sync::Arc;

use crate::application::Backend;
use crate::domain::entities::ProgramScheduleItem;
use crate::domain::error::Result;

#[derive(Clone)]
pub struct CreateProgramScheduleItemArgs {
    pub token: String,
    pub program_id: String,
    pub order_index: i32,
    pub workout_id: Option<String>,
    pub days_count: i32,
}

pub struct CreateProgramScheduleItemUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> CreateProgramScheduleItemUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: CreateProgramScheduleItemArgs) -> Result<ProgramScheduleItem> {
        self.backend
            .create_program_schedule_item(
                &args.token,
                &args.program_id,
                args.order_index,
                args.workout_id.as_deref(),
                args.days_count,
            )
            .await
    }
}
