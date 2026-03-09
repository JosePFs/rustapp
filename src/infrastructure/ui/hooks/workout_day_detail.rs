use std::sync::Arc;

use futures::try_join;

use crate::application::ports::Backend;
use crate::domain::entities::{SessionExerciseFeedback, WorkoutExercise, WorkoutSession};
use crate::domain::error::{DomainError, Result};
use crate::domain::session::Session;
use crate::infrastructure::supabase::api::build_agenda_schedule;
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct WorkoutDayDetail {
    pub patient_program_id: String,
    pub program_name: String,
    pub workout_name: String,
    pub workout_description: Option<String>,
    pub day_index: i32,
    pub session: Option<WorkoutSession>,
    pub exercises: Vec<WorkoutExercise>,
    pub feedback: Vec<SessionExerciseFeedback>,
}

pub fn use_workout_day_detail(
    app_session: Option<Session>,
    backend: Arc<dyn Backend>,
    patient_program_id: String,
    day_index: i32,
) -> Resource<Result<WorkoutDayDetail>> {
    use_resource(move || {
        let app_session = app_session.clone();
        let backend = backend.clone();
        let patient_program_id = patient_program_id.clone();

        async move {
            let sess = app_session.ok_or(DomainError::SessionNotFound)?;
            let token = sess.access_token();

            let patient_program = backend
                .get_patient_program_by_id(token, &patient_program_id)
                .await?;
            let patient_program = match patient_program {
                Some(patient_program) if patient_program.is_active() => patient_program,
                _ => {
                    return Err(DomainError::Api(
                        "No se encuentra la asignación activa".to_string(),
                    ));
                }
            };

            let program = backend.get_program(token, &patient_program.program_id);
            let schedule = backend.list_program_schedule(token, &patient_program.program_id);
            let workouts = backend.list_workouts_for_program(token, &patient_program.program_id);
            let sessions = backend.list_workout_sessions(token, &patient_program.id);

            let (program_opt, schedule, workouts, sessions) =
                try_join!(program, schedule, workouts, sessions)?;

            let program = match program_opt {
                Some(p) => p,
                None => return Err(DomainError::Api("Programa no encontrado".to_string())),
            };

            let day_schedule = build_agenda_schedule(&schedule, &workouts);
            let (_, workout_id_opt, label) = day_schedule
                .iter()
                .find(|(i, _, _)| *i == day_index)
                .cloned()
                .ok_or(DomainError::Api(
                    "Día no encontrado en la programación".to_string(),
                ))?;

            let workout_id = match workout_id_opt {
                Some(id) => id,
                None => {
                    return Err(DomainError::Api(
                        "Este día es de descanso (sin entrenamiento)".to_string(),
                    ))
                }
            };

            let (workout_name, workout_description) = workouts
                .iter()
                .find(|w| w.id == workout_id)
                .map(|w| (w.name.clone(), w.description.clone()))
                .unwrap_or((label, None));

            let session_for_day = sessions.into_iter().find(|s| s.day_index == day_index);

            let exercises_fut = backend.list_exercises_for_workout(token, &workout_id);
            let feedback_fut = async {
                if let Some(ref workout_session) = session_for_day {
                    backend
                        .list_session_exercise_feedback(token, &workout_session.id)
                        .await
                } else {
                    Ok(Vec::<SessionExerciseFeedback>::new())
                }
            };

            let (exercises, feedback) = try_join!(exercises_fut, feedback_fut)?;

            Ok(WorkoutDayDetail {
                patient_program_id: patient_program.id,
                program_name: program.name,
                workout_name,
                workout_description,
                day_index,
                session: session_for_day,
                exercises,
                feedback,
            })
        }
    })
}
