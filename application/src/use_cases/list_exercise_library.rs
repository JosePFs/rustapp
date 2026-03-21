use std::sync::Arc;

use domain::error::Result;
use domain::repositories::ListExerciseLibraryRead;
use domain::vos::id::Id;
use domain::vos::{AccessToken, LibraryNameFilter};

#[derive(Clone)]
pub struct ListExerciseLibraryArgs {
    pub token: String,
    pub specialist_id: String,
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

pub struct ListExerciseLibraryUseCase<R: ListExerciseLibraryRead> {
    catalog_read: Arc<R>,
}

impl<R: ListExerciseLibraryRead> ListExerciseLibraryUseCase<R> {
    pub fn new(catalog_read: Arc<R>) -> Self {
        Self { catalog_read }
    }

    pub async fn execute(&self, args: ListExerciseLibraryArgs) -> Result<Vec<ExerciseLibraryItem>> {
        let access = AccessToken::try_from(args.token)?;
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
            .list_exercise_library(&access, &specialist_id, name_filter_ref)
            .await?;
        Ok(rows
            .into_iter()
            .map(|e| ExerciseLibraryItem {
                id: e.id.to_string(),
                name: e.name,
                description: e.description,
                order_index: e.order_index,
                video_url: e.video_url,
                deleted_at: e.deleted_at,
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::test_mocks::FakeListExerciseLibrary;
    use domain::entities::Exercise;

    const TOKEN: &str = "t";
    const SPEC: &str = "550e8400-e29b-41d4-a716-446655440050";

    #[tokio::test]
    async fn list_exercise_library_maps_rows() {
        let ex = Exercise {
            id: Id::try_from("550e8400-e29b-41d4-a716-446655440051").unwrap(),
            specialist_id: Id::try_from(SPEC).unwrap(),
            name: "Squat".to_string(),
            description: None,
            order_index: 0,
            video_url: None,
            deleted_at: None,
            created_at: None,
        };
        let fake = FakeListExerciseLibrary::new_ok(vec![ex.clone()]);
        let uc = ListExerciseLibraryUseCase::new(Arc::new(fake));

        let rows = uc
            .execute(ListExerciseLibraryArgs {
                token: TOKEN.to_string(),
                specialist_id: SPEC.to_string(),
                name_filter: None,
            })
            .await
            .unwrap();

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, ex.name);
    }
}
