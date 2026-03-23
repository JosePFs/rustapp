use std::sync::Arc;

use domain::entities::Exercise;
use domain::error::Result;
use domain::repositories::CreateExerciseWrite;
use domain::vos::id::Id;
use domain::vos::{Description, ExerciseName, ScheduleOrderIndex, VideoUrl};

#[derive(Clone)]
pub struct CreateExerciseArgs {
    pub specialist_id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
    pub video_url: Option<String>,
}

pub struct CreateExerciseUseCase<W: CreateExerciseWrite> {
    catalog_write: Arc<W>,
}

impl<W: CreateExerciseWrite> CreateExerciseUseCase<W> {
    pub fn new(catalog_write: Arc<W>) -> Self {
        Self { catalog_write }
    }

    pub async fn execute(&self, args: CreateExerciseArgs) -> Result<Exercise> {
        let specialist_id = Id::try_from(args.specialist_id)?;
        let name = ExerciseName::try_from(args.name)?;
        let description = args
            .description
            .as_ref()
            .map(|s| Description::try_from(s.as_str()))
            .transpose()?;
        let description_ref = description.as_ref();
        let order_index = ScheduleOrderIndex::try_from(args.order_index)?;
        let video_url = args
            .video_url
            .as_ref()
            .map(|s| VideoUrl::try_from(s.as_str()))
            .transpose()?;
        let video_url_ref = video_url.as_ref();
        self.catalog_write
            .create_exercise(
                &specialist_id,
                &name,
                description_ref,
                order_index,
                video_url_ref,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    use domain::error::DomainError;

    const SPEC: &str = "550e8400-e29b-41d4-a716-446655440310";

    #[tokio::test]
    async fn create_exercise_invalid_name() {
        let ex = Exercise {
            id: Id::try_from("550e8400-e29b-41d4-a716-446655440311").unwrap(),
            specialist_id: Id::try_from(SPEC).unwrap(),
            name: "X".to_string(),
            description: None,
            order_index: 0,
            video_url: None,
            deleted_at: None,
            created_at: None,
        };
        let fake = MockCreateExerciseWrite::new_ok(ex);
        let uc = CreateExerciseUseCase::new(Arc::new(fake));

        let err = uc
            .execute(CreateExerciseArgs {
                specialist_id: SPEC.to_string(),
                name: "".to_string(),
                description: None,
                order_index: 0,
                video_url: None,
            })
            .await
            .unwrap_err();

        assert!(matches!(err, DomainError::InvalidParameter(_, _)));
    }

    #[tokio::test]
    async fn create_exercise_forwards_name() {
        let ex = Exercise {
            id: Id::try_from("550e8400-e29b-41d4-a716-446655440312").unwrap(),
            specialist_id: Id::try_from(SPEC).unwrap(),
            name: "Push-up".to_string(),
            description: None,
            order_index: 1,
            video_url: None,
            deleted_at: None,
            created_at: None,
        };
        let fake = MockCreateExerciseWrite::new_ok(ex.clone());
        let uc = CreateExerciseUseCase::new(Arc::new(fake.clone()));

        let got = uc
            .execute(CreateExerciseArgs {
                specialist_id: SPEC.to_string(),
                name: "Push-up".to_string(),
                description: None,
                order_index: 1,
                video_url: None,
            })
            .await
            .unwrap();

        assert_eq!(got.name, ex.name);
        assert_eq!(fake.last_name.lock().unwrap().as_deref(), Some("Push-up"));
    }

    #[derive(Clone)]
    struct MockCreateExerciseWrite {
        last_name: Arc<Mutex<Option<String>>>,
        outcome: Arc<Mutex<Result<Exercise>>>,
    }

    impl MockCreateExerciseWrite {
        fn new_ok(exercise: Exercise) -> Self {
            Self {
                last_name: Arc::new(Mutex::new(None)),
                outcome: Arc::new(Mutex::new(Ok(exercise))),
            }
        }
    }

    #[common::async_trait_platform]
    impl CreateExerciseWrite for MockCreateExerciseWrite {
        async fn create_exercise(
            &self,
            _specialist_id: &Id,
            name: &ExerciseName,
            _description: Option<&Description>,
            _order_index: ScheduleOrderIndex,
            _video_url: Option<&VideoUrl>,
        ) -> Result<Exercise> {
            *self.last_name.lock().unwrap() = Some(name.value().to_string());
            self.outcome.lock().unwrap().clone()
        }
    }
}
