use dioxus::prelude::*;

use crate::components::Skeleton;

const DEFAULT_COUNT: usize = 3;

#[component]
pub fn SkeletonCard(count: Option<usize>) -> Element {
    let count = count.unwrap_or(DEFAULT_COUNT);
    rsx! {
        for _ in 0..count {
            div { class: "flex flex-col gap-2",
                Skeleton { style: "width: 100%; height: 24rem; border-radius: 0.75rem; margin-bottom: 1rem;" }
            }
        }
    }
}
