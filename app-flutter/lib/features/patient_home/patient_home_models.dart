import '../../src/rust/api.dart' as rust_api;

class ExerciseFeedbackDraft {
  ExerciseFeedbackDraft({
    required this.effort,
    required this.pain,
    required this.comment,
  });

  int effort;
  int pain;
  String comment;
}

typedef MarkDayAsCompletedCallback =
    Future<void> Function(rust_api.MarkDayAsCompletedRequest request);
typedef MarkDayAsUncompletedCallback =
    Future<void> Function(rust_api.MarkDayAsUncompletedRequest request);
