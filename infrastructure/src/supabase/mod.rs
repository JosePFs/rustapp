use std::sync::{Arc, LazyLock};

use crate::supabase::{client::SupabaseClient, config::SupabaseConfig};

pub mod api;
pub mod auth;
pub mod client;
pub mod config;
#[cfg(not(target_arch = "wasm32"))]
pub mod native_api;
#[cfg(target_arch = "wasm32")]
pub mod native_api {
    #[derive(Clone)]
    pub struct NativeApi;
}

pub static DEFAULT_CLIENT: LazyLock<Arc<SupabaseClient>> = LazyLock::new(|| {
    Arc::new(SupabaseClient::new(
        SupabaseConfig::from_env().expect("Failed to load Supabase config"),
    ))
});
