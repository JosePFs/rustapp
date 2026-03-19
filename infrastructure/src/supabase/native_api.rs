use async_trait::async_trait;
use serde::Deserialize;

use super::client::SupabaseClient;
use crate::application::ports::{AuthServiceSend, DataMutatorSend, DataProviderSend};
use crate::infrastructure::api::dtos::{
    PatientProgramDto, ProfileDto, ProgramDto, ProgramScheduleItemDto, SessionExerciseFeedbackDto,
    WorkoutDto, WorkoutExerciseRow, WorkoutSessionDto,
};
use domain::entities::{
    PatientProgram, Program, ProgramScheduleItem, SessionExerciseFeedback, Workout,
    WorkoutExercise, WorkoutSession,
};
use domain::{
    credentials::Credentials, error::DomainError, error::Result, profile::Profile, session::Session,
};

fn parse_json<T: for<'de> Deserialize<'de>>(body: &[u8]) -> std::result::Result<T, String> {
    serde_json::from_slice(body).map_err(|e| e.to_string())
}

#[derive(Clone)]
pub struct NativeApi {
    client: SupabaseClient,
}

impl NativeApi {
    pub fn new(client: SupabaseClient) -> Self {
        Self { client }
    }

    async fn get_workouts_by_ids(
        &self,
        access_token: &str,
        ids: &[String],
    ) -> Result<Vec<Workout>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let ids_param = ids.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(",");
        let path = format!(
            "/workouts?id=in.({})&select=id,specialist_id,name,description,order_index,created_at,updated_at",
            ids_param
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<WorkoutDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
}

#[async_trait]
impl AuthServiceSend for NativeApi {
    async fn sign_in(&self, credentials: &Credentials) -> Result<Session> {
        self.client
            .sign_in(credentials)
            .await
            .map_err(|e| {
                log::warn!("Login failed: {}", e);
                DomainError::Login("wrong_credentials".to_string())
            })
            .map(|auth| Session::new(auth.access_token, auth.refresh_token, auth.user.id))
    }
}

#[async_trait]
impl DataProviderSend for NativeApi {
    async fn get_profiles_by_ids(
        &self,
        ids: &[String],
        access_token: &str,
    ) -> Result<Vec<Profile>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let filter = format!("id=in.({})", ids.join(","));
        let path = format!(
            "/profiles?select=id,email,full_name,role,created_at,updated_at&{}",
            filter
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<ProfileDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(|row| row.into()).collect())
    }

    async fn get_program(&self, access_token: &str, program_id: &str) -> Result<Option<Program>> {
        let path = format!(
            "/programs?id=eq.{}&select=id,specialist_id,name,description,created_at,updated_at",
            program_id
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<ProgramDto> = parse_json(&body)?;
        Ok(rows.into_iter().next().map(Into::into))
    }

    async fn list_workouts_for_program(
        &self,
        access_token: &str,
        program_id: &str,
    ) -> Result<Vec<Workout>> {
        let schedule = self.list_program_schedule(access_token, program_id).await?;
        let ids: Vec<String> = schedule
            .iter()
            .filter_map(|item| item.workout_id.clone())
            .collect::<std::collections::HashSet<String>>()
            .into_iter()
            .collect();

        self.get_workouts_by_ids(access_token, &ids).await
    }

    async fn list_program_schedule(
        &self,
        access_token: &str,
        program_id: &str,
    ) -> Result<Vec<ProgramScheduleItem>> {
        let path = format!(
            "/program_schedule?program_id=eq.{}&select=id,program_id,order_index,workout_id,days_count,created_at&order=order_index.asc",
            program_id
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<ProgramScheduleItemDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_workout_sessions(
        &self,
        access_token: &str,
        patient_program_id: &str,
    ) -> Result<Vec<WorkoutSession>> {
        let path = format!(
            "/workout_sessions?patient_program_id=eq.{}&select=id,patient_program_id,day_index,session_date,completed_at,created_at,updated_at&order=day_index.asc",
            patient_program_id
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<WorkoutSessionDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_exercises_for_workout(
        &self,
        access_token: &str,
        workout_id: &str,
    ) -> Result<Vec<WorkoutExercise>> {
        let path = format!(
            "/workout_exercises?workout_id=eq.{}&select=order_index,exercise_id,sets,reps,exercises(id,specialist_id,name,description,order_index,video_url,deleted_at,created_at)&order=order_index.asc",
            workout_id
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<WorkoutExerciseRow> = parse_json(&body)?;
        Ok(rows
            .into_iter()
            .filter_map(|row| {
                row.exercises.map(|exercise| WorkoutExercise {
                    exercise: exercise.into(),
                    order_index: row.order_index,
                    sets: row.sets,
                    reps: row.reps,
                })
            })
            .collect())
    }

    async fn list_session_exercise_feedback_for_program(
        &self,
        access_token: &str,
        patient_program_id: &str,
    ) -> Result<Vec<SessionExerciseFeedback>> {
        let sessions_path = format!(
            "/workout_sessions?patient_program_id=eq.{}&select=id",
            patient_program_id
        );
        let body = self
            .client
            .rest_get(Some(access_token), &sessions_path)
            .await?;

        #[derive(Deserialize)]
        struct IdRow {
            id: String,
        }

        let session_rows: Vec<IdRow> = parse_json(&body)?;
        let ids: Vec<&str> = session_rows.iter().map(|row| row.id.as_str()).collect();
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let in_filter = ids.join(",");
        let path = format!(
            "/session_exercise_feedback?workout_session_id=in.({})&select=workout_session_id,exercise_id,effort,pain,comment",
            in_filter
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<SessionExerciseFeedbackDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_active_patient_programs(
        &self,
        access_token: &str,
    ) -> Result<Vec<PatientProgram>> {
        let path = "/patient_programs?status=eq.active&select=id,patient_id,program_id,status,assigned_at,created_at,updated_at&order=assigned_at.desc";
        let body = self.client.rest_get(Some(access_token), path).await?;
        let rows: Vec<PatientProgramDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
}

#[async_trait]
impl DataMutatorSend for NativeApi {
    async fn get_or_create_session(
        &self,
        access_token: &str,
        patient_program_id: &str,
        day_index: i32,
        session_date: &str,
    ) -> Result<WorkoutSession> {
        let path = format!(
            "/workout_sessions?patient_program_id=eq.{}&day_index=eq.{}&select=id,patient_program_id,day_index,session_date,completed_at,created_at,updated_at",
            patient_program_id, day_index
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<WorkoutSessionDto> = parse_json(&body)?;
        if let Some(session) = rows.into_iter().next() {
            return Ok(session.into());
        }

        let payload = serde_json::json!({
            "patient_program_id": patient_program_id,
            "day_index": day_index,
            "session_date": session_date
        });
        let body = self
            .client
            .rest_post(Some(access_token), "/workout_sessions", &payload)
            .await?;
        let rows: Vec<WorkoutSessionDto> = parse_json(&body)?;
        rows.into_iter()
            .next()
            .map(Into::into)
            .ok_or_else(|| "No row returned".to_string())
            .map_err(DomainError::from)
    }

    async fn complete_session(
        &self,
        access_token: &str,
        session_id: &str,
        session_date: &str,
    ) -> Result<()> {
        let mut payload = serde_json::json!({});
        payload["session_date"] = serde_json::Value::String(session_date.to_string());
        payload["completed_at"] = serde_json::Value::String(chrono::Utc::now().to_rfc3339());
        let path = format!("/workout_sessions?id=eq.{}", session_id);
        self.client
            .rest_patch(Some(access_token), &path, &payload)
            .await?;
        Ok(())
    }

    async fn uncomplete_session(&self, access_token: &str, session_id: &str) -> Result<()> {
        let payload = serde_json::json!({
            "completed_at": serde_json::Value::Null
        });
        let path = format!("/workout_sessions?id=eq.{}", session_id);
        self.client
            .rest_patch(Some(access_token), &path, &payload)
            .await?;
        Ok(())
    }

    async fn upsert_session_exercise_feedback(
        &self,
        access_token: &str,
        workout_session_id: &str,
        exercise_id: &str,
        effort: Option<i32>,
        pain: Option<i32>,
        comment: Option<&str>,
    ) -> Result<()> {
        let payload = serde_json::json!({
            "workout_session_id": workout_session_id,
            "exercise_id": exercise_id,
            "effort": effort,
            "pain": pain,
            "comment": comment
        });
        let path = format!(
            "/session_exercise_feedback?workout_session_id=eq.{}&exercise_id=eq.{}",
            workout_session_id, exercise_id
        );
        self.client
            .rest_upsert(Some(access_token), &path, &payload)
            .await
            .ok();
        Ok(())
    }
}

impl application::ports::MobileBackend for NativeApi {}
