use dioxus::prelude::*;

use crate::infrastructure::app_context::AppContext;

pub fn use_app_context() -> AppContext {
    use_context::<AppContext>()
}
