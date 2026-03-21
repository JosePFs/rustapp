use std::sync::Arc;

use domain::repositories::UpdateWorkoutWrite;
use domain::error::Result;
use domain::vos::id::Id;
use domain::vos::{AccessToken, Description, Patch, WorkoutName};

#[derive(Clone)]
pub struct UpdateWorkoutArgs {
    pub token: String,
    pub workout_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Clone)]
pub struct UpdateWorkoutInput {
    pub workout_id: String,
    pub name: String,
    pub description: String,
}

pub struct UpdateWorkoutUseCase<W: UpdateWorkoutWrite> {
    catalog_write: Arc<W>,
}

impl<W: UpdateWorkoutWrite> UpdateWorkoutUseCase<W> {
    pub fn new(catalog_write: Arc<W>) -> Self {
        Self { catalog_write }
    }

    pub async fn execute(&self, args: UpdateWorkoutArgs) -> Result<()> {
        let access = AccessToken::try_from(args.token)?;
        let workout_id = Id::try_from(args.workout_id)?;
        let name = args
            .name
            .as_ref()
            .map(|s| WorkoutName::try_from(s.as_str()))
            .transpose()?;
        let name_ref = name.as_ref();
        let description = match &args.description {
            None => Patch::Omit,
            Some(s) if s.trim().is_empty() => Patch::Clear,
            Some(s) => Patch::Set(Description::try_from(s.as_str())?),
        };
        self.catalog_write
            .update_workout(&access, &workout_id, name_ref, description, None)
            .await
    }
}
