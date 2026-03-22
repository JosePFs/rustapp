use std::sync::Arc;

use domain::entities::Workout;
use domain::error::Result;
use domain::repositories::CreateWorkoutWrite;
use domain::vos::id::Id;
use domain::vos::{AccessToken, Description, WorkoutName};

#[derive(Clone)]
pub struct CreateWorkoutArgs {
    pub token: String,
    pub specialist_id: String,
    pub name: String,
    pub description: Option<String>,
}

pub struct CreateWorkoutUseCase<W: CreateWorkoutWrite> {
    catalog_write: Arc<W>,
}

impl<W: CreateWorkoutWrite> CreateWorkoutUseCase<W> {
    pub fn new(catalog_write: Arc<W>) -> Self {
        Self { catalog_write }
    }

    pub async fn execute(&self, args: CreateWorkoutArgs) -> Result<Workout> {
        let access = AccessToken::try_from(args.token)?;
        let specialist_id = Id::try_from(args.specialist_id)?;
        let name = WorkoutName::try_from(args.name)?;
        let description = args
            .description
            .as_ref()
            .map(|s| Description::try_from(s.as_str()))
            .transpose()?;
        let description_ref = description.as_ref();
        self.catalog_write
            .create_workout(&access, &specialist_id, &name, description_ref)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use domain::entities::Workout;
    use domain::error::DomainError;
    use domain::error::Result;
    use domain::repositories::CreateWorkoutWrite;
    use domain::vos::{AccessToken, Description, WorkoutName};

    #[tokio::test]
    async fn create_workout_invalid_token() {
        let w = Workout {
            id: Id::try_from("550e8400-e29b-41d4-a716-446655440220").unwrap(),
            specialist_id: Id::try_from("550e8400-e29b-41d4-a716-446655440221").unwrap(),
            name: "N".to_string(),
            description: None,
            order_index: 0,
            created_at: None,
            updated_at: None,
        };
        let fake = MockCreateWorkoutWrite::new_ok(w);
        let uc = CreateWorkoutUseCase::new(Arc::new(fake));

        let err = uc
            .execute(CreateWorkoutArgs {
                token: "".to_string(),
                specialist_id: "550e8400-e29b-41d4-a716-446655440221".to_string(),
                name: "Legs".to_string(),
                description: None,
            })
            .await
            .unwrap_err();

        assert!(matches!(err, DomainError::InvalidParameter(_, _)));
    }

    #[tokio::test]
    async fn create_workout_forwards_name() {
        let w = Workout {
            id: Id::try_from("550e8400-e29b-41d4-a716-446655440222").unwrap(),
            specialist_id: Id::try_from("550e8400-e29b-41d4-a716-446655440223").unwrap(),
            name: "Arms".to_string(),
            description: None,
            order_index: 0,
            created_at: None,
            updated_at: None,
        };
        let fake = MockCreateWorkoutWrite::new_ok(w.clone());
        let uc = CreateWorkoutUseCase::new(Arc::new(fake.clone()));

        let got = uc
            .execute(CreateWorkoutArgs {
                token: "tok".to_string(),
                specialist_id: "550e8400-e29b-41d4-a716-446655440223".to_string(),
                name: "Arms".to_string(),
                description: None,
            })
            .await
            .unwrap();

        assert_eq!(got.name, w.name);
        assert_eq!(fake.last_name.lock().unwrap().as_deref(), Some("Arms"));
    }

    #[derive(Clone)]
    struct MockCreateWorkoutWrite {
        last_name: Arc<Mutex<Option<String>>>,
        outcome: Arc<Mutex<Result<Workout>>>,
    }

    impl MockCreateWorkoutWrite {
        fn new_ok(workout: Workout) -> Self {
            Self {
                last_name: Arc::new(Mutex::new(None)),
                outcome: Arc::new(Mutex::new(Ok(workout))),
            }
        }
    }

    #[common::async_trait_platform]
    impl CreateWorkoutWrite for MockCreateWorkoutWrite {
        async fn create_workout(
            &self,
            _access_token: &AccessToken,
            _specialist_id: &Id,
            name: &WorkoutName,
            _description: Option<&Description>,
        ) -> Result<Workout> {
            *self.last_name.lock().unwrap() = Some(name.value().to_string());
            self.outcome.lock().unwrap().clone()
        }
    }
}
