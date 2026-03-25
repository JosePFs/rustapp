mod catalog_read;
mod catalog_write;
mod patient_session_repository;
mod specialist_read_repository;
mod specialist_write_repository;

pub use catalog_read::*;
pub use catalog_write::*;
pub use patient_session_repository::PatientSessionRepository;
pub use specialist_read_repository::SpecialistCatalogReadRepository;
pub use specialist_write_repository::SpecialistCatalogWriteRepository;

pub trait SpecialistRepository:
    Send + Sync + SpecialistCatalogReadRepository + SpecialistCatalogWriteRepository
{
}

impl<T> SpecialistRepository for T where
    T: Send + Sync + SpecialistCatalogReadRepository + SpecialistCatalogWriteRepository
{
}
