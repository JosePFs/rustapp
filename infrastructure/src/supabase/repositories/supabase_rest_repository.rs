use std::sync::Arc;

use application::ports::HttpRestClient;
use serde::de::DeserializeOwned;

use crate::api::dtos::{
    ExerciseDto, PatientProgramDto, PatientProgramFullRpcDto, ProfileDto, ProgramDto,
    ProgramScheduleItemDto, ProgramWithAgendaRpcDto, SessionExerciseFeedbackDto,
    SpecialistDashboardRpcDto, SpecialistPatientDto, WorkoutDto, WorkoutExerciseRow,
    WorkoutSessionDto, WorkoutWithExercisesRpcDto,
};
use crate::supabase::{client::SupabaseClient, DEFAULT_CLIENT};
use domain::aggregates::{
    PatientProgramFull, ProgramWithAgenda, SpecialistDashboard, WorkoutWithExercises,
};
use domain::entities::SessionExerciseFeedback;
use domain::repositories::*;
use domain::{
    entities::{
        Exercise, PatientProgram, Program, ProgramScheduleItem, SpecialistPatient, Workout,
        WorkoutExercise, WorkoutSession,
    },
    error::DomainError,
    error::Result,
    vos::email::Email,
    vos::id::Id,
    vos::library_name_filter::LibraryNameFilter,
    vos::profile::Profile,
    vos::{
        DayIndex, DaysInBlock, Description, EffortScore, ExerciseName, FeedbackComment, PainScore,
        Patch, ProgramName, Reps, ScheduleOrderIndex, SessionDate, Sets, VideoUrl, WorkoutName,
    },
};

#[derive(Clone)]
pub struct SupabaseRestRepository {
    client: Arc<SupabaseClient>,
}

impl SupabaseRestRepository {
    pub fn new(client: Arc<SupabaseClient>) -> Self {
        Self { client }
    }

    pub fn client(&self) -> &SupabaseClient {
        &self.client
    }
}

#[common::async_trait_platform]
impl GetProfilesByIdsRead for SupabaseRestRepository {
    async fn get_profiles_by_ids(&self, ids: &[Id]) -> Result<Vec<Profile>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let ids_str: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
        let filter = format!("id=in.({})", ids_str.join(","));
        let path = format!(
            "/profiles?select=id,email,full_name,role,created_at,updated_at&{}",
            filter
        );
        let body = self.client.get(&path).await?;
        let rows: Vec<ProfileDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[common::async_trait_platform]
impl GetPatientIdByEmailRead for SupabaseRestRepository {
    async fn get_patient_id_by_email(&self, email: &Email) -> Result<Option<Id>> {
        let path = "/rpc/get_patient_id_by_email";
        let body = serde_json::json!({ "p_email": email.value() });
        let response = self.client.post(path, &body.to_string()).await?;
        let id: Option<String> = serde_json::from_slice(&response).map_err(|e| e.to_string())?;
        match id {
            Some(s) => Ok(Some(Id::try_from(s)?)),
            None => Ok(None),
        }
    }
}

#[common::async_trait_platform]
impl ListSpecialistPatientsRead for SupabaseRestRepository {
    async fn list_specialist_patients(&self) -> Result<Vec<SpecialistPatient>> {
        let body = self
            .client
            .get("/specialist_patients?select=id,specialist_id,patient_id,created_at")
            .await?;
        let rows: Vec<SpecialistPatientDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
}

#[common::async_trait_platform]
impl ListProgramsRead for SupabaseRestRepository {
    async fn list_programs(&self) -> Result<Vec<Program>> {
        let body = self
            .client
            .get("/programs?select=id,specialist_id,name,description,created_at,updated_at&order=created_at.desc")
            .await?;
        let rows: Vec<ProgramDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
}

#[common::async_trait_platform]
impl GetProgramRead for SupabaseRestRepository {
    async fn get_program(&self, program_id: &Id) -> Result<Option<Program>> {
        let path = format!(
            "/programs?id=eq.{}&select=id,specialist_id,name,description,created_at,updated_at",
            program_id.to_string()
        );
        let body = self.client.get(&path).await?;
        let rows: Vec<ProgramDto> = parse_json(&body)?;
        Ok(rows.into_iter().next().map(Into::into))
    }
}

#[common::async_trait_platform]
impl ListWorkoutLibraryRead for SupabaseRestRepository {
    async fn list_workout_library(
        &self,
        specialist_id: &Id,
        name_filter: Option<&LibraryNameFilter>,
    ) -> Result<Vec<Workout>> {
        let path = format!(
            "/workouts?specialist_id=eq.{}&select=id,specialist_id,name,description,order_index,created_at,updated_at&order=order_index.asc,name.asc",
            specialist_id.to_string()
        );
        let body = self.client.get(&path).await?;
        let rows: Vec<WorkoutDto> = parse_json(&body)?;
        let filtered: Vec<WorkoutDto> = if let Some(f) = name_filter {
            let needle = f.value().to_lowercase();
            rows.into_iter()
                .filter(|w| w.name.to_lowercase().contains(&needle))
                .collect()
        } else {
            rows
        };
        Ok(filtered.into_iter().map(Into::into).collect())
    }
}

#[common::async_trait_platform]
impl GetWorkoutsByIdsRead for SupabaseRestRepository {
    async fn get_workouts_by_ids(&self, ids: &[Id]) -> Result<Vec<Workout>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let ids_str: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
        let ids_param = ids_str.join(",");
        let path = format!(
            "/workouts?id=in.({})&select=id,specialist_id,name,description,order_index,created_at,updated_at",
            ids_param
        );
        let body = self.client.get(&path).await?;
        let rows: Vec<WorkoutDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
}

#[common::async_trait_platform]
impl ListWorkoutsForProgramRead for SupabaseRestRepository {
    async fn list_workouts_for_program(&self, program_id: &Id) -> Result<Vec<Workout>> {
        let schedule = ListProgramScheduleRead::list_program_schedule(self, program_id).await?;
        let ids: Vec<Id> = schedule
            .iter()
            .filter_map(|s| s.workout_id.clone())
            .collect();
        GetWorkoutsByIdsRead::get_workouts_by_ids(self, &ids).await
    }
}

#[common::async_trait_platform]
impl ListProgramScheduleRead for SupabaseRestRepository {
    async fn list_program_schedule(&self, program_id: &Id) -> Result<Vec<ProgramScheduleItem>> {
        let path = format!(
            "/program_schedule?program_id=eq.{}&select=id,program_id,order_index,workout_id,days_count,created_at&order=order_index.asc",
            program_id.to_string()
        );
        let body = self.client.get(&path).await?;
        let rows: Vec<ProgramScheduleItemDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
}

#[common::async_trait_platform]
impl ListExercisesForWorkoutRead for SupabaseRestRepository {
    async fn list_exercises_for_workout(&self, workout_id: &Id) -> Result<Vec<WorkoutExercise>> {
        let path = format!(
            "/workout_exercises?workout_id=eq.{}&select=order_index,exercise_id,sets,reps,exercises(id,specialist_id,name,description,order_index,video_url,deleted_at,created_at)&order=order_index.asc",
            workout_id.to_string()
        );
        let body = self.client.get(&path).await?;
        let rows: Vec<WorkoutExerciseRow> = parse_json(&body)?;
        Ok(rows
            .into_iter()
            .filter_map(|r| {
                r.exercises.map(|e| WorkoutExercise {
                    exercise: e.into(),
                    order_index: ScheduleOrderIndex::try_from(r.order_index).unwrap(),
                    sets: Sets::try_from(r.sets).unwrap(),
                    reps: Reps::try_from(r.reps).unwrap(),
                })
            })
            .collect())
    }
}

#[common::async_trait_platform]
impl ListExerciseLibraryRead for SupabaseRestRepository {
    async fn list_exercise_library(
        &self,
        specialist_id: &Id,
        name_filter: Option<&LibraryNameFilter>,
    ) -> Result<Vec<Exercise>> {
        let path = format!(
            "/exercises?specialist_id=eq.{}&select=id,specialist_id,name,description,order_index,video_url,deleted_at,created_at&order=name.asc",
            specialist_id.to_string()
        );
        let body = self.client.get(&path).await?;
        let rows: Vec<ExerciseDto> = parse_json(&body)?;
        let filtered: Vec<ExerciseDto> = if let Some(f) = name_filter {
            let needle = f.value().to_lowercase();
            rows.into_iter()
                .filter(|e| e.name.to_lowercase().contains(&needle))
                .collect()
        } else {
            rows
        };
        Ok(filtered.into_iter().map(Into::into).collect())
    }
}

#[common::async_trait_platform]
impl ListPatientProgramsForSpecialistRead for SupabaseRestRepository {
    async fn list_patient_programs_for_specialist(&self) -> Result<Vec<PatientProgram>> {
        let body = self.client.get("/patient_programs?select=id,patient_id,program_id,status,assigned_at,created_at,updated_at&order=assigned_at.desc").await?;
        let rows: Vec<PatientProgramDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
}

#[common::async_trait_platform]
impl GetPatientProgramByIdRead for SupabaseRestRepository {
    async fn get_patient_program_by_id(&self, id: &Id) -> Result<Option<PatientProgram>> {
        let path = format!(
            "/patient_programs?id=eq.{}&select=id,patient_id,program_id,status,assigned_at,created_at,updated_at&limit=1",
            id.to_string()
        );
        let body = self.client.get(&path).await?;
        let rows: Vec<PatientProgramDto> = parse_json(&body)?;
        Ok(rows.into_iter().next().map(Into::into))
    }
}

#[common::async_trait_platform]
impl ListWorkoutSessionsRead for SupabaseRestRepository {
    async fn list_workout_sessions(&self, patient_program_id: &Id) -> Result<Vec<WorkoutSession>> {
        let path = format!(
            "/workout_sessions?patient_program_id=eq.{}&select=id,patient_program_id,day_index,session_date,completed_at,created_at,updated_at&order=day_index.asc",
            patient_program_id.to_string()
        );
        let body = self.client.get(&path).await?;
        let rows: Vec<WorkoutSessionDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
}

#[common::async_trait_platform]
impl ListSessionExerciseFeedbackRead for SupabaseRestRepository {
    async fn list_session_exercise_feedback(
        &self,
        workout_session_id: &Id,
    ) -> Result<Vec<SessionExerciseFeedback>> {
        let path = format!(
            "/session_exercise_feedback?workout_session_id=eq.{}&select=workout_session_id,exercise_id,effort,pain,comment",
            workout_session_id.to_string()
        );
        let body = self.client.get(&path).await?;
        let rows: Vec<SessionExerciseFeedbackDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
}

#[common::async_trait_platform]
impl ListSessionExerciseFeedbackForProgramRead for SupabaseRestRepository {
    async fn list_session_exercise_feedback_for_program(
        &self,
        patient_program_id: &Id,
    ) -> Result<Vec<SessionExerciseFeedback>> {
        let path = "/rpc/list_session_exercise_feedback_for_patient_program";
        let body = serde_json::json!({ "p_patient_program_id": patient_program_id.to_string() });
        let response = self.client.post(path, &body.to_string()).await?;
        let rows: Vec<SessionExerciseFeedbackDto> = parse_json(&response)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
}

#[common::async_trait_platform]
impl ListActivePatientProgramsRead for SupabaseRestRepository {
    async fn list_active_patient_programs(&self) -> Result<Vec<PatientProgram>> {
        let path = "/patient_programs?status=eq.active&select=id,patient_id,program_id,status,assigned_at,created_at,updated_at&order=assigned_at.desc";
        let body = self.client.get(&path).await?;
        let rows: Vec<PatientProgramDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
}

#[common::async_trait_platform]
impl GetWorkoutWithExercisesRead for SupabaseRestRepository {
    async fn get_workout_with_exercises(
        &self,
        workout_id: &Id,
    ) -> Result<Option<WorkoutWithExercises>> {
        let path = "/rpc/get_workout_with_exercises";
        let body = serde_json::json!({ "p_workout_id": workout_id.to_string() });
        let response = self.client.post(path, &body.to_string()).await?;
        let dto: Option<WorkoutWithExercisesRpcDto> = parse_json(&response)?;
        Ok(dto.map(Into::into))
    }
}

#[common::async_trait_platform]
impl GetProgramWithAgendaRead for SupabaseRestRepository {
    async fn get_program_with_agenda(&self, program_id: &Id) -> Result<Option<ProgramWithAgenda>> {
        let path = "/rpc/get_program_with_agenda";
        let body = serde_json::json!({ "p_program_id": program_id.to_string() });
        let response = self.client.post(path, &body.to_string()).await?;
        let dto: Option<ProgramWithAgendaRpcDto> = parse_json(&response)?;
        Ok(dto.map(Into::into))
    }
}

#[common::async_trait_platform]
impl GetPatientProgramFullRead for SupabaseRestRepository {
    async fn get_patient_program_full(
        &self,
        patient_program_id: &Id,
    ) -> Result<Option<PatientProgramFull>> {
        let path = "/rpc/get_patient_program_full";
        let body = serde_json::json!({ "p_patient_program_id": patient_program_id.to_string() });
        let response = self.client.post(path, &body.to_string()).await?;
        let dto: Option<PatientProgramFullRpcDto> = parse_json(&response)?;
        Ok(dto.map(Into::into))
    }
}

#[common::async_trait_platform]
impl GetSpecialistDashboardRead for SupabaseRestRepository {
    async fn get_specialist_dashboard(&self, specialist_id: &Id) -> Result<SpecialistDashboard> {
        let path = "/rpc/get_specialist_dashboard";
        let body = serde_json::json!({ "p_specialist_id": specialist_id.to_string() });
        let response = self.client.post(path, &body.to_string()).await?;
        let dto: SpecialistDashboardRpcDto = parse_json(&response)?;
        Ok(dto.into())
    }
}

#[common::async_trait_platform]
impl AddSpecialistPatientWrite for SupabaseRestRepository {
    async fn add_specialist_patient(
        &self,
        specialist_id: &Id,
        patient_id: &Id,
    ) -> Result<SpecialistPatient> {
        let payload = serde_json::json!({
            "specialist_id": specialist_id.to_string(),
            "patient_id": patient_id.to_string()
        });
        let body = self
            .client
            .post("/specialist_patients", &payload.to_string())
            .await?;
        let rows: Vec<SpecialistPatientDto> = parse_json(&body)?;
        rows.into_iter()
            .next()
            .map(Into::into)
            .ok_or_else(|| "No row returned".to_string())
            .map_err(DomainError::from)
    }
}

#[common::async_trait_platform]
impl CreateProgramWrite for SupabaseRestRepository {
    async fn create_program(
        &self,
        specialist_id: &Id,
        name: &ProgramName,
        description: Option<&Description>,
    ) -> Result<Program> {
        let payload = serde_json::json!({
            "specialist_id": specialist_id.to_string(),
            "name": name.value(),
            "description": description.map(|d| d.value())
        });
        let body = self.client.post("/programs", &payload.to_string()).await?;
        let rows: Vec<ProgramDto> = parse_json(&body)?;
        rows.into_iter()
            .next()
            .map(Into::into)
            .ok_or_else(|| "No row returned".to_string())
            .map_err(DomainError::from)
    }
}

#[common::async_trait_platform]
impl CreateWorkoutWrite for SupabaseRestRepository {
    async fn create_workout(
        &self,
        specialist_id: &Id,
        name: &WorkoutName,
        description: Option<&Description>,
    ) -> Result<Workout> {
        let payload = serde_json::json!({
            "specialist_id": specialist_id.to_string(),
            "name": name.value(),
            "description": description.map(|d| d.value()),
            "order_index": 0
        });
        let body = self.client.post("/workouts", &payload.to_string()).await?;
        let rows: Vec<WorkoutDto> = parse_json(&body)?;
        rows.into_iter()
            .next()
            .map(Into::into)
            .ok_or_else(|| "No row returned".to_string())
            .map_err(DomainError::from)
    }
}

#[common::async_trait_platform]
impl UpdateWorkoutWrite for SupabaseRestRepository {
    async fn update_workout(
        &self,
        workout_id: &Id,
        name: Option<&WorkoutName>,
        description: Patch<Description>,
        order_index: Option<ScheduleOrderIndex>,
    ) -> Result<()> {
        let mut payload = serde_json::json!({});
        if let Some(n) = name {
            payload["name"] = serde_json::Value::String(n.value().to_string());
        }
        match description {
            Patch::Omit => {}
            Patch::Clear => {
                payload["description"] = serde_json::Value::Null;
            }
            Patch::Set(d) => {
                payload["description"] = serde_json::Value::String(d.value().to_string());
            }
        }
        if let Some(o) = order_index {
            payload["order_index"] = serde_json::Number::from(o.value()).into();
        }
        let path = format!("/workouts?id=eq.{}", workout_id.to_string());
        self.client.patch(&path, &payload.to_string()).await?;
        Ok(())
    }
}

#[common::async_trait_platform]
impl DeleteWorkoutWrite for SupabaseRestRepository {
    async fn delete_workout(&self, workout_id: &Id) -> Result<()> {
        let path = format!("/workouts?id=eq.{}", workout_id.to_string());
        self.client.delete(&path).await?;
        Ok(())
    }
}

#[common::async_trait_platform]
impl CreateProgramScheduleItemWrite for SupabaseRestRepository {
    async fn create_program_schedule_item(
        &self,
        program_id: &Id,
        order_index: ScheduleOrderIndex,
        workout_id: Option<&Id>,
        days_count: DaysInBlock,
    ) -> Result<ProgramScheduleItem> {
        let mut payload = serde_json::json!({
            "program_id": program_id.to_string(),
            "order_index": order_index.value(),
            "days_count": days_count.value()
        });
        if let Some(wid) = workout_id {
            payload["workout_id"] = serde_json::Value::String(wid.to_string());
        } else {
            payload["workout_id"] = serde_json::Value::Null;
        }
        let body = self
            .client
            .post("/program_schedule", &payload.to_string())
            .await?;
        let rows: Vec<ProgramScheduleItemDto> = parse_json(&body)?;
        rows.into_iter()
            .next()
            .map(Into::into)
            .ok_or_else(|| "No row returned".to_string())
            .map_err(DomainError::from)
    }
}

#[common::async_trait_platform]
impl DeleteProgramScheduleItemWrite for SupabaseRestRepository {
    async fn delete_program_schedule_item(&self, schedule_id: &Id) -> Result<()> {
        let path = format!("/program_schedule?id=eq.{}", schedule_id.to_string());
        self.client.delete(&path).await?;
        Ok(())
    }
}

#[common::async_trait_platform]
impl CreateExerciseWrite for SupabaseRestRepository {
    async fn create_exercise(
        &self,
        specialist_id: &Id,
        name: &ExerciseName,
        description: Option<&Description>,
        order_index: ScheduleOrderIndex,
        video_url: Option<&VideoUrl>,
    ) -> Result<Exercise> {
        let mut payload = serde_json::json!({
            "specialist_id": specialist_id.to_string(),
            "name": name.value(),
            "description": description.map(|d| d.value()),
            "order_index": order_index.value()
        });
        if let Some(url) = video_url {
            payload["video_url"] = serde_json::Value::String(url.value().to_string());
        }
        let body = self.client.post("/exercises", &payload.to_string()).await?;
        let rows: Vec<ExerciseDto> = parse_json(&body)?;
        rows.into_iter()
            .next()
            .map(Into::into)
            .ok_or_else(|| "No row returned".to_string())
            .map_err(DomainError::from)
    }
}

#[common::async_trait_platform]
impl AddExerciseToWorkoutWrite for SupabaseRestRepository {
    async fn add_exercise_to_workout(
        &self,
        workout_id: &Id,
        exercise_id: &Id,
        order_index: ScheduleOrderIndex,
        sets: Sets,
        reps: Reps,
    ) -> Result<()> {
        let payload = serde_json::json!({
            "workout_id": workout_id.to_string(),
            "exercise_id": exercise_id.to_string(),
            "order_index": order_index.value(),
            "sets": sets.value(),
            "reps": reps.value()
        });
        self.client
            .post("/workout_exercises", &payload.to_string())
            .await?;
        Ok(())
    }
}

#[common::async_trait_platform]
impl UpdateWorkoutExerciseWrite for SupabaseRestRepository {
    async fn update_workout_exercise(
        &self,
        workout_id: &Id,
        exercise_id: &Id,
        sets: Sets,
        reps: Reps,
        order_index: Option<ScheduleOrderIndex>,
    ) -> Result<()> {
        let mut payload = serde_json::json!({
            "sets": sets.value(),
            "reps": reps.value()
        });
        if let Some(o) = order_index {
            payload["order_index"] = serde_json::Number::from(o.value()).into();
        }
        let path = format!(
            "/workout_exercises?workout_id=eq.{}&exercise_id=eq.{}",
            workout_id.to_string(),
            exercise_id.to_string()
        );
        self.client.patch(&path, &payload.to_string()).await?;
        Ok(())
    }
}

#[common::async_trait_platform]
impl RemoveExerciseFromWorkoutWrite for SupabaseRestRepository {
    async fn remove_exercise_from_workout(&self, workout_id: &Id, exercise_id: &Id) -> Result<()> {
        let path = format!(
            "/workout_exercises?workout_id=eq.{}&exercise_id=eq.{}",
            workout_id.to_string(),
            exercise_id.to_string()
        );
        self.client.delete(&path).await?;
        Ok(())
    }
}

#[common::async_trait_platform]
impl UpdateExerciseWrite for SupabaseRestRepository {
    async fn update_exercise(
        &self,
        exercise_id: &Id,
        name: Option<&ExerciseName>,
        description: Option<&Description>,
        order_index: Option<ScheduleOrderIndex>,
        video_url: Patch<VideoUrl>,
    ) -> Result<()> {
        let mut payload = serde_json::json!({});
        if let Some(n) = name {
            payload["name"] = serde_json::Value::String(n.value().to_string());
        }
        if let Some(d) = description {
            payload["description"] = serde_json::Value::String(d.value().to_string());
        }
        if let Some(o) = order_index {
            payload["order_index"] = serde_json::Value::Number(serde_json::Number::from(o.value()));
        }
        match video_url {
            Patch::Omit => {}
            Patch::Clear => {
                payload["video_url"] = serde_json::Value::Null;
            }
            Patch::Set(u) => {
                payload["video_url"] = serde_json::Value::String(u.value().to_string());
            }
        }
        let path = format!("/exercises?id=eq.{}", exercise_id.to_string());
        self.client.patch(&path, &payload.to_string()).await?;
        Ok(())
    }
}

#[common::async_trait_platform]
impl SoftDeleteExerciseWrite for SupabaseRestRepository {
    async fn soft_delete_exercise(&self, exercise_id: &Id) -> Result<()> {
        let payload = serde_json::json!({
            "deleted_at": chrono::Utc::now().to_rfc3339()
        });
        let path = format!("/exercises?id=eq.{}", exercise_id.to_string());
        self.client.patch(&path, &payload.to_string()).await?;
        Ok(())
    }
}

#[common::async_trait_platform]
impl RestoreExerciseWrite for SupabaseRestRepository {
    async fn restore_exercise(&self, exercise_id: &Id) -> Result<()> {
        let payload = serde_json::json!({ "deleted_at": serde_json::Value::Null });
        let path = format!("/exercises?id=eq.{}", exercise_id.to_string());
        self.client.patch(&path, &payload.to_string()).await?;
        Ok(())
    }
}

#[common::async_trait_platform]
impl AssignProgramToPatientWrite for SupabaseRestRepository {
    async fn assign_program_to_patient(
        &self,
        patient_id: &Id,
        program_id: &Id,
    ) -> Result<PatientProgram> {
        let payload = serde_json::json!({
            "patient_id": patient_id.to_string(),
            "program_id": program_id.to_string(),
            "status": "active"
        });
        let body = self
            .client
            .post("/patient_programs", &payload.to_string())
            .await?;
        let rows: Vec<PatientProgramDto> = parse_json(&body)?;
        rows.into_iter()
            .next()
            .map(Into::into)
            .ok_or_else(|| "No row returned".to_string())
            .map_err(DomainError::from)
    }
}

#[common::async_trait_platform]
impl UnassignProgramFromPatientWrite for SupabaseRestRepository {
    async fn unassign_program_from_patient(&self, patient_program_id: &Id) -> Result<()> {
        let path = format!("/patient_programs?id=eq.{}", patient_program_id.to_string());
        self.client.delete(&path).await?;
        Ok(())
    }
}

#[common::async_trait_platform]
impl GetOrCreateSessionCatalogWrite for SupabaseRestRepository {
    async fn get_or_create_session(
        &self,
        patient_program_id: &Id,
        day_index: DayIndex,
        session_date: &SessionDate,
    ) -> Result<WorkoutSession> {
        let path = format!(
            "/workout_sessions?patient_program_id=eq.{}&day_index=eq.{}&select=id,patient_program_id,day_index,session_date,completed_at,created_at,updated_at",
            patient_program_id.to_string(),
            day_index.value()
        );
        let body = self.client.get(&path).await?;
        let rows: Vec<WorkoutSessionDto> = parse_json(&body)?;
        if let Some(s) = rows.into_iter().next() {
            return Ok(s.into());
        }
        let payload = serde_json::json!({
            "patient_program_id": patient_program_id.to_string(),
            "day_index": day_index.value(),
            "session_date": session_date.value()
        });
        let body = self
            .client
            .post("/workout_sessions", &payload.to_string())
            .await?;
        let rows: Vec<WorkoutSessionDto> = parse_json(&body)?;
        rows.into_iter()
            .next()
            .map(Into::into)
            .ok_or_else(|| "No row returned".to_string())
            .map_err(DomainError::from)
    }
}

#[common::async_trait_platform]
impl CompleteSessionCatalogWrite for SupabaseRestRepository {
    async fn complete_session(&self, session_id: &Id) -> Result<()> {
        let payload = serde_json::json!({
            "completed_at": chrono::Utc::now().to_rfc3339()
        });
        let path = format!("/workout_sessions?id=eq.{}", session_id.to_string());
        self.client.patch(&path, &payload.to_string()).await?;
        Ok(())
    }
}

#[common::async_trait_platform]
impl UpdateSessionWrite for SupabaseRestRepository {
    async fn update_session(
        &self,
        session_id: &Id,
        session_date: Option<&SessionDate>,
    ) -> Result<()> {
        let mut payload = serde_json::json!({});
        if let Some(d) = session_date {
            payload["session_date"] = serde_json::Value::String(d.value().to_string());
        }
        let path = format!("/workout_sessions?id=eq.{}", session_id.to_string());
        self.client.patch(&path, &payload.to_string()).await?;
        Ok(())
    }
}

#[common::async_trait_platform]
impl UpsertSessionExerciseFeedbackCatalogWrite for SupabaseRestRepository {
    async fn upsert_session_exercise_feedback(
        &self,
        workout_session_id: &Id,
        exercise_id: &Id,
        effort: Option<EffortScore>,
        pain: Option<PainScore>,
        comment: Option<&FeedbackComment>,
    ) -> Result<()> {
        let payload = serde_json::json!({
            "workout_session_id": workout_session_id.to_string(),
            "exercise_id": exercise_id.to_string(),
            "effort": effort.map(|e| e.value()),
            "pain": pain.map(|p| p.value()),
            "comment": comment.map(|c| c.value())
        });
        let path = format!(
            "/session_exercise_feedback?workout_session_id=eq.{}&exercise_id=eq.{}",
            workout_session_id.to_string(),
            exercise_id.to_string()
        );
        let _ = self.client.delete(&path).await;
        self.client
            .post("/session_exercise_feedback", &payload.to_string())
            .await?;
        Ok(())
    }
}

#[common::async_trait_platform]
impl UncompleteSessionCatalogWrite for SupabaseRestRepository {
    async fn uncomplete_session(&self, session_id: &Id) -> Result<()> {
        let payload = serde_json::json!({ "completed_at": serde_json::Value::Null });
        let path = format!("/workout_sessions?id=eq.{}", session_id.to_string());
        self.client.patch(&path, &payload.to_string()).await?;
        Ok(())
    }
}

#[common::async_trait_platform]
impl PatientSessionWriteRepository for SupabaseRestRepository {
    async fn get_or_create_session(
        &self,
        patient_program_id: &Id,
        day_index: DayIndex,
        session_date: &SessionDate,
    ) -> Result<WorkoutSession> {
        GetOrCreateSessionCatalogWrite::get_or_create_session(
            self,
            patient_program_id,
            day_index,
            session_date,
        )
        .await
    }

    async fn complete_session(&self, session_id: &Id, session_date: &SessionDate) -> Result<()> {
        let mut payload = serde_json::json!({});
        payload["session_date"] = serde_json::Value::String(session_date.value().to_string());
        payload["completed_at"] = serde_json::Value::String(chrono::Utc::now().to_rfc3339());
        let path = format!("/workout_sessions?id=eq.{}", session_id.to_string());
        self.client.patch(&path, &payload.to_string()).await?;
        Ok(())
    }

    async fn uncomplete_session(&self, session_id: &Id) -> Result<()> {
        UncompleteSessionCatalogWrite::uncomplete_session(self, session_id).await
    }

    async fn upsert_session_exercise_feedback(
        &self,
        workout_session_id: &Id,
        exercise_id: &Id,
        effort: Option<EffortScore>,
        pain: Option<PainScore>,
        comment: Option<&FeedbackComment>,
    ) -> Result<()> {
        let payload = serde_json::json!({
            "workout_session_id": workout_session_id.to_string(),
            "exercise_id": exercise_id.to_string(),
            "effort": effort.map(|e| e.value()),
            "pain": pain.map(|p| p.value()),
            "comment": comment.map(|c| c.value())
        });
        let path = format!(
            "/session_exercise_feedback?workout_session_id=eq.{}&exercise_id=eq.{}",
            workout_session_id.to_string(),
            exercise_id.to_string()
        );
        let _ = self.client.upsert(&path, &payload.to_string()).await;
        Ok(())
    }
}

fn parse_json<T: DeserializeOwned>(body: &[u8]) -> std::result::Result<T, String> {
    serde_json::from_slice(body).map_err(|e| e.to_string())
}

pub struct SupabaseRestRepositoryBuilder {
    client: Option<Arc<SupabaseClient>>,
}

impl SupabaseRestRepositoryBuilder {
    pub fn new() -> Self {
        Self { client: None }
    }

    pub fn with_client(mut self, client: Arc<SupabaseClient>) -> Self {
        self.client = Some(client);
        self
    }

    pub fn build(self) -> Arc<SupabaseRestRepository> {
        let client = self.client.unwrap_or_else(|| DEFAULT_CLIENT.clone());
        Arc::new(SupabaseRestRepository::new(client))
    }
}
