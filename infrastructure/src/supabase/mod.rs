pub mod api;
pub mod client;
pub mod config;
#[cfg(not(target_arch = "wasm32"))]
pub mod native_api;
#[cfg(target_arch = "wasm32")]
pub mod native_api {
    #[derive(Clone)]
    pub struct NativeApi;
}
