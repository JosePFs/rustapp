use std::sync::Arc;

use futures::try_join;
use futures::TryFutureExt;

use crate::ports::error::{ApplicationError, Result};
use domain::entities::Exercise;
use domain::repositories::{GetWorkoutWithExercisesRead, ListExerciseLibraryRead};
use domain::vos::id::Id;

#[derive(Clone)]
pub struct WorkoutEditorDataArgs {
    pub specialist_id: String,
    pub workout_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkoutEditorWorkout {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkoutEditorExerciseItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
    pub video_url: Option<String>,
    pub deleted_at: Option<String>,
}

impl From<Exercise> for WorkoutEditorExerciseItem {
    fn from(e: Exercise) -> Self {
        WorkoutEditorExerciseItem {
            id: e.id.to_string(),
            name: e.name,
            description: e.description,
            order_index: e.order_index.value(),
            video_url: e.video_url.map(|url| url.value().to_string()),
            deleted_at: e.deleted_at,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkoutEditorLine {
    pub exercise: WorkoutEditorExerciseItem,
    pub order_index: i32,
    pub sets: i32,
    pub reps: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkoutEditorDataResult {
    pub workout: Option<WorkoutEditorWorkout>,
    pub exercises: Vec<WorkoutEditorLine>,
    pub library: Vec<WorkoutEditorExerciseItem>,
}

pub struct WorkoutEditorDataUseCase<R: GetWorkoutWithExercisesRead + ListExerciseLibraryRead> {
    catalog_read: Arc<R>,
}

impl<R: GetWorkoutWithExercisesRead + ListExerciseLibraryRead> WorkoutEditorDataUseCase<R> {
    pub fn new(catalog_read: Arc<R>) -> Self {
        Self { catalog_read }
    }

    pub async fn execute(&self, args: WorkoutEditorDataArgs) -> Result<WorkoutEditorDataResult> {
        let specialist_id = Id::try_from(args.specialist_id)?;
        let workout_id = Id::try_from(args.workout_id)?;

        let (workout_with_exercises, library_domain) = try_join!(
            self.catalog_read
                .get_workout_with_exercises(&workout_id)
                .map_err(ApplicationError::from),
            self.catalog_read
                .list_exercise_library(&specialist_id, None)
                .map_err(ApplicationError::from),
        )?;

        let (workout, exercises) = workout_with_exercises
            .map(|w| {
                let row = WorkoutEditorWorkout {
                    id: w.workout.id.to_string(),
                    name: w.workout.name,
                    description: w.workout.description,
                    order_index: w.workout.order_index.value(),
                };
                let lines: Vec<WorkoutEditorLine> = w
                    .exercises
                    .into_iter()
                    .map(|we| WorkoutEditorLine {
                        exercise: we.exercise.into(),
                        order_index: we.order_index.value(),
                        sets: we.sets.value(),
                        reps: we.reps.value(),
                    })
                    .collect();
                (Some(row), lines)
            })
            .unwrap_or((None, vec![]));

        let library: Vec<WorkoutEditorExerciseItem> =
            library_domain.into_iter().map(Into::into).collect();

        Ok(WorkoutEditorDataResult {
            workout,
            exercises,
            library,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    use domain::aggregates::WorkoutWithExercises;
    use domain::entities::{Exercise, Workout};
    use domain::error::Result;
    use domain::repositories::{GetWorkoutWithExercisesRead, ListExerciseLibraryRead};
    use domain::vos::library_name_filter::LibraryNameFilter;
    use domain::vos::ScheduleOrderIndex;

    const SPEC: &str = "550e8400-e29b-41d4-a716-446655440360";
    const WID: &str = "550e8400-e29b-41d4-a716-446655440361";

    #[tokio::test]
    async fn workout_editor_empty_when_workout_missing() {
        let fake = MockWorkoutEditorRead::new(Ok(None), Ok(vec![]));
        let uc = WorkoutEditorDataUseCase::new(Arc::new(fake));

        let res = uc
            .execute(WorkoutEditorDataArgs {
                specialist_id: SPEC.to_string(),
                workout_id: WID.to_string(),
            })
            .await
            .unwrap();

        assert!(res.workout.is_none());
        assert!(res.exercises.is_empty());
        assert!(res.library.is_empty());
    }

    #[tokio::test]
    async fn workout_editor_maps_workout_and_library() {
        let spec = Id::try_from(SPEC).unwrap();
        let wid = Id::try_from(WID).unwrap();
        let workout = Workout {
            id: wid.clone(),
            specialist_id: spec.clone(),
            name: "Edit me".to_string(),
            description: Some("desc".to_string()),
            order_index: ScheduleOrderIndex::TWO,
            created_at: None,
            updated_at: None,
        };
        let lib_row = Exercise {
            id: Id::try_from("550e8400-e29b-41d4-a716-446655440362").unwrap(),
            specialist_id: spec,
            name: "Lib ex".to_string(),
            description: None,
            order_index: ScheduleOrderIndex::ZERO,
            video_url: None,
            deleted_at: None,
            created_at: None,
        };
        let fake = MockWorkoutEditorRead::new(
            Ok(Some(WorkoutWithExercises {
                workout: workout.clone(),
                exercises: vec![],
            })),
            Ok(vec![lib_row.clone()]),
        );
        let uc = WorkoutEditorDataUseCase::new(Arc::new(fake));

        let res = uc
            .execute(WorkoutEditorDataArgs {
                specialist_id: SPEC.to_string(),
                workout_id: WID.to_string(),
            })
            .await
            .unwrap();

        assert_eq!(
            res.workout.as_ref().map(|w| w.name.as_str()),
            Some("Edit me")
        );
        assert_eq!(res.library.len(), 1);
        assert_eq!(res.library[0].name, "Lib ex");
    }

    #[derive(Clone)]
    struct MockWorkoutEditorRead {
        workout: Arc<Mutex<Result<Option<WorkoutWithExercises>>>>,
        library: Arc<Mutex<Result<Vec<Exercise>>>>,
    }

    impl MockWorkoutEditorRead {
        fn new(
            workout: Result<Option<WorkoutWithExercises>>,
            library: Result<Vec<Exercise>>,
        ) -> Self {
            Self {
                workout: Arc::new(Mutex::new(workout)),
                library: Arc::new(Mutex::new(library)),
            }
        }
    }

    #[common::async_trait_platform]
    impl GetWorkoutWithExercisesRead for MockWorkoutEditorRead {
        async fn get_workout_with_exercises(
            &self,
            _workout_id: &Id,
        ) -> Result<Option<WorkoutWithExercises>> {
            self.workout.lock().unwrap().clone()
        }
    }

    #[common::async_trait_platform]
    impl ListExerciseLibraryRead for MockWorkoutEditorRead {
        async fn list_exercise_library(
            &self,
            _specialist_id: &Id,
            _name_filter: Option<&LibraryNameFilter>,
        ) -> Result<Vec<Exercise>> {
            self.library.lock().unwrap().clone()
        }
    }
}
