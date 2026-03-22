use std::sync::{Arc, LazyLock};

use crate::supabase::{auth::SupabaseAuth, client::SupabaseClient, config::SupabaseConfig};

pub mod auth;
pub mod client;
pub mod config;
pub mod repositories;

static DEFAULT_AUTH: LazyLock<Arc<SupabaseAuth>> =
    LazyLock::new(|| SupabaseAuth::builder().build());

pub(crate) static DEFAULT_CLIENT: LazyLock<Arc<SupabaseClient>> = LazyLock::new(|| {
    let config = SupabaseConfig::from_env().expect("Failed to load Supabase config");
    let auth = DEFAULT_AUTH.clone();
    Arc::new(SupabaseClient::new(config, auth))
});

pub fn default_auth() -> Arc<SupabaseAuth> {
    DEFAULT_AUTH.clone()
}
