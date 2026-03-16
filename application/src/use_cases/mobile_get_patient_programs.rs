use std::sync::Arc;

use futures::stream::{self, StreamExt};
use futures::try_join;

use crate::application::MobileBackend;
use crate::application::use_cases::agenda_schedule::build_agenda_schedule;
use crate::application::use_cases::get_patient_programs::GetPatientProgramsUseCaseArgs;
use crate::domain::error::Result;
use crate::domain::entities::{SessionExerciseFeedback, WorkoutExercise};

#[derive(Clone, PartialEq)]
pub struct MobileExerciseInstruction {
    pub exercise_id: String,
    pub name: String,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub sets: i32,
    pub reps: i32,
    pub effort: Option<i32>,
    pub pain: Option<i32>,
    pub comment: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct MobileProgramDay {
    pub day_index: i32,
    pub day_number: i32,
    pub workout_name: Option<String>,
    pub workout_description: Option<String>,
    pub is_rest_day: bool,
    pub session_date: Option<String>,
    pub completed_at: Option<String>,
    pub exercises: Vec<MobileExerciseInstruction>,
}

#[derive(Clone, PartialEq)]
pub struct MobilePatientProgram {
    pub patient_program_id: String,
    pub program_id: String,
    pub program_name: String,
    pub program_description: Option<String>,
    pub days: Vec<MobileProgramDay>,
}

#[derive(Clone, PartialEq)]
pub struct MobileGetPatientProgramsUseCaseResult {
    pub patient_programs: Vec<MobilePatientProgram>,
}

pub struct MobileGetPatientProgramsUseCase<B: MobileBackend> {
    backend: Arc<B>,
}

impl<B: MobileBackend> MobileGetPatientProgramsUseCase<B> {
    const MAX_CONCURRENT_PROGRAM_REQUESTS: usize = 4;

    pub fn new(backend: Arc<B>) -> Self {
        Self { backend }
    }

    pub async fn execute(
        &self,
        args: GetPatientProgramsUseCaseArgs,
    ) -> Result<MobileGetPatientProgramsUseCaseResult> {
        let patient_programs = self
            .backend
            .list_active_patient_programs(&args.token)
            .await?;

        let patient_programs_data = stream::iter(patient_programs.into_iter())
            .map(|ass| {
                let backend = self.backend.clone();
                let token = args.token.clone();

                async move {
                    let (prog, workouts, schedule, sessions, feedback) = try_join!(
                        backend.get_program(&token, &ass.program_id),
                        backend.list_workouts_for_program(&token, &ass.program_id),
                        backend.list_program_schedule(&token, &ass.program_id),
                        backend.list_workout_sessions(&token, &ass.id),
                        backend.list_session_exercise_feedback_for_program(&token, &ass.id),
                    )?;

                    let prog = match prog {
                        Some(p) => p,
                        None => return Ok(None),
                    };

                    let workout_exercises = stream::iter(workouts.iter().cloned())
                        .map(|workout| {
                            let backend = backend.clone();
                            let token = token.clone();

                            async move {
                                let exercises =
                                    backend.list_exercises_for_workout(&token, &workout.id).await?;
                                Ok::<(String, Vec<WorkoutExercise>), crate::domain::error::DomainError>((
                                    workout.id,
                                    exercises,
                                ))
                            }
                        })
                        .buffer_unordered(Self::MAX_CONCURRENT_PROGRAM_REQUESTS)
                        .collect::<Vec<Result<(String, Vec<WorkoutExercise>)>>>()
                        .await
                        .into_iter()
                        .collect::<Result<Vec<(String, Vec<WorkoutExercise>)>>>()?;

                    let workout_exercises = workout_exercises
                        .into_iter()
                        .collect::<std::collections::HashMap<String, Vec<WorkoutExercise>>>();

                    let days = build_agenda_schedule(&schedule, &workouts)
                        .into_iter()
                        .map(|(day_index, workout_id_opt, label)| {
                            let session = sessions.iter().find(|session| session.day_index == day_index);
                            let feedback_for_day: Vec<&SessionExerciseFeedback> = session
                                .map(|session| {
                                    feedback
                                        .iter()
                                        .filter(|entry| entry.workout_session_id == session.id)
                                        .collect()
                                })
                                .unwrap_or_default();

                            let (workout_name, workout_description, exercises, is_rest_day) =
                                match workout_id_opt.as_ref() {
                                    Some(workout_id) => {
                                        let workout = workouts.iter().find(|workout| workout.id == *workout_id);
                                        let exercises = workout_exercises
                                            .get(workout_id)
                                            .cloned()
                                            .unwrap_or_default()
                                            .into_iter()
                                            .map(|exercise| {
                                                let existing_feedback = feedback_for_day
                                                    .iter()
                                                    .find(|entry| entry.exercise_id == exercise.exercise.id);
                                                MobileExerciseInstruction {
                                                    exercise_id: exercise.exercise.id.clone(),
                                                    name: exercise.exercise.name.clone(),
                                                    description: exercise.exercise.description.clone(),
                                                    video_url: exercise.exercise.video_url.clone(),
                                                    sets: exercise.sets,
                                                    reps: exercise.reps,
                                                    effort: existing_feedback.and_then(|entry| entry.effort),
                                                    pain: existing_feedback.and_then(|entry| entry.pain),
                                                    comment: existing_feedback
                                                        .and_then(|entry| entry.comment.clone()),
                                                }
                                            })
                                            .collect();

                                        (
                                            Some(
                                                workout
                                                    .map(|workout| workout.name.clone())
                                                    .unwrap_or(label.clone()),
                                            ),
                                            workout.and_then(|workout| workout.description.clone()),
                                            exercises,
                                            false,
                                        )
                                    }
                                    None => (None, None, Vec::new(), true),
                                };

                            MobileProgramDay {
                                day_index,
                                day_number: day_index + 1,
                                workout_name,
                                workout_description,
                                is_rest_day,
                                session_date: session.map(|session| session.session_date.clone()),
                                completed_at: session.and_then(|session| session.completed_at.clone()),
                                exercises,
                            }
                        })
                        .collect();

                    Ok(Some(MobilePatientProgram {
                        patient_program_id: ass.id.clone(),
                        program_id: ass.program_id.clone(),
                        program_name: prog.name,
                        program_description: prog.description,
                        days,
                    }))
                }
            })
            .buffer_unordered(Self::MAX_CONCURRENT_PROGRAM_REQUESTS)
            .collect::<Vec<Result<Option<MobilePatientProgram>>>>()
            .await;

        Ok(MobileGetPatientProgramsUseCaseResult {
            patient_programs: patient_programs_data
                .into_iter()
                .filter_map(|result| match result {
                    Ok(Some(value)) => Some(value),
                    _ => None,
                })
                .collect(),
        })
    }
}
