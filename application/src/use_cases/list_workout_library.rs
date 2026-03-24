use std::sync::Arc;

use crate::ports::error::{ApplicationError, Result};
use domain::repositories::ListWorkoutLibraryRead;
use domain::vos::id::Id;
use domain::vos::LibraryNameFilter;

#[derive(Clone)]
pub struct ListWorkoutLibraryArgs {
    pub specialist_id: String,
    pub name_filter: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkoutLibraryItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
}

pub struct ListWorkoutLibraryUseCase<R: ListWorkoutLibraryRead> {
    catalog_read: Arc<R>,
}

impl<R: ListWorkoutLibraryRead> ListWorkoutLibraryUseCase<R> {
    pub fn new(catalog_read: Arc<R>) -> Self {
        Self { catalog_read }
    }

    pub async fn execute(&self, args: ListWorkoutLibraryArgs) -> Result<Vec<WorkoutLibraryItem>> {
        let name_filter = args
            .name_filter
            .as_deref()
            .filter(|s| !s.is_empty())
            .map(LibraryNameFilter::try_from)
            .transpose()?;
        let name_filter_ref = name_filter.as_ref();
        let specialist_id = Id::try_from(args.specialist_id)?;
        let rows = self
            .catalog_read
            .list_workout_library(&specialist_id, name_filter_ref)
            .await
            .map_err(ApplicationError::from)?;
        Ok(rows
            .into_iter()
            .map(|w| WorkoutLibraryItem {
                id: w.id.to_string(),
                name: w.name,
                description: w.description,
                order_index: w.order_index.value(),
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    use domain::entities::Workout;
    use domain::error::Result;
    use domain::repositories::ListWorkoutLibraryRead;
    use domain::vos::id::Id;
    use domain::vos::library_name_filter::LibraryNameFilter;
    use domain::vos::ScheduleOrderIndex;

    #[tokio::test]
    async fn maps_workout_rows() {
        let spec = Id::try_from("550e8400-e29b-41d4-a716-446655440210").unwrap();
        let w = Workout {
            id: Id::try_from("550e8400-e29b-41d4-a716-446655440211").unwrap(),
            specialist_id: spec,
            name: "W1".to_string(),
            description: None,
            order_index: ScheduleOrderIndex::ZERO,
            created_at: None,
            updated_at: None,
        };
        let fake = MockListWorkoutLibraryRead::new_ok(vec![w.clone()]);
        let uc = ListWorkoutLibraryUseCase::new(Arc::new(fake));

        let rows = uc
            .execute(ListWorkoutLibraryArgs {
                specialist_id: "550e8400-e29b-41d4-a716-446655440210".to_string(),
                name_filter: None,
            })
            .await
            .unwrap();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, w.name);
    }

    #[derive(Clone)]
    struct MockListWorkoutLibraryRead {
        workouts: Arc<Mutex<Result<Vec<Workout>>>>,
    }

    impl MockListWorkoutLibraryRead {
        fn new_ok(workouts: Vec<Workout>) -> Self {
            Self {
                workouts: Arc::new(Mutex::new(Ok(workouts))),
            }
        }
    }

    #[common::async_trait_platform]
    impl ListWorkoutLibraryRead for MockListWorkoutLibraryRead {
        async fn list_workout_library(
            &self,
            _specialist_id: &Id,
            _name_filter: Option<&LibraryNameFilter>,
        ) -> Result<Vec<Workout>> {
            self.workouts.lock().unwrap().clone()
        }
    }
}
