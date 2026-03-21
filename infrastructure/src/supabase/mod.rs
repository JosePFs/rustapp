use std::sync::{Arc, LazyLock};

use crate::supabase::{client::SupabaseClient, config::SupabaseConfig};

pub mod auth;
pub mod client;
pub mod config;
pub mod repositories;

pub static DEFAULT_CLIENT: LazyLock<Arc<SupabaseClient>> = LazyLock::new(|| {
    Arc::new(SupabaseClient::new(
        SupabaseConfig::from_env().expect("Failed to load Supabase config"),
    ))
});
