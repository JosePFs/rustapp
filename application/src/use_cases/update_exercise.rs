use std::sync::Arc;

use domain::error::Result;
use domain::repositories::UpdateExerciseWrite;
use domain::vos::id::Id;
use domain::vos::{AccessToken, Description, ExerciseName, Patch, ScheduleOrderIndex, VideoUrl};

#[derive(Clone)]
pub struct UpdateExerciseArgs {
    pub token: String,
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
        let access = AccessToken::try_from(args.token)?;
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
                &access,
                &exercise_id,
                name_ref,
                description_ref,
                order_index,
                video_url,
            )
            .await
    }
}
