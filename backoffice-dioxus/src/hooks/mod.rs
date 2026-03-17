use domain::error::DomainError;

pub mod add_exercise_to_workout;
pub mod add_specialist_patient;
pub mod app_context;
pub mod assign_program_to_patient;
pub mod create_exercise;
pub mod create_program;
pub mod create_program_schedule_item;
pub mod create_workout;
pub mod delete_program_schedule_item;
pub mod delete_workout;
pub mod exercise_library;
pub mod list_program_schedule;
pub mod login;
pub mod patient_programs;
pub mod patient_progress;
pub mod remove_exercise_from_workout;
pub mod restore_exercise;
pub mod soft_delete_exercise;
pub mod specialist_patients;
pub mod specialist_programs;
pub mod submit_workout_feedback;
pub mod uncomplete_workout_session;
pub mod update_exercise;
pub mod update_workout;
pub mod update_workout_exercise;
pub mod workout_day_detail;
pub mod workout_editor;
pub mod workout_library;
pub mod workout_library_data;

#[derive(Clone, Debug, PartialEq)]
pub enum AsyncState<T> {
    Idle,
    Loading,
    Error(DomainError),
    Ready(T),
}

impl<T> AsyncState<T> {
    pub fn is_idle(&self) -> bool {
        matches!(self, AsyncState::Idle)
    }

    pub fn is_loading(&self) -> bool {
        matches!(self, AsyncState::Loading)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, AsyncState::Error(_))
    }

    pub fn is_ready(&self) -> bool {
        matches!(self, AsyncState::Ready(_))
    }

    pub fn data(&self) -> Option<&T> {
        match self {
            AsyncState::Ready(data) => Some(data),
            _ => None,
        }
    }

    pub fn error(&self) -> Option<&DomainError> {
        match self {
            AsyncState::Error(error) => Some(error),
            _ => None,
        }
    }
}
