use std::sync::Arc;

use crate::application::Backend;
use crate::domain::error::Result;

#[derive(Clone)]
pub struct DeleteProgramScheduleItemArgs {
    pub token: String,
    pub schedule_item_id: String,
}

pub struct DeleteProgramScheduleItemUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> DeleteProgramScheduleItemUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: DeleteProgramScheduleItemArgs) -> Result<()> {
        self.backend
            .delete_program_schedule_item(&args.token, &args.schedule_item_id)
            .await
    }
}
