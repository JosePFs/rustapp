use std::sync::Arc;

use crate::ports::error::{ApplicationError, Result};
use domain::repositories::UpdateExerciseWrite;
use domain::vos::id::Id;
use domain::vos::{Description, ExerciseName, Patch, ScheduleOrderIndex, VideoUrl};

#[derive(Clone)]
pub struct UpdateExerciseArgs {
    pub exercise_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub order_index: Option<i32>,
    pub video_url: Option<String>,
}

pub struct UpdateExerciseUseCase<W: UpdateExerciseWrite> {
    catalog_write: Arc<W>,
}

impl<W: UpdateExerciseWrite> UpdateExerciseUseCase<W> {
    pub fn new(catalog_write: Arc<W>) -> Self {
        Self { catalog_write }
    }

    pub async fn execute(&self, args: UpdateExerciseArgs) -> Result<()> {
        let exercise_id = Id::try_from(args.exercise_id)?;
        let name = args
            .name
            .as_ref()
            .map(|s| ExerciseName::try_from(s.as_str()))
            .transpose()?;
        let name_ref = name.as_ref();
        let description = args
            .description
            .as_ref()
            .map(|s| Description::try_from(s.as_str()))
            .transpose()?;
        let description_ref = description.as_ref();
        let order_index = args
            .order_index
            .map(ScheduleOrderIndex::try_from)
            .transpose()?;
        let video_url = match &args.video_url {
            None => Patch::Omit,
            Some(s) if s.trim().is_empty() => Patch::Clear,
            Some(s) => Patch::Set(VideoUrl::try_from(s.as_str())?),
        };
        self.catalog_write
            .update_exercise(
                &exercise_id,
                name_ref,
                description_ref,
                order_index,
                video_url,
            )
            .await
            .map_err(ApplicationError::from)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    use domain::error::Result;
    use domain::repositories::UpdateExerciseWrite;
    use domain::vos::Patch;

    const EID: &str = "550e8400-e29b-41d4-a716-446655440340";

    #[tokio::test]
    async fn update_exercise_forwards_exercise_id() {
        let fake = MockUpdateExerciseWrite::new_ok();
        let eid = Id::try_from(EID).unwrap();
        let uc = UpdateExerciseUseCase::new(Arc::new(fake.clone()));

        uc.execute(UpdateExerciseArgs {
            exercise_id: EID.to_string(),
            name: None,
            description: None,
            order_index: None,
            video_url: None,
        })
        .await
        .unwrap();

        assert_eq!(*fake.last_exercise_id.lock().unwrap(), Some(eid));
    }

    #[derive(Clone)]
    struct MockUpdateExerciseWrite {
        last_exercise_id: Arc<Mutex<Option<Id>>>,
        outcome: Arc<Mutex<Result<()>>>,
    }

    impl MockUpdateExerciseWrite {
        fn new_ok() -> Self {
            Self {
                last_exercise_id: Arc::new(Mutex::new(None)),
                outcome: Arc::new(Mutex::new(Ok(()))),
            }
        }
    }

    #[common::async_trait_platform]
    impl UpdateExerciseWrite for MockUpdateExerciseWrite {
        async fn update_exercise(
            &self,
            exercise_id: &Id,
            _name: Option<&ExerciseName>,
            _description: Option<&Description>,
            _order_index: Option<ScheduleOrderIndex>,
            _video_url: Patch<VideoUrl>,
        ) -> Result<()> {
            *self.last_exercise_id.lock().unwrap() = Some(exercise_id.clone());
            self.outcome.lock().unwrap().clone()
        }
    }
}
