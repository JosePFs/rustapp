use std::sync::{Arc, Mutex};

use domain::entities::{PatientProgram, Program, Workout};
use domain::error::Result;
use domain::repositories::{
    AddExerciseToWorkoutWrite, AssignProgramToPatientWrite,
    CreateProgramWrite, CreateWorkoutWrite, DeleteProgramScheduleItemWrite, DeleteWorkoutWrite,
    RemoveExerciseFromWorkoutWrite, RestoreExerciseWrite, SoftDeleteExerciseWrite,
};
use domain::vos::id::Id;
use domain::vos::{
    AccessToken, Description, ProgramName, Reps, ScheduleOrderIndex, Sets, WorkoutName,
};

#[derive(Clone)]
pub struct FakeDeleteWorkout {
    pub last_workout_id: Arc<Mutex<Option<Id>>>,
    pub outcome: Arc<Mutex<Result<()>>>,
}

impl FakeDeleteWorkout {
    pub fn new_ok() -> Self {
        Self {
            last_workout_id: Arc::new(Mutex::new(None)),
            outcome: Arc::new(Mutex::new(Ok(()))),
        }
    }

    pub fn new_err(e: domain::error::DomainError) -> Self {
        Self {
            last_workout_id: Arc::new(Mutex::new(None)),
            outcome: Arc::new(Mutex::new(Err(e))),
        }
    }
}

#[common::async_trait_platform]
impl DeleteWorkoutWrite for FakeDeleteWorkout {
    async fn delete_workout(&self, _access_token: &AccessToken, workout_id: &Id) -> Result<()> {
        *self.last_workout_id.lock().unwrap() = Some(workout_id.clone());
        self.outcome.lock().unwrap().clone()
    }
}

#[derive(Clone)]
pub struct FakeDeleteProgramScheduleItem {
    pub last_schedule_id: Arc<Mutex<Option<Id>>>,
    pub outcome: Arc<Mutex<Result<()>>>,
}

impl FakeDeleteProgramScheduleItem {
    pub fn new_ok() -> Self {
        Self {
            last_schedule_id: Arc::new(Mutex::new(None)),
            outcome: Arc::new(Mutex::new(Ok(()))),
        }
    }
}

#[common::async_trait_platform]
impl DeleteProgramScheduleItemWrite for FakeDeleteProgramScheduleItem {
    async fn delete_program_schedule_item(
        &self,
        _access_token: &AccessToken,
        schedule_id: &Id,
    ) -> Result<()> {
        *self.last_schedule_id.lock().unwrap() = Some(schedule_id.clone());
        self.outcome.lock().unwrap().clone()
    }
}

#[derive(Clone)]
pub struct FakeCreateProgram {
    pub last_name: Arc<Mutex<Option<String>>>,
    pub outcome: Arc<Mutex<Result<Program>>>,
}

impl FakeCreateProgram {
    pub fn new_ok(program: Program) -> Self {
        Self {
            last_name: Arc::new(Mutex::new(None)),
            outcome: Arc::new(Mutex::new(Ok(program))),
        }
    }
}

#[derive(Clone)]
pub struct FakeCreateWorkout {
    pub last_name: Arc<Mutex<Option<String>>>,
    pub outcome: Arc<Mutex<Result<Workout>>>,
}

impl FakeCreateWorkout {
    pub fn new_ok(workout: Workout) -> Self {
        Self {
            last_name: Arc::new(Mutex::new(None)),
            outcome: Arc::new(Mutex::new(Ok(workout))),
        }
    }
}

#[common::async_trait_platform]
impl CreateWorkoutWrite for FakeCreateWorkout {
    async fn create_workout(
        &self,
        _access_token: &AccessToken,
        _specialist_id: &Id,
        name: &WorkoutName,
        _description: Option<&Description>,
    ) -> Result<Workout> {
        *self.last_name.lock().unwrap() = Some(name.value().to_string());
        self.outcome.lock().unwrap().clone()
    }
}

#[common::async_trait_platform]
impl CreateProgramWrite for FakeCreateProgram {
    async fn create_program(
        &self,
        _access_token: &AccessToken,
        _specialist_id: &Id,
        name: &ProgramName,
        _description: Option<&Description>,
    ) -> Result<Program> {
        *self.last_name.lock().unwrap() = Some(name.value().to_string());
        self.outcome.lock().unwrap().clone()
    }
}

#[derive(Clone)]
pub struct FakeSoftDeleteExercise {
    pub last_exercise_id: Arc<Mutex<Option<Id>>>,
    pub outcome: Arc<Mutex<Result<()>>>,
}

impl FakeSoftDeleteExercise {
    pub fn new_ok() -> Self {
        Self {
            last_exercise_id: Arc::new(Mutex::new(None)),
            outcome: Arc::new(Mutex::new(Ok(()))),
        }
    }
}

#[common::async_trait_platform]
impl SoftDeleteExerciseWrite for FakeSoftDeleteExercise {
    async fn soft_delete_exercise(
        &self,
        _access_token: &AccessToken,
        exercise_id: &Id,
    ) -> Result<()> {
        *self.last_exercise_id.lock().unwrap() = Some(exercise_id.clone());
        self.outcome.lock().unwrap().clone()
    }
}

#[derive(Clone)]
pub struct FakeAssignProgramToPatient {
    pub last_pair: Arc<Mutex<Option<(Id, Id)>>>,
    pub outcome: Arc<Mutex<Result<PatientProgram>>>,
}

impl FakeAssignProgramToPatient {
    pub fn new_ok(pp: PatientProgram) -> Self {
        Self {
            last_pair: Arc::new(Mutex::new(None)),
            outcome: Arc::new(Mutex::new(Ok(pp))),
        }
    }
}

#[common::async_trait_platform]
impl AssignProgramToPatientWrite for FakeAssignProgramToPatient {
    async fn assign_program_to_patient(
        &self,
        _access_token: &AccessToken,
        patient_id: &Id,
        program_id: &Id,
    ) -> Result<PatientProgram> {
        *self.last_pair.lock().unwrap() = Some((patient_id.clone(), program_id.clone()));
        self.outcome.lock().unwrap().clone()
    }
}

#[derive(Clone)]
pub struct FakeRestoreExercise {
    pub last_exercise_id: Arc<Mutex<Option<Id>>>,
    pub outcome: Arc<Mutex<Result<()>>>,
}

impl FakeRestoreExercise {
    pub fn new_ok() -> Self {
        Self {
            last_exercise_id: Arc::new(Mutex::new(None)),
            outcome: Arc::new(Mutex::new(Ok(()))),
        }
    }
}

#[common::async_trait_platform]
impl RestoreExerciseWrite for FakeRestoreExercise {
    async fn restore_exercise(&self, _access_token: &AccessToken, exercise_id: &Id) -> Result<()> {
        *self.last_exercise_id.lock().unwrap() = Some(exercise_id.clone());
        self.outcome.lock().unwrap().clone()
    }
}

#[derive(Clone)]
pub struct FakeRemoveExerciseFromWorkout {
    pub last_pair: Arc<Mutex<Option<(Id, Id)>>>,
    pub outcome: Arc<Mutex<Result<()>>>,
}

impl FakeRemoveExerciseFromWorkout {
    pub fn new_ok() -> Self {
        Self {
            last_pair: Arc::new(Mutex::new(None)),
            outcome: Arc::new(Mutex::new(Ok(()))),
        }
    }
}

#[common::async_trait_platform]
impl RemoveExerciseFromWorkoutWrite for FakeRemoveExerciseFromWorkout {
    async fn remove_exercise_from_workout(
        &self,
        _access_token: &AccessToken,
        workout_id: &Id,
        exercise_id: &Id,
    ) -> Result<()> {
        *self.last_pair.lock().unwrap() = Some((workout_id.clone(), exercise_id.clone()));
        self.outcome.lock().unwrap().clone()
    }
}

#[derive(Clone)]
pub struct FakeAddExerciseToWorkout {
    pub last_key: Arc<Mutex<Option<(Id, Id)>>>,
    pub outcome: Arc<Mutex<Result<()>>>,
}

impl FakeAddExerciseToWorkout {
    pub fn new_ok() -> Self {
        Self {
            last_key: Arc::new(Mutex::new(None)),
            outcome: Arc::new(Mutex::new(Ok(()))),
        }
    }
}

#[common::async_trait_platform]
impl AddExerciseToWorkoutWrite for FakeAddExerciseToWorkout {
    async fn add_exercise_to_workout(
        &self,
        _access_token: &AccessToken,
        workout_id: &Id,
        exercise_id: &Id,
        _order_index: ScheduleOrderIndex,
        _sets: Sets,
        _reps: Reps,
    ) -> Result<()> {
        *self.last_key.lock().unwrap() = Some((workout_id.clone(), exercise_id.clone()));
        self.outcome.lock().unwrap().clone()
    }
}
