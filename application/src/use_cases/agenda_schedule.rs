use std::collections::HashMap;

use domain::entities::{ProgramScheduleItem, Workout};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramScheduleRow {
    pub workout_id: Option<String>,
    pub days_count: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorkoutSummaryRow {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgendaWorkoutSession {
    pub id: String,
    pub day_index: i32,
    pub session_date: String,
    pub completed_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AgendaSessionFeedback {
    pub workout_session_id: String,
    pub exercise_id: String,
    pub effort: Option<i32>,
    pub pain: Option<i32>,
    pub comment: Option<String>,
}

pub fn build_agenda_schedule_rows(
    schedule: &[ProgramScheduleRow],
    workouts: &[WorkoutSummaryRow],
) -> Vec<(i32, Option<String>, String)> {
    let workout_names: HashMap<&str, String> = workouts
        .iter()
        .map(|w| (w.id.as_str(), w.name.clone()))
        .collect();
    let mut out = Vec::new();
    let mut day_index = 0i32;
    for item in schedule {
        let label = item
            .workout_id
            .as_deref()
            .and_then(|id| workout_names.get(id).cloned())
            .unwrap_or_else(|| "Rest".to_string());
        let maybe_workout_id = item.workout_id.clone();
        for _ in 0..item.days_count.max(1) {
            out.push((day_index, maybe_workout_id.clone(), label.clone()));
            day_index += 1;
        }
    }
    out
}

pub fn build_agenda_schedule(
    schedule: &[ProgramScheduleItem],
    workouts: &[Workout],
) -> Vec<(i32, Option<String>, String)> {
    let rows: Vec<ProgramScheduleRow> = schedule
        .iter()
        .map(|item| ProgramScheduleRow {
            workout_id: item.workout_id.as_ref().map(|id| id.to_string()),
            days_count: item.days_count,
        })
        .collect();
    let wrows: Vec<WorkoutSummaryRow> = workouts
        .iter()
        .map(|w| WorkoutSummaryRow {
            id: w.id.to_string(),
            name: w.name.clone(),
        })
        .collect();
    build_agenda_schedule_rows(&rows, &wrows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_agenda_schedule_rows_expands_days_and_rest_label() {
        let schedule = vec![
            ProgramScheduleRow {
                workout_id: Some("w1".to_string()),
                days_count: 2,
            },
            ProgramScheduleRow {
                workout_id: None,
                days_count: 1,
            },
        ];
        let workouts = vec![WorkoutSummaryRow {
            id: "w1".to_string(),
            name: "Leg day".to_string(),
        }];

        let rows = build_agenda_schedule_rows(&schedule, &workouts);

        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0], (0, Some("w1".to_string()), "Leg day".to_string()));
        assert_eq!(rows[1], (1, Some("w1".to_string()), "Leg day".to_string()));
        assert_eq!(rows[2], (2, None, "Rest".to_string()));
    }

    #[test]
    fn build_agenda_schedule_rows_treats_non_positive_days_as_one() {
        let schedule = vec![ProgramScheduleRow {
            workout_id: None,
            days_count: 0,
        }];

        let rows = build_agenda_schedule_rows(&schedule, &[]);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0, 0);
    }
}
