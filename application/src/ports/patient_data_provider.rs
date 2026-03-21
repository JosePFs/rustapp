use async_trait::async_trait;

pub use domain::repositories::SpecialistCatalogReadRepository;

#[async_trait]
pub trait PatientDataProvider: SpecialistCatalogReadRepository + Send + Sync {}

impl<T: SpecialistCatalogReadRepository + Send + Sync> PatientDataProvider for T {}
