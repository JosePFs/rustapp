use std::sync::Arc;

use futures::try_join;

use domain::entities::Exercise;
use domain::error::Result;
use domain::repositories::{GetWorkoutWithExercisesRead, ListExerciseLibraryRead};
use domain::vos::id::Id;
use domain::vos::AccessToken;

#[derive(Clone)]
pub struct WorkoutEditorDataArgs {
    pub token: String,
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

fn map_exercise_item(e: Exercise) -> WorkoutEditorExerciseItem {
    WorkoutEditorExerciseItem {
        id: e.id.to_string(),
        name: e.name,
        description: e.description,
        order_index: e.order_index,
        video_url: e.video_url,
        deleted_at: e.deleted_at,
    }
}

pub struct WorkoutEditorDataUseCase<R: GetWorkoutWithExercisesRead + ListExerciseLibraryRead> {
    catalog_read: Arc<R>,
}

impl<R: GetWorkoutWithExercisesRead + ListExerciseLibraryRead> WorkoutEditorDataUseCase<R> {
    pub fn new(catalog_read: Arc<R>) -> Self {
        Self { catalog_read }
    }

    pub async fn execute(&self, args: WorkoutEditorDataArgs) -> Result<WorkoutEditorDataResult> {
        let access = AccessToken::try_from(args.token)?;
        let specialist_id = Id::try_from(args.specialist_id)?;
        let workout_id = Id::try_from(args.workout_id)?;

        let (workout_with_exercises, library_domain) = try_join!(
            self.catalog_read
                .get_workout_with_exercises(&access, &workout_id),
            self.catalog_read
                .list_exercise_library(&access, &specialist_id, None),
        )?;

        let (workout, exercises) = workout_with_exercises
            .map(|w| {
                let row = WorkoutEditorWorkout {
                    id: w.workout.id.to_string(),
                    name: w.workout.name,
                    description: w.workout.description,
                    order_index: w.workout.order_index,
                };
                let lines: Vec<WorkoutEditorLine> = w
                    .exercises
                    .into_iter()
                    .map(|we| WorkoutEditorLine {
                        exercise: map_exercise_item(we.exercise),
                        order_index: we.order_index,
                        sets: we.sets,
                        reps: we.reps,
                    })
                    .collect();
                (Some(row), lines)
            })
            .unwrap_or((None, vec![]));

        let library: Vec<WorkoutEditorExerciseItem> =
            library_domain.into_iter().map(map_exercise_item).collect();

        Ok(WorkoutEditorDataResult {
            workout,
            exercises,
            library,
        })
    }
}
