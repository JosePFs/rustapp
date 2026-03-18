// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Galician (`gl`).
class AppLocalizationsGl extends AppLocalizations {
  AppLocalizationsGl([String locale = 'gl']) : super(locale);

  @override
  String get appTitle => 'Eixe';

  @override
  String get commonFollowDevice => 'Usar o idioma do dispositivo';

  @override
  String get commonLanguage => 'Idioma';

  @override
  String get commonEnglish => 'Inglés';

  @override
  String get commonSpanish => 'Castelán';

  @override
  String get commonGalician => 'Galego';

  @override
  String get authSignIn => 'Iniciar sesión';

  @override
  String get authEmailLabel => 'Correo';

  @override
  String get authPasswordLabel => 'Contrasinal';

  @override
  String get authEmailHint => 'nome@exemplo.com';

  @override
  String get authPasswordHint => 'O teu contrasinal';

  @override
  String get authLoginFailedTitle => 'Non se puido iniciar sesión';

  @override
  String get authLoginFailedWrongCredentials =>
      'Correo ou contrasinal incorrectos.';

  @override
  String get authLoginFailedGeneric => 'Inténtao de novo.';

  @override
  String get statusStartingApp => 'Iniciando a aplicación…';

  @override
  String get statusInitializingBridge => 'Inicializando a ponte de Rust…';

  @override
  String get statusBridgeReady => 'Ponte lista. Xa podes iniciar sesión.';

  @override
  String get statusCallingLogin => 'Iniciando sesión…';

  @override
  String get statusSavingFeedback => 'Gardando feedback…';

  @override
  String get statusFeedbackSaved => 'Feedback gardado.';

  @override
  String get statusUpdatingSessionState => 'Actualizando o estado da sesión…';

  @override
  String get statusSessionMarkedCompleted => 'Rutina marcada como completada.';

  @override
  String get statusSessionSavedAsCompleted => 'Rutina gardada como completada.';

  @override
  String get statusSessionMarkedNotCompleted =>
      'Rutina marcada como non completada.';

  @override
  String get statusSignedOut => 'Sesión pechada. Podes iniciar sesión de novo.';

  @override
  String get bootstrapWelcomeBack => 'Benvido/a';

  @override
  String get bootstrapContinue => 'Continuar';

  @override
  String get bootstrapRetry => 'Reintentar';

  @override
  String get bootstrapUnableToStartTitle => 'Non se pode iniciar a aplicación';

  @override
  String get errorMissingSupabaseConfig =>
      'Falta a configuración de Supabase. Pasa SUPABASE_URL e SUPABASE_ANON_KEY con --dart-define.';

  @override
  String errorBridgeInitFailed(Object error) {
    return 'Erro ao inicializar a ponte: $error';
  }

  @override
  String errorRustCallFailed(Object error) {
    return 'A chamada a Rust fallou: $error';
  }

  @override
  String statusSignedInLoadedPrograms(String profileType, int count) {
    return 'Sesión iniciada como $profileType. $count programa(s) cargado(s).';
  }

  @override
  String get patientHomeSignOut => 'Pechar sesión';

  @override
  String get patientHomeNoProgramsTitle => 'Non hai programas asignados';

  @override
  String get patientHomeNoProgramsSubtitle =>
      'O teu especialista aínda non che asignou programas.';

  @override
  String get patientHomeNoProgramsBody =>
      'Se cres que é un erro, contacta co teu especialista.';

  @override
  String get programsYourProgramsTitle => 'Os teus programas';

  @override
  String programsAssignedCount(int count) {
    return '$count asignado(s)';
  }

  @override
  String get programsNoDescription => 'Sen descrición.';

  @override
  String get programsProgressLabel => 'Progreso';

  @override
  String programsProgressPercent(int percent) {
    return '$percent%';
  }

  @override
  String programsEffortPainSummary(String effort, String pain) {
    return 'Esforzo: $effort / 10 · Dor: $pain / 10';
  }

  @override
  String get programDetailSelectProgram =>
      'Selecciona un programa para ver os detalles.';

  @override
  String get programDetailSelectDay => 'Selecciona un día';

  @override
  String programDetailRestDayLabel(int dayNumber) {
    return 'Día $dayNumber • Descanso';
  }

  @override
  String programDetailDayLabel(int dayNumber) {
    return 'Día $dayNumber';
  }

  @override
  String get programDetailCompletionDateLabel => 'Data de realización';

  @override
  String get programDetailSave => 'Gardar';

  @override
  String get programDetailSaveAsCompleted => 'Gardar como completado';

  @override
  String get programDetailMarkAsNotCompleted => 'Marcar como non completado';

  @override
  String exerciseSetsReps(int sets, int reps) {
    return '$sets series · $reps repeticións';
  }

  @override
  String get exerciseEffortLabel => 'Esforzo';

  @override
  String get exercisePainLabel => 'Dor';

  @override
  String get exerciseCommentLabel => 'Comentario (opcional)';

  @override
  String get exerciseCommentHint =>
      'Engade notas sobre dor, dificultade ou calquera outra cousa…';

  @override
  String get exerciseVideoPlaceholder => 'Vídeo do exercicio';
}
