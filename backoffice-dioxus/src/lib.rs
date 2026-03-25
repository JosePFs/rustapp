use application::ports::error::ApplicationError;
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
    #[layout(AppLayout)]
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

#[component]
fn ErrorView(error: ErrorContext) -> Element {
    let nav = use_navigator();

    let is_auth_error = match error.error() {
        Some(e) if e.downcast_ref::<ApplicationError>().is_some() => e
            .downcast_ref::<ApplicationError>()
            .map(|ae| ae.is_auth_error())
            .unwrap_or(false),
        _ => false,
    };

    use_effect(move || {
        if is_auth_error {
            nav.push(Route::LoginView {});
        }
    });

    if is_auth_error {
        return rsx! {};
    }

    let msg = error.error().map(|e| e.to_string()).unwrap_or_default();

    rsx! {
        div { class: "min-h-screen flex items-center justify-center bg-gray-50 p-4",
        div { class: "max-w-md w-full bg-white rounded-lg shadow-lg p-6 border-l-4 border-red-500",
            div { class: "flex items-center gap-3 mb-4",
                svg { class: "w-8 h-8 text-red-500 flex-shrink-0", view_box: "0 0 24 24", fill: "none", stroke: "currentColor", stroke_width: "2",
                    circle { cx: "12", cy: "12", r: "10" }
                    line { x1: "12", y1: "8", x2: "12", y2: "12" }
                    line { x1: "12", y1: "16", x2: "12.01", y2: "16" }
                }
                h2 { class: "text-xl font-semibold text-gray-800", { t!("error_unexpected_title") } }
            }
            p { class: "text-gray-600 text-sm mb-4", { t!("error_unexpected", detail: msg) } }
        }
    }
    }
}

#[component]
fn AppLayout() -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: |error: ErrorContext| rsx! {
                ErrorView { error }
            },
            Outlet::<Route> {}
        }
    }
}

#[component]
fn App() -> Element {
    use_init_i18n(|| {
        I18nConfig::new(langid!("es-ES"))
            .with_locale((langid!("es-ES"), include_str!("../i18n/es-ES.ftl")))
            .with_locale((langid!("gl-ES"), include_str!("../i18n/gl-ES.ftl")))
            .with_locale((langid!("en-EN"), include_str!("../i18n/en-EN.ftl")))
    });

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
        Title { "Eixe" }
        Router::<Route> {}
    }
}

pub fn launch() {
    init_logging();
    log::debug!("Launching backoffice app");
    dioxus::launch(App);
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
