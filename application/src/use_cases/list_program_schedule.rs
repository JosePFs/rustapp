use std::sync::Arc;

use crate::ports::error::{ApplicationError, Result};
use domain::repositories::{GetWorkoutsByIdsRead, ListProgramScheduleRead};
use domain::vos::id::Id;

#[derive(Clone)]
pub struct ListProgramScheduleArgs {
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
        let program_id = Id::try_from(args.program_id)?;
        let schedule_domain = self
            .catalog_read
            .list_program_schedule(&program_id)
            .await
            .map_err(ApplicationError::from)?;

        let ids: Vec<Id> = schedule_domain
            .iter()
            .filter_map(|s| s.workout_id.clone())
            .collect();

        let workouts_domain = self
            .catalog_read
            .get_workouts_by_ids(&ids)
            .await
            .unwrap_or_default();

        let schedule: Vec<ProgramScheduleEntry> = schedule_domain
            .into_iter()
            .map(|s| ProgramScheduleEntry {
                id: s.id.to_string(),
                order_index: s.order_index.value(),
                workout_id: s.workout_id.map(|id| id.to_string()),
                days_count: s.days_count.value(),
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

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    use domain::entities::{ProgramScheduleItem, Workout};
    use domain::error::Result;
    use domain::repositories::{GetWorkoutsByIdsRead, ListProgramScheduleRead};
    use domain::vos::{DaysInBlock, ScheduleOrderIndex};

    const PRG: &str = "550e8400-e29b-41d4-a716-446655440350";
    const SID: &str = "550e8400-e29b-41d4-a716-446655440351";
    const WID: &str = "550e8400-e29b-41d4-a716-446655440352";

    #[tokio::test]
    async fn list_program_schedule_maps_rows_and_workouts() {
        let pid = Id::try_from(PRG).unwrap();
        let wid = Id::try_from(WID).unwrap();
        let sched = ProgramScheduleItem {
            id: Id::try_from(SID).unwrap(),
            program_id: pid,
            order_index: ScheduleOrderIndex::ZERO,
            workout_id: Some(wid.clone()),
            days_count: DaysInBlock::TWO,
            created_at: None,
        };
        let w = Workout {
            id: wid.clone(),
            specialist_id: Id::try_from("550e8400-e29b-41d4-a716-446655440353").unwrap(),
            name: "Leg day".to_string(),
            description: None,
            order_index: ScheduleOrderIndex::ZERO,
            created_at: None,
            updated_at: None,
        };
        let fake = MockListProgramScheduleRead::new(Ok(vec![sched]), Ok(vec![w.clone()]));
        let uc = ListProgramScheduleUseCase::new(Arc::new(fake));

        let res = uc
            .execute(ListProgramScheduleArgs {
                program_id: PRG.to_string(),
            })
            .await
            .unwrap();

        assert_eq!(res.schedule.len(), 1);
        assert_eq!(res.schedule[0].workout_id.as_deref(), Some(WID));
        assert_eq!(res.workouts.len(), 1);
        assert_eq!(res.workouts[0].name, "Leg day");
    }

    #[derive(Clone)]
    struct MockListProgramScheduleRead {
        schedule: Arc<Mutex<Result<Vec<ProgramScheduleItem>>>>,
        workouts: Arc<Mutex<Result<Vec<Workout>>>>,
    }

    impl MockListProgramScheduleRead {
        fn new(schedule: Result<Vec<ProgramScheduleItem>>, workouts: Result<Vec<Workout>>) -> Self {
            Self {
                schedule: Arc::new(Mutex::new(schedule)),
                workouts: Arc::new(Mutex::new(workouts)),
            }
        }
    }

    #[common::async_trait_platform]
    impl ListProgramScheduleRead for MockListProgramScheduleRead {
        async fn list_program_schedule(
            &self,
            _program_id: &Id,
        ) -> Result<Vec<ProgramScheduleItem>> {
            self.schedule.lock().unwrap().clone()
        }
    }

    #[common::async_trait_platform]
    impl GetWorkoutsByIdsRead for MockListProgramScheduleRead {
        async fn get_workouts_by_ids(&self, _ids: &[Id]) -> Result<Vec<Workout>> {
            self.workouts.lock().unwrap().clone()
        }
    }
}
