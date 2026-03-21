pub mod auth;
pub mod patient_data_mutator;
pub mod patient_data_provider;
pub mod specialist_data_mutator;
pub mod specialist_data_provider;

pub use patient_data_mutator::{PatientDataMutator, PatientSessionWriteRepository};
pub use patient_data_provider::PatientDataProvider;
pub use specialist_data_mutator::{SpecialistCatalogWriteRepository, SpecialistDataMutator};
pub use specialist_data_provider::{SpecialistCatalogReadRepository, SpecialistDataProvider};

pub trait Backend: SpecialistDataMutator + SpecialistDataProvider + Send + Sync {}

impl<T: SpecialistDataMutator + SpecialistDataProvider + Send + Sync> Backend for T {}

pub trait MobileBackend: PatientDataProvider + PatientDataMutator + Send + Sync {}

impl<T: PatientDataProvider + PatientDataMutator + Send + Sync> MobileBackend for T {}
