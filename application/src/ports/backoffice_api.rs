use chrono::{DateTime, Utc};

use crate::error::Result;

#[derive(Clone, Debug)]
pub struct LoginArgs {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct LoginResult {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub user_id: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub user_profile_type: String,
}

impl LoginResult {
    pub fn is_login_as_specialist(&self) -> bool {
        self.user_profile_type == "specialist"
    }
}

pub struct AddSpecialistPatientArgs {
    pub patient_email: String,
}

pub struct AddSpecialistPatientResult {
    pub id: String,
    pub specialist_id: String,
    pub patient_id: String,
    pub created_at: Option<String>,
}

pub struct AddExerciseToWorkoutArgs {
    pub workout_id: String,
    pub exercise_id: String,
    pub order_index: i32,
    pub sets: i32,
    pub reps: i32,
}

pub struct AssignProgramToPatientArgs {
    pub patient_id: String,
    pub program_id: String,
}

pub struct AssignProgramToPatientResult {
    pub id: String,
    pub patient_id: String,
    pub program_id: String,
    pub status: String,
}

pub struct CreateExerciseArgs {
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
    pub video_url: Option<String>,
}

pub struct CreateExerciseResult {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
    pub video_url: Option<String>,
    pub deleted_at: Option<String>,
}

pub struct CreateProgramArgs {
    pub name: String,
    pub description: Option<String>,
}

pub struct CreateProgramResult {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

pub struct CreateProgramScheduleItemArgs {
    pub program_id: String,
    pub order_index: i32,
    pub workout_id: Option<String>,
    pub days_count: i32,
}

pub struct CreateProgramScheduleItemResult {
    pub id: String,
    pub program_id: String,
    pub order_index: i32,
    pub workout_id: Option<String>,
    pub days_count: i32,
}

pub struct CreateWorkoutArgs {
    pub name: String,
    pub description: Option<String>,
}

pub struct CreateWorkoutResult {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
}

pub struct DeleteProgramScheduleItemArgs {
    pub schedule_item_id: String,
}

pub struct DeleteWorkoutArgs {
    pub workout_id: String,
}

#[derive(Clone)]
pub struct GetSpecialistPatientsWithProfilesArgs {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpecialistPatientLink {
    pub link_id: String,
    pub patient_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatientProfileSummary {
    pub patient_id: String,
    pub full_name: String,
    pub email: String,
}

#[derive(Clone, Debug, Default)]
pub struct GetSpecialistPatientsWithProfilesResult {
    pub links: Vec<SpecialistPatientLink>,
    pub profiles: Vec<PatientProfileSummary>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramSummary {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatientProgramAssignment {
    pub id: String,
    pub patient_id: String,
    pub program_id: String,
    pub status: String,
}

#[derive(Clone, Debug, Default)]
pub struct SpecialistProgramsDataResult {
    pub links: Vec<SpecialistPatientLink>,
    pub profiles: Vec<PatientProfileSummary>,
    pub programs: Vec<ProgramSummary>,
    pub assignments: Vec<PatientProgramAssignment>,
}

pub struct ListExerciseLibraryArgs {
    pub name_filter: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExerciseLibraryItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
    pub video_url: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ListExerciseLibraryResult {
    pub items: Vec<ExerciseLibraryItem>,
}

pub struct ListProgramScheduleArgs {
    pub program_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramScheduleEntry {
    pub id: String,
    pub order_index: i32,
    pub workout_id: Option<String>,
    pub days_count: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkoutList {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Default)]
pub struct ListProgramScheduleResult {
    pub schedule: Vec<ProgramScheduleEntry>,
    pub workouts: Vec<WorkoutList>,
}

pub struct ListWorkoutLibraryArgs {
    pub name_filter: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkoutLibraryItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
}

#[derive(Clone, Debug, Default)]
pub struct ListWorkoutLibraryResult {
    pub items: Vec<WorkoutLibraryItem>,
}

#[derive(Clone)]
pub struct ListUnassignedPatientsArgs {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnassignedPatient {
    pub patient_id: String,
    pub email: String,
    pub full_name: String,
}

#[derive(Clone, Debug, Default)]
pub struct ListUnassignedPatientsResult {
    pub patients: Vec<UnassignedPatient>,
}

pub struct PatientProgressArgs {
    pub patient_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatientProgressProfile {
    pub full_name: String,
    pub email: String,
}

pub use crate::use_cases::agenda_schedule::{
    AgendaSessionFeedback, AgendaWorkoutSession, ProgramScheduleRow, WorkoutSummaryRow,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatientProgressProgramBlock {
    pub program_name: String,
    pub program_description: Option<String>,
    pub assignment_status: String,
    pub sessions: Vec<AgendaWorkoutSession>,
    pub program_feedback: Vec<AgendaSessionFeedback>,
    pub schedule: Vec<ProgramScheduleRow>,
    pub workouts: Vec<WorkoutSummaryRow>,
}

pub struct PatientProgressResult {
    pub profile: PatientProgressProfile,
    pub programs_with_sessions: Vec<PatientProgressProgramBlock>,
}

pub struct RemoveExerciseFromWorkoutArgs {
    pub workout_id: String,
    pub exercise_id: String,
}

pub struct RestoreExerciseArgs {
    pub exercise_id: String,
}

pub struct SoftDeleteExerciseArgs {
    pub exercise_id: String,
}

pub struct UpdateExerciseArgs {
    pub exercise_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub order_index: Option<i32>,
    pub video_url: Option<String>,
}

pub struct UpdateWorkoutArgs {
    pub workout_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

pub struct UpdateWorkoutExerciseArgs {
    pub workout_id: String,
    pub exercise_id: String,
    pub sets: i32,
    pub reps: i32,
    pub order_index: Option<i32>,
}

pub struct WorkoutEditorDataArgs {
    pub workout_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkoutEditorWorkout {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkoutEditorExerciseItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_index: i32,
    pub video_url: Option<String>,
    pub deleted_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkoutEditorLine {
    pub exercise: WorkoutEditorExerciseItem,
    pub order_index: i32,
    pub sets: i32,
    pub reps: i32,
}

#[derive(Clone, Debug, Default)]
pub struct WorkoutEditorDataResult {
    pub workout: Option<WorkoutEditorWorkout>,
    pub exercises: Vec<WorkoutEditorLine>,
    pub library: Vec<WorkoutEditorExerciseItem>,
}

#[common::async_trait_platform]
pub trait BackofficeApi {
    async fn login(&self, args: LoginArgs) -> Result<LoginResult>;

    async fn add_specialist_patient(
        &self,
        args: AddSpecialistPatientArgs,
    ) -> Result<AddSpecialistPatientResult>;

    async fn add_exercise_to_workout(&self, args: AddExerciseToWorkoutArgs) -> Result<()>;

    async fn assign_program_to_patient(
        &self,
        args: AssignProgramToPatientArgs,
    ) -> Result<AssignProgramToPatientResult>;

    async fn create_exercise(&self, args: CreateExerciseArgs) -> Result<CreateExerciseResult>;

    async fn create_program(&self, args: CreateProgramArgs) -> Result<CreateProgramResult>;

    async fn create_program_schedule_item(
        &self,
        args: CreateProgramScheduleItemArgs,
    ) -> Result<CreateProgramScheduleItemResult>;

    async fn create_workout(&self, args: CreateWorkoutArgs) -> Result<CreateWorkoutResult>;

    async fn delete_program_schedule_item(&self, args: DeleteProgramScheduleItemArgs)
        -> Result<()>;

    async fn delete_workout(&self, args: DeleteWorkoutArgs) -> Result<()>;

    async fn get_specialist_patients_with_profiles(
        &self,
        args: GetSpecialistPatientsWithProfilesArgs,
    ) -> Result<GetSpecialistPatientsWithProfilesResult>;

    async fn specialist_programs_data(&self) -> Result<SpecialistProgramsDataResult>;

    async fn list_exercise_library(
        &self,
        args: ListExerciseLibraryArgs,
    ) -> Result<ListExerciseLibraryResult>;

    async fn list_program_schedule(
        &self,
        args: ListProgramScheduleArgs,
    ) -> Result<ListProgramScheduleResult>;

    async fn list_workout_library(
        &self,
        args: ListWorkoutLibraryArgs,
    ) -> Result<ListWorkoutLibraryResult>;

    async fn list_unassigned_patients(
        &self,
        args: ListUnassignedPatientsArgs,
    ) -> Result<ListUnassignedPatientsResult>;

    async fn patient_progress(&self, args: PatientProgressArgs) -> Result<PatientProgressResult>;

    async fn remove_exercise_from_workout(&self, args: RemoveExerciseFromWorkoutArgs)
        -> Result<()>;

    async fn restore_exercise(&self, args: RestoreExerciseArgs) -> Result<()>;

    async fn soft_delete_exercise(&self, args: SoftDeleteExerciseArgs) -> Result<()>;

    async fn update_exercise(&self, args: UpdateExerciseArgs) -> Result<()>;

    async fn update_workout(&self, args: UpdateWorkoutArgs) -> Result<()>;

    async fn update_workout_exercise(&self, args: UpdateWorkoutExerciseArgs) -> Result<()>;

    async fn workout_editor_data(
        &self,
        args: WorkoutEditorDataArgs,
    ) -> Result<WorkoutEditorDataResult>;
}

use crate::use_cases::add_specialist_patient::AddSpecialistPatientArgs as UcAddSpecialistPatientArgs;
use crate::use_cases::add_exercise_to_workout::AddExerciseToWorkoutArgs as UcAddExerciseToWorkoutArgs;
use crate::use_cases::assign_program_to_patient::AssignProgramToPatientArgs as UcAssignProgramToPatientArgs;
use crate::use_cases::create_exercise::CreateExerciseArgs as UcCreateExerciseArgs;
use crate::use_cases::create_program::CreateProgramArgs as UcCreateProgramArgs;
use crate::use_cases::create_program_schedule_item::CreateProgramScheduleItemArgs as UcCreateProgramScheduleItemArgs;
use crate::use_cases::create_workout::CreateWorkoutArgs as UcCreateWorkoutArgs;
use crate::use_cases::delete_program_schedule_item::DeleteProgramScheduleItemArgs as UcDeleteProgramScheduleItemArgs;
use crate::use_cases::delete_workout::DeleteWorkoutArgs as UcDeleteWorkoutArgs;
use crate::use_cases::get_specialist_patients_with_profiles::{GetSpecialistPatientsWithProfilesArgs as UcGetSpecialistPatientsWithProfilesArgs, GetSpecialistPatientsWithProfilesResult as UcGetSpecialistPatientsWithProfilesResult};
use crate::use_cases::list_exercise_library::{ExerciseLibraryItem as UcExerciseLibraryItem, ListExerciseLibraryArgs as UcListExerciseLibraryArgs};
use crate::use_cases::list_program_schedule::{ListProgramScheduleArgs as UcListProgramScheduleArgs, ProgramScheduleData as UcProgramScheduleData};
use crate::use_cases::list_unassigned_patients::{UnassignedPatientsArgs as UcUnassignedPatientsArgs, UnassignedPatientsResult as UcUnassignedPatientsResult};
use crate::use_cases::list_workout_library::{ListWorkoutLibraryArgs as UcListWorkoutLibraryArgs, WorkoutLibraryItem as UcWorkoutLibraryItem};
use crate::use_cases::patient_progress::{PatientProgressArgs as UcPatientProgressArgs, PatientProgressResult as UcPatientProgressResult};
use crate::use_cases::remove_exercise_from_workout::RemoveExerciseFromWorkoutArgs as UcRemoveExerciseFromWorkoutArgs;
use crate::use_cases::restore_exercise::RestoreExerciseArgs as UcRestoreExerciseArgs;
use crate::use_cases::soft_delete_exercise::SoftDeleteExerciseArgs as UcSoftDeleteExerciseArgs;
use crate::use_cases::specialist_programs_data::SpecialistProgramsDataResult as UcSpecialistProgramsDataResult;
use crate::use_cases::update_exercise::UpdateExerciseArgs as UcUpdateExerciseArgs;
use crate::use_cases::update_workout::UpdateWorkoutArgs as UcUpdateWorkoutArgs;
use crate::use_cases::update_workout_exercise::UpdateWorkoutExerciseArgs as UcUpdateWorkoutExerciseArgs;
use crate::use_cases::workout_editor_data::{WorkoutEditorDataArgs as UcWorkoutEditorDataArgs, WorkoutEditorDataResult as UcWorkoutEditorDataResult};

impl From<AddSpecialistPatientArgs> for UcAddSpecialistPatientArgs {
    fn from(args: AddSpecialistPatientArgs) -> Self {
        UcAddSpecialistPatientArgs { patient_email: args.patient_email }
    }
}

impl From<AddExerciseToWorkoutArgs> for UcAddExerciseToWorkoutArgs {
    fn from(args: AddExerciseToWorkoutArgs) -> Self {
        UcAddExerciseToWorkoutArgs {
            workout_id: args.workout_id,
            exercise_id: args.exercise_id,
            order_index: args.order_index,
            sets: args.sets,
            reps: args.reps,
        }
    }
}

impl From<AssignProgramToPatientArgs> for UcAssignProgramToPatientArgs {
    fn from(args: AssignProgramToPatientArgs) -> Self {
        UcAssignProgramToPatientArgs {
            patient_id: args.patient_id,
            program_id: args.program_id,
        }
    }
}

impl From<CreateExerciseArgs> for UcCreateExerciseArgs {
    fn from(args: CreateExerciseArgs) -> Self {
        UcCreateExerciseArgs {
            name: args.name,
            description: args.description,
            order_index: args.order_index,
            video_url: args.video_url,
        }
    }
}

impl From<CreateProgramArgs> for UcCreateProgramArgs {
    fn from(args: CreateProgramArgs) -> Self {
        UcCreateProgramArgs {
            name: args.name,
            description: args.description,
        }
    }
}

impl From<CreateProgramScheduleItemArgs> for UcCreateProgramScheduleItemArgs {
    fn from(args: CreateProgramScheduleItemArgs) -> Self {
        UcCreateProgramScheduleItemArgs {
            program_id: args.program_id,
            order_index: args.order_index,
            workout_id: args.workout_id,
            days_count: args.days_count,
        }
    }
}

impl From<CreateWorkoutArgs> for UcCreateWorkoutArgs {
    fn from(args: CreateWorkoutArgs) -> Self {
        UcCreateWorkoutArgs {
            name: args.name,
            description: args.description,
        }
    }
}

impl From<DeleteProgramScheduleItemArgs> for UcDeleteProgramScheduleItemArgs {
    fn from(args: DeleteProgramScheduleItemArgs) -> Self {
        UcDeleteProgramScheduleItemArgs {
            schedule_item_id: args.schedule_item_id,
        }
    }
}

impl From<DeleteWorkoutArgs> for UcDeleteWorkoutArgs {
    fn from(args: DeleteWorkoutArgs) -> Self {
        UcDeleteWorkoutArgs {
            workout_id: args.workout_id,
        }
    }
}

impl From<GetSpecialistPatientsWithProfilesArgs> for UcGetSpecialistPatientsWithProfilesArgs {
    fn from(_args: GetSpecialistPatientsWithProfilesArgs) -> Self {
        UcGetSpecialistPatientsWithProfilesArgs {}
    }
}

impl From<UcGetSpecialistPatientsWithProfilesResult> for GetSpecialistPatientsWithProfilesResult {
    fn from(result: UcGetSpecialistPatientsWithProfilesResult) -> Self {
        GetSpecialistPatientsWithProfilesResult {
            links: result.links.into_iter().map(|l| SpecialistPatientLink {
                link_id: l.link_id,
                patient_id: l.patient_id,
            }).collect(),
            profiles: result.profiles.into_iter().map(|p| PatientProfileSummary {
                patient_id: p.patient_id,
                full_name: p.full_name,
                email: p.email,
            }).collect(),
        }
    }
}

impl From<ListExerciseLibraryArgs> for UcListExerciseLibraryArgs {
    fn from(args: ListExerciseLibraryArgs) -> Self {
        UcListExerciseLibraryArgs {
            name_filter: args.name_filter,
        }
    }
}

impl From<Vec<UcExerciseLibraryItem>> for ListExerciseLibraryResult {
    fn from(items: Vec<UcExerciseLibraryItem>) -> Self {
        ListExerciseLibraryResult { 
            items: items.into_iter().map(|i| ExerciseLibraryItem {
                id: i.id,
                name: i.name,
                description: i.description,
                order_index: i.order_index,
                video_url: i.video_url,
                deleted_at: i.deleted_at,
            }).collect()
        }
    }
}

impl From<ListProgramScheduleArgs> for UcListProgramScheduleArgs {
    fn from(args: ListProgramScheduleArgs) -> Self {
        UcListProgramScheduleArgs {
            program_id: args.program_id,
        }
    }
}

impl From<UcProgramScheduleData> for ListProgramScheduleResult {
    fn from(data: UcProgramScheduleData) -> Self {
        ListProgramScheduleResult {
            schedule: data.schedule.into_iter().map(|s| ProgramScheduleEntry {
                id: s.id,
                order_index: s.order_index,
                workout_id: s.workout_id,
                days_count: s.days_count,
            }).collect(),
            workouts: data.workouts.into_iter().map(|w| WorkoutList {
                id: w.id,
                name: w.name,
            }).collect(),
        }
    }
}

impl From<ListWorkoutLibraryArgs> for UcListWorkoutLibraryArgs {
    fn from(args: ListWorkoutLibraryArgs) -> Self {
        UcListWorkoutLibraryArgs {
            name_filter: args.name_filter,
        }
    }
}

impl From<Vec<UcWorkoutLibraryItem>> for ListWorkoutLibraryResult {
    fn from(items: Vec<UcWorkoutLibraryItem>) -> Self {
        ListWorkoutLibraryResult { 
            items: items.into_iter().map(|i| WorkoutLibraryItem {
                id: i.id,
                name: i.name,
                description: i.description,
                order_index: i.order_index,
            }).collect()
        }
    }
}

impl From<ListUnassignedPatientsArgs> for UcUnassignedPatientsArgs {
    fn from(_args: ListUnassignedPatientsArgs) -> Self {
        UcUnassignedPatientsArgs {}
    }
}

impl From<UcUnassignedPatientsResult> for ListUnassignedPatientsResult {
    fn from(result: UcUnassignedPatientsResult) -> Self {
        ListUnassignedPatientsResult {
            patients: result.patients.into_iter().map(|p| UnassignedPatient {
                patient_id: p.patient_id,
                email: p.email,
                full_name: p.full_name,
            }).collect(),
        }
    }
}

impl From<PatientProgressArgs> for UcPatientProgressArgs {
    fn from(args: PatientProgressArgs) -> Self {
        UcPatientProgressArgs {
            patient_id: args.patient_id,
        }
    }
}

impl From<UcPatientProgressResult> for PatientProgressResult {
    fn from(result: UcPatientProgressResult) -> Self {
        PatientProgressResult {
            profile: PatientProgressProfile {
                full_name: result.profile.full_name,
                email: result.profile.email,
            },
            programs_with_sessions: result.programs_with_sessions.into_iter().map(|p| PatientProgressProgramBlock {
                program_name: p.program_name,
                program_description: p.program_description,
                assignment_status: p.assignment_status,
                sessions: p.sessions,
                program_feedback: p.program_feedback,
                schedule: p.schedule,
                workouts: p.workouts,
            }).collect(),
        }
    }
}

impl From<RemoveExerciseFromWorkoutArgs> for UcRemoveExerciseFromWorkoutArgs {
    fn from(args: RemoveExerciseFromWorkoutArgs) -> Self {
        UcRemoveExerciseFromWorkoutArgs {
            workout_id: args.workout_id,
            exercise_id: args.exercise_id,
        }
    }
}

impl From<RestoreExerciseArgs> for UcRestoreExerciseArgs {
    fn from(args: RestoreExerciseArgs) -> Self {
        UcRestoreExerciseArgs {
            exercise_id: args.exercise_id,
        }
    }
}

impl From<SoftDeleteExerciseArgs> for UcSoftDeleteExerciseArgs {
    fn from(args: SoftDeleteExerciseArgs) -> Self {
        UcSoftDeleteExerciseArgs {
            exercise_id: args.exercise_id,
        }
    }
}

impl From<UpdateExerciseArgs> for UcUpdateExerciseArgs {
    fn from(args: UpdateExerciseArgs) -> Self {
        UcUpdateExerciseArgs {
            exercise_id: args.exercise_id,
            name: args.name,
            description: args.description,
            order_index: args.order_index,
            video_url: args.video_url,
        }
    }
}

impl From<UpdateWorkoutArgs> for UcUpdateWorkoutArgs {
    fn from(args: UpdateWorkoutArgs) -> Self {
        UcUpdateWorkoutArgs {
            workout_id: args.workout_id,
            name: args.name,
            description: args.description,
        }
    }
}

impl From<UpdateWorkoutExerciseArgs> for UcUpdateWorkoutExerciseArgs {
    fn from(args: UpdateWorkoutExerciseArgs) -> Self {
        UcUpdateWorkoutExerciseArgs {
            workout_id: args.workout_id,
            exercise_id: args.exercise_id,
            sets: args.sets,
            reps: args.reps,
            order_index: args.order_index,
        }
    }
}

impl From<WorkoutEditorDataArgs> for UcWorkoutEditorDataArgs {
    fn from(args: WorkoutEditorDataArgs) -> Self {
        UcWorkoutEditorDataArgs {
            workout_id: args.workout_id,
        }
    }
}

impl From<UcWorkoutEditorDataResult> for WorkoutEditorDataResult {
    fn from(result: UcWorkoutEditorDataResult) -> Self {
        WorkoutEditorDataResult {
            workout: result.workout.map(|w| WorkoutEditorWorkout {
                id: w.id,
                name: w.name,
                description: w.description,
                order_index: w.order_index,
            }),
            exercises: result.exercises.into_iter().map(|e| WorkoutEditorLine {
                exercise: WorkoutEditorExerciseItem {
                    id: e.exercise.id,
                    name: e.exercise.name,
                    description: e.exercise.description,
                    order_index: e.exercise.order_index,
                    video_url: e.exercise.video_url,
                    deleted_at: e.exercise.deleted_at,
                },
                order_index: e.order_index,
                sets: e.sets,
                reps: e.reps,
            }).collect(),
            library: result.library.into_iter().map(|l| WorkoutEditorExerciseItem {
                id: l.id,
                name: l.name,
                description: l.description,
                order_index: l.order_index,
                video_url: l.video_url,
                deleted_at: l.deleted_at,
            }).collect(),
        }
    }
}

impl From<UcSpecialistProgramsDataResult> for SpecialistProgramsDataResult {
    fn from(result: UcSpecialistProgramsDataResult) -> Self {
        SpecialistProgramsDataResult {
            links: result.links.into_iter().map(|l| SpecialistPatientLink {
                link_id: l.link_id,
                patient_id: l.patient_id,
            }).collect(),
            profiles: result.profiles.into_iter().map(|p| PatientProfileSummary {
                patient_id: p.patient_id,
                full_name: p.full_name,
                email: p.email,
            }).collect(),
            programs: result.programs.into_iter().map(|p| ProgramSummary {
                id: p.id,
                name: p.name,
                description: p.description,
            }).collect(),
            assignments: result.assignments.into_iter().map(|a| PatientProgramAssignment {
                id: a.id,
                patient_id: a.patient_id,
                program_id: a.program_id,
                status: a.status,
            }).collect(),
        }
    }
}
