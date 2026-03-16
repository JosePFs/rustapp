use std::sync::Arc;

use futures::try_join;

use crate::application::use_cases::agenda_schedule::build_agenda_schedule;
use crate::application::Backend;
use crate::domain::entities::{SessionExerciseFeedback, WorkoutExercise, WorkoutSession};
use crate::domain::error::{DomainError, Result};

#[derive(Clone, PartialEq)]
pub struct PatientWorkoutSessionArgs {
    pub token: String,
    pub patient_program_id: String,
    pub day_index: i32,
}

#[derive(Clone, PartialEq)]
pub struct PatientWorkoutSession {
    pub patient_program_id: String,
    pub program_name: String,
    pub workout_name: String,
    pub workout_description: Option<String>,
    pub day_index: i32,
    pub session: Option<WorkoutSession>,
    pub exercises: Vec<WorkoutExercise>,
    pub feedback: Vec<SessionExerciseFeedback>,
}

pub struct PatientWorkoutSessionUseCase<B: Backend> {
    backend: Arc<B>,
}

impl<B: Backend> PatientWorkoutSessionUseCase<B> {
    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(&self, args: PatientWorkoutSessionArgs) -> Result<PatientWorkoutSession> {
        let token = &args.token;

        let patient_program = self
            .backend
            .get_patient_program_by_id(token, &args.patient_program_id)
            .await?;

        let patient_program = match patient_program {
            Some(patient_program) if patient_program.is_active() => patient_program,
            _ => {
                return Err(DomainError::Api("error_no_active_assignment".to_string()));
            }
        };

        let program = self.backend.get_program(token, &patient_program.program_id);
        let schedule = self
            .backend
            .list_program_schedule(token, &patient_program.program_id);
        let workouts = self
            .backend
            .list_workouts_for_program(token, &patient_program.program_id);
        let sessions = self
            .backend
            .list_workout_sessions(token, &patient_program.id);

        let (program_opt, schedule, workouts, sessions) =
            try_join!(program, schedule, workouts, sessions)?;

        let program = match program_opt {
            Some(p) => p,
            None => return Err(DomainError::Api("error_program_not_found".to_string())),
        };

        let day_schedule = build_agenda_schedule(&schedule, &workouts);
        let (_, workout_id_opt, label) = day_schedule
            .iter()
            .find(|(i, _, _)| *i == args.day_index)
            .cloned()
            .ok_or(DomainError::Api("error_day_not_found".to_string()))?;

        let workout_id = match workout_id_opt {
            Some(id) => id,
            None => return Err(DomainError::Api("error_rest_day".to_string())),
        };

        let (workout_name, workout_description) = workouts
            .iter()
            .find(|w| w.id == workout_id)
            .map(|w| (w.name.clone(), w.description.clone()))
            .unwrap_or((label, None));

        let session_for_day = sessions.into_iter().find(|s| s.day_index == args.day_index);

        let exercises_fut = self.backend.list_exercises_for_workout(token, &workout_id);
        let feedback_fut = async {
            if let Some(ref workout_session) = session_for_day {
                self.backend
                    .list_session_exercise_feedback(token, &workout_session.id)
                    .await
            } else {
                Ok(Vec::<SessionExerciseFeedback>::new())
            }
        };

        let (exercises, feedback) = try_join!(exercises_fut, feedback_fut)?;

        Ok(PatientWorkoutSession {
            patient_program_id: patient_program.id,
            program_name: program.name,
            workout_name,
            workout_description,
            day_index: args.day_index,
            session: session_for_day,
            exercises,
            feedback,
        })
    }
}
