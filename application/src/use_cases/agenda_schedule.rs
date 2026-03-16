use crate::domain::entities::{ProgramScheduleItem, Workout};

/// Build agenda as ordered list of days from program schedule.
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
            .unwrap_or_else(|| "Rest".to_string());
        for _ in 0..item.days_count.max(1) {
            out.push((day_index, item.workout_id.clone(), label.clone()));
            day_index += 1;
        }
    }
    out
}
