//! UI pages (presentation layer).

pub mod exercise_library;
pub mod login;
pub mod specialist_dashboard;
pub mod patient_dashboard;
pub mod patient_progress;
pub mod program_editor;
pub mod workout_editor;
pub mod workout_library;

pub use exercise_library::ExerciseLibrary;
pub use login::Login;
pub use specialist_dashboard::SpecialistDashboard;
pub use patient_dashboard::PatientDashboard;
pub use patient_progress::PatientProgress;
pub use program_editor::ProgramEditor;
pub use workout_editor::WorkoutEditor;
pub use workout_library::WorkoutLibrary;
