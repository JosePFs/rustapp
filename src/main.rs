//! MVP Phase 1 — Physiotherapy clinic app.
//! Entry point and router setup.

mod components;
mod pages;
mod services;

use dioxus::prelude::*;
use dioxus_router::{Router, Routable};

use pages::{ExerciseLibrary, Login, SpecialistDashboard, PatientDashboard, PatientProgress, ProgramEditor, WorkoutEditor, WorkoutLibrary};
use services::supabase_client::{AuthSession, SupabaseConfig};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Login {},
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
    #[route("/programs/:id/edit")]
    ProgramEditor { id: String },
}

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let config = use_signal(|| SupabaseConfig::from_env());
    let session = use_signal(|| Option::<AuthSession>::None);
    use_context_provider(|| config);
    use_context_provider(|| session);

    rsx! {
        Router::<Route> {}
    }
}
