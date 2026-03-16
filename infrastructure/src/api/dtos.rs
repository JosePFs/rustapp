use serde::{Deserialize, Serialize};

use crate::domain::entities::{
    Exercise, PatientProgram, Program, ProgramScheduleItem, SessionExerciseFeedback,
    SpecialistPatient, Workout, WorkoutSession,
};
use crate::domain::{email::Email, fullname::FullName, id::Id, profile::Profile, role::Role};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileDto {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub role: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecialistPatientDto {
    pub id: String,
    pub specialist_id: String,
    pub patient_id: String,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramDto {
    pub id: String,
    pub specialist_id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkoutDto {
    pub id: String,
    pub specialist_id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramScheduleItemDto {
    pub id: String,
    pub program_id: String,
    pub order_index: i32,
    #[serde(default)]
    pub workout_id: Option<String>,
    pub days_count: i32,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExerciseDto {
    pub id: String,
    pub specialist_id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
    #[serde(default)]
    pub video_url: Option<String>,
    #[serde(default)]
    pub deleted_at: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientProgramDto {
    pub id: String,
    pub patient_id: String,
    pub program_id: String,
    pub status: String,
    #[serde(default)]
    pub assigned_at: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkoutSessionDto {
    pub id: String,
    pub patient_program_id: String,
    pub day_index: i32,
    pub session_date: String,
    pub completed_at: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionExerciseFeedbackDto {
    pub workout_session_id: String,
    pub exercise_id: String,
    pub effort: Option<i32>,
    pub pain: Option<i32>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutExerciseRow {
    pub order_index: i32,
    pub exercise_id: String,
    #[serde(default)]
    pub sets: i32,
    #[serde(default)]
    pub reps: i32,
    #[serde(default)]
    pub exercises: Option<ExerciseDto>,
}

impl From<ProfileDto> for Profile {
    fn from(dto: ProfileDto) -> Self {
        Profile::new(
            Id::new(dto.id),
            Email::new(dto.email),
            FullName::new(dto.full_name),
            Role::new(&dto.role),
        )
    }
}

impl From<SpecialistPatientDto> for SpecialistPatient {
    fn from(dto: SpecialistPatientDto) -> Self {
        SpecialistPatient {
            id: dto.id,
            specialist_id: dto.specialist_id,
            patient_id: dto.patient_id,
            created_at: dto.created_at,
        }
    }
}

impl From<ProgramDto> for Program {
    fn from(dto: ProgramDto) -> Self {
        Program {
            id: dto.id,
            specialist_id: dto.specialist_id,
            name: dto.name,
            description: dto.description,
        }
    }
}

impl From<WorkoutDto> for Workout {
    fn from(dto: WorkoutDto) -> Self {
        Workout {
            id: dto.id,
            specialist_id: dto.specialist_id,
            name: dto.name,
            description: dto.description,
            order_index: dto.order_index,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

impl From<ProgramScheduleItemDto> for ProgramScheduleItem {
    fn from(dto: ProgramScheduleItemDto) -> Self {
        ProgramScheduleItem {
            id: dto.id,
            program_id: dto.program_id,
            order_index: dto.order_index,
            workout_id: dto.workout_id,
            days_count: dto.days_count,
            created_at: dto.created_at,
        }
    }
}

impl From<ExerciseDto> for Exercise {
    fn from(dto: ExerciseDto) -> Self {
        Exercise {
            id: dto.id,
            specialist_id: dto.specialist_id,
            name: dto.name,
            description: dto.description,
            order_index: dto.order_index,
            video_url: dto.video_url,
            deleted_at: dto.deleted_at,
            created_at: dto.created_at,
        }
    }
}

impl From<PatientProgramDto> for PatientProgram {
    fn from(dto: PatientProgramDto) -> Self {
        PatientProgram {
            id: dto.id,
            patient_id: dto.patient_id,
            program_id: dto.program_id,
            status: dto.status,
        }
    }
}

impl From<WorkoutSessionDto> for WorkoutSession {
    fn from(dto: WorkoutSessionDto) -> Self {
        WorkoutSession {
            id: dto.id,
            patient_program_id: dto.patient_program_id,
            day_index: dto.day_index,
            session_date: dto.session_date,
            completed_at: dto.completed_at,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
        }
    }
}

impl From<SessionExerciseFeedbackDto> for SessionExerciseFeedback {
    fn from(dto: SessionExerciseFeedbackDto) -> Self {
        SessionExerciseFeedback {
            workout_session_id: dto.workout_session_id,
            exercise_id: dto.exercise_id,
            effort: dto.effort,
            pain: dto.pain,
            comment: dto.comment,
        }
    }
}
