use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_primitives::slider::SliderValue;

use crate::components::{
    Card, CardContent, CardDescription, CardHeader, CardTitle, Slider, SliderRange, SliderThumb,
    SliderTrack, Textarea, TextareaVariant,
};

#[component]
pub fn PatientWorkout(
    exercise_id: String,
    exercise_name: String,
    exercise_desc: Option<String>,
    embed_url: Option<String>,
    sets: i32,
    reps: i32,
    exercise_feedback: Signal<HashMap<String, (i32, i32, String)>>,
) -> Element {
    let ex_id_effort = exercise_id.clone();
    let ex_id_pain = exercise_id.clone();
    let ex_id_comment = exercise_id.clone();
    let ex_id_effect = exercise_id.clone();

    let mut effort = use_signal(|| 5i32);
    let mut pain = use_signal(|| 0i32);
    let mut comment = use_signal(|| String::new());

    use_effect(move || {
        let (new_effort, new_pain, new_comment) = exercise_feedback
            .read()
            .get(&ex_id_effect)
            .cloned()
            .unwrap_or((5, 0, String::new()));
        effort.set(new_effort);
        pain.set(new_pain);
        comment.set(new_comment);
    });

    rsx! {
        article { class: "mb-4",
            Card {
                CardHeader {
                    CardTitle { "{exercise_name}" }
                    if let Some(ref desc) = exercise_desc {
                        if !desc.is_empty() {
                            CardDescription { "{desc}" }
                        }
                    }
                }
                CardContent {
                    p { class: "text-sm font-semibold text-text-muted mb-2", "Series: {sets} × Repeticiones: {reps}" }
                    if let Some(ref embed) = embed_url {
                        iframe {
                            class: "w-full mb-6 aspect-video rounded-md border border-border bg-black",
                            src: "{embed}",
                            allow: "accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share",
                            allowfullscreen: "true",
                        }
                    }
                    div {
                        div { class: "flex items-center justify-start gap-4 mb-4",
                            label { class: "text-sm mt-0 mb-0 w-1/4", "Esfuerzo" }
                            Slider {
                                label: "Esfuerzo",
                                min: 1.0,
                                max: 10.0,
                                step: 1.0,
                                horizontal: true,
                                value: Some(SliderValue::Single(effort() as f64)),
                                on_value_change: move |value: SliderValue| {
                                    let SliderValue::Single(v) = value;
                                    effort.set(v as i32);
                                    if let Some(entry) = exercise_feedback.write().get_mut(&ex_id_effort) {
                                        entry.0 = v as i32;
                                    }
                                },
                                SliderTrack {
                                    SliderRange {}
                                    SliderThumb {}
                                }
                            }
                            span { class: "text-sm font-semibold", "{effort}" } }
                        div { class: "flex items-center justify-start gap-4 mb-4",
                            label { class: "text-sm mt-0 mb-0 w-1/4", "Dolor" }
                            Slider {
                                min: 0.0,
                                max: 10.0,
                                step: 1.0,
                                horizontal: true,
                                value: Some(SliderValue::Single(pain() as f64)),
                                on_value_change: move |value: SliderValue| {
                                    let SliderValue::Single(v) = value;
                                    pain.set(v as i32);
                                    if let Some(entry) = exercise_feedback.write().get_mut(&ex_id_pain) {
                                        entry.1 = v as i32;
                                    }
                                },
                                SliderTrack {
                                    SliderRange {}
                                    SliderThumb {}
                                }
                            }
                            span { class: "text-sm font-semibold", "{pain}" }
                        }
                        label { class: "text-sm mt-0 mb-0", for: "comment-{exercise_id}", "Comentario" }
                        Textarea {
                            id: "comment-{exercise_id}",
                            variant: TextareaVariant::Outline,
                            placeholder: "Opcional",
                            value: "{comment()}",
                            oninput: move |e: FormEvent| {
                                comment.set(e.value().clone());
                                if let Some(entry) = exercise_feedback.write().get_mut(&ex_id_comment) {
                                    entry.2 = e.value().clone();
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
