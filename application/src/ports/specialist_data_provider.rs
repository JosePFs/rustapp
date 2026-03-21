use async_trait::async_trait;

pub use domain::repositories::SpecialistCatalogReadRepository;

#[async_trait]
pub trait SpecialistDataProvider: SpecialistCatalogReadRepository {}

impl<T: SpecialistCatalogReadRepository + ?Sized> SpecialistDataProvider for T {}
