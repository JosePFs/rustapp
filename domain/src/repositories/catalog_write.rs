use async_trait::async_trait;

use crate::entities::{
    Exercise, PatientProgram, Program, ProgramScheduleItem, SpecialistPatient, Workout,
    WorkoutSession,
};
use crate::error::Result;
use crate::vos::id::Id;
use crate::vos::{
    AccessToken, DayIndex, DaysInBlock, Description, EffortScore, ExerciseName, FeedbackComment,
    PainScore, Patch, ProgramName, Reps, ScheduleOrderIndex, SessionDate, Sets, VideoUrl,
    WorkoutName,
};

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait AddSpecialistPatientWrite: Send + Sync {
    async fn add_specialist_patient(
        &self,
        access_token: &AccessToken,
        specialist_id: &Id,
        patient_id: &Id,
    ) -> Result<SpecialistPatient>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait CreateProgramWrite: Send + Sync {
    async fn create_program(
        &self,
        access_token: &AccessToken,
        specialist_id: &Id,
        name: &ProgramName,
        description: Option<&Description>,
    ) -> Result<Program>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait CreateWorkoutWrite: Send + Sync {
    async fn create_workout(
        &self,
        access_token: &AccessToken,
        specialist_id: &Id,
        name: &WorkoutName,
        description: Option<&Description>,
    ) -> Result<Workout>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait UpdateWorkoutWrite: Send + Sync {
    async fn update_workout(
        &self,
        access_token: &AccessToken,
        workout_id: &Id,
        name: Option<&WorkoutName>,
        description: Patch<Description>,
        order_index: Option<ScheduleOrderIndex>,
    ) -> Result<()>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait DeleteWorkoutWrite: Send + Sync {
    async fn delete_workout(&self, access_token: &AccessToken, workout_id: &Id) -> Result<()>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait CreateProgramScheduleItemWrite: Send + Sync {
    async fn create_program_schedule_item(
        &self,
        access_token: &AccessToken,
        program_id: &Id,
        order_index: ScheduleOrderIndex,
        workout_id: Option<&Id>,
        days_count: DaysInBlock,
    ) -> Result<ProgramScheduleItem>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait DeleteProgramScheduleItemWrite: Send + Sync {
    async fn delete_program_schedule_item(
        &self,
        access_token: &AccessToken,
        schedule_id: &Id,
    ) -> Result<()>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait CreateExerciseWrite: Send + Sync {
    async fn create_exercise(
        &self,
        access_token: &AccessToken,
        specialist_id: &Id,
        name: &ExerciseName,
        description: Option<&Description>,
        order_index: ScheduleOrderIndex,
        video_url: Option<&VideoUrl>,
    ) -> Result<Exercise>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait AddExerciseToWorkoutWrite: Send + Sync {
    async fn add_exercise_to_workout(
        &self,
        access_token: &AccessToken,
        workout_id: &Id,
        exercise_id: &Id,
        order_index: ScheduleOrderIndex,
        sets: Sets,
        reps: Reps,
    ) -> Result<()>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait RemoveExerciseFromWorkoutWrite: Send + Sync {
    async fn remove_exercise_from_workout(
        &self,
        access_token: &AccessToken,
        workout_id: &Id,
        exercise_id: &Id,
    ) -> Result<()>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait UpdateWorkoutExerciseWrite: Send + Sync {
    async fn update_workout_exercise(
        &self,
        access_token: &AccessToken,
        workout_id: &Id,
        exercise_id: &Id,
        sets: Sets,
        reps: Reps,
        order_index: Option<ScheduleOrderIndex>,
    ) -> Result<()>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait UpdateExerciseWrite: Send + Sync {
    async fn update_exercise(
        &self,
        access_token: &AccessToken,
        exercise_id: &Id,
        name: Option<&ExerciseName>,
        description: Option<&Description>,
        order_index: Option<ScheduleOrderIndex>,
        video_url: Patch<VideoUrl>,
    ) -> Result<()>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait SoftDeleteExerciseWrite: Send + Sync {
    async fn soft_delete_exercise(
        &self,
        access_token: &AccessToken,
        exercise_id: &Id,
    ) -> Result<()>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait RestoreExerciseWrite: Send + Sync {
    async fn restore_exercise(&self, access_token: &AccessToken, exercise_id: &Id) -> Result<()>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait AssignProgramToPatientWrite: Send + Sync {
    async fn assign_program_to_patient(
        &self,
        access_token: &AccessToken,
        patient_id: &Id,
        program_id: &Id,
    ) -> Result<PatientProgram>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait UnassignProgramFromPatientWrite: Send + Sync {
    async fn unassign_program_from_patient(
        &self,
        access_token: &AccessToken,
        patient_program_id: &Id,
    ) -> Result<()>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait GetOrCreateSessionCatalogWrite: Send + Sync {
    async fn get_or_create_session(
        &self,
        access_token: &AccessToken,
        patient_program_id: &Id,
        day_index: DayIndex,
        session_date: &SessionDate,
    ) -> Result<WorkoutSession>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait CompleteSessionCatalogWrite: Send + Sync {
    async fn complete_session(&self, access_token: &AccessToken, session_id: &Id) -> Result<()>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait UpdateSessionWrite: Send + Sync {
    async fn update_session(
        &self,
        access_token: &AccessToken,
        session_id: &Id,
        session_date: Option<&SessionDate>,
    ) -> Result<()>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait UpsertSessionExerciseFeedbackCatalogWrite: Send + Sync {
    async fn upsert_session_exercise_feedback(
        &self,
        access_token: &AccessToken,
        workout_session_id: &Id,
        exercise_id: &Id,
        effort: Option<EffortScore>,
        pain: Option<PainScore>,
        comment: Option<&FeedbackComment>,
    ) -> Result<()>;
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait UncompleteSessionCatalogWrite: Send + Sync {
    async fn uncomplete_session(&self, access_token: &AccessToken, session_id: &Id) -> Result<()>;
}
