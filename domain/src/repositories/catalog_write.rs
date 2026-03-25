use crate::entities::{
    Exercise, PatientProgram, Program, ProgramScheduleItem, SpecialistPatient, Workout,
    WorkoutSession,
};
use crate::error::Result;
use crate::vos::id::Id;
use crate::vos::{
    DayIndex, DaysInBlock, Description, EffortScore, ExerciseName, FeedbackComment, PainScore,
    Patch, ProgramName, Reps, ScheduleOrderIndex, SessionDate, Sets, VideoUrl, WorkoutName,
};

#[common::async_trait_platform]
pub trait AddSpecialistPatient: Send + Sync {
    async fn add_specialist_patient(&self, patient_id: &Id) -> Result<SpecialistPatient>;
}

#[common::async_trait_platform]
pub trait CreateProgram: Send + Sync {
    async fn create_program(
        &self,
        name: &ProgramName,
        description: Option<&Description>,
    ) -> Result<Program>;
}

#[common::async_trait_platform]
pub trait CreateWorkout: Send + Sync {
    async fn create_workout(
        &self,
        name: &WorkoutName,
        description: Option<&Description>,
    ) -> Result<Workout>;
}

#[common::async_trait_platform]
pub trait UpdateWorkoutWrite: Send + Sync {
    async fn update_workout(
        &self,
        workout_id: &Id,
        name: Option<&WorkoutName>,
        description: Patch<Description>,
        order_index: Option<ScheduleOrderIndex>,
    ) -> Result<()>;
}

#[common::async_trait_platform]
pub trait DeleteWorkoutWrite: Send + Sync {
    async fn delete_workout(&self, workout_id: &Id) -> Result<()>;
}

#[common::async_trait_platform]
pub trait CreateProgramScheduleItemWrite: Send + Sync {
    async fn create_program_schedule_item(
        &self,
        program_id: &Id,
        order_index: ScheduleOrderIndex,
        workout_id: Option<&Id>,
        days_count: DaysInBlock,
    ) -> Result<ProgramScheduleItem>;
}

#[common::async_trait_platform]
pub trait DeleteProgramScheduleItemWrite: Send + Sync {
    async fn delete_program_schedule_item(&self, schedule_id: &Id) -> Result<()>;
}

#[common::async_trait_platform]
pub trait CreateExerciseWrite: Send + Sync {
    async fn create_exercise(
        &self,
        name: &ExerciseName,
        description: Option<&Description>,
        order_index: ScheduleOrderIndex,
        video_url: Option<&VideoUrl>,
    ) -> Result<Exercise>;
}

#[common::async_trait_platform]
pub trait AddExerciseToWorkoutWrite: Send + Sync {
    async fn add_exercise_to_workout(
        &self,
        workout_id: &Id,
        exercise_id: &Id,
        order_index: ScheduleOrderIndex,
        sets: Sets,
        reps: Reps,
    ) -> Result<()>;
}

#[common::async_trait_platform]
pub trait RemoveExerciseFromWorkoutWrite: Send + Sync {
    async fn remove_exercise_from_workout(&self, workout_id: &Id, exercise_id: &Id) -> Result<()>;
}

#[common::async_trait_platform]
pub trait UpdateWorkoutExerciseWrite: Send + Sync {
    async fn update_workout_exercise(
        &self,
        workout_id: &Id,
        exercise_id: &Id,
        sets: Sets,
        reps: Reps,
        order_index: Option<ScheduleOrderIndex>,
    ) -> Result<()>;
}

#[common::async_trait_platform]
pub trait UpdateExerciseWrite: Send + Sync {
    async fn update_exercise(
        &self,
        exercise_id: &Id,
        name: Option<&ExerciseName>,
        description: Option<&Description>,
        order_index: Option<ScheduleOrderIndex>,
        video_url: Patch<VideoUrl>,
    ) -> Result<()>;
}

#[common::async_trait_platform]
pub trait SoftDeleteExerciseWrite: Send + Sync {
    async fn soft_delete_exercise(&self, exercise_id: &Id) -> Result<()>;
}

#[common::async_trait_platform]
pub trait RestoreExerciseWrite: Send + Sync {
    async fn restore_exercise(&self, exercise_id: &Id) -> Result<()>;
}

#[common::async_trait_platform]
pub trait AssignProgramToPatientWrite: Send + Sync {
    async fn assign_program_to_patient(
        &self,
        patient_id: &Id,
        program_id: &Id,
    ) -> Result<PatientProgram>;
}

#[common::async_trait_platform]
pub trait UnassignProgramFromPatientWrite: Send + Sync {
    async fn unassign_program_from_patient(&self, patient_program_id: &Id) -> Result<()>;
}

#[common::async_trait_platform]
pub trait GetOrCreateSessionCatalogWrite: Send + Sync {
    async fn get_or_create_session(
        &self,
        patient_program_id: &Id,
        day_index: DayIndex,
        session_date: &SessionDate,
    ) -> Result<WorkoutSession>;
}

#[common::async_trait_platform]
pub trait CompleteSessionCatalogWrite: Send + Sync {
    async fn complete_session(&self, session_id: &Id) -> Result<()>;
}

#[common::async_trait_platform]
pub trait UpdateSessionWrite: Send + Sync {
    async fn update_session(
        &self,
        session_id: &Id,
        session_date: Option<&SessionDate>,
    ) -> Result<()>;
}

#[common::async_trait_platform]
pub trait UpsertSessionExerciseFeedbackCatalogWrite: Send + Sync {
    async fn upsert_session_exercise_feedback(
        &self,
        workout_session_id: &Id,
        exercise_id: &Id,
        effort: Option<EffortScore>,
        pain: Option<PainScore>,
        comment: Option<&FeedbackComment>,
    ) -> Result<()>;
}

#[common::async_trait_platform]
pub trait UncompleteSessionCatalogWrite: Send + Sync {
    async fn uncomplete_session(&self, session_id: &Id) -> Result<()>;
}
