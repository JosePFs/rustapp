use crate::domain::error::DomainError;

pub mod app_context;
pub mod login;
pub mod patient_programs;
pub mod workout_day_detail;

#[derive(Clone, Debug, PartialEq)]
pub enum AsyncState<T> {
    Loading,
    Error(DomainError),
    Ready(T),
}

impl<T> AsyncState<T> {
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
}
