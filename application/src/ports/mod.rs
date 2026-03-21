pub mod auth;
pub mod patient_data_mutator;
pub mod patient_data_provider;
pub mod specialist_data_mutator;
pub mod specialist_data_provider;

pub use patient_data_mutator::DataMutatorSend;
pub use patient_data_provider::DataProviderSend;
pub use specialist_data_mutator::SpecialistDataMutator;
pub use specialist_data_provider::DataProvider;

pub trait Backend: SpecialistDataMutator + DataProvider + Send + Sync {}

pub trait MobileBackend: DataProviderSend + DataMutatorSend + Send + Sync {}
