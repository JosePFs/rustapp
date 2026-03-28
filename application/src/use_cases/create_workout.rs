use std::sync::Arc;

use crate::error::{ApplicationError, Result};
use domain::entities::Workout;
use domain::repositories::CreateWorkout;
use domain::vos::{Description, WorkoutName};

#[derive(Clone)]
pub struct CreateWorkoutArgs {
    pub name: String,
    pub description: Option<String>,
}

pub struct CreateWorkoutUseCase<W: CreateWorkout> {
    catalog_write: Arc<W>,
}

impl<W: CreateWorkout> CreateWorkoutUseCase<W> {
    pub fn new(catalog_write: Arc<W>) -> Self {
        Self { catalog_write }
    }

    pub async fn execute(&self, args: CreateWorkoutArgs) -> Result<Workout> {
        let name = WorkoutName::try_from(args.name)?;
        let description = args
            .description
            .as_ref()
            .map(|s| Description::try_from(s.as_str()))
            .transpose()?;
        let description_ref = description.as_ref();
        self.catalog_write
            .create_workout(&name, description_ref)
            .await
            .map_err(ApplicationError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Mutex;

    use domain::{
        error::Result,
        vos::{id::Id, ScheduleOrderIndex},
    };

    #[tokio::test]
    async fn create_workout_forwards_name() {
        let w = Workout {
            id: Id::try_from("550e8400-e29b-41d4-a716-446655440222").unwrap(),
            specialist_id: Id::try_from("550e8400-e29b-41d4-a716-446655440223").unwrap(),
            name: "Arms".to_string(),
            description: None,
            order_index: ScheduleOrderIndex::ZERO,
            created_at: None,
            updated_at: None,
        };
        let fake = MockCreateWorkoutWrite::new_ok(w.clone());
        let uc = CreateWorkoutUseCase::new(Arc::new(fake.clone()));

        let got = uc
            .execute(CreateWorkoutArgs {
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
    impl CreateWorkout for MockCreateWorkoutWrite {
        async fn create_workout(
            &self,
            name: &WorkoutName,
            _description: Option<&Description>,
        ) -> Result<Workout> {
            *self.last_name.lock().unwrap() = Some(name.value().to_string());
            self.outcome.lock().unwrap().clone()
        }
    }
}
