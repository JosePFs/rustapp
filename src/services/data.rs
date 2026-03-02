//! Data access: profiles, programs, exercises, patient_programs, workout_sessions.
//! All functions take config + optional access_token for RLS.

use serde::{Deserialize, Serialize};

use super::supabase_client::{rest_get, rest_patch, rest_post, rest_request, SupabaseConfig};

// -----------------------------------------------------------------------------
// Domain types (match DB)
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Profile {
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
pub struct SpecialistPatient {
    pub id: String,
    pub specialist_id: String,
    pub patient_id: String,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub id: String,
    pub specialist_id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Workout (entrenamiento): specialist's library; reusable in programs via program_schedule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Workout {
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

/// One block in the program schedule: N days of a workout, or N rest days (workout_id = null).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgramScheduleItem {
    pub id: String,
    pub program_id: String,
    pub order_index: i32,
    #[serde(default)]
    pub workout_id: Option<String>,
    pub days_count: i32,
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Exercise in the specialist's library (reusable across workouts).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Exercise {
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

/// Join row: exercise in a workout (order_index = order within that workout).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkoutExerciseRow {
    pub order_index: i32,
    pub exercise_id: String,
    #[serde(default)]
    pub exercises: Option<Exercise>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatientProgram {
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
pub struct WorkoutSession {
    pub id: String,
    pub patient_program_id: String,
    pub day_index: i32,
    pub session_date: String,
    pub completed_at: Option<String>,
    pub effort: Option<i32>,
    pub pain: Option<i32>,
    pub comment: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

// -----------------------------------------------------------------------------
// API helpers
// -----------------------------------------------------------------------------

fn parse_json<T: for<'de> Deserialize<'de>>(body: &[u8]) -> Result<T, String> {
    serde_json::from_slice(body).map_err(|e| e.to_string())
}

/// RPC call to get patient id by email (for specialist adding patient).
pub async fn get_patient_id_by_email(
    config: &SupabaseConfig,
    access_token: &str,
    email: &str,
) -> Result<Option<String>, String> {
    let path = "/rpc/get_patient_id_by_email";
    let body = serde_json::json!({ "p_email": email });
    let body_bytes = body.to_string().into_bytes();
    let response = rest_request(
        config,
        Some(access_token),
        "POST",
        path,
        Some(&body_bytes),
    )
    .await?;
    let id: Option<String> = serde_json::from_slice(&response).map_err(|e| e.to_string())?;
    Ok(id)
}

/// List patients linked to the specialist.
pub async fn list_specialist_patients(
    config: &SupabaseConfig,
    access_token: &str,
) -> Result<Vec<SpecialistPatient>, String> {
    let body = rest_get(
        config,
        Some(access_token),
        "/specialist_patients?select=id,specialist_id,patient_id,created_at",
    )
    .await?;
    let rows: Vec<SpecialistPatient> = parse_json(&body)?;
    Ok(rows)
}

/// Get profiles by ids (e.g. patient profiles).
pub async fn get_profiles_by_ids(
    config: &SupabaseConfig,
    access_token: &str,
    ids: &[String],
) -> Result<Vec<Profile>, String> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    let filter = format!("id=in.({})", ids.join(","));
    let path = format!("/profiles?select=id,email,full_name,role,created_at,updated_at&{}", filter);
    let body = rest_get(config, Some(access_token), &path).await?;
    let rows: Vec<Profile> = parse_json(&body)?;
    Ok(rows)
}

/// Link a patient to the specialist (add patient).
pub async fn add_specialist_patient(
    config: &SupabaseConfig,
    access_token: &str,
    specialist_id: &str,
    patient_id: &str,
) -> Result<SpecialistPatient, String> {
    let payload = serde_json::json!({
        "specialist_id": specialist_id,
        "patient_id": patient_id
    });
    let body = rest_post(config, Some(access_token), "/specialist_patients", &payload).await?;
    let rows: Vec<SpecialistPatient> = parse_json(&body)?;
    rows.into_iter().next().ok_or_else(|| "No row returned".to_string())
}

/// List programs for the specialist.
pub async fn list_programs(
    config: &SupabaseConfig,
    access_token: &str,
) -> Result<Vec<Program>, String> {
    let body = rest_get(
        config,
        Some(access_token),
        "/programs?select=id,specialist_id,name,description,created_at,updated_at&order=created_at.desc",
    )
    .await?;
    let rows: Vec<Program> = parse_json(&body)?;
    Ok(rows)
}

/// Create a program (no fixed duration; schedule is built from program_schedule blocks).
pub async fn create_program(
    config: &SupabaseConfig,
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
    let body = rest_post(config, Some(access_token), "/programs", &payload).await?;
    let rows: Vec<Program> = parse_json(&body)?;
    rows.into_iter().next().ok_or_else(|| "No row returned".to_string())
}

/// List specialist's workout library (optional name filter).
pub async fn list_workout_library(
    config: &SupabaseConfig,
    access_token: &str,
    specialist_id: &str,
    name_filter: Option<&str>,
) -> Result<Vec<Workout>, String> {
    let path = format!(
        "/workouts?specialist_id=eq.{}&select=id,specialist_id,name,description,order_index,created_at,updated_at&order=order_index.asc,name.asc",
        specialist_id
    );
    let body = rest_get(config, Some(access_token), &path).await?;
    let rows: Vec<Workout> = parse_json(&body)?;
    let filtered = if let Some(f) = name_filter {
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
    Ok(filtered)
}

/// Fetch workouts by ids (for resolving schedule workout_ids).
pub async fn get_workouts_by_ids(
    config: &SupabaseConfig,
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
    let body = rest_get(config, Some(access_token), &path).await?;
    let rows: Vec<Workout> = parse_json(&body)?;
    Ok(rows)
}

/// List workouts that appear in a program's schedule (for agenda display).
pub async fn list_workouts_for_program(
    config: &SupabaseConfig,
    access_token: &str,
    program_id: &str,
) -> Result<Vec<Workout>, String> {
    let schedule = list_program_schedule(config, access_token, program_id).await?;
    let ids: Vec<String> = schedule
        .iter()
        .filter_map(|s| s.workout_id.clone())
        .collect::<std::collections::HashSet<String>>()
        .into_iter()
        .collect();
    get_workouts_by_ids(config, access_token, &ids).await
}

/// Create a workout in the specialist's library.
pub async fn create_workout(
    config: &SupabaseConfig,
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
    let body = rest_post(config, Some(access_token), "/workouts", &payload).await?;
    let rows: Vec<Workout> = parse_json(&body)?;
    rows.into_iter().next().ok_or_else(|| "No row returned".to_string())
}

/// Update workout (name, description, order_index).
pub async fn update_workout(
    config: &SupabaseConfig,
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
    rest_patch(config, Some(access_token), &path, &payload).await?;
    Ok(())
}

/// List program schedule (blocks of workout or rest). Program defines the agenda.
pub async fn list_program_schedule(
    config: &SupabaseConfig,
    access_token: &str,
    program_id: &str,
) -> Result<Vec<ProgramScheduleItem>, String> {
    let path = format!(
        "/program_schedule?program_id=eq.{}&select=id,program_id,order_index,workout_id,days_count,created_at&order=order_index.asc",
        program_id
    );
    let body = rest_get(config, Some(access_token), &path).await?;
    let rows: Vec<ProgramScheduleItem> = parse_json(&body)?;
    Ok(rows)
}

/// Add a block to the program schedule (workout or rest).
pub async fn create_program_schedule_item(
    config: &SupabaseConfig,
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
    let body = rest_post(config, Some(access_token), "/program_schedule", &payload).await?;
    let rows: Vec<ProgramScheduleItem> = parse_json(&body)?;
    rows.into_iter().next().ok_or_else(|| "No row returned".to_string())
}

/// Update program schedule item (workout_id, days_count).
pub async fn update_program_schedule_item(
    config: &SupabaseConfig,
    access_token: &str,
    schedule_id: &str,
    workout_id: Option<Option<&str>>,
    days_count: Option<i32>,
) -> Result<(), String> {
    let mut payload = serde_json::json!({});
    if let Some(opt) = workout_id {
        payload["workout_id"] = opt
            .map(|s| serde_json::Value::String(s.to_string()))
            .unwrap_or(serde_json::Value::Null);
    }
    if let Some(d) = days_count {
        payload["days_count"] = serde_json::Number::from(d.max(1)).into();
    }
    let path = format!("/program_schedule?id=eq.{}", schedule_id);
    rest_patch(config, Some(access_token), &path, &payload).await?;
    Ok(())
}

/// Delete a program schedule item.
pub async fn delete_program_schedule_item(
    config: &SupabaseConfig,
    access_token: &str,
    schedule_id: &str,
) -> Result<(), String> {
    let path = format!("/program_schedule?id=eq.{}", schedule_id);
    rest_request(config, Some(access_token), "DELETE", &path, None).await?;
    Ok(())
}

/// Delete a workout (and its exercises via CASCADE).
pub async fn delete_workout(
    config: &SupabaseConfig,
    access_token: &str,
    workout_id: &str,
) -> Result<(), String> {
    let path = format!("/workouts?id=eq.{}", workout_id);
    rest_request(config, Some(access_token), "DELETE", &path, None).await?;
    Ok(())
}

/// List exercises in a workout (via workout_exercises join).
pub async fn list_exercises_for_workout(
    config: &SupabaseConfig,
    access_token: &str,
    workout_id: &str,
) -> Result<Vec<Exercise>, String> {
    let path = format!(
        "/workout_exercises?workout_id=eq.{}&select=order_index,exercise_id,exercises(id,specialist_id,name,description,order_index,video_url,deleted_at,created_at)&order=order_index.asc",
        workout_id
    );
    let body = rest_get(config, Some(access_token), &path).await?;
    let rows: Vec<WorkoutExerciseRow> = parse_json(&body)?;
    Ok(rows
        .into_iter()
        .filter_map(|r| r.exercises)
        .collect())
}

/// List exercise library for a specialist (optionally filter by name substring).
pub async fn list_exercise_library(
    config: &SupabaseConfig,
    access_token: &str,
    specialist_id: &str,
    name_filter: Option<&str>,
) -> Result<Vec<Exercise>, String> {
    let path = format!(
        "/exercises?specialist_id=eq.{}&select=id,specialist_id,name,description,order_index,video_url,deleted_at,created_at&order=name.asc",
        specialist_id
    );
    let body = rest_get(config, Some(access_token), &path).await?;
    let rows: Vec<Exercise> = parse_json(&body)?;
    let filtered = if let Some(f) = name_filter {
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
    Ok(filtered)
}

/// Create exercise in the specialist's library (reusable).
pub async fn create_exercise(
    config: &SupabaseConfig,
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
    let body = rest_post(config, Some(access_token), "/exercises", &payload).await?;
    let rows: Vec<Exercise> = parse_json(&body)?;
    rows.into_iter().next().ok_or_else(|| "No row returned".to_string())
}

/// Add an exercise from the library to a workout.
pub async fn add_exercise_to_workout(
    config: &SupabaseConfig,
    access_token: &str,
    workout_id: &str,
    exercise_id: &str,
    order_index: i32,
) -> Result<(), String> {
    let payload = serde_json::json!({
        "workout_id": workout_id,
        "exercise_id": exercise_id,
        "order_index": order_index
    });
    rest_post(config, Some(access_token), "/workout_exercises", &payload).await?;
    Ok(())
}

/// Remove an exercise from a workout (does not delete the exercise from the library).
pub async fn remove_exercise_from_workout(
    config: &SupabaseConfig,
    access_token: &str,
    workout_id: &str,
    exercise_id: &str,
) -> Result<(), String> {
    let path = format!(
        "/workout_exercises?workout_id=eq.{}&exercise_id=eq.{}",
        workout_id, exercise_id
    );
    rest_request(config, Some(access_token), "DELETE", &path, None).await?;
    Ok(())
}

/// Update exercise (name, description, order_index, video_url). Omit fields to leave unchanged.
pub async fn update_exercise(
    config: &SupabaseConfig,
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
    rest_patch(config, Some(access_token), &path, &payload).await?;
    Ok(())
}

/// Soft delete: set deleted_at so the exercise is hidden from the editor but patients still see it.
pub async fn soft_delete_exercise(
    config: &SupabaseConfig,
    access_token: &str,
    exercise_id: &str,
) -> Result<(), String> {
    let payload = serde_json::json!({
        "deleted_at": chrono::Utc::now().to_rfc3339()
    });
    let path = format!("/exercises?id=eq.{}", exercise_id);
    rest_patch(config, Some(access_token), &path, &payload).await?;
    Ok(())
}

/// Restore a soft-deleted exercise (clear deleted_at).
pub async fn restore_exercise(
    config: &SupabaseConfig,
    access_token: &str,
    exercise_id: &str,
) -> Result<(), String> {
    let payload = serde_json::json!({ "deleted_at": serde_json::Value::Null });
    let path = format!("/exercises?id=eq.{}", exercise_id);
    rest_patch(config, Some(access_token), &path, &payload).await?;
    Ok(())
}

/// Build agenda as ordered list of days from program_schedule (no fixed duration; one entry per day slot).
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

/// Assign program to patient.
pub async fn assign_program_to_patient(
    config: &SupabaseConfig,
    access_token: &str,
    patient_id: &str,
    program_id: &str,
) -> Result<PatientProgram, String> {
    let payload = serde_json::json!({
        "patient_id": patient_id,
        "program_id": program_id,
        "status": "active"
    });
    let body = rest_post(config, Some(access_token), "/patient_programs", &payload).await?;
    let rows: Vec<PatientProgram> = parse_json(&body)?;
    rows.into_iter().next().ok_or_else(|| "No row returned".to_string())
}

/// List patient_programs for the specialist's patients (for compliance).
pub async fn list_patient_programs_for_specialist(
    config: &SupabaseConfig,
    access_token: &str,
) -> Result<Vec<PatientProgram>, String> {
    let body = rest_get(
        config,
        Some(access_token),
        "/patient_programs?select=id,patient_id,program_id,status,assigned_at,created_at,updated_at&order=assigned_at.desc",
    )
    .await?;
    let rows: Vec<PatientProgram> = parse_json(&body)?;
    Ok(rows)
}

/// List workout_sessions for a patient_program (by day_index order).
pub async fn list_workout_sessions(
    config: &SupabaseConfig,
    access_token: &str,
    patient_program_id: &str,
) -> Result<Vec<WorkoutSession>, String> {
    let path = format!(
        "/workout_sessions?patient_program_id=eq.{}&select=id,patient_program_id,day_index,session_date,completed_at,effort,pain,comment,created_at,updated_at&order=day_index.asc",
        patient_program_id
    );
    let body = rest_get(config, Some(access_token), &path).await?;
    let rows: Vec<WorkoutSession> = parse_json(&body)?;
    Ok(rows)
}

// -----------------------------------------------------------------------------
// Patient-side: active program, sessions, feedback
// -----------------------------------------------------------------------------

/// Get active patient_program for current patient.
pub async fn get_active_patient_program(
    config: &SupabaseConfig,
    access_token: &str,
) -> Result<Option<PatientProgram>, String> {
    let path = "/patient_programs?status=eq.active&select=id,patient_id,program_id,status,assigned_at,created_at,updated_at&limit=1";
    let body = rest_get(config, Some(access_token), path).await?;
    let rows: Vec<PatientProgram> = parse_json(&body)?;
    Ok(rows.into_iter().next())
}

/// Get program by id (patient or specialist).
pub async fn get_program(
    config: &SupabaseConfig,
    access_token: &str,
    program_id: &str,
) -> Result<Option<Program>, String> {
    let path = format!(
        "/programs?id=eq.{}&select=id,specialist_id,name,description,created_at,updated_at",
        program_id
    );
    let body = rest_get(config, Some(access_token), &path).await?;
    let rows: Vec<Program> = parse_json(&body)?;
    Ok(rows.into_iter().next())
}

/// Get or create a workout session for a program day (day_index). session_date defaults to today when creating.
pub async fn get_or_create_session(
    config: &SupabaseConfig,
    access_token: &str,
    patient_program_id: &str,
    day_index: i32,
    session_date: &str,
) -> Result<WorkoutSession, String> {
    let path = format!(
        "/workout_sessions?patient_program_id=eq.{}&day_index=eq.{}&select=id,patient_program_id,day_index,session_date,completed_at,effort,pain,comment,created_at,updated_at",
        patient_program_id, day_index
    );
    let body = rest_get(config, Some(access_token), &path).await?;
    let rows: Vec<WorkoutSession> = parse_json(&body)?;
    if let Some(s) = rows.into_iter().next() {
        return Ok(s);
    }
    let payload = serde_json::json!({
        "patient_program_id": patient_program_id,
        "day_index": day_index,
        "session_date": session_date
    });
    let body = rest_post(config, Some(access_token), "/workout_sessions", &payload).await?;
    let rows: Vec<WorkoutSession> = parse_json(&body)?;
    rows.into_iter().next().ok_or_else(|| "No row returned".to_string())
}

/// List all active program assignments for the current patient.
pub async fn list_active_patient_programs(
    config: &SupabaseConfig,
    access_token: &str,
) -> Result<Vec<PatientProgram>, String> {
    let path = "/patient_programs?status=eq.active&select=id,patient_id,program_id,status,assigned_at,created_at,updated_at&order=assigned_at.desc";
    let body = rest_get(config, Some(access_token), path).await?;
    let rows: Vec<PatientProgram> = parse_json(&body)?;
    Ok(rows)
}

/// Mark session completed and set feedback.
pub async fn complete_session(
    config: &SupabaseConfig,
    access_token: &str,
    session_id: &str,
    effort: Option<i32>,
    pain: Option<i32>,
    comment: Option<&str>,
) -> Result<(), String> {
    let payload = serde_json::json!({
        "completed_at": chrono::Utc::now().to_rfc3339(),
        "effort": effort,
        "pain": pain,
        "comment": comment
    });
    let path = format!("/workout_sessions?id=eq.{}", session_id);
    rest_patch(config, Some(access_token), &path, &payload).await?;
    Ok(())
}

/// Update feedback (effort, pain, comment, session_date) for an already completed session.
pub async fn update_session_feedback(
    config: &SupabaseConfig,
    access_token: &str,
    session_id: &str,
    effort: Option<i32>,
    pain: Option<i32>,
    comment: Option<&str>,
    session_date: Option<&str>,
) -> Result<(), String> {
    let mut payload = serde_json::json!({
        "effort": effort,
        "pain": pain,
        "comment": comment
    });
    if let Some(d) = session_date {
        payload["session_date"] = serde_json::Value::String(d.to_string());
    }
    let path = format!("/workout_sessions?id=eq.{}", session_id);
    rest_patch(config, Some(access_token), &path, &payload).await?;
    Ok(())
}

/// Mark a session as not completed (clears completed_at). Feedback fields are left as-is.
pub async fn uncomplete_session(
    config: &SupabaseConfig,
    access_token: &str,
    session_id: &str,
) -> Result<(), String> {
    let payload = serde_json::json!({ "completed_at": serde_json::Value::Null });
    let path = format!("/workout_sessions?id=eq.{}", session_id);
    rest_patch(config, Some(access_token), &path, &payload).await?;
    Ok(())
}
