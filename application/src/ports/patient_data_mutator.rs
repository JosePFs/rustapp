use async_trait::async_trait;

pub use domain::repositories::PatientSessionWriteRepository;

#[async_trait]
pub trait PatientDataMutator: PatientSessionWriteRepository + Send + Sync {}

impl<T: PatientSessionWriteRepository + Send + Sync> PatientDataMutator for T {}
