use dioxus::prelude::*;

use dioxus_free_icons::{icons::io_icons::IoCaretBack, Icon};
use dioxus_router::Link;

use crate::Route;

#[component]
pub fn Backview(to: Route, children: Element) -> Element {
    rsx! {
        div { class: "backview relative flex items-center mb-6",
            Link {
                to,
                class: "absolute left-0 flex items-center w-8 h-8",
                Icon { width: 24, height: 24, icon: IoCaretBack }
            }
            h1 { class: "text-2xl font-semibold w-full text-center",
                {children}
            }
        }
    }
}
