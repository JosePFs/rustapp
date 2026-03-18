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

typedef SubmitDayFeedbackCallback =
    Future<void> Function(rust_api.SubmitDayFeedbackRequest request);
typedef UpdateDayCompletionCallback =
    Future<void> Function(rust_api.UpdateDayCompletionRequest request);

