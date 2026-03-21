//! Workspace proc-macros. See [`async_trait_platform`].

use proc_macro::TokenStream;
use quote::quote;

/// Expands to the platform-specific `async_trait` attributes used for wasm vs native.
///
/// ```ignore
/// #[common::async_trait_platform]
/// pub trait MyPort: Send + Sync {
///     async fn run(&self) -> ();
/// }
/// ```
#[proc_macro_attribute]
pub fn async_trait_platform(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    let expanded = quote! {
        #[cfg_attr(target_arch = "wasm32", ::async_trait::async_trait(?Send))]
        #[cfg_attr(not(target_arch = "wasm32"), ::async_trait::async_trait)]
        #item
    };
    TokenStream::from(expanded)
}
