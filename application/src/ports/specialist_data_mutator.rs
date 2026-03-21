use async_trait::async_trait;

pub use domain::repositories::SpecialistCatalogWriteRepository;

#[async_trait]
pub trait SpecialistDataMutator: SpecialistCatalogWriteRepository {}

impl<T: SpecialistCatalogWriteRepository + ?Sized> SpecialistDataMutator for T {}
