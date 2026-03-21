mod auth;
mod catalog_read;
mod catalog_write;
mod patient_session;
mod specialist_onboarding;

pub use auth::FakeAuthService;
pub use catalog_read::{
    FakeGetProfilesByIds, FakeListExerciseLibrary, FakeListWorkoutLibrary,
    FakePatientProgressCatalog, FakeSpecialistDashboardRead, FakeSpecialistPatientsAndProfiles,
};
pub use catalog_write::{
    FakeAddExerciseToWorkout, FakeAssignProgramToPatient, FakeCreateProgram, FakeCreateWorkout,
    FakeDeleteProgramScheduleItem, FakeDeleteWorkout, FakeRemoveExerciseFromWorkout,
    FakeRestoreExercise, FakeSoftDeleteExercise,
};
pub use patient_session::FakePatientSessionWrite;
pub use specialist_onboarding::FakeOnboardSpecialistPatient;
