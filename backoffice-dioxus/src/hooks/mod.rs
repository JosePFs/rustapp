use domain::error::DomainError;

pub mod app_context;
pub mod exercise_library;
pub mod login;
pub mod patient_programs;
pub mod patient_progress;
pub mod specialist_patients;
pub mod specialist_programs;
pub mod submit_workout_feedback;
pub mod uncomplete_workout_session;
pub mod workout_day_detail;
pub mod workout_editor;
pub mod workout_library;

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
