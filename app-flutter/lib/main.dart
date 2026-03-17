import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:webview_flutter_android/webview_flutter_android.dart';
import 'package:webview_flutter_wkwebview/webview_flutter_wkwebview.dart';

import 'src/rust/api.dart' as rust_api;
import 'src/rust/frb_generated.dart';

extension _IterableFirstOrNull<E> on Iterable<E> {
  E? get firstOrNull => isEmpty ? null : first;
}

enum ExerciseVideoRenderMode { placeholder, youtubeIframe }

ExerciseVideoRenderMode chooseExerciseVideoRenderMode({
  required bool isTestEnv,
  required bool isAndroid,
  required bool isIos,
  required bool isLinux,
}) {
  if (isTestEnv) {
    return ExerciseVideoRenderMode.placeholder;
  }
  if (isAndroid || isIos) {
    return ExerciseVideoRenderMode.youtubeIframe;
  }
  return ExerciseVideoRenderMode.placeholder;
}

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

String? extractYouTubeVideoId(String url) {
  final trimmed = url.trim();
  if (trimmed.isEmpty) {
    return null;
  }

  String? normalizedId(String? candidate) {
    if (candidate == null) {
      return null;
    }
    final match = RegExp(r'[_\-a-zA-Z0-9]{11}').firstMatch(candidate.trim());
    return match?.group(0);
  }

  final fromQuery = RegExp(r'[?&]v=([^&#?/]+)').firstMatch(trimmed);
  final queryId = normalizedId(fromQuery?.group(1));
  if (queryId != null) {
    return queryId;
  }

  final fromEmbed = RegExp(r'/embed/([^&#?/]+)').firstMatch(trimmed);
  final embedId = normalizedId(fromEmbed?.group(1));
  if (embedId != null) {
    return embedId;
  }

  final fromShort = RegExp(r'youtu\.be/([^&#?/]+)').firstMatch(trimmed);
  final shortId = normalizedId(fromShort?.group(1));
  if (shortId != null) {
    return shortId;
  }

  final fromPath = RegExp(r'/(shorts|live)/([^&#?/]+)').firstMatch(trimmed);
  final pathId = normalizedId(fromPath?.group(2));
  if (pathId != null) {
    return pathId;
  }

  final lastSegment = normalizedId(
    trimmed.split('/').last.split('?').first.split('&').first,
  );
  if (lastSegment != null) {
    return lastSegment;
  }

  final fallbackId = normalizedId(trimmed);
  if (fallbackId != null) {
    return fallbackId;
  }

  return null;
}

String? buildYouTubeEmbedUrl(String url) {
  final videoId = extractYouTubeVideoId(url);
  if (videoId == null) {
    return null;
  }

  return 'https://www.youtube.com/embed/$videoId';
}

Uri? buildExternalVideoLaunchUri(String url) {
  final trimmed = url.trim();
  if (trimmed.isEmpty) {
    return null;
  }

  final withScheme = trimmed.contains('://') ? trimmed : 'https://$trimmed';
  final uri = Uri.tryParse(withScheme);
  if (uri == null || !uri.hasScheme || uri.host.isEmpty) {
    return null;
  }

  return uri;
}

Future<void> openExternalVideoUrl(String url) async {
  final uri = buildExternalVideoLaunchUri(url);
  if (uri == null) {
    throw const FormatException('Invalid video URL.');
  }

  final launched = await launchUrl(uri, mode: LaunchMode.externalApplication);
  if (!launched) {
    throw StateError('Unable to open external video URL.');
  }
}

bool shouldOpenEmbeddedVideoNavigationExternally(String url) {
  final uri = buildExternalVideoLaunchUri(url);
  if (uri == null) {
    return false;
  }

  final scheme = uri.scheme.toLowerCase();
  if (scheme != 'http' && scheme != 'https') {
    return true;
  }

  final host = uri.host.toLowerCase();
  final path = uri.path.toLowerCase();
  final isYouTubeHost =
      host == 'youtube.com' ||
      host == 'www.youtube.com' ||
      host == 'm.youtube.com' ||
      host == 'youtu.be';
  if (!isYouTubeHost) {
    return false;
  }

  final isEmbedPath = path.startsWith('/embed/');
  return !isEmbedPath;
}

String buildEmbeddedYouTubeHtml(String embedUrl) {
  final playerUrl = Uri.parse(embedUrl).replace(
    queryParameters: <String, String>{
      'playsinline': '1',
      'rel': '0',
      'modestbranding': '1',
    },
  );

  return '''
<!DOCTYPE html>
<html>
  <head>
    <meta name="referrer" content="strict-origin-when-cross-origin">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0">
    <style>
      html, body {
        margin: 0;
        padding: 0;
        height: 100%;
        background: #000;
        overflow: hidden;
      }
      iframe {
        border: 0;
        width: 100%;
        height: 100%;
      }
    </style>
  </head>
  <body>
    <iframe
      src="$playerUrl"
      allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
      allowfullscreen
      referrerpolicy="strict-origin-when-cross-origin">
    </iframe>
  </body>
</html>
''';
}

String completionButtonLabel({
  required bool isCompleted,
  required bool isBusy,
}) {
  if (isBusy) {
    return 'Saving...';
  }

  if (isCompleted) {
    return 'Save';
  }

  return 'Save as completed';
}

typedef _SelectedTrainingDay = ({
  rust_api.PatientProgramSummary program,
  rust_api.ProgramDaySummary day,
});

_SelectedTrainingDay? _pickSelectedTrainingDay({
  required List<rust_api.PatientProgramSummary> programs,
  required String? selectedProgramId,
  required int? selectedDayIndex,
}) {
  final selectedProgram = programs
      .where((program) => program.patientProgramId == selectedProgramId)
      .firstOrNull;
  final selectedDay = selectedProgram?.days
      .where((day) => day.dayIndex == selectedDayIndex)
      .firstOrNull;

  if (selectedProgram == null || selectedDay == null || selectedDay.isRestDay) {
    return null;
  }

  return (program: selectedProgram, day: selectedDay);
}

rust_api.SubmitDayFeedbackRequest _buildSubmitDayFeedbackRequest({
  required rust_api.PatientProgramSummary program,
  required rust_api.ProgramDaySummary day,
  required Map<String, _ExerciseFeedbackDraft> feedbackDrafts,
  required String Function(String, int, String) exerciseKeyFor,
  required String sessionDate,
}) {
  return rust_api.SubmitDayFeedbackRequest(
    patientProgramId: program.patientProgramId,
    dayIndex: day.dayIndex,
    sessionDate: sessionDate,
    feedback: day.exercises.map((exercise) {
      final key = exerciseKeyFor(
        program.patientProgramId,
        day.dayIndex,
        exercise.exerciseId,
      );
      final draft = feedbackDrafts[key]!;
      return rust_api.ExerciseFeedbackInput(
        exerciseId: exercise.exerciseId,
        effort: draft.effort,
        pain: draft.pain,
        comment: draft.comment.isEmpty ? null : draft.comment,
      );
    }).toList(),
  );
}

rust_api.UpdateDayCompletionRequest _buildUpdateDayCompletionRequest({
  required rust_api.PatientProgramSummary program,
  required rust_api.ProgramDaySummary day,
  required String sessionDate,
  required bool completed,
}) {
  return rust_api.UpdateDayCompletionRequest(
    patientProgramId: program.patientProgramId,
    dayIndex: day.dayIndex,
    sessionDate: sessionDate,
    completed: completed,
  );
}

String _defaultSessionDateForDay(rust_api.ProgramDaySummary day) =>
    day.sessionDate ?? DateTime.now().toIso8601String().split('T').first;

void main() {
  runApp(MyApp(bridgeConfig: BridgeRuntimeConfig.fromEnvironment()));
}

class MyApp extends StatelessWidget {
  const MyApp({
    required this.bridgeConfig,
    this.autoInitializeBridge = true,
    super.key,
  });

  final BridgeRuntimeConfig bridgeConfig;
  final bool autoInitializeBridge;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Eixe Patient Front',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: const Color(0xFF35B339)),
        filledButtonTheme: FilledButtonThemeData(
          style: FilledButton.styleFrom(
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(8),
            ),
          ),
        ),
        outlinedButtonTheme: OutlinedButtonThemeData(
          style: OutlinedButton.styleFrom(
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(8),
            ),
          ),
        ),
        textButtonTheme: TextButtonThemeData(
          style: TextButton.styleFrom(
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(8),
            ),
          ),
        ),
      ),
      home: PatientAppBootstrapPage(
        bridgeConfig: bridgeConfig,
        autoInitializeBridge: autoInitializeBridge,
      ),
    );
  }
}

class BridgeRuntimeConfig {
  const BridgeRuntimeConfig({
    required this.supabaseUrl,
    required this.supabaseAnonKey,
  });

  const BridgeRuntimeConfig.fromEnvironment()
    : supabaseUrl = const String.fromEnvironment('SUPABASE_URL'),
      supabaseAnonKey = const String.fromEnvironment('SUPABASE_ANON_KEY');

  final String supabaseUrl;
  final String supabaseAnonKey;

  bool get isConfigured =>
      supabaseUrl.trim().isNotEmpty && supabaseAnonKey.trim().isNotEmpty;

  rust_api.BridgeConfig toBridgeConfig() => rust_api.BridgeConfig(
    url: supabaseUrl.trim(),
    anonKey: supabaseAnonKey.trim(),
  );
}

enum _BootstrapStage { starting, readyForLogin, error }

typedef SubmitDayFeedbackCallback =
    Future<void> Function(rust_api.SubmitDayFeedbackRequest request);
typedef UpdateDayCompletionCallback =
    Future<void> Function(rust_api.UpdateDayCompletionRequest request);

class _ExerciseFeedbackDraft {
  _ExerciseFeedbackDraft({
    required this.effort,
    required this.pain,
    required this.comment,
  });

  int effort;
  int pain;
  String comment;
}

class PatientHomePage extends StatefulWidget {
  const PatientHomePage({
    required this.loginResponse,
    required this.patientPrograms,
    this.onSignOut,
    this.onSubmitDayFeedback,
    this.onUpdateDayCompletion,
    super.key,
  });

  final rust_api.LoginResponse loginResponse;
  final List<rust_api.PatientProgramSummary> patientPrograms;
  final VoidCallback? onSignOut;
  final SubmitDayFeedbackCallback? onSubmitDayFeedback;
  final UpdateDayCompletionCallback? onUpdateDayCompletion;

  @override
  State<PatientHomePage> createState() => _PatientHomePageState();
}

class _PatientHomePageState extends State<PatientHomePage> {
  String? _selectedProgramId;
  int? _selectedDayIndex;
  bool _submittingFeedback = false;
  final Map<String, _ExerciseFeedbackDraft> _feedbackDrafts = {};
  final Map<String, String> _completionDateDrafts = {};
  final Map<String, TextEditingController> _commentControllers = {};
  final _completionDateController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _syncSelection();
  }

  @override
  void didUpdateWidget(covariant PatientHomePage oldWidget) {
    super.didUpdateWidget(oldWidget);
    _syncSelection();
  }

  @override
  void dispose() {
    for (final controller in _commentControllers.values) {
      controller.dispose();
    }
    _completionDateController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final selectedProgram = widget.patientPrograms
        .where((program) => program.patientProgramId == _selectedProgramId)
        .cast<rust_api.PatientProgramSummary?>()
        .firstOrNull;
    final selectedDay = selectedProgram?.days
        .where((day) => day.dayIndex == _selectedDayIndex)
        .cast<rust_api.ProgramDaySummary?>()
        .firstOrNull;

    return Scaffold(
      appBar: AppBar(
        title: const Text('Eixe Patient Front'),
        actions: [
          if (widget.onSignOut != null)
            TextButton(
              onPressed: widget.onSignOut,
              child: const Text('Sign out'),
            ),
        ],
      ),
      body: LayoutBuilder(
        builder: (context, constraints) {
          final isWide = constraints.maxWidth >= 900;
          final listPanel = _ProgramListPanel(
            programs: widget.patientPrograms,
            selectedProgramId: _selectedProgramId,
            onProgramSelected: _selectProgram,
          );
          final detailPanel = _ProgramDetailPanel(
            selectedProgram: selectedProgram,
            selectedDay: selectedDay,
            submittingFeedback: _submittingFeedback,
            completionDateController: _completionDateController,
            onDaySelected: _selectDay,
            onPickCompletionDate: _pickCompletionDate,
            feedbackDraftForExercise: _draftForExercise,
            commentControllerForExercise: _commentControllerForExercise,
            onEffortChanged: (exerciseKey, value) {
              setState(() {
                _feedbackDrafts[exerciseKey]?.effort = value;
              });
            },
            onPainChanged: (exerciseKey, value) {
              setState(() {
                _feedbackDrafts[exerciseKey]?.pain = value;
              });
            },
            onCommentChanged: (exerciseKey, value) {
              _feedbackDrafts[exerciseKey]?.comment = value;
            },
            onSaveDay: _saveSelectedDay,
            onMarkNotCompleted: () => _updateSelectedDayCompletion(false),
          );

          return ListView(
            padding: const EdgeInsets.all(8),
            children: [
              if (widget.patientPrograms.isEmpty)
                Card(
                  child: Padding(
                    padding: const EdgeInsets.all(8),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          'No programs assigned',
                          style: theme.textTheme.titleLarge,
                        ),
                        const SizedBox(height: 8),
                        const Text(
                          'Your specialist has not assigned any programs yet.',
                        ),
                      ],
                    ),
                  ),
                )
              else if (isWide)
                SizedBox(
                  height: 420,
                  child: Row(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Expanded(flex: 3, child: listPanel),
                      const SizedBox(width: 16),
                      Expanded(flex: 4, child: detailPanel),
                    ],
                  ),
                )
              else
                Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    listPanel,
                    const SizedBox(height: 16),
                    detailPanel,
                  ],
                ),
            ],
          );
        },
      ),
    );
  }

  void _syncSelection() {
    if (widget.patientPrograms.isEmpty) {
      _selectedProgramId = null;
      _selectedDayIndex = null;
      return;
    }

    final selectedProgramExists = widget.patientPrograms.any(
      (program) => program.patientProgramId == _selectedProgramId,
    );
    if (!selectedProgramExists) {
      _selectedProgramId = widget.patientPrograms.isNotEmpty
          ? widget.patientPrograms.first.patientProgramId
          : null;
    }

    final selectedProgram = widget.patientPrograms.firstWhere(
      (program) => program.patientProgramId == _selectedProgramId,
      orElse: () => widget.patientPrograms.first,
    );

    final selectedDay = selectedProgram.days
        .where((day) => day.dayIndex == _selectedDayIndex)
        .firstOrNull;
    final preferredDayIndex = choosePreferredTrainingDayIndex(
      selectedProgram.days,
    );
    final selectedDayExists = selectedDay != null && !selectedDay.isRestDay;
    if (!selectedDayExists ||
        (selectedDay.completedAt != null &&
            preferredDayIndex != null &&
            preferredDayIndex != selectedDay.dayIndex)) {
      _selectedDayIndex = preferredDayIndex;
    }

    _syncCompletionDateControllerForCurrentSelection();
  }

  _ExerciseFeedbackDraft _draftForExercise(
    String programId,
    int dayIndex,
    rust_api.ExerciseInstructionSummary exercise,
  ) {
    final key = _exerciseKey(programId, dayIndex, exercise.exerciseId);
    return _feedbackDrafts.putIfAbsent(
      key,
      () => _ExerciseFeedbackDraft(
        effort: (exercise.effort ?? 1).clamp(1, 10),
        pain: (exercise.pain ?? 0).clamp(0, 10),
        comment: exercise.comment ?? '',
      ),
    );
  }

  TextEditingController _commentControllerForExercise(
    String programId,
    int dayIndex,
    rust_api.ExerciseInstructionSummary exercise,
  ) {
    final key = _exerciseKey(programId, dayIndex, exercise.exerciseId);
    final draft = _draftForExercise(programId, dayIndex, exercise);
    return _commentControllers.putIfAbsent(
      key,
      () => TextEditingController(text: draft.comment),
    );
  }

  Future<void> _saveSelectedDay() async {
    final selection = _pickSelectedTrainingDay(
      programs: widget.patientPrograms,
      selectedProgramId: _selectedProgramId,
      selectedDayIndex: _selectedDayIndex,
    );
    if (selection == null || widget.onSubmitDayFeedback == null) {
      return;
    }
    final selectedProgram = selection.program;
    final selectedDay = selection.day;
    final isCompleted = selectedDay.completedAt != null;
    final sessionDate = _selectedCompletionDateForDay(
      selectedProgram.patientProgramId,
      selectedDay,
    );

    setState(() {
      _submittingFeedback = true;
    });

    try {
      await widget.onSubmitDayFeedback!(
        _buildSubmitDayFeedbackRequest(
          program: selectedProgram,
          day: selectedDay,
          feedbackDrafts: _feedbackDrafts,
          exerciseKeyFor: _exerciseKey,
          sessionDate: sessionDate,
        ),
      );

      if (!isCompleted && widget.onUpdateDayCompletion != null) {
        await widget.onUpdateDayCompletion!(
          _buildUpdateDayCompletionRequest(
            program: selectedProgram,
            day: selectedDay,
            sessionDate: sessionDate,
            completed: true,
          ),
        );
      }

      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              isCompleted ? 'Changes saved.' : 'Saved as completed.',
            ),
          ),
        );
      }
    } catch (error) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Unable to save changes: $error')),
        );
      }
    } finally {
      if (mounted) {
        setState(() {
          _submittingFeedback = false;
        });
      }
    }
  }

  Future<void> _updateSelectedDayCompletion(bool completed) async {
    final selection = _pickSelectedTrainingDay(
      programs: widget.patientPrograms,
      selectedProgramId: _selectedProgramId,
      selectedDayIndex: _selectedDayIndex,
    );
    if (selection == null || widget.onUpdateDayCompletion == null) {
      return;
    }
    final selectedProgram = selection.program;
    final selectedDay = selection.day;

    setState(() {
      _submittingFeedback = true;
    });

    try {
      await widget.onUpdateDayCompletion!(
        _buildUpdateDayCompletionRequest(
          program: selectedProgram,
          day: selectedDay,
          sessionDate: _selectedCompletionDateForDay(
            selectedProgram.patientProgramId,
            selectedDay,
          ),
          completed: completed,
        ),
      );
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(
              completed ? 'Marked as completed.' : 'Marked as not completed.',
            ),
          ),
        );
      }
    } catch (error) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Unable to update session state: $error')),
        );
      }
    } finally {
      if (mounted) {
        setState(() {
          _submittingFeedback = false;
        });
      }
    }
  }

  String _exerciseKey(String programId, int dayIndex, String exerciseId) =>
      '$programId::$dayIndex::$exerciseId';

  String _sessionDateForDay(rust_api.ProgramDaySummary day) =>
      _defaultSessionDateForDay(day);

  String _completionDateKey(String programId, int dayIndex) =>
      '$programId::$dayIndex::completion-date';

  String _selectedCompletionDateForDay(
    String programId,
    rust_api.ProgramDaySummary day,
  ) {
    final key = _completionDateKey(programId, day.dayIndex);
    return _completionDateDrafts.putIfAbsent(
      key,
      () => _sessionDateForDay(day),
    );
  }

  void _syncCompletionDateControllerForCurrentSelection() {
    final selectedProgram = widget.patientPrograms
        .where((program) => program.patientProgramId == _selectedProgramId)
        .firstOrNull;
    final selectedDay = selectedProgram?.days
        .where((day) => day.dayIndex == _selectedDayIndex)
        .firstOrNull;
    if (selectedProgram == null || selectedDay == null) {
      if (_completionDateController.text.isNotEmpty) {
        _completionDateController.clear();
      }
      return;
    }

    final date = _selectedCompletionDateForDay(
      selectedProgram.patientProgramId,
      selectedDay,
    );
    if (_completionDateController.text != date) {
      _completionDateController.text = date;
    }
  }

  void _selectProgram(String programId) {
    final program = widget.patientPrograms.firstWhere(
      (program) => program.patientProgramId == programId,
    );
    setState(() {
      _selectedProgramId = programId;
      _selectedDayIndex = choosePreferredTrainingDayIndex(program.days);
      _syncCompletionDateControllerForCurrentSelection();
    });
  }

  void _selectDay(int dayIndex) {
    setState(() {
      _selectedDayIndex = dayIndex;
      _syncCompletionDateControllerForCurrentSelection();
    });
  }

  Future<void> _pickCompletionDate() async {
    final selectedProgram = widget.patientPrograms
        .where((program) => program.patientProgramId == _selectedProgramId)
        .firstOrNull;
    final selectedDay = selectedProgram?.days
        .where((day) => day.dayIndex == _selectedDayIndex)
        .firstOrNull;
    if (selectedProgram == null || selectedDay == null) {
      return;
    }

    final currentDate = DateTime.tryParse(
      _selectedCompletionDateForDay(
        selectedProgram.patientProgramId,
        selectedDay,
      ),
    );
    final initialDate = currentDate ?? DateTime.now();
    final pickedDate = await showDatePicker(
      context: context,
      initialDate: initialDate,
      firstDate: DateTime(2020),
      lastDate: DateTime(2100),
    );
    if (pickedDate == null || !mounted) {
      return;
    }

    final formattedDate = pickedDate.toIso8601String().split('T').first;
    setState(() {
      _completionDateDrafts[_completionDateKey(
            selectedProgram.patientProgramId,
            selectedDay.dayIndex,
          )] =
          formattedDate;
      _completionDateController.text = formattedDate;
    });
  }
}

class _ProgramListPanel extends StatelessWidget {
  const _ProgramListPanel({
    required this.programs,
    required this.selectedProgramId,
    required this.onProgramSelected,
  });

  final List<rust_api.PatientProgramSummary> programs;
  final String? selectedProgramId;
  final ValueChanged<String> onProgramSelected;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Card(
      color: theme.colorScheme.surfaceVariant,
      child: Padding(
        padding: const EdgeInsets.all(8),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Padding(
              padding: const EdgeInsets.all(8),
              child: Row(
                children: [
                  Text('Your programs', style: theme.textTheme.titleLarge),
                  const Spacer(),
                  Text(
                    '${programs.length} assigned',
                    style: theme.textTheme.bodyMedium,
                  ),
                ],
              ),
            ),
            for (final program in programs)
              Card(
                elevation: program.patientProgramId == selectedProgramId
                    ? 2
                    : 0,
                color: theme.colorScheme.surface,
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(8),
                  side: BorderSide(
                    color: program.patientProgramId == selectedProgramId
                        ? theme.colorScheme.primary.withOpacity(0.4)
                        : theme.dividerColor.withOpacity(0.2),
                  ),
                ),
                child: ListTile(
                  selected: program.patientProgramId == selectedProgramId,
                  title: Text(program.programName),
                  subtitle: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        program.programDescription ??
                            'No description available.',
                      ),
                      const SizedBox(height: 8),
                      LinearProgressIndicator(
                        value: (program.progressPercent.clamp(0, 100) / 100)
                            .toDouble(),
                      ),
                      const SizedBox(height: 4),
                      Text(
                        'Progress: ${program.progressPercent.clamp(0, 100)}%',
                        style: theme.textTheme.bodySmall,
                      ),
                      if (program.averageEffort != null ||
                          program.averagePain != null)
                        Text(
                          'Effort: ${program.averageEffort?.toStringAsFixed(1) ?? '-'} / 10 · '
                          'Pain: ${program.averagePain?.toStringAsFixed(1) ?? '-'} / 10',
                          style: theme.textTheme.bodySmall,
                        ),
                    ],
                  ),
                  onTap: () => onProgramSelected(program.patientProgramId),
                ),
              ),
          ],
        ),
      ),
    );
  }
}

class _ProgramDetailPanel extends StatelessWidget {
  const _ProgramDetailPanel({
    required this.selectedProgram,
    required this.selectedDay,
    required this.submittingFeedback,
    required this.completionDateController,
    required this.onDaySelected,
    required this.onPickCompletionDate,
    required this.feedbackDraftForExercise,
    required this.commentControllerForExercise,
    required this.onEffortChanged,
    required this.onPainChanged,
    required this.onCommentChanged,
    required this.onSaveDay,
    required this.onMarkNotCompleted,
  });

  final rust_api.PatientProgramSummary? selectedProgram;
  final rust_api.ProgramDaySummary? selectedDay;
  final bool submittingFeedback;
  final TextEditingController completionDateController;
  final ValueChanged<int> onDaySelected;
  final Future<void> Function() onPickCompletionDate;
  final _ExerciseFeedbackDraft Function(
    String programId,
    int dayIndex,
    rust_api.ExerciseInstructionSummary exercise,
  )
  feedbackDraftForExercise;
  final TextEditingController Function(
    String programId,
    int dayIndex,
    rust_api.ExerciseInstructionSummary exercise,
  )
  commentControllerForExercise;
  final void Function(String exerciseKey, int value) onEffortChanged;
  final void Function(String exerciseKey, int value) onPainChanged;
  final void Function(String exerciseKey, String value) onCommentChanged;
  final Future<void> Function() onSaveDay;
  final Future<void> Function() onMarkNotCompleted;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final isCompleted = selectedDay?.completedAt != null;

    return Padding(
      padding: const EdgeInsets.all(8),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (selectedProgram == null)
            const Text('Select a program to see its details.')
          else ...[
            Text(
              selectedProgram!.programName,
              style: theme.textTheme.headlineSmall,
            ),
            const SizedBox(height: 8),
            Text(
              selectedProgram!.programDescription ??
                  'No description available.',
            ),
            const SizedBox(height: 24),
            if (selectedProgram!.days.isEmpty)
              const Text('No training days available.')
            else
              Builder(
                builder: (context) {
                  final days = selectedProgram!.days;

                  return GridView.builder(
                    physics: const NeverScrollableScrollPhysics(),
                    shrinkWrap: true,
                    padding: EdgeInsets.zero,
                    gridDelegate:
                        const SliverGridDelegateWithFixedCrossAxisCount(
                          crossAxisCount: 2,
                          mainAxisSpacing: 4,
                          crossAxisSpacing: 4,
                          childAspectRatio: 6,
                        ),
                    itemCount: days.length,
                    itemBuilder: (context, index) {
                      final day = days[index];
                      final isSelected = selectedDay?.dayIndex == day.dayIndex;
                      if (day.isRestDay) {
                        return Container(
                          alignment: Alignment.centerLeft,
                          padding: const EdgeInsets.symmetric(horizontal: 8),
                          child: Text(
                            'Day ${day.dayNumber} • Rest',
                            style: theme.textTheme.bodySmall?.copyWith(
                              color: theme.textTheme.bodySmall?.color
                                  ?.withOpacity(0.6),
                            ),
                          ),
                        );
                      }

                      return InkWell(
                        onTap: () => onDaySelected(day.dayIndex),
                        borderRadius: BorderRadius.circular(8),
                        child: Container(
                          decoration: BoxDecoration(
                            borderRadius: BorderRadius.circular(8),
                            color: isSelected
                                ? theme.colorScheme.primary.withOpacity(0.08)
                                : Colors.transparent,
                            border: Border.all(
                              color: isSelected
                                  ? theme.colorScheme.primary.withOpacity(0.6)
                                  : theme.dividerColor.withOpacity(0.3),
                            ),
                          ),
                          padding: const EdgeInsets.symmetric(horizontal: 8),
                          alignment: Alignment.centerLeft,
                          child: Row(
                            children: [
                              if (day.completedAt != null)
                                Icon(
                                  Icons.check,
                                  size: 16,
                                  color: theme.colorScheme.primary,
                                ),
                              if (day.completedAt != null)
                                const SizedBox(width: 4),
                              Expanded(
                                child: Column(
                                  crossAxisAlignment: CrossAxisAlignment.start,
                                  mainAxisSize: MainAxisSize.min,
                                  children: [
                                    Text(
                                      'Day ${day.dayNumber}',
                                      style: theme.textTheme.bodyMedium,
                                    ),
                                    if (day.workoutName != null)
                                      Text(
                                        day.workoutName!,
                                        style: theme.textTheme.bodySmall,
                                        maxLines: 1,
                                        overflow: TextOverflow.ellipsis,
                                      ),
                                  ],
                                ),
                              ),
                            ],
                          ),
                        ),
                      );
                    },
                  );
                },
              ),
            if (selectedDay != null) ...[
              const SizedBox(height: 24),
              Text(
                'Day ${selectedDay!.dayNumber}',
                style: theme.textTheme.titleLarge,
              ),
              const SizedBox(height: 8),
              Builder(
                builder: (_) {
                  final isCompleted = selectedDay!.completedAt != null;
                  final statusText = isCompleted ? 'Completed' : 'Planned';
                  final date = isCompleted ? selectedDay!.sessionDate : null;
                  final text = date != null
                      ? '$date • $statusText'
                      : statusText;
                  return Text(text);
                },
              ),
              const SizedBox(height: 16),
              if (selectedDay!.isRestDay)
                const Text('Rest day. No feedback needed.')
              else ...[
                Text(
                  selectedDay!.workoutName ?? 'Workout',
                  style: theme.textTheme.titleMedium,
                ),
                if (selectedDay!.workoutDescription != null) ...[
                  const SizedBox(height: 4),
                  Text(selectedDay!.workoutDescription!),
                ],
                const SizedBox(height: 16),
                if (selectedDay!.exercises.isEmpty)
                  const Text('No exercises available.')
                else
                  ...selectedDay!.exercises.map(
                    (exercise) => _ExerciseFeedbackCard(
                      programId: selectedProgram!.patientProgramId,
                      dayIndex: selectedDay!.dayIndex,
                      exercise: exercise,
                      draft: feedbackDraftForExercise(
                        selectedProgram!.patientProgramId,
                        selectedDay!.dayIndex,
                        exercise,
                      ),
                      commentController: commentControllerForExercise(
                        selectedProgram!.patientProgramId,
                        selectedDay!.dayIndex,
                        exercise,
                      ),
                      onEffortChanged: onEffortChanged,
                      onPainChanged: onPainChanged,
                      onCommentChanged: onCommentChanged,
                    ),
                  ),
                const SizedBox(height: 24),
                TextFormField(
                  key: const Key('completion-date-field'),
                  controller: completionDateController,
                  readOnly: true,
                  decoration: InputDecoration(
                    labelText: 'Completion date',
                    border: const OutlineInputBorder(),
                    suffixIcon: IconButton(
                      onPressed: submittingFeedback
                          ? null
                          : onPickCompletionDate,
                      icon: const Icon(Icons.calendar_today),
                    ),
                  ),
                  onTap: submittingFeedback ? null : onPickCompletionDate,
                ),
                const SizedBox(height: 24),
                Wrap(
                  spacing: 16,
                  runSpacing: 16,
                  children: [
                    FilledButton(
                      style: FilledButton.styleFrom(
                        padding: const EdgeInsets.symmetric(
                          horizontal: 20,
                          vertical: 20,
                        ),
                      ),
                      onPressed: submittingFeedback ? null : onSaveDay,
                      child: Text(
                        completionButtonLabel(
                          isCompleted: isCompleted,
                          isBusy: submittingFeedback,
                        ),
                      ),
                    ),
                    if (isCompleted)
                      OutlinedButton(
                        style: OutlinedButton.styleFrom(
                          padding: const EdgeInsets.symmetric(
                            horizontal: 20,
                            vertical: 20,
                          ),
                        ),
                        onPressed: submittingFeedback
                            ? null
                            : onMarkNotCompleted,
                        child: const Text('Mark as not completed'),
                      ),
                  ],
                ),
                const SizedBox(height: 36),
              ],
            ],
          ],
        ],
      ),
    );
  }
}

class _ExerciseFeedbackCard extends StatelessWidget {
  const _ExerciseFeedbackCard({
    required this.programId,
    required this.dayIndex,
    required this.exercise,
    required this.draft,
    required this.commentController,
    required this.onEffortChanged,
    required this.onPainChanged,
    required this.onCommentChanged,
  });

  final String programId;
  final int dayIndex;
  final rust_api.ExerciseInstructionSummary exercise;
  final _ExerciseFeedbackDraft draft;
  final TextEditingController commentController;
  final void Function(String exerciseKey, int value) onEffortChanged;
  final void Function(String exerciseKey, int value) onPainChanged;
  final void Function(String exerciseKey, String value) onCommentChanged;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final exerciseKey = '$programId::$dayIndex::${exercise.exerciseId}';

    return Card(
      color: Colors.transparent,
      elevation: 0,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(8),
        side: BorderSide(color: theme.dividerColor.withOpacity(0.4)),
      ),
      margin: const EdgeInsets.only(bottom: 16),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              exercise.name,
              style: theme.textTheme.titleMedium?.copyWith(
                fontWeight: FontWeight.w600,
              ),
            ),
            if (exercise.description != null) ...[
              const SizedBox(height: 8),
              Text(
                exercise.description!,
                style: theme.textTheme.bodyMedium?.copyWith(
                  color: theme.textTheme.bodySmall?.color,
                ),
              ),
            ],
            const SizedBox(height: 16),
            Text(
              '${exercise.sets} sets • ${exercise.reps} reps',
              style: theme.textTheme.bodyMedium?.copyWith(
                fontWeight: FontWeight.w500,
              ),
            ),
            if (exercise.videoUrl != null) ...[
              const SizedBox(height: 8),
              _ExerciseVideoPanel(
                key: Key('exercise-video-${exercise.exerciseId}'),
                exerciseId: exercise.exerciseId,
                videoUrl: exercise.videoUrl!,
              ),
            ],
            const SizedBox(height: 16),
            Text('Effort: ${draft.effort}/10'),
            Slider(
              value: draft.effort.toDouble(),
              min: 1,
              max: 10,
              divisions: 9,
              label: draft.effort.toString(),
              onChanged: (value) {
                onEffortChanged(exerciseKey, value.round());
              },
            ),
            Text('Pain: ${draft.pain}/10'),
            Slider(
              value: draft.pain.toDouble(),
              min: 0,
              max: 10,
              divisions: 10,
              label: draft.pain.toString(),
              onChanged: (value) {
                onPainChanged(exerciseKey, value.round());
              },
            ),
            const SizedBox(height: 16),
            TextField(
              controller: commentController,
              minLines: 2,
              maxLines: 5,
              decoration: InputDecoration(
                labelText: 'Comment (optional)',
                enabledBorder: OutlineInputBorder(
                  borderRadius: const BorderRadius.all(Radius.circular(8)),
                  borderSide: BorderSide(
                    color: theme.dividerColor.withOpacity(0.4),
                  ),
                ),
                focusedBorder: OutlineInputBorder(
                  borderRadius: const BorderRadius.all(Radius.circular(8)),
                  borderSide: BorderSide(
                    color: theme.colorScheme.primary,
                    width: 1.4,
                  ),
                ),
              ),
              onChanged: (value) => onCommentChanged(exerciseKey, value),
            ),
            const SizedBox(height: 16),
          ],
        ),
      ),
    );
  }
}

class _ExerciseVideoPanel extends StatefulWidget {
  const _ExerciseVideoPanel({
    required super.key,
    required this.exerciseId,
    required this.videoUrl,
  });

  final String exerciseId;
  final String videoUrl;

  @override
  State<_ExerciseVideoPanel> createState() => _ExerciseVideoPanelState();
}

class _ExerciseVideoPanelState extends State<_ExerciseVideoPanel> {
  late final String? _embedUrl;
  late final ExerciseVideoRenderMode _renderMode;
  late final Widget? _webViewWidget;

  @override
  void initState() {
    super.initState();
    _embedUrl = buildYouTubeEmbedUrl(widget.videoUrl);
    _renderMode = chooseExerciseVideoRenderMode(
      isTestEnv: Platform.environment.containsKey('FLUTTER_TEST'),
      isAndroid: Platform.isAndroid,
      isIos: Platform.isIOS,
      isLinux: Platform.isLinux,
    );
    if (_renderMode == ExerciseVideoRenderMode.youtubeIframe &&
        _embedUrl != null) {
      PlatformWebViewControllerCreationParams controllerParams =
          const PlatformWebViewControllerCreationParams();
      if (!kIsWeb && Platform.isAndroid) {
        controllerParams =
            AndroidWebViewControllerCreationParams.fromPlatformWebViewControllerCreationParams(
              controllerParams,
            );
      } else if (!kIsWeb && Platform.isIOS) {
        controllerParams =
            WebKitWebViewControllerCreationParams.fromPlatformWebViewControllerCreationParams(
              controllerParams,
              allowsInlineMediaPlayback: true,
              mediaTypesRequiringUserAction: const <PlaybackMediaTypes>{},
            );
      }

      final controller = WebViewController.fromPlatformCreationParams(
        controllerParams,
      );
      controller
        ..setNavigationDelegate(
          NavigationDelegate(
            onNavigationRequest: (request) async {
              if (shouldOpenEmbeddedVideoNavigationExternally(request.url)) {
                try {
                  await openExternalVideoUrl(request.url);
                } catch (_) {
                  // Keep the request blocked even if the handoff fails.
                }
                return NavigationDecision.prevent;
              }

              return NavigationDecision.navigate;
            },
          ),
        )
        ..setJavaScriptMode(JavaScriptMode.unrestricted)
        ..setBackgroundColor(const Color(0xFF000000))
        ..loadHtmlString(
          buildEmbeddedYouTubeHtml(_embedUrl),
          baseUrl: 'https://org.eixe.patientfront/',
        );
      if (!kIsWeb && Platform.isAndroid) {
        (controller.platform as AndroidWebViewController)
            .setMediaPlaybackRequiresUserGesture(false);
      }

      PlatformWebViewWidgetCreationParams widgetParams =
          PlatformWebViewWidgetCreationParams(
            controller: controller.platform,
            layoutDirection: TextDirection.ltr,
            gestureRecognizers: {
              Factory<OneSequenceGestureRecognizer>(
                () => EagerGestureRecognizer(),
              ),
            },
          );
      if (!kIsWeb && Platform.isAndroid) {
        widgetParams =
            AndroidWebViewWidgetCreationParams.fromPlatformWebViewWidgetCreationParams(
              widgetParams,
              displayWithHybridComposition: true,
            );
      }
      _webViewWidget = WebViewWidget.fromPlatformCreationParams(
        params: widgetParams,
      );
    } else {
      _webViewWidget = null;
    }
  }

  @override
  void dispose() {
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (_embedUrl == null) {
      return const Text('Video unavailable');
    }

    switch (_renderMode) {
      case ExerciseVideoRenderMode.placeholder:
        return Container(
          height: 180,
          width: double.infinity,
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.surfaceContainerHighest,
            borderRadius: BorderRadius.circular(12),
          ),
          alignment: Alignment.center,
          child: const Text('Exercise video'),
        );
      case ExerciseVideoRenderMode.youtubeIframe:
        return ClipRRect(
          borderRadius: BorderRadius.circular(12),
          child: SizedBox(
            height: 220,
            width: double.infinity,
            child: _webViewWidget!,
          ),
        );
    }
  }
}

class PatientAppBootstrapPage extends StatefulWidget {
  const PatientAppBootstrapPage({
    required this.bridgeConfig,
    this.autoInitializeBridge = true,
    super.key,
  });

  final BridgeRuntimeConfig bridgeConfig;
  final bool autoInitializeBridge;

  @override
  State<PatientAppBootstrapPage> createState() =>
      _PatientAppBootstrapPageState();
}

class _PatientAppBootstrapPageState extends State<PatientAppBootstrapPage> {
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();

  bool _bridgeInitialized = false;
  bool _bridgeRuntimeInitialized = false;
  bool _busy = false;
  String _status = 'Starting app...';
  _BootstrapStage _stage = _BootstrapStage.starting;
  rust_api.LoginResponse? _loginResponse;
  List<rust_api.PatientProgramSummary> _patientPrograms = const [];

  ExternalLibrary? _bridgeLibrary() {
    if (kIsWeb) {
      return null;
    }

    if (Platform.isAndroid) {
      return ExternalLibrary.open('libmobile_bridge_frb.so');
    }

    if (Platform.isLinux) {
      final executableDir = File(Platform.resolvedExecutable).parent;
      final bundledLibrary = File(
        '${executableDir.path}/lib/libmobile_bridge_frb.so',
      );
      if (bundledLibrary.existsSync()) {
        return ExternalLibrary.open(bundledLibrary.path);
      }

      final repoReleaseLibrary = File(
        '${Directory.current.path}/../target/release/libmobile_bridge_frb.so',
      );
      if (repoReleaseLibrary.existsSync()) {
        return ExternalLibrary.open(repoReleaseLibrary.path);
      }

      return ExternalLibrary.open('libmobile_bridge_frb.so');
    }

    return null;
  }

  @override
  void initState() {
    super.initState();
    if (widget.autoInitializeBridge) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        _initializeBridge();
      });
    }
  }

  Future<void> _initializeBridge() async {
    if (_bridgeInitialized || _busy) {
      return;
    }

    if (!widget.bridgeConfig.isConfigured) {
      setState(() {
        _stage = _BootstrapStage.error;
        _status =
            'Missing Supabase configuration. Pass SUPABASE_URL and SUPABASE_ANON_KEY with --dart-define.';
      });
      return;
    }

    if (!widget.autoInitializeBridge) {
      setState(() {
        _bridgeInitialized = true;
        _stage = _BootstrapStage.readyForLogin;
        _status = 'Bridge ready. You can sign in now.';
      });
      return;
    }

    setState(() {
      _busy = true;
      _status = 'Initializing Rust bridge...';
    });

    try {
      await RustLib.init(externalLibrary: _bridgeLibrary());
      setState(() {
        _bridgeInitialized = true;
        _bridgeRuntimeInitialized = true;
        _stage = _BootstrapStage.readyForLogin;
        _status = 'Bridge ready. You can sign in now.';
      });
    } catch (error) {
      setState(() {
        _stage = _BootstrapStage.error;
        _status = 'Bridge initialization failed: $error';
      });
    } finally {
      if (mounted) {
        setState(() {
          _busy = false;
        });
      }
    }
  }

  Future<void> _loginAndLoadPrograms() async {
    if (!widget.bridgeConfig.isConfigured) {
      setState(() {
        _stage = _BootstrapStage.error;
        _status =
            'Missing Supabase configuration. Pass SUPABASE_URL and SUPABASE_ANON_KEY with --dart-define.';
      });
      return;
    }

    if (!_bridgeInitialized) {
      await _initializeBridge();
      if (!_bridgeInitialized) {
        return;
      }
    }

    setState(() {
      _busy = true;
      _status = 'Calling Rust login...';
      _patientPrograms = const [];
    });

    try {
      final loginResponse = await rust_api.login(
        request: rust_api.LoginRequest(
          email: _emailController.text.trim(),
          password: _passwordController.text,
        ),
        config: widget.bridgeConfig.toBridgeConfig(),
      );
      final patientPrograms = await _loadPatientPrograms(
        loginResponse.accessToken,
      );
      setState(() {
        _stage = _BootstrapStage.readyForLogin;
        _loginResponse = loginResponse;
        _patientPrograms = patientPrograms;
        _status =
            'Signed in as ${loginResponse.userProfileType}. Loaded ${patientPrograms.length} program(s).';
      });
    } catch (error) {
      setState(() {
        _stage = _BootstrapStage.error;
        _status = 'Rust call failed: $error';
      });
    } finally {
      if (mounted) {
        setState(() {
          _busy = false;
        });
      }
    }
  }

  Future<List<rust_api.PatientProgramSummary>> _loadPatientPrograms(
    String token,
  ) {
    return rust_api.getPatientPrograms(
      token: token,
      config: widget.bridgeConfig.toBridgeConfig(),
    );
  }

  Future<void> _submitDayFeedback(
    rust_api.SubmitDayFeedbackRequest request,
  ) async {
    final loginResponse = _loginResponse;
    if (loginResponse == null) {
      throw StateError('Not signed in.');
    }

    setState(() {
      _busy = true;
      _status = 'Saving feedback...';
    });

    try {
      await rust_api.submitDayFeedback(
        token: loginResponse.accessToken,
        request: request,
        config: widget.bridgeConfig.toBridgeConfig(),
      );
      final patientPrograms = await _loadPatientPrograms(
        loginResponse.accessToken,
      );
      if (mounted) {
        setState(() {
          _patientPrograms = patientPrograms;
          _status = 'Feedback saved.';
        });
      }
    } finally {
      if (mounted) {
        setState(() {
          _busy = false;
        });
      }
    }
  }

  Future<void> _updateDayCompletion(
    rust_api.UpdateDayCompletionRequest request,
  ) async {
    final loginResponse = _loginResponse;
    if (loginResponse == null) {
      throw StateError('Not signed in.');
    }

    setState(() {
      _busy = true;
      _status = 'Updating session state...';
    });

    try {
      await rust_api.updateDayCompletion(
        token: loginResponse.accessToken,
        request: request,
        config: widget.bridgeConfig.toBridgeConfig(),
      );
      final patientPrograms = await _loadPatientPrograms(
        loginResponse.accessToken,
      );
      if (mounted) {
        setState(() {
          _patientPrograms = patientPrograms;
          _status = request.completed
              ? 'Session marked as completed.'
              : 'Session marked as not completed.';
        });
      }
    } finally {
      if (mounted) {
        setState(() {
          _busy = false;
        });
      }
    }
  }

  @override
  void dispose() {
    _emailController.dispose();
    _passwordController.dispose();
    if (_bridgeRuntimeInitialized) {
      RustLib.dispose();
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final bootstrapBody = _buildBootstrapBody(theme);
    if (bootstrapBody != null) {
      return bootstrapBody;
    }

    if (_loginResponse != null) {
      return PatientHomePage(
        loginResponse: _loginResponse!,
        patientPrograms: _patientPrograms,
        onSignOut: _signOut,
        onSubmitDayFeedback: _submitDayFeedback,
        onUpdateDayCompletion: _updateDayCompletion,
      );
    }

    return Scaffold(
      appBar: AppBar(title: const Text('Eixe Patient Front')),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          Text('Welcome back', style: theme.textTheme.titleMedium),
          const SizedBox(height: 16),
          Text(
            widget.bridgeConfig.isConfigured
                ? 'Supabase configuration loaded from Dart defines.'
                : 'Supabase configuration missing. Use --dart-define for SUPABASE_URL and SUPABASE_ANON_KEY.',
          ),
          const SizedBox(height: 12),
          TextField(
            controller: _emailController,
            decoration: const InputDecoration(
              labelText: 'Patient email',
              border: OutlineInputBorder(),
            ),
          ),
          const SizedBox(height: 12),
          TextField(
            controller: _passwordController,
            decoration: const InputDecoration(
              labelText: 'Password',
              border: OutlineInputBorder(),
            ),
            obscureText: true,
          ),
          const SizedBox(height: 16),
          Wrap(
            spacing: 12,
            runSpacing: 12,
            children: [
              FilledButton(
                onPressed: _busy ? null : _loginAndLoadPrograms,
                child: const Text('Sign in'),
              ),
            ],
          ),
          const SizedBox(height: 16),
          Text(_status),
        ],
      ),
    );
  }

  void _signOut() {
    setState(() {
      _loginResponse = null;
      _patientPrograms = const [];
      _emailController.clear();
      _passwordController.clear();
      _status = 'Signed out. You can sign in again.';
    });
  }

  Widget? _buildBootstrapBody(ThemeData theme) {
    if (_stage == _BootstrapStage.starting) {
      return Scaffold(
        appBar: AppBar(title: const Text('Eixe Patient Front')),
        body: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'Starting Eixe Patient Front...',
                style: theme.textTheme.headlineSmall,
              ),
              const SizedBox(height: 12),
              Text(
                widget.bridgeConfig.isConfigured
                    ? 'Preparing the app and connecting the Rust core.'
                    : 'Missing runtime configuration. Add SUPABASE_URL and SUPABASE_ANON_KEY with --dart-define.',
              ),
              const SizedBox(height: 24),
              if (_busy) const CircularProgressIndicator(),
              if (!_busy) ...[
                FilledButton(
                  onPressed: () async {
                    await _initializeBridge();
                  },
                  child: const Text('Continue'),
                ),
              ],
            ],
          ),
        ),
      );
    }

    if (_stage == _BootstrapStage.error) {
      return Scaffold(
        appBar: AppBar(title: const Text('Eixe Patient Front')),
        body: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'Unable to start the app',
                style: theme.textTheme.headlineSmall,
              ),
              const SizedBox(height: 12),
              Text(_status),
              const SizedBox(height: 24),
              FilledButton(
                onPressed: _busy
                    ? null
                    : () async {
                        setState(() {
                          _stage = _BootstrapStage.starting;
                        });
                        await _initializeBridge();
                      },
                child: const Text('Retry'),
              ),
            ],
          ),
        ),
      );
    }

    return null;
  }
}
