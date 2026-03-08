use async_trait::async_trait;

use dioxus_i18n::t;
use serde::Deserialize;

use super::client::SupabaseClient;
use crate::application::ports::Backend;
use crate::application::services::data_mutator::DataMutator;
use crate::application::services::data_provider::DataProvider;
use crate::application::services::AuthService;
use crate::domain::credentials::Credentials;
use crate::domain::entities::{
    Exercise, PatientProgram, Program, ProgramScheduleItem, SessionExerciseFeedback,
    SpecialistPatient, Workout, WorkoutExercise, WorkoutSession,
};
use crate::domain::error::DomainError;
use crate::domain::profile::Profile;
use crate::domain::session::Session;
use crate::infrastructure::api::dtos::{
    ExerciseDto, PatientProgramDto, ProfileDto, ProgramDto, ProgramScheduleItemDto,
    SessionExerciseFeedbackDto, SpecialistPatientDto, WorkoutDto, WorkoutExerciseRow,
    WorkoutSessionDto,
};

fn parse_json<T: for<'de> Deserialize<'de>>(body: &[u8]) -> Result<T, String> {
    serde_json::from_slice(body).map_err(|e| e.to_string())
}

#[derive(Clone)]
pub struct Api {
    client: SupabaseClient,
}

impl Api {
    pub fn new(client: SupabaseClient) -> Self {
        Self { client }
    }
}

#[async_trait(?Send)]
impl AuthService for Api {
    async fn sign_in(&self, credentials: &Credentials) -> crate::domain::error::Result<Session> {
        let auth = self
            .client
            .sign_in(credentials)
            .await
            .map_err(|_| DomainError::Login(t!("wrong_credentials")))?;
        Ok(Session::new(auth.access_token, auth.user.id))
    }
}

#[async_trait(?Send)]
impl DataProvider for Api {
    async fn get_profiles_by_ids(
        &self,
        ids: &[String],
        access_token: &str,
    ) -> Result<Vec<Profile>, String> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let filter = format!("id=in.({})", ids.join(","));
        let path = format!(
            "/profiles?select=id,email,full_name,role,created_at,updated_at&{}",
            filter
        );
        let body = self
            .client
            .rest_get(Some(access_token), &path)
            .await
            .map_err(|e| e.to_string())?;
        let rows: Vec<ProfileDto> = parse_json(&body).map_err(|e| e.to_string())?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn get_patient_id_by_email(
        &self,
        access_token: &str,
        email: &str,
    ) -> Result<Option<String>, String> {
        let path = "/rpc/get_patient_id_by_email";
        let body = serde_json::json!({ "p_email": email });
        let body_bytes = body.to_string().into_bytes();
        let response = self
            .client
            .rest_request(Some(access_token), "POST", path, Some(&body_bytes))
            .await?;
        let id: Option<String> = serde_json::from_slice(&response).map_err(|e| e.to_string())?;
        Ok(id)
    }

    async fn list_specialist_patients(
        &self,
        access_token: &str,
    ) -> Result<Vec<SpecialistPatient>, String> {
        let body = self
            .client
            .rest_get(
                Some(access_token),
                "/specialist_patients?select=id,specialist_id,patient_id,created_at",
            )
            .await?;
        let rows: Vec<SpecialistPatientDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_programs(&self, access_token: &str) -> Result<Vec<Program>, String> {
        let body = self
            .client
            .rest_get(
                Some(access_token),
                "/programs?select=id,specialist_id,name,description,created_at,updated_at&order=created_at.desc",
            )
            .await?;
        let rows: Vec<ProgramDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn get_program(
        &self,
        access_token: &str,
        program_id: &str,
    ) -> Result<Option<Program>, String> {
        let path = format!(
            "/programs?id=eq.{}&select=id,specialist_id,name,description,created_at,updated_at",
            program_id
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<ProgramDto> = parse_json(&body)?;
        Ok(rows.into_iter().next().map(Into::into))
    }

    async fn list_workout_library(
        &self,
        access_token: &str,
        specialist_id: &str,
        name_filter: Option<&str>,
    ) -> Result<Vec<Workout>, String> {
        let path = format!(
            "/workouts?specialist_id=eq.{}&select=id,specialist_id,name,description,order_index,created_at,updated_at&order=order_index.asc,name.asc",
            specialist_id
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<WorkoutDto> = parse_json(&body)?;
        let filtered: Vec<WorkoutDto> = if let Some(f) = name_filter {
            let f = f.trim().to_lowercase();
            if f.is_empty() {
                rows
            } else {
                rows.into_iter()
                    .filter(|w| w.name.to_lowercase().contains(&f))
                    .collect()
            }
        } else {
            rows
        };
        Ok(filtered.into_iter().map(Into::into).collect())
    }

    async fn get_workouts_by_ids(
        &self,
        access_token: &str,
        ids: &[String],
    ) -> Result<Vec<Workout>, String> {
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

    async fn list_workouts_for_program(
        &self,
        access_token: &str,
        program_id: &str,
    ) -> Result<Vec<Workout>, String> {
        let schedule = self.list_program_schedule(access_token, program_id).await?;
        let ids: Vec<String> = schedule
            .iter()
            .filter_map(|s| s.workout_id.clone())
            .collect::<std::collections::HashSet<String>>()
            .into_iter()
            .collect();
        self.get_workouts_by_ids(access_token, &ids).await
    }

    async fn list_program_schedule(
        &self,
        access_token: &str,
        program_id: &str,
    ) -> Result<Vec<ProgramScheduleItem>, String> {
        let path = format!(
            "/program_schedule?program_id=eq.{}&select=id,program_id,order_index,workout_id,days_count,created_at&order=order_index.asc",
            program_id
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<ProgramScheduleItemDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_exercises_for_workout(
        &self,
        access_token: &str,
        workout_id: &str,
    ) -> Result<Vec<WorkoutExercise>, String> {
        let path = format!(
            "/workout_exercises?workout_id=eq.{}&select=order_index,exercise_id,sets,reps,exercises(id,specialist_id,name,description,order_index,video_url,deleted_at,created_at)&order=order_index.asc",
            workout_id
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<WorkoutExerciseRow> = parse_json(&body)?;
        Ok(rows
            .into_iter()
            .filter_map(|r| {
                r.exercises.map(|e| WorkoutExercise {
                    exercise: e.into(),
                    order_index: r.order_index,
                    sets: r.sets,
                    reps: r.reps,
                })
            })
            .collect())
    }

    async fn list_exercise_library(
        &self,
        access_token: &str,
        specialist_id: &str,
        name_filter: Option<&str>,
    ) -> Result<Vec<Exercise>, String> {
        let path = format!(
            "/exercises?specialist_id=eq.{}&select=id,specialist_id,name,description,order_index,video_url,deleted_at,created_at&order=name.asc",
            specialist_id
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<ExerciseDto> = parse_json(&body)?;
        let filtered: Vec<ExerciseDto> = if let Some(f) = name_filter {
            let f = f.trim().to_lowercase();
            if f.is_empty() {
                rows
            } else {
                rows.into_iter()
                    .filter(|e| e.name.to_lowercase().contains(&f))
                    .collect()
            }
        } else {
            rows
        };
        Ok(filtered.into_iter().map(Into::into).collect())
    }

    async fn list_patient_programs_for_specialist(
        &self,
        access_token: &str,
    ) -> Result<Vec<PatientProgram>, String> {
        let body = self
            .client
            .rest_get(
                Some(access_token),
                "/patient_programs?select=id,patient_id,program_id,status,assigned_at,created_at,updated_at&order=assigned_at.desc",
            )
            .await?;
        let rows: Vec<PatientProgramDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn get_patient_program_by_id(
        &self,
        access_token: &str,
        id: &str,
    ) -> Result<Option<PatientProgram>, String> {
        let path = format!(
            "/patient_programs?id=eq.{}&select=id,patient_id,program_id,status,assigned_at,created_at,updated_at&limit=1",
            id
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<PatientProgramDto> = parse_json(&body)?;
        Ok(rows.into_iter().next().map(Into::into))
    }

    async fn list_workout_sessions(
        &self,
        access_token: &str,
        patient_program_id: &str,
    ) -> Result<Vec<WorkoutSession>, String> {
        let path = format!(
            "/workout_sessions?patient_program_id=eq.{}&select=id,patient_program_id,day_index,session_date,completed_at,created_at,updated_at&order=day_index.asc",
            patient_program_id
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<WorkoutSessionDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_session_exercise_feedback(
        &self,
        access_token: &str,
        workout_session_id: &str,
    ) -> Result<Vec<SessionExerciseFeedback>, String> {
        let path = format!(
            "/session_exercise_feedback?workout_session_id=eq.{}&select=workout_session_id,exercise_id,effort,pain,comment",
            workout_session_id
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<SessionExerciseFeedbackDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    async fn list_session_exercise_feedback_for_program(
        &self,
        access_token: &str,
        patient_program_id: &str,
    ) -> Result<Vec<SessionExerciseFeedback>, String> {
        let sessions_path = format!(
            "/workout_sessions?patient_program_id=eq.{}&select=id",
            patient_program_id
        );
        let body = self.client.rest_get(Some(access_token), &sessions_path).await?;
        #[derive(Deserialize)]
        struct IdRow {
            id: String,
        }
        let session_rows: Vec<IdRow> = parse_json(&body)?;
        let ids: Vec<&str> = session_rows.iter().map(|r| r.id.as_str()).collect();
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
    ) -> Result<Vec<PatientProgram>, String> {
        let path = "/patient_programs?status=eq.active&select=id,patient_id,program_id,status,assigned_at,created_at,updated_at&order=assigned_at.desc";
        let body = self.client.rest_get(Some(access_token), path).await?;
        let rows: Vec<PatientProgramDto> = parse_json(&body)?;
        Ok(rows.into_iter().map(Into::into).collect())
    }
}

#[async_trait(?Send)]
impl DataMutator for Api {
    async fn add_specialist_patient(
        &self,
        access_token: &str,
        specialist_id: &str,
        patient_id: &str,
    ) -> Result<SpecialistPatient, String> {
        let payload = serde_json::json!({
            "specialist_id": specialist_id,
            "patient_id": patient_id
        });
        let body = self
            .client
            .rest_post(Some(access_token), "/specialist_patients", &payload)
            .await?;
        let rows: Vec<SpecialistPatientDto> = parse_json(&body)?;
        rows.into_iter()
            .next()
            .map(Into::into)
            .ok_or_else(|| "No row returned".to_string())
    }

    async fn create_program(
        &self,
        access_token: &str,
        specialist_id: &str,
        name: &str,
        description: Option<&str>,
    ) -> Result<Program, String> {
        let payload = serde_json::json!({
            "specialist_id": specialist_id,
            "name": name,
            "description": description
        });
        let body = self
            .client
            .rest_post(Some(access_token), "/programs", &payload)
            .await?;
        let rows: Vec<ProgramDto> = parse_json(&body)?;
        rows.into_iter()
            .next()
            .map(Into::into)
            .ok_or_else(|| "No row returned".to_string())
    }

    async fn create_workout(
        &self,
        access_token: &str,
        specialist_id: &str,
        name: &str,
        description: Option<&str>,
    ) -> Result<Workout, String> {
        let payload = serde_json::json!({
            "specialist_id": specialist_id,
            "name": name,
            "description": description,
            "order_index": 0
        });
        let body = self
            .client
            .rest_post(Some(access_token), "/workouts", &payload)
            .await?;
        let rows: Vec<WorkoutDto> = parse_json(&body)?;
        rows.into_iter()
            .next()
            .map(Into::into)
            .ok_or_else(|| "No row returned".to_string())
    }

    async fn update_workout(
        &self,
        access_token: &str,
        workout_id: &str,
        name: Option<&str>,
        description: Option<Option<&str>>,
        order_index: Option<i32>,
    ) -> Result<(), String> {
        let mut payload = serde_json::json!({});
        if let Some(n) = name {
            payload["name"] = serde_json::Value::String(n.to_string());
        }
        if let Some(d) = description {
            payload["description"] = d
                .map(|s| serde_json::Value::String(s.to_string()))
                .unwrap_or(serde_json::Value::Null);
        }
        if let Some(o) = order_index {
            payload["order_index"] = serde_json::Number::from(o).into();
        }
        let path = format!("/workouts?id=eq.{}", workout_id);
        self.client
            .rest_patch(Some(access_token), &path, &payload)
            .await?;
        Ok(())
    }

    async fn delete_workout(&self, access_token: &str, workout_id: &str) -> Result<(), String> {
        let path = format!("/workouts?id=eq.{}", workout_id);
        self.client.rest_delete(Some(access_token), &path).await?;
        Ok(())
    }

    async fn create_program_schedule_item(
        &self,
        access_token: &str,
        program_id: &str,
        order_index: i32,
        workout_id: Option<&str>,
        days_count: i32,
    ) -> Result<ProgramScheduleItem, String> {
        let mut payload = serde_json::json!({
            "program_id": program_id,
            "order_index": order_index,
            "days_count": days_count.max(1)
        });
        if let Some(wid) = workout_id {
            payload["workout_id"] = serde_json::Value::String(wid.to_string());
        } else {
            payload["workout_id"] = serde_json::Value::Null;
        }
        let body = self
            .client
            .rest_post(Some(access_token), "/program_schedule", &payload)
            .await?;
        let rows: Vec<ProgramScheduleItemDto> = parse_json(&body)?;
        rows.into_iter()
            .next()
            .map(Into::into)
            .ok_or_else(|| "No row returned".to_string())
    }

    async fn delete_program_schedule_item(
        &self,
        access_token: &str,
        schedule_id: &str,
    ) -> Result<(), String> {
        let path = format!("/program_schedule?id=eq.{}", schedule_id);
        self.client.rest_delete(Some(access_token), &path).await?;
        Ok(())
    }

    async fn create_exercise(
        &self,
        access_token: &str,
        specialist_id: &str,
        name: &str,
        description: Option<&str>,
        order_index: i32,
        video_url: Option<&str>,
    ) -> Result<Exercise, String> {
        let mut payload = serde_json::json!({
            "specialist_id": specialist_id,
            "name": name,
            "description": description,
            "order_index": order_index
        });
        if let Some(url) = video_url {
            payload["video_url"] = serde_json::Value::String(url.to_string());
        }
        let body = self
            .client
            .rest_post(Some(access_token), "/exercises", &payload)
            .await?;
        let rows: Vec<ExerciseDto> = parse_json(&body)?;
        rows.into_iter()
            .next()
            .map(Into::into)
            .ok_or_else(|| "No row returned".to_string())
    }

    async fn add_exercise_to_workout(
        &self,
        access_token: &str,
        workout_id: &str,
        exercise_id: &str,
        order_index: i32,
        sets: i32,
        reps: i32,
    ) -> Result<(), String> {
        let payload = serde_json::json!({
            "workout_id": workout_id,
            "exercise_id": exercise_id,
            "order_index": order_index,
            "sets": sets,
            "reps": reps
        });
        self.client
            .rest_post(Some(access_token), "/workout_exercises", &payload)
            .await?;
        Ok(())
    }

    async fn update_workout_exercise(
        &self,
        access_token: &str,
        workout_id: &str,
        exercise_id: &str,
        sets: i32,
        reps: i32,
        order_index: Option<i32>,
    ) -> Result<(), String> {
        let mut payload = serde_json::json!({
            "sets": sets,
            "reps": reps
        });
        if let Some(o) = order_index {
            payload["order_index"] = serde_json::Number::from(o).into();
        }
        let path = format!(
            "/workout_exercises?workout_id=eq.{}&exercise_id=eq.{}",
            workout_id, exercise_id
        );
        self.client
            .rest_patch(Some(access_token), &path, &payload)
            .await?;
        Ok(())
    }

    async fn remove_exercise_from_workout(
        &self,
        access_token: &str,
        workout_id: &str,
        exercise_id: &str,
    ) -> Result<(), String> {
        let path = format!(
            "/workout_exercises?workout_id=eq.{}&exercise_id=eq.{}",
            workout_id, exercise_id
        );
        self.client.rest_delete(Some(access_token), &path).await?;
        Ok(())
    }

    async fn update_exercise(
        &self,
        access_token: &str,
        exercise_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        order_index: Option<i32>,
        video_url: Option<Option<&str>>,
    ) -> Result<(), String> {
        let mut payload = serde_json::json!({});
        if let Some(n) = name {
            payload["name"] = serde_json::Value::String(n.to_string());
        }
        if let Some(d) = description {
            payload["description"] = serde_json::Value::String(d.to_string());
        }
        if let Some(o) = order_index {
            payload["order_index"] = serde_json::Value::Number(serde_json::Number::from(o));
        }
        if let Some(opt) = video_url {
            payload["video_url"] = opt
                .map(|u| serde_json::Value::String(u.to_string()))
                .unwrap_or(serde_json::Value::Null);
        }
        let path = format!("/exercises?id=eq.{}", exercise_id);
        self.client
            .rest_patch(Some(access_token), &path, &payload)
            .await?;
        Ok(())
    }

    async fn soft_delete_exercise(
        &self,
        access_token: &str,
        exercise_id: &str,
    ) -> Result<(), String> {
        let payload = serde_json::json!({
            "deleted_at": chrono::Utc::now().to_rfc3339()
        });
        let path = format!("/exercises?id=eq.{}", exercise_id);
        self.client
            .rest_patch(Some(access_token), &path, &payload)
            .await?;
        Ok(())
    }

    async fn restore_exercise(&self, access_token: &str, exercise_id: &str) -> Result<(), String> {
        let payload = serde_json::json!({ "deleted_at": serde_json::Value::Null });
        let path = format!("/exercises?id=eq.{}", exercise_id);
        self.client
            .rest_patch(Some(access_token), &path, &payload)
            .await?;
        Ok(())
    }

    async fn assign_program_to_patient(
        &self,
        access_token: &str,
        patient_id: &str,
        program_id: &str,
    ) -> Result<PatientProgram, String> {
        let payload = serde_json::json!({
            "patient_id": patient_id,
            "program_id": program_id,
            "status": "active"
        });
        let body = self
            .client
            .rest_post(Some(access_token), "/patient_programs", &payload)
            .await?;
        let rows: Vec<PatientProgramDto> = parse_json(&body)?;
        rows.into_iter()
            .next()
            .map(Into::into)
            .ok_or_else(|| "No row returned".to_string())
    }

    async fn unassign_program_from_patient(
        &self,
        access_token: &str,
        patient_program_id: &str,
    ) -> Result<(), String> {
        let path = format!("/patient_programs?id=eq.{}", patient_program_id);
        self.client.rest_delete(Some(access_token), &path).await?;
        Ok(())
    }

    async fn get_or_create_session(
        &self,
        access_token: &str,
        patient_program_id: &str,
        day_index: i32,
        session_date: &str,
    ) -> Result<WorkoutSession, String> {
        let path = format!(
            "/workout_sessions?patient_program_id=eq.{}&day_index=eq.{}&select=id,patient_program_id,day_index,session_date,completed_at,created_at,updated_at",
            patient_program_id, day_index
        );
        let body = self.client.rest_get(Some(access_token), &path).await?;
        let rows: Vec<WorkoutSessionDto> = parse_json(&body)?;
        if let Some(s) = rows.into_iter().next() {
            return Ok(s.into());
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
    }

    async fn complete_session(
        &self,
        access_token: &str,
        session_id: &str,
    ) -> Result<(), String> {
        let payload = serde_json::json!({
            "completed_at": chrono::Utc::now().to_rfc3339()
        });
        let path = format!("/workout_sessions?id=eq.{}", session_id);
        self.client
            .rest_patch(Some(access_token), &path, &payload)
            .await?;
        Ok(())
    }

    async fn update_session(
        &self,
        access_token: &str,
        session_id: &str,
        session_date: Option<&str>,
    ) -> Result<(), String> {
        let mut payload = serde_json::json!({});
        if let Some(d) = session_date {
            payload["session_date"] = serde_json::Value::String(d.to_string());
        }
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
    ) -> Result<(), String> {
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
        self.client.rest_delete(Some(access_token), &path).await.ok();
        self.client
            .rest_post(Some(access_token), "/session_exercise_feedback", &payload)
            .await?;
        Ok(())
    }

    async fn uncomplete_session(&self, access_token: &str, session_id: &str) -> Result<(), String> {
        let payload = serde_json::json!({ "completed_at": serde_json::Value::Null });
        let path = format!("/workout_sessions?id=eq.{}", session_id);
        self.client
            .rest_patch(Some(access_token), &path, &payload)
            .await?;
        Ok(())
    }
}

impl Backend for Api {}

/// Build agenda as ordered list of days from program_schedule (pure helper; no API needed).
pub fn build_agenda_schedule(
    schedule: &[ProgramScheduleItem],
    workouts: &[Workout],
) -> Vec<(i32, Option<String>, String)> {
    let workout_names: std::collections::HashMap<String, String> = workouts
        .iter()
        .map(|w| (w.id.clone(), w.name.clone()))
        .collect();
    let mut out = Vec::new();
    let mut day_index = 0i32;
    for item in schedule {
        let label = item
            .workout_id
            .as_ref()
            .and_then(|id| workout_names.get(id).cloned())
            .unwrap_or_else(|| "Descanso".to_string());
        for _ in 0..item.days_count.max(1) {
            out.push((day_index, item.workout_id.clone(), label.clone()));
            day_index += 1;
        }
    }
    out
}
