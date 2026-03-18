// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for English (`en`).
class AppLocalizationsEn extends AppLocalizations {
  AppLocalizationsEn([String locale = 'en']) : super(locale);

  @override
  String get appTitle => 'Eixe';

  @override
  String get commonFollowDevice => 'Follow device';

  @override
  String get commonLanguage => 'Language';

  @override
  String get commonEnglish => 'English';

  @override
  String get commonSpanish => 'Spanish';

  @override
  String get commonGalician => 'Galician';

  @override
  String get authSignIn => 'Sign in';

  @override
  String get authEmailLabel => 'Email';

  @override
  String get authPasswordLabel => 'Password';

  @override
  String get authEmailHint => 'name@example.com';

  @override
  String get authPasswordHint => 'Your password';

  @override
  String get authLoginFailedTitle => 'Could not sign in';

  @override
  String get authLoginFailedWrongCredentials => 'Incorrect email or password.';

  @override
  String get authLoginFailedGeneric => 'Please try again.';

  @override
  String get statusStartingApp => 'Starting app…';

  @override
  String get statusInitializingBridge => 'Initializing Rust bridge…';

  @override
  String get statusBridgeReady => 'Bridge ready. You can sign in now.';

  @override
  String get statusCallingLogin => 'Calling Rust login…';

  @override
  String get statusSavingFeedback => 'Saving feedback…';

  @override
  String get statusFeedbackSaved => 'Feedback saved.';

  @override
  String get statusUpdatingSessionState => 'Updating session state…';

  @override
  String get statusSessionMarkedCompleted => 'Session marked as completed.';

  @override
  String get statusSessionSavedAsCompleted => 'Session saved as completed.';

  @override
  String get statusSessionMarkedNotCompleted =>
      'Session marked as not completed.';

  @override
  String get statusSignedOut => 'Signed out. You can sign in again.';

  @override
  String get bootstrapWelcomeBack => 'Welcome back';

  @override
  String get bootstrapContinue => 'Continue';

  @override
  String get bootstrapRetry => 'Retry';

  @override
  String get bootstrapUnableToStartTitle => 'Unable to start the app';

  @override
  String get errorMissingSupabaseConfig =>
      'Missing Supabase configuration. Pass SUPABASE_URL and SUPABASE_ANON_KEY with --dart-define.';

  @override
  String errorBridgeInitFailed(Object error) {
    return 'Bridge initialization failed: $error';
  }

  @override
  String errorRustCallFailed(Object error) {
    return 'Rust call failed: $error';
  }

  @override
  String statusSignedInLoadedPrograms(String profileType, int count) {
    return 'Signed in as $profileType. Loaded $count program(s).';
  }

  @override
  String get patientHomeSignOut => 'Sign out';

  @override
  String get patientHomeNoProgramsTitle => 'No programs assigned';

  @override
  String get patientHomeNoProgramsSubtitle =>
      'Your specialist has not assigned any programs yet.';

  @override
  String get patientHomeNoProgramsBody =>
      'If you think this is an error, contact your specialist.';

  @override
  String get programsYourProgramsTitle => 'Your programs';

  @override
  String programsAssignedCount(int count) {
    return '$count assigned';
  }

  @override
  String get programsNoDescription => 'No description available.';

  @override
  String get programsProgressLabel => 'Progress';

  @override
  String programsProgressPercent(int percent) {
    return '$percent%';
  }

  @override
  String programsEffortPainSummary(String effort, String pain) {
    return 'Effort: $effort / 10 · Pain: $pain / 10';
  }

  @override
  String get programDetailSelectProgram =>
      'Select a program to see its details.';

  @override
  String get programDetailSelectDay => 'Select a day';

  @override
  String programDetailRestDayLabel(int dayNumber) {
    return 'Day $dayNumber • Rest day';
  }

  @override
  String programDetailDayLabel(int dayNumber) {
    return 'Day $dayNumber';
  }

  @override
  String get programDetailCompletionDateLabel => 'Completion date';

  @override
  String get programDetailSave => 'Save';

  @override
  String get programDetailSaveAsCompleted => 'Save as completed';

  @override
  String get programDetailMarkAsNotCompleted => 'Mark as not completed';

  @override
  String exerciseSetsReps(int sets, int reps) {
    return '$sets sets · $reps reps';
  }

  @override
  String get exerciseEffortLabel => 'Effort';

  @override
  String get exercisePainLabel => 'Pain';

  @override
  String get exerciseCommentLabel => 'Comment (optional)';

  @override
  String get exerciseCommentHint =>
      'Add notes about pain, difficulty, or anything else…';

  @override
  String get exerciseVideoPlaceholder => 'Exercise video';
}
