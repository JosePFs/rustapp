use crate::entities::{
    PatientProgram, Program, ProgramScheduleItem, SessionExerciseFeedback, SpecialistPatient,
    Workout, WorkoutExercise, WorkoutSession,
};
use crate::vos::profile::Profile;

#[derive(Debug, Clone)]
pub struct ProgramAgenda {
    pub program: Program,
    pub schedule: Vec<ProgramScheduleItem>,
}

#[derive(Debug, Clone)]
pub struct WorkoutWithExercises {
    pub workout: Workout,
    pub exercises: Vec<WorkoutExercise>,
}

#[derive(Debug, Clone)]
pub struct PatientProgramSessionProgress {
    pub patient_program_id: String,
    pub sessions: Vec<WorkoutSession>,
    pub session_exercise_feedback: Vec<SessionExerciseFeedback>,
}

#[derive(Debug, Clone)]
pub struct ProgramWithAgenda {
    pub program: Program,
    pub schedule: Vec<ProgramScheduleItem>,
    pub workouts: Vec<WorkoutWithExercises>,
}

#[derive(Debug, Clone)]
pub struct PatientProgramFull {
    pub patient_program: PatientProgram,
    pub program: Program,
    pub schedule: Vec<ProgramScheduleItem>,
    pub workouts: Vec<WorkoutWithExercises>,
    pub sessions: Vec<WorkoutSession>,
    pub feedback: Vec<SessionExerciseFeedback>,
}

#[derive(Debug, Clone)]
pub struct SpecialistDashboard {
    pub links: Vec<SpecialistPatient>,
    pub profiles: Vec<Profile>,
    pub programs: Vec<Program>,
    pub assignments: Vec<PatientProgram>,
}
