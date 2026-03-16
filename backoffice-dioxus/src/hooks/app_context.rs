use dioxus::prelude::*;

use crate::app_context::AppContext;

pub fn use_app_context() -> AppContext {
    use_context::<AppContext>()
}
