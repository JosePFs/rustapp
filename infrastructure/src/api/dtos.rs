use serde::{Deserialize, Serialize};

use domain::aggregates::{
    PatientProgramFull, ProgramWithAgenda, SpecialistDashboard, WorkoutWithExercises,
};
use domain::entities::{
    Exercise, PatientProgram, Program, ProgramScheduleItem, SessionExerciseFeedback,
    SpecialistPatient, Workout, WorkoutExercise, WorkoutSession,
};
use domain::vos::{email::Email, fullname::FullName, id::Id, profile::Profile, role::Role};

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
            Id::try_from(dto.id).unwrap(),
            Email::try_from(dto.email).unwrap(),
            FullName::try_from(dto.full_name).unwrap(),
            Role::try_from(dto.role).unwrap(),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutExerciseRpcDto {
    pub order_index: i32,
    #[serde(default)]
    pub sets: i32,
    #[serde(default)]
    pub reps: i32,
    pub exercise: ExerciseDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutWithExercisesRpcDto {
    pub workout: WorkoutDto,
    #[serde(default)]
    pub exercises: Vec<WorkoutExerciseRpcDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramWithAgendaRpcDto {
    pub program: ProgramDto,
    #[serde(default)]
    pub schedule: Vec<ProgramScheduleItemDto>,
    #[serde(default)]
    pub workouts: Vec<WorkoutWithExercisesRpcDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientProgramFullRpcDto {
    pub patient_program: PatientProgramDto,
    pub program: ProgramDto,
    #[serde(default)]
    pub schedule: Vec<ProgramScheduleItemDto>,
    #[serde(default)]
    pub workouts: Vec<WorkoutWithExercisesRpcDto>,
    #[serde(default)]
    pub sessions: Vec<WorkoutSessionDto>,
    #[serde(default)]
    pub feedback: Vec<SessionExerciseFeedbackDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialistDashboardRpcDto {
    #[serde(default)]
    pub links: Vec<SpecialistPatientDto>,
    #[serde(default)]
    pub profiles: Vec<ProfileDto>,
    #[serde(default)]
    pub programs: Vec<ProgramDto>,
    #[serde(default)]
    pub assignments: Vec<PatientProgramDto>,
}

impl From<WorkoutExerciseRpcDto> for WorkoutExercise {
    fn from(dto: WorkoutExerciseRpcDto) -> Self {
        WorkoutExercise {
            exercise: dto.exercise.into(),
            order_index: dto.order_index,
            sets: dto.sets,
            reps: dto.reps,
        }
    }
}

impl From<WorkoutWithExercisesRpcDto> for WorkoutWithExercises {
    fn from(dto: WorkoutWithExercisesRpcDto) -> Self {
        WorkoutWithExercises {
            workout: dto.workout.into(),
            exercises: dto.exercises.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<ProgramWithAgendaRpcDto> for ProgramWithAgenda {
    fn from(dto: ProgramWithAgendaRpcDto) -> Self {
        ProgramWithAgenda {
            program: dto.program.into(),
            schedule: dto.schedule.into_iter().map(Into::into).collect(),
            workouts: dto.workouts.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<PatientProgramFullRpcDto> for PatientProgramFull {
    fn from(dto: PatientProgramFullRpcDto) -> Self {
        PatientProgramFull {
            patient_program: dto.patient_program.into(),
            program: dto.program.into(),
            schedule: dto.schedule.into_iter().map(Into::into).collect(),
            workouts: dto.workouts.into_iter().map(Into::into).collect(),
            sessions: dto.sessions.into_iter().map(Into::into).collect(),
            feedback: dto.feedback.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<SpecialistDashboardRpcDto> for SpecialistDashboard {
    fn from(dto: SpecialistDashboardRpcDto) -> Self {
        SpecialistDashboard {
            links: dto.links.into_iter().map(Into::into).collect(),
            profiles: dto.profiles.into_iter().map(Into::into).collect(),
            programs: dto.programs.into_iter().map(Into::into).collect(),
            assignments: dto.assignments.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_workout_dto() -> WorkoutDto {
        WorkoutDto {
            id: "w1".to_string(),
            specialist_id: "s1".to_string(),
            name: "Workout A".to_string(),
            description: Some("Description".to_string()),
            order_index: 0,
            created_at: None,
            updated_at: None,
        }
    }

    fn sample_exercise_dto() -> ExerciseDto {
        ExerciseDto {
            id: "e1".to_string(),
            specialist_id: "s1".to_string(),
            name: "Exercise A".to_string(),
            description: None,
            order_index: 0,
            video_url: None,
            deleted_at: None,
            created_at: None,
        }
    }

    fn sample_program_dto() -> ProgramDto {
        ProgramDto {
            id: "p1".to_string(),
            specialist_id: "s1".to_string(),
            name: "Program A".to_string(),
            description: None,
            created_at: None,
            updated_at: None,
        }
    }

    #[test]
    fn workout_with_exercises_rpc_dto_converts_correctly() {
        let dto = WorkoutWithExercisesRpcDto {
            workout: sample_workout_dto(),
            exercises: vec![WorkoutExerciseRpcDto {
                order_index: 0,
                sets: 3,
                reps: 10,
                exercise: sample_exercise_dto(),
            }],
        };

        let aggregate: WorkoutWithExercises = dto.into();
        assert_eq!(aggregate.workout.id, "w1");
        assert_eq!(aggregate.exercises.len(), 1);
        assert_eq!(aggregate.exercises[0].sets, 3);
        assert_eq!(aggregate.exercises[0].reps, 10);
    }

    #[test]
    fn program_with_agenda_rpc_dto_converts_correctly() {
        let dto = ProgramWithAgendaRpcDto {
            program: sample_program_dto(),
            schedule: vec![ProgramScheduleItemDto {
                id: "ps1".to_string(),
                program_id: "p1".to_string(),
                order_index: 0,
                workout_id: Some("w1".to_string()),
                days_count: 1,
                created_at: None,
            }],
            workouts: vec![WorkoutWithExercisesRpcDto {
                workout: sample_workout_dto(),
                exercises: vec![],
            }],
        };

        let aggregate: ProgramWithAgenda = dto.into();
        assert_eq!(aggregate.program.id, "p1");
        assert_eq!(aggregate.schedule.len(), 1);
        assert_eq!(aggregate.workouts.len(), 1);
    }

    #[test]
    fn specialist_dashboard_rpc_dto_converts_correctly() {
        let dto = SpecialistDashboardRpcDto {
            links: vec![SpecialistPatientDto {
                id: "sp1".to_string(),
                specialist_id: "s1".to_string(),
                patient_id: "pat1".to_string(),
                created_at: None,
            }],
            profiles: vec![ProfileDto {
                id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
                email: "patient@test.com".to_string(),
                full_name: "Test Patient".to_string(),
                role: "patient".to_string(),
                created_at: None,
                updated_at: None,
            }],
            programs: vec![sample_program_dto()],
            assignments: vec![PatientProgramDto {
                id: "pp1".to_string(),
                patient_id: "pat1".to_string(),
                program_id: "p1".to_string(),
                status: "active".to_string(),
                assigned_at: None,
                created_at: None,
                updated_at: None,
            }],
        };

        let aggregate: SpecialistDashboard = dto.into();
        assert_eq!(aggregate.links.len(), 1);
        assert_eq!(aggregate.profiles.len(), 1);
        assert_eq!(aggregate.programs.len(), 1);
        assert_eq!(aggregate.assignments.len(), 1);
    }

    #[test]
    fn patient_program_full_rpc_dto_converts_correctly() {
        let dto = PatientProgramFullRpcDto {
            patient_program: PatientProgramDto {
                id: "pp1".to_string(),
                patient_id: "pat1".to_string(),
                program_id: "p1".to_string(),
                status: "active".to_string(),
                assigned_at: None,
                created_at: None,
                updated_at: None,
            },
            program: sample_program_dto(),
            schedule: vec![],
            workouts: vec![],
            sessions: vec![WorkoutSessionDto {
                id: "ws1".to_string(),
                patient_program_id: "pp1".to_string(),
                day_index: 0,
                session_date: "2024-01-01".to_string(),
                completed_at: None,
                created_at: None,
                updated_at: None,
            }],
            feedback: vec![SessionExerciseFeedbackDto {
                workout_session_id: "ws1".to_string(),
                exercise_id: "e1".to_string(),
                effort: Some(7),
                pain: Some(2),
                comment: Some("Good session".to_string()),
            }],
        };

        let aggregate: PatientProgramFull = dto.into();
        assert_eq!(aggregate.patient_program.id, "pp1");
        assert_eq!(aggregate.program.id, "p1");
        assert_eq!(aggregate.sessions.len(), 1);
        assert_eq!(aggregate.feedback.len(), 1);
        assert_eq!(aggregate.feedback[0].effort, Some(7));
    }
}
