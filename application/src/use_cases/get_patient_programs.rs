use std::collections::HashMap;
use std::sync::Arc;

use futures::stream::{self, StreamExt};

use crate::ports::error::{ApplicationError, Result};
use crate::use_cases::agenda_schedule::build_agenda_schedule;
use domain::aggregates::PatientProgramFull;
use domain::entities::SessionExerciseFeedback;
use domain::repositories::{GetPatientProgramFullRead, ListActivePatientProgramsRead};
use domain::vos::id::Id;

#[derive(Clone, PartialEq)]
pub struct ExerciseInstruction {
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
pub struct ProgramDay {
    pub session_id: Option<String>,
    pub day_index: i32,
    pub day_number: i32,
    pub workout_name: Option<String>,
    pub workout_description: Option<String>,
    pub is_rest_day: bool,
    pub session_date: Option<String>,
    pub completed_at: Option<String>,
    pub exercises: Vec<ExerciseInstruction>,
}

#[derive(Clone, PartialEq)]
pub struct PatientProgram {
    pub patient_program_id: String,
    pub program_id: String,
    pub program_name: String,
    pub program_description: Option<String>,
    pub days: Vec<ProgramDay>,
    pub progress_percent: i32,
    pub average_effort: Option<f32>,
    pub average_pain: Option<f32>,
}

#[derive(Clone, PartialEq)]
pub struct GetPatientProgramsUseCaseResult {
    pub patient_programs: Vec<PatientProgram>,
}

pub struct GetPatientProgramsUseCase<R: GetPatientProgramFullRead + ListActivePatientProgramsRead> {
    catalog_read: Arc<R>,
}

impl<R: GetPatientProgramFullRead + ListActivePatientProgramsRead> GetPatientProgramsUseCase<R> {
    const MAX_CONCURRENT_PROGRAM_REQUESTS: usize = 4;

    pub fn new(catalog_read: Arc<R>) -> Self {
        Self { catalog_read }
    }

    pub async fn execute(&self) -> Result<GetPatientProgramsUseCaseResult> {
        let patient_programs = self
            .catalog_read
            .list_active_patient_programs()
            .await
            .map_err(ApplicationError::from)?;
        let patient_programs_data = stream::iter(patient_programs.into_iter().enumerate())
            .map(|(order_index, ass)| {
                let catalog_read = self.catalog_read.clone();

                async move {
                    let full = catalog_read
                        .get_patient_program_full(&ass.id)
                        .await
                        .map_err(ApplicationError::from)?;

                    let Some(full) = full else {
                        return Ok(None);
                    };

                    Result::Ok(Some((order_index, Self::build_program(full))))
                }
            })
            .buffer_unordered(Self::MAX_CONCURRENT_PROGRAM_REQUESTS)
            .collect::<Vec<Result<Option<(usize, PatientProgram)>>>>()
            .await;

        Ok(GetPatientProgramsUseCaseResult {
            patient_programs: {
                let mut programs: Vec<(usize, PatientProgram)> = patient_programs_data
                    .into_iter()
                    .filter_map(|result| match result {
                        Ok(Some(value)) => Some(value),
                        _ => None,
                    })
                    .collect();

                programs.sort_by_key(|(order, _)| *order);
                programs.into_iter().map(|(_, program)| program).collect()
            },
        })
    }

    fn build_program(full: PatientProgramFull) -> PatientProgram {
        let workout_exercises: HashMap<Id, _> = full
            .workouts
            .iter()
            .map(|w| (w.workout.id.clone(), w.exercises.clone()))
            .collect();

        let workouts: Vec<_> = full.workouts.iter().map(|w| w.workout.clone()).collect();

        let days: Vec<ProgramDay> = build_agenda_schedule(&full.schedule, &workouts)
            .into_iter()
            .map(|(day_index, workout_id_opt, label)| {
                let session = full
                    .sessions
                    .iter()
                    .find(|session| session.day_index == day_index);
                let feedback_for_day: Vec<&SessionExerciseFeedback> = session
                    .map(|session| {
                        full.feedback
                            .iter()
                            .filter(|entry| entry.workout_session_id == session.id)
                            .collect()
                    })
                    .unwrap_or_default();

                let workout_id_opt: Option<Id> = workout_id_opt
                    .as_ref()
                    .map(|s| Id::try_from(s.as_str()).unwrap());

                let (session_id, workout_name, workout_description, exercises, is_rest_day) =
                    match workout_id_opt {
                        Some(workout_id) => {
                            let workout = workouts.iter().find(|workout| workout.id == workout_id);
                            let exercises = workout_exercises
                                .get(&workout_id)
                                .cloned()
                                .unwrap_or_default()
                                .into_iter()
                                .map(|exercise| {
                                    let existing_feedback = feedback_for_day
                                        .iter()
                                        .find(|entry| entry.exercise_id == exercise.exercise.id);
                                    ExerciseInstruction {
                                        exercise_id: exercise.exercise.id.value().to_string(),
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
                                session.map(|session| session.id.value().to_string()),
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
                        None => (None, None, None, Vec::new(), true),
                    };

                ProgramDay {
                    session_id,
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

        let total_training_days = days.iter().filter(|day| !day.is_rest_day).count() as f32;
        let completed_training_days = days
            .iter()
            .filter(|day| !day.is_rest_day && day.completed_at.is_some())
            .count() as f32;

        let progress_percent = if total_training_days > 0.0 {
            ((completed_training_days / total_training_days) * 100.0).round() as i32
        } else {
            0
        };

        let mut effort_sum = 0_i32;
        let mut effort_count = 0_i32;
        let mut pain_sum = 0_i32;
        let mut pain_count = 0_i32;

        for fb in &full.feedback {
            if let Some(e) = fb.effort {
                effort_sum += e;
                effort_count += 1;
            }
            if let Some(p) = fb.pain {
                pain_sum += p;
                pain_count += 1;
            }
        }

        let average_effort = if effort_count > 0 {
            Some(effort_sum as f32 / effort_count as f32)
        } else {
            None
        };

        let average_pain = if pain_count > 0 {
            Some(pain_sum as f32 / pain_count as f32)
        } else {
            None
        };

        PatientProgram {
            patient_program_id: full.patient_program.id.value().to_string(),
            program_id: full.patient_program.program_id.value().to_string(),
            program_name: full.program.name.clone(),
            program_description: full.program.description.clone(),
            days,
            progress_percent,
            average_effort,
            average_pain,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Mutex;

    use domain::entities::PatientProgram;
    use domain::error::Result;

    #[tokio::test]
    async fn get_patient_programs_empty_when_no_active_programs() {
        let fake = MockGetPatientProgramsRead::new(Ok(vec![]));
        let uc = GetPatientProgramsUseCase::new(Arc::new(fake.clone()));

        let res = uc.execute().await.unwrap();

        assert!(res.patient_programs.is_empty());
        assert_eq!(*fake.full_calls.lock().unwrap(), 0);
    }

    #[derive(Clone)]
    struct MockGetPatientProgramsRead {
        active: Arc<Mutex<Result<Vec<PatientProgram>>>>,
        full_calls: Arc<Mutex<usize>>,
    }

    impl MockGetPatientProgramsRead {
        fn new(active: Result<Vec<PatientProgram>>) -> Self {
            Self {
                active: Arc::new(Mutex::new(active)),
                full_calls: Arc::new(Mutex::new(0)),
            }
        }
    }

    #[common::async_trait_platform]
    impl ListActivePatientProgramsRead for MockGetPatientProgramsRead {
        async fn list_active_patient_programs(&self) -> Result<Vec<PatientProgram>> {
            self.active.lock().unwrap().clone()
        }
    }

    #[common::async_trait_platform]
    impl GetPatientProgramFullRead for MockGetPatientProgramsRead {
        async fn get_patient_program_full(
            &self,
            _patient_program_id: &Id,
        ) -> Result<Option<PatientProgramFull>> {
            *self.full_calls.lock().unwrap() += 1;
            Ok(None)
        }
    }
}
