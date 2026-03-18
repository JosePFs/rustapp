import 'package:flutter/material.dart';

import 'package:app_flutter/shared/widgets/app_brand_title.dart';
import 'package:app_flutter/shared/widgets/section.dart';
import 'package:app_flutter/shared/utils/iterable_ext.dart';
import 'package:app_flutter/src/rust/api.dart' as rust_api;
import 'package:app_flutter/core/locale_controller.dart';
import 'package:app_flutter/l10n/app_localizations_ext.dart';

import 'patient_home_models.dart';
import 'program_selection.dart';
import 'program_list_panel.dart';
import 'program_detail_panel.dart';

class PatientHomePage extends StatefulWidget {
  const PatientHomePage({
    required this.loginResponse,
    required this.patientPrograms,
    required this.localeController,
    required this.localeLoaded,
    this.onSignOut,
    this.onSubmitDayFeedback,
    this.onUpdateDayCompletion,
    super.key,
  });

  final rust_api.LoginResponse loginResponse;
  final List<rust_api.PatientProgramSummary> patientPrograms;
  final LocaleController localeController;
  final bool localeLoaded;
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
  final Map<String, ExerciseFeedbackDraft> _feedbackDrafts = {};
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
        title: const AppBrandTitle(),
        actions: [
          PopupMenuButton<String>(
            tooltip: context.l10n.commonLanguage,
            icon: const Icon(Icons.language),
            onSelected: (value) async {
              switch (value) {
                case LocaleController.deviceKey:
                  await widget.localeController.setLocale(null);
                  break;
                case 'en':
                case 'es':
                case 'gl':
                  await widget.localeController.setLocale(Locale(value));
                  break;
              }
            },
            itemBuilder: (context) {
              final selected = widget.localeController.locale?.languageCode;
              return [
                CheckedPopupMenuItem<String>(
                  value: LocaleController.deviceKey,
                  checked: selected == null && widget.localeLoaded,
                  child: Text(context.l10n.commonFollowDevice),
                ),
                const PopupMenuDivider(),
                CheckedPopupMenuItem<String>(
                  value: 'en',
                  checked: selected == 'en',
                  child: Text(context.l10n.commonEnglish),
                ),
                CheckedPopupMenuItem<String>(
                  value: 'es',
                  checked: selected == 'es',
                  child: Text(context.l10n.commonSpanish),
                ),
                CheckedPopupMenuItem<String>(
                  value: 'gl',
                  checked: selected == 'gl',
                  child: Text(context.l10n.commonGalician),
                ),
              ];
            },
          ),
          if (widget.onSignOut != null)
            TextButton(
              onPressed: widget.onSignOut,
              child: Text(context.l10n.patientHomeSignOut),
            ),
        ],
      ),
      body: LayoutBuilder(
        builder: (context, constraints) {
          final isWide = constraints.maxWidth >= 900;
          final listPanel = ProgramListPanel(
            programs: widget.patientPrograms,
            selectedProgramId: _selectedProgramId,
            onProgramSelected: _selectProgram,
          );
          final detailPanel = ProgramDetailPanel(
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
            padding: const EdgeInsets.all(16),
            children: [
              if (widget.patientPrograms.isEmpty)
                SectionCard(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      SectionHeader(
                        title: context.l10n.patientHomeNoProgramsTitle,
                        subtitle: context.l10n.patientHomeNoProgramsSubtitle,
                      ),
                      const SizedBox(height: 12),
                      Text(
                        context.l10n.patientHomeNoProgramsBody,
                        style: theme.textTheme.bodyMedium,
                      ),
                    ],
                  ),
                )
              else if (isWide)
                Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Expanded(flex: 3, child: listPanel),
                    const SizedBox(width: 16),
                    Expanded(flex: 4, child: detailPanel),
                  ],
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

  String _exerciseKey(String programId, int dayIndex, String exerciseId) =>
      '$programId::$dayIndex::$exerciseId';

  ExerciseFeedbackDraft _draftForExercise(
    String programId,
    int dayIndex,
    rust_api.ExerciseInstructionSummary exercise,
  ) {
    final key = _exerciseKey(programId, dayIndex, exercise.exerciseId);
    return _feedbackDrafts.putIfAbsent(
      key,
      () => ExerciseFeedbackDraft(
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
    return _commentControllers.putIfAbsent(key, () {
      final controller = TextEditingController(text: exercise.comment ?? '');
      controller.addListener(() {
        _feedbackDrafts[key]?.comment = controller.text;
      });
      return controller;
    });
  }

  void _selectProgram(String programId) {
    setState(() {
      _selectedProgramId = programId;
    });
    _syncSelection();
  }

  void _selectDay(int dayIndex) {
    setState(() {
      _selectedDayIndex = dayIndex;
    });
    _syncCompletionDateControllerForCurrentSelection();
  }

  String _completionDateKey(String programId, int dayIndex) =>
      '$programId::$dayIndex';

  void _syncCompletionDateControllerForCurrentSelection() {
    final selectedProgram = widget.patientPrograms
        .where((program) => program.patientProgramId == _selectedProgramId)
        .firstOrNull;
    final selectedDay = selectedProgram?.days
        .where((day) => day.dayIndex == _selectedDayIndex)
        .firstOrNull;
    if (selectedProgram == null || selectedDay == null) {
      return;
    }

    final key = _completionDateKey(
      selectedProgram.patientProgramId,
      selectedDay.dayIndex,
    );
    final completionDate =
        _completionDateDrafts[key] ??
        (selectedDay.completedAt != null
            ? (selectedDay.sessionDate ?? '')
            : '');
    if (completionDate.isNotEmpty) {
      _completionDateController.text = completionDate;
    } else {
      _completionDateController.text = DateTime.now()
          .toIso8601String()
          .split('T')
          .first;
    }
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

    final currentDate = DateTime.tryParse(_completionDateController.text);
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

  Future<void> _saveSelectedDay() async {
    final selectedProgram = widget.patientPrograms
        .where((program) => program.patientProgramId == _selectedProgramId)
        .firstOrNull;
    final selectedDay = selectedProgram?.days
        .where((day) => day.dayIndex == _selectedDayIndex)
        .firstOrNull;
    if (selectedProgram == null || selectedDay == null) {
      return;
    }

    if (widget.onSubmitDayFeedback == null ||
        widget.onUpdateDayCompletion == null) {
      return;
    }

    setState(() {
      _submittingFeedback = true;
    });

    try {
      final feedback = selectedDay.exercises.map((exercise) {
        final key = _exerciseKey(
          selectedProgram.patientProgramId,
          selectedDay.dayIndex,
          exercise.exerciseId,
        );
        final draft =
            _feedbackDrafts[key] ??
            ExerciseFeedbackDraft(
              effort: (exercise.effort ?? 1).clamp(1, 10),
              pain: (exercise.pain ?? 0).clamp(0, 10),
              comment: exercise.comment ?? '',
            );
        return rust_api.ExerciseFeedbackInput(
          exerciseId: exercise.exerciseId,
          effort: draft.effort,
          pain: draft.pain,
          comment: draft.comment.trim().isEmpty ? null : draft.comment.trim(),
        );
      }).toList();

      await widget.onSubmitDayFeedback!(
        rust_api.SubmitDayFeedbackRequest(
          patientProgramId: selectedProgram.patientProgramId,
          dayIndex: selectedDay.dayIndex,
          sessionDate: _completionDateController.text.trim(),
          feedback: feedback,
        ),
      );

      final sessionDate = _completionDateController.text.trim();
      await widget.onUpdateDayCompletion!(
        rust_api.UpdateDayCompletionRequest(
          patientProgramId: selectedProgram.patientProgramId,
          dayIndex: selectedDay.dayIndex,
          sessionDate: sessionDate,
          completed: true,
        ),
      );

      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(context.l10n.statusSessionSavedAsCompleted),
          ),
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
    final selectedProgram = widget.patientPrograms
        .where((program) => program.patientProgramId == _selectedProgramId)
        .firstOrNull;
    final selectedDay = selectedProgram?.days
        .where((day) => day.dayIndex == _selectedDayIndex)
        .firstOrNull;
    if (selectedProgram == null || selectedDay == null) {
      return;
    }
    if (widget.onUpdateDayCompletion == null) {
      return;
    }

    setState(() {
      _submittingFeedback = true;
    });
    try {
      final sessionDate = _completionDateController.text.trim();
      await widget.onUpdateDayCompletion!(
        rust_api.UpdateDayCompletionRequest(
          patientProgramId: selectedProgram.patientProgramId,
          dayIndex: selectedDay.dayIndex,
          sessionDate: sessionDate,
          completed: completed,
        ),
      );

      if (mounted) {
        final message = completed
            ? context.l10n.statusSessionMarkedCompleted
            : context.l10n.statusSessionMarkedNotCompleted;
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(
            content: Text(message),
          ),
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
}
