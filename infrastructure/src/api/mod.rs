use std::sync::LazyLock;

pub mod axum_client;
pub mod config;
pub mod dtos;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) static SHARED_REQWEST_CLIENT: LazyLock<reqwest::Client> =
    LazyLock::new(reqwest::Client::new);