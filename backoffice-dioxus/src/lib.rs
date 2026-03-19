use dioxus::prelude::*;

use dioxus_i18n::prelude::*;
use dioxus_i18n::t;
use dioxus_router::{Routable, Router};
use unic_langid::langid;

use app_context::build_app_context;
use views::{
    ExerciseLibrary, LoginView, PatientProgress, ProgramEditor, SpecialistPatients,
    SpecialistPrograms, WorkoutEditor, WorkoutLibrary,
};

mod app_context;
mod components;
mod hooks;
mod views;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    LoginView {},
    #[route("/specialist")]
    SpecialistPatients {},
    #[route("/specialist/programs")]
    SpecialistPrograms {},
    #[route("/specialist/exercises")]
    ExerciseLibrary {},
    #[route("/specialist/workouts")]
    WorkoutLibrary {},
    #[route("/specialist/workouts/:id")]
    WorkoutEditor { id: String },
    #[route("/specialist/patient/:id")]
    PatientProgress { id: String },
    #[route("/programs/:id/edit")]
    ProgramEditor { id: String },
}

pub fn launch() {
    init_logging();
    log::debug!("Launching backoffice app");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    init_i18n();

    let app_context = match build_app_context() {
        Ok(ctx) => ctx,
        Err(e) => {
            return rsx! {
                div {
                    class: "p-4 text-destructive bg-destructive/10 rounded",
                    { t!("error_config", detail: e.to_string()) }
                }
            };
        }
    };
    use_context_provider(|| app_context);

    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.png") }
        document::Stylesheet { href: asset!("/assets/app.css") }

        Title { "Eixe - Backoffice" }

        ErrorBoundary {
            handle_error: |error: ErrorContext| {
                let msg = error.error().map(|e| e.to_string()).unwrap_or_default();
                rsx! {
                    div { { t!("error_unexpected", detail: msg) } }
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
            .with_tag("Backoffice"),
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
