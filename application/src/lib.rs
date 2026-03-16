pub mod domain {
    pub use ::domain::*;
}

pub mod ports;
pub mod use_cases;

pub trait Backend:
    ports::AuthService + ports::DataMutator + ports::DataProvider + Send + Sync
{
}

pub trait MobileBackend:
    ports::AuthServiceSend + ports::DataProviderSend + ports::DataMutatorSend + Send + Sync
{
}

pub mod application {
    pub use super::ports;
    pub use super::use_cases;
    pub use super::Backend;
    pub use super::MobileBackend;
}
