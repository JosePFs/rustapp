use std::sync::{Arc, Mutex};

use domain::aggregates::{PatientProgramFull, SpecialistDashboard};
use domain::entities::{Exercise, PatientProgram, SpecialistPatient, Workout};
use domain::error::Result;
use domain::repositories::{
    GetPatientProgramFullRead, GetProfilesByIdsRead, GetSpecialistDashboardRead,
    ListExerciseLibraryRead, ListPatientProgramsForSpecialistRead, ListSpecialistPatientsRead,
    ListWorkoutLibraryRead,
};
use domain::vos::id::Id;
use domain::vos::library_name_filter::LibraryNameFilter;
use domain::vos::profile::Profile;
use domain::vos::AccessToken;

#[derive(Clone)]
pub struct FakeGetProfilesByIds {
    pub profiles: Arc<Mutex<Result<Vec<Profile>>>>,
}

impl FakeGetProfilesByIds {
    pub fn new_ok(profiles: Vec<Profile>) -> Self {
        Self {
            profiles: Arc::new(Mutex::new(Ok(profiles))),
        }
    }
}

#[common::async_trait_platform]
impl GetProfilesByIdsRead for FakeGetProfilesByIds {
    async fn get_profiles_by_ids(
        &self,
        _ids: &[Id],
        _access_token: &AccessToken,
    ) -> Result<Vec<Profile>> {
        self.profiles.lock().unwrap().clone()
    }
}

#[derive(Clone)]
pub struct FakeSpecialistPatientsAndProfiles {
    pub patients: Arc<Mutex<Result<Vec<SpecialistPatient>>>>,
    pub profiles: Arc<Mutex<Result<Vec<Profile>>>>,
}

impl FakeSpecialistPatientsAndProfiles {
    pub fn new_ok(patients: Vec<SpecialistPatient>, profiles: Vec<Profile>) -> Self {
        Self {
            patients: Arc::new(Mutex::new(Ok(patients))),
            profiles: Arc::new(Mutex::new(Ok(profiles))),
        }
    }
}

#[common::async_trait_platform]
impl ListSpecialistPatientsRead for FakeSpecialistPatientsAndProfiles {
    async fn list_specialist_patients(
        &self,
        _access_token: &AccessToken,
    ) -> Result<Vec<SpecialistPatient>> {
        self.patients.lock().unwrap().clone()
    }
}

#[common::async_trait_platform]
impl GetProfilesByIdsRead for FakeSpecialistPatientsAndProfiles {
    async fn get_profiles_by_ids(
        &self,
        _ids: &[Id],
        _access_token: &AccessToken,
    ) -> Result<Vec<Profile>> {
        self.profiles.lock().unwrap().clone()
    }
}

#[derive(Clone)]
pub struct FakeListExerciseLibrary {
    pub exercises: Arc<Mutex<Result<Vec<Exercise>>>>,
}

impl FakeListExerciseLibrary {
    pub fn new_ok(exercises: Vec<Exercise>) -> Self {
        Self {
            exercises: Arc::new(Mutex::new(Ok(exercises))),
        }
    }
}

#[common::async_trait_platform]
impl ListExerciseLibraryRead for FakeListExerciseLibrary {
    async fn list_exercise_library(
        &self,
        _access_token: &AccessToken,
        _specialist_id: &Id,
        _name_filter: Option<&LibraryNameFilter>,
    ) -> Result<Vec<Exercise>> {
        self.exercises.lock().unwrap().clone()
    }
}

#[derive(Clone, Default)]
pub struct FakePatientProgressCatalog;

#[common::async_trait_platform]
impl GetProfilesByIdsRead for FakePatientProgressCatalog {
    async fn get_profiles_by_ids(
        &self,
        _ids: &[Id],
        _access_token: &AccessToken,
    ) -> Result<Vec<Profile>> {
        Ok(vec![])
    }
}

#[common::async_trait_platform]
impl ListPatientProgramsForSpecialistRead for FakePatientProgressCatalog {
    async fn list_patient_programs_for_specialist(
        &self,
        _access_token: &AccessToken,
    ) -> Result<Vec<PatientProgram>> {
        Ok(vec![])
    }
}

#[common::async_trait_platform]
impl GetPatientProgramFullRead for FakePatientProgressCatalog {
    async fn get_patient_program_full(
        &self,
        _access_token: &AccessToken,
        _patient_program_id: &Id,
    ) -> Result<Option<PatientProgramFull>> {
        Ok(None)
    }
}

#[derive(Clone)]
pub struct FakeSpecialistDashboardRead {
    pub dashboard: Arc<Mutex<Result<SpecialistDashboard>>>,
}

impl FakeSpecialistDashboardRead {
    pub fn new_ok(dashboard: SpecialistDashboard) -> Self {
        Self {
            dashboard: Arc::new(Mutex::new(Ok(dashboard))),
        }
    }
}

#[common::async_trait_platform]
impl GetSpecialistDashboardRead for FakeSpecialistDashboardRead {
    async fn get_specialist_dashboard(
        &self,
        _access_token: &AccessToken,
        _specialist_id: &Id,
    ) -> Result<SpecialistDashboard> {
        self.dashboard.lock().unwrap().clone()
    }
}

#[derive(Clone)]
pub struct FakeListWorkoutLibrary {
    pub workouts: Arc<Mutex<Result<Vec<Workout>>>>,
}

impl FakeListWorkoutLibrary {
    pub fn new_ok(workouts: Vec<Workout>) -> Self {
        Self {
            workouts: Arc::new(Mutex::new(Ok(workouts))),
        }
    }
}

#[common::async_trait_platform]
impl ListWorkoutLibraryRead for FakeListWorkoutLibrary {
    async fn list_workout_library(
        &self,
        _access_token: &AccessToken,
        _specialist_id: &Id,
        _name_filter: Option<&LibraryNameFilter>,
    ) -> Result<Vec<Workout>> {
        self.workouts.lock().unwrap().clone()
    }
}
