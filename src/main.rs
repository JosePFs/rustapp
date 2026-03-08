use std::sync::Arc;

use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use dioxus_router::{Routable, Router};
use unic_langid::langid;

use crate::infrastructure::{
    app_context::AppContext,
    supabase::{api::Api, client::SupabaseClient, config::SupabaseConfig},
    ui::views::{
        ExerciseLibrary, LoginView, PatientDashboard, PatientProgress, PatientWorkoutDay,
        ProgramEditor, SpecialistDashboard, WorkoutEditor, WorkoutLibrary,
    },
};
use application::ports::Backend;

mod application;
mod domain;
mod infrastructure;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    LoginView {},
    #[route("/specialist")]
    SpecialistDashboard {},
    #[route("/specialist/exercises")]
    ExerciseLibrary {},
    #[route("/specialist/workouts")]
    WorkoutLibrary {},
    #[route("/specialist/workouts/:id")]
    WorkoutEditor { id: String },
    #[route("/specialist/patient/:id")]
    PatientProgress { id: String },
    #[route("/patient")]
    PatientDashboard {},
    #[route("/patient/program/:patient_program_id/day/:day_index")]
    PatientWorkoutDay {
        patient_program_id: String,
        day_index: String,
    },
    #[route("/programs/:id/edit")]
    ProgramEditor { id: String },
}

fn main() {
    init_logging();
    log::debug!("Launching app");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    init_i18n();

    let config = SupabaseConfig::from_env();
    if config.is_none() {
        return rsx! { div { "Configuration error" } };
    }

    let api = Api::new(SupabaseClient::new(config.unwrap()));
    let backend: Arc<dyn Backend> = Arc::new(api);
    use_context_provider(|| AppContext::new(backend, None));

    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.png") }
        document::Stylesheet { href: asset!("/assets/dx-components-theme.css") }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        document::Stylesheet { href: asset!("/assets/styling/main.css") }

        Title { "Eixe - MVP" }

        ErrorBoundary {
            handle_error: |error: ErrorContext| {
                let msg = error.error().map(|e| e.to_string()).unwrap_or_else(|| String::new());
                rsx! {
                    div { "Oops, we encountered an error: {msg}" }
                }
            },
            Router::<Route> {}
        }
    }
}

fn init_logging() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::default());
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("MiApp"),
    );
}

fn init_i18n() {
    use_init_i18n(|| {
        I18nConfig::new(langid!("es-ES"))
            .with_locale((langid!("es-ES"), include_str!("../i18n/es-ES.ftl")))
            .with_locale((langid!("gl-ES"), include_str!("../i18n/gl-ES.ftl")))
            .with_locale((langid!("en-EN"), include_str!("../i18n/en-EN.ftl")))
    });
}
