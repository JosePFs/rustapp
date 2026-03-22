use std::sync::Arc;

use domain::error::Result;
use domain::repositories::UpdateWorkoutWrite;
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

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use domain::error::DomainError;
    use domain::error::Result;
    use domain::repositories::UpdateWorkoutWrite;
    use domain::vos::{AccessToken, Patch, ScheduleOrderIndex};

    const TOKEN: &str = "t";
    const WID: &str = "550e8400-e29b-41d4-a716-446655440330";

    #[tokio::test]
    async fn update_workout_invalid_token() {
        let fake = MockUpdateWorkoutWrite::new_ok();
        let uc = UpdateWorkoutUseCase::new(Arc::new(fake));

        let err = uc
            .execute(UpdateWorkoutArgs {
                token: "  ".to_string(),
                workout_id: WID.to_string(),
                name: None,
                description: None,
            })
            .await
            .unwrap_err();

        assert!(matches!(err, DomainError::InvalidParameter(_, _)));
    }

    #[tokio::test]
    async fn update_workout_forwards_name() {
        let fake = MockUpdateWorkoutWrite::new_ok();
        let wid = Id::try_from(WID).unwrap();
        let uc = UpdateWorkoutUseCase::new(Arc::new(fake.clone()));

        uc.execute(UpdateWorkoutArgs {
            token: TOKEN.to_string(),
            workout_id: WID.to_string(),
            name: Some("Renamed".to_string()),
            description: None,
        })
        .await
        .unwrap();

        assert_eq!(*fake.last_workout_id.lock().unwrap(), Some(wid));
        assert_eq!(fake.last_name.lock().unwrap().as_deref(), Some("Renamed"));
    }

    #[derive(Clone)]
    struct MockUpdateWorkoutWrite {
        last_workout_id: Arc<Mutex<Option<Id>>>,
        last_name: Arc<Mutex<Option<String>>>,
        outcome: Arc<Mutex<Result<()>>>,
    }

    impl MockUpdateWorkoutWrite {
        fn new_ok() -> Self {
            Self {
                last_workout_id: Arc::new(Mutex::new(None)),
                last_name: Arc::new(Mutex::new(None)),
                outcome: Arc::new(Mutex::new(Ok(()))),
            }
        }
    }

    #[common::async_trait_platform]
    impl UpdateWorkoutWrite for MockUpdateWorkoutWrite {
        async fn update_workout(
            &self,
            _access_token: &AccessToken,
            workout_id: &Id,
            name: Option<&WorkoutName>,
            _description: Patch<Description>,
            _order_index: Option<ScheduleOrderIndex>,
        ) -> Result<()> {
            *self.last_workout_id.lock().unwrap() = Some(workout_id.clone());
            *self.last_name.lock().unwrap() = name.map(|n| n.value().to_string());
            self.outcome.lock().unwrap().clone()
        }
    }
}
