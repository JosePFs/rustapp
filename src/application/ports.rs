use crate::application::services::{AuthService, DataMutator, DataProvider};

pub trait Backend: AuthService + DataProvider + DataMutator + Send + Sync {}
