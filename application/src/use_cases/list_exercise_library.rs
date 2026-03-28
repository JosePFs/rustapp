use std::sync::Arc;

use crate::error::{ApplicationError, Result};
use domain::repositories::ListExerciseLibrary;
use domain::vos::LibraryNameFilter;

#[derive(Clone)]
pub struct ListExerciseLibraryArgs {
    pub name_filter: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExerciseLibraryItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
    pub video_url: Option<String>,
    pub deleted_at: Option<String>,
}

pub struct ListExerciseLibraryUseCase<R: ListExerciseLibrary> {
    catalog_read: Arc<R>,
}

impl<R: ListExerciseLibrary> ListExerciseLibraryUseCase<R> {
    pub fn new(catalog_read: Arc<R>) -> Self {
        Self { catalog_read }
    }

    pub async fn execute(&self, args: ListExerciseLibraryArgs) -> Result<Vec<ExerciseLibraryItem>> {
        let name_filter = args
            .name_filter
            .as_deref()
            .filter(|s| !s.is_empty())
            .map(LibraryNameFilter::try_from)
            .transpose()?;
        let name_filter_ref = name_filter.as_ref();
        let rows = self
            .catalog_read
            .list_exercise_library(name_filter_ref)
            .await
            .map_err(ApplicationError::from)?;
        Ok(rows
            .into_iter()
            .map(|e| ExerciseLibraryItem {
                id: e.id.to_string(),
                name: e.name,
                description: e.description,
                order_index: e.order_index.value(),
                video_url: e.video_url.map(|url| url.value().to_string()),
                deleted_at: e.deleted_at,
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    use domain::entities::Exercise;
    use domain::error::Result;
    use domain::repositories::ListExerciseLibrary;
    use domain::vos::id::Id;
    use domain::vos::library_name_filter::LibraryNameFilter;
    use domain::vos::ScheduleOrderIndex;

    const SPEC: &str = "550e8400-e29b-41d4-a716-446655440050";

    #[tokio::test]
    async fn list_exercise_library_maps_rows() {
        let ex = Exercise {
            id: Id::try_from("550e8400-e29b-41d4-a716-446655440051").unwrap(),
            specialist_id: Id::try_from(SPEC).unwrap(),
            name: "Squat".to_string(),
            description: None,
            order_index: ScheduleOrderIndex::ZERO,
            video_url: None,
            deleted_at: None,
            created_at: None,
        };
        let fake = MockListExerciseLibraryRead::new_ok(vec![ex.clone()]);
        let uc = ListExerciseLibraryUseCase::new(Arc::new(fake));

        let rows = uc
            .execute(ListExerciseLibraryArgs { name_filter: None })
            .await
            .unwrap();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, ex.name);
    }

    #[derive(Clone)]
    struct MockListExerciseLibraryRead {
        exercises: Arc<Mutex<Result<Vec<Exercise>>>>,
    }

    impl MockListExerciseLibraryRead {
        fn new_ok(exercises: Vec<Exercise>) -> Self {
            Self {
                exercises: Arc::new(Mutex::new(Ok(exercises))),
            }
        }
    }

    #[common::async_trait_platform]
    impl ListExerciseLibrary for MockListExerciseLibraryRead {
        async fn list_exercise_library(
            &self,
            _name_filter: Option<&LibraryNameFilter>,
        ) -> Result<Vec<Exercise>> {
            self.exercises.lock().unwrap().clone()
        }
    }
}
