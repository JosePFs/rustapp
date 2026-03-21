use std::sync::Arc;

use domain::entities::Exercise;
use domain::error::Result;
use domain::repositories::CreateExerciseWrite;
use domain::vos::id::Id;
use domain::vos::{AccessToken, Description, ExerciseName, ScheduleOrderIndex, VideoUrl};

#[derive(Clone)]
pub struct CreateExerciseArgs {
    pub token: String,
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
        let access = AccessToken::try_from(args.token)?;
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
                &access,
                &specialist_id,
                &name,
                description_ref,
                order_index,
                video_url_ref,
            )
            .await
    }
}
