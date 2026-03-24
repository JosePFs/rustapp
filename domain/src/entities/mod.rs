use crate::vos::{
    id::Id, DayIndex, DaysInBlock, EffortScore, FeedbackComment, PainScore, Reps,
    ScheduleOrderIndex, SessionDate, Sets, VideoUrl,
};

#[derive(Debug, Clone, PartialEq)]
pub struct SpecialistPatient {
    pub id: Id,
    pub specialist_id: Id,
    pub patient_id: Id,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub id: Id,
    pub specialist_id: Id,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Workout {
    pub id: Id,
    pub specialist_id: Id,
    pub name: String,
    pub description: Option<String>,
    pub order_index: ScheduleOrderIndex,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramScheduleItem {
    pub id: Id,
    pub program_id: Id,
    pub order_index: ScheduleOrderIndex,
    pub workout_id: Option<Id>,
    pub days_count: DaysInBlock,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Exercise {
    pub id: Id,
    pub specialist_id: Id,
    pub name: String,
    pub description: Option<String>,
    pub order_index: ScheduleOrderIndex,
    pub video_url: Option<VideoUrl>,
    pub deleted_at: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorkoutExercise {
    pub exercise: Exercise,
    pub order_index: ScheduleOrderIndex,
    pub sets: Sets,
    pub reps: Reps,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SessionExerciseFeedback {
    pub workout_session_id: Id,
    pub exercise_id: Id,
    pub effort: Option<EffortScore>,
    pub pain: Option<PainScore>,
    pub comment: Option<FeedbackComment>,
}

#[derive(Debug, Clone)]
pub struct PatientProgram {
    pub id: Id,
    pub patient_id: Id,
    pub program_id: Id,
    pub status: String,
}

impl PatientProgram {
    pub fn is_active(&self) -> bool {
        self.status == "active"
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorkoutSession {
    pub id: Id,
    pub patient_program_id: Id,
    pub day_index: DayIndex,
    pub session_date: SessionDate,
    pub completed_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
