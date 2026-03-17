use std::sync::Arc;

use crate::application::Backend;
use crate::domain::entities::ProgramScheduleItem;
use crate::domain::error::Result;

#[derive(Clone)]
pub struct ListProgramScheduleArgs {
    pub token: String,
    pub program_id: String,
}

#[derive(Clone)]
pub struct ProgramScheduleData {
    pub schedule: Vec<ProgramScheduleItem>,
    pub workouts: Vec<domain::entities::Workout>,
}

pub struct ListProgramScheduleUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> ListProgramScheduleUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: ListProgramScheduleArgs) -> Result<ProgramScheduleData> {
        let schedule = self
            .backend
            .list_program_schedule(&args.token, &args.program_id)
            .await?;

        let ids: Vec<String> = schedule
            .iter()
            .filter_map(|s| s.workout_id.clone())
            .collect::<std::collections::HashSet<String>>()
            .into_iter()
            .collect();

        let workouts = self
            .backend
            .get_workouts_by_ids(&args.token, &ids)
            .await
            .unwrap_or_default();

        Ok(ProgramScheduleData { schedule, workouts })
    }
}
