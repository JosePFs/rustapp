#[derive(Debug, Clone, PartialEq)]
pub struct SpecialistPatient {
    pub id: String,
    pub specialist_id: String,
    pub patient_id: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub id: String,
    pub specialist_id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Workout {
    pub id: String,
    pub specialist_id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramScheduleItem {
    pub id: String,
    pub program_id: String,
    pub order_index: i32,
    pub workout_id: Option<String>,
    pub days_count: i32,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Exercise {
    pub id: String,
    pub specialist_id: String,
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
    pub workout_session_id: String,
    pub exercise_id: String,
    pub effort: Option<i32>,
    pub pain: Option<i32>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PatientProgram {
    pub id: String,
    pub patient_id: String,
    pub program_id: String,
    pub status: String,
}

impl PatientProgram {
    pub fn is_active(&self) -> bool {
        self.status == "active"
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorkoutSession {
    pub id: String,
    pub patient_program_id: String,
    pub day_index: i32,
    pub session_date: String,
    pub completed_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
