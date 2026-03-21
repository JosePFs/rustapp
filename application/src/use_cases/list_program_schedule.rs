use std::sync::Arc;

use domain::error::Result;
use domain::repositories::{GetWorkoutsByIdsRead, ListProgramScheduleRead};
use domain::vos::id::Id;
use domain::vos::AccessToken;

#[derive(Clone)]
pub struct ListProgramScheduleArgs {
    pub token: String,
    pub program_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramScheduleEntry {
    pub id: String,
    pub order_index: i32,
    pub workout_id: Option<String>,
    pub days_count: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkoutList {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramScheduleData {
    pub schedule: Vec<ProgramScheduleEntry>,
    pub workouts: Vec<WorkoutList>,
}

pub struct ListProgramScheduleUseCase<R: ListProgramScheduleRead + GetWorkoutsByIdsRead> {
    catalog_read: Arc<R>,
}

impl<R: ListProgramScheduleRead + GetWorkoutsByIdsRead> ListProgramScheduleUseCase<R> {
    pub fn new(catalog_read: Arc<R>) -> Self {
        Self { catalog_read }
    }

    pub async fn execute(&self, args: ListProgramScheduleArgs) -> Result<ProgramScheduleData> {
        let access = AccessToken::try_from(args.token)?;
        let program_id = Id::try_from(args.program_id)?;
        let schedule_domain = self
            .catalog_read
            .list_program_schedule(&access, &program_id)
            .await?;

        let ids: Vec<Id> = schedule_domain
            .iter()
            .filter_map(|s| s.workout_id.clone())
            .collect();

        let workouts_domain = self
            .catalog_read
            .get_workouts_by_ids(&access, &ids)
            .await
            .unwrap_or_default();

        let schedule: Vec<ProgramScheduleEntry> = schedule_domain
            .into_iter()
            .map(|s| ProgramScheduleEntry {
                id: s.id.to_string(),
                order_index: s.order_index,
                workout_id: s.workout_id.map(|id| id.to_string()),
                days_count: s.days_count,
            })
            .collect();

        let workouts: Vec<WorkoutList> = workouts_domain
            .into_iter()
            .map(|w| WorkoutList {
                id: w.id.to_string(),
                name: w.name,
            })
            .collect();

        Ok(ProgramScheduleData { schedule, workouts })
    }
}
