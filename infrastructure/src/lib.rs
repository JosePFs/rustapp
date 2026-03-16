pub mod application {
    pub use ::application::*;
}

pub mod domain {
    pub use ::domain::*;
}

pub mod api;
pub mod supabase;

pub mod infrastructure {
    pub use super::api;
    pub use super::supabase;
}
