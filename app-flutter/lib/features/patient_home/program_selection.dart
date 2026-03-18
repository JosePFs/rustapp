import '../../src/rust/api.dart' as rust_api;

int? choosePreferredTrainingDayIndex(List<rust_api.ProgramDaySummary> days) {
  final trainingDays = days.where((day) => !day.isRestDay).toList();
  if (trainingDays.isEmpty) {
    return null;
  }

  for (final day in trainingDays) {
    if (day.completedAt == null) {
      return day.dayIndex;
    }
  }

  return trainingDays.last.dayIndex;
}

