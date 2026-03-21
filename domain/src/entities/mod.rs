use crate::vos::id::Id;

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
    pub order_index: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramScheduleItem {
    pub id: Id,
    pub program_id: Id,
    pub order_index: i32,
    pub workout_id: Option<Id>,
    pub days_count: i32,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Exercise {
    pub id: Id,
    pub specialist_id: Id,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
    pub video_url: Option<String>,
    pub deleted_at: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorkoutExercise {
    pub exercise: Exercise,
    pub order_index: i32,
    pub sets: i32,
    pub reps: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SessionExerciseFeedback {
    pub workout_session_id: Id,
    pub exercise_id: Id,
    pub effort: Option<i32>,
    pub pain: Option<i32>,
    pub comment: Option<String>,
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
    pub day_index: i32,
    pub session_date: String,
    pub completed_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
