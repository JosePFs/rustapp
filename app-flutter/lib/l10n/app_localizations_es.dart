// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Spanish Castilian (`es`).
class AppLocalizationsEs extends AppLocalizations {
  AppLocalizationsEs([String locale = 'es']) : super(locale);

  @override
  String get appTitle => 'Eixe';

  @override
  String get commonFollowDevice => 'Usar el idioma del dispositivo';

  @override
  String get commonLanguage => 'Idioma';

  @override
  String get commonEnglish => 'Inglés';

  @override
  String get commonSpanish => 'Español';

  @override
  String get commonGalician => 'Gallego';

  @override
  String get authSignIn => 'Iniciar sesión';

  @override
  String get authEmailLabel => 'Correo';

  @override
  String get authPasswordLabel => 'Contraseña';

  @override
  String get authEmailHint => 'nombre@ejemplo.com';

  @override
  String get authPasswordHint => 'Tu contraseña';

  @override
  String get authLoginFailedTitle => 'No se ha podido iniciar sesión';

  @override
  String get authLoginFailedWrongCredentials =>
      'Correo o contraseña incorrectos.';

  @override
  String get authLoginFailedGeneric => 'Inténtalo de nuevo.';

  @override
  String get statusStartingApp => 'Iniciando la aplicación…';

  @override
  String get statusInitializingBridge => 'Inicializando el bridge de Rust…';

  @override
  String get statusBridgeReady => 'Bridge listo. Ya puedes iniciar sesión.';

  @override
  String get statusCallingLogin => 'Iniciando sesión…';

  @override
  String get statusSavingFeedback => 'Guardando feedback…';

  @override
  String get statusFeedbackSaved => 'Feedback guardado.';

  @override
  String get statusUpdatingSessionState => 'Actualizando estado de la sesión…';

  @override
  String get statusSessionMarkedCompleted => 'Rutina marcada como completada.';

  @override
  String get statusSessionSavedAsCompleted =>
      'Rutina guardada como completada.';

  @override
  String get statusSessionMarkedNotCompleted =>
      'Rutina marcada como no completada.';

  @override
  String get statusSignedOut =>
      'Sesión cerrada. Puedes iniciar sesión de nuevo.';

  @override
  String get bootstrapWelcomeBack => 'Bienvenido/a';

  @override
  String get bootstrapContinue => 'Continuar';

  @override
  String get bootstrapRetry => 'Reintentar';

  @override
  String get bootstrapUnableToStartTitle => 'No se puede iniciar la aplicación';

  @override
  String get errorMissingSupabaseConfig =>
      'Falta la configuración de Supabase. Pasa SUPABASE_URL y SUPABASE_ANON_KEY con --dart-define.';

  @override
  String errorBridgeInitFailed(Object error) {
    return 'Error al inicializar el bridge: $error';
  }

  @override
  String errorRustCallFailed(Object error) {
    return 'La llamada a Rust falló: $error';
  }

  @override
  String statusSignedInLoadedPrograms(String profileType, int count) {
    return 'Sesión iniciada como $profileType. $count programa(s) cargado(s).';
  }

  @override
  String get patientHomeSignOut => 'Cerrar sesión';

  @override
  String get patientHomeNoProgramsTitle => 'No hay programas asignados';

  @override
  String get patientHomeNoProgramsSubtitle =>
      'Tu especialista todavía no te ha asignado programas.';

  @override
  String get patientHomeNoProgramsBody =>
      'Si crees que es un error, contacta con tu especialista.';

  @override
  String get programsYourProgramsTitle => 'Tus programas';

  @override
  String programsAssignedCount(int count) {
    return '$count asignado(s)';
  }

  @override
  String get programsNoDescription => 'Sin descripción.';

  @override
  String get programsProgressLabel => 'Progreso';

  @override
  String programsProgressPercent(int percent) {
    return '$percent%';
  }

  @override
  String programsEffortPainSummary(String effort, String pain) {
    return 'Esfuerzo: $effort / 10 · Dolor: $pain / 10';
  }

  @override
  String get programDetailSelectProgram =>
      'Selecciona un programa para ver los detalles.';

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
  String get programDetailCompletionDateLabel => 'Fecha de realización';

  @override
  String get programDetailSave => 'Guardar';

  @override
  String get programDetailSaveAsCompleted => 'Guardar como completado';

  @override
  String get programDetailMarkAsNotCompleted => 'Marcar como no completado';

  @override
  String exerciseSetsReps(int sets, int reps) {
    return '$sets series · $reps repeticiones';
  }

  @override
  String get exerciseEffortLabel => 'Esfuerzo';

  @override
  String get exercisePainLabel => 'Dolor';

  @override
  String get exerciseCommentLabel => 'Comentario (opcional)';

  @override
  String get exerciseCommentHint =>
      'Añade notas sobre dolor, dificultad o cualquier otra cosa…';

  @override
  String get exerciseVideoPlaceholder => 'Vídeo del ejercicio';
}
