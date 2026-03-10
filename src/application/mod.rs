use crate::application::ports::{AuthService, DataMutator, DataProvider};

pub mod ports;
pub mod use_cases;

pub trait Backend: AuthService + DataProvider + DataMutator + Send + Sync {}
