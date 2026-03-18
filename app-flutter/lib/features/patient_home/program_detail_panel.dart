import 'package:flutter/material.dart';

import 'package:app_flutter/shared/widgets/section.dart';
import 'package:app_flutter/src/rust/api.dart' as rust_api;
import 'package:app_flutter/l10n/app_localizations_ext.dart';
import 'exercise_feedback_card.dart';
import 'patient_home_models.dart';

class ProgramDetailPanel extends StatelessWidget {
  const ProgramDetailPanel({
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
    super.key,
  });

  final rust_api.PatientProgramSummary? selectedProgram;
  final rust_api.ProgramDaySummary? selectedDay;
  final bool submittingFeedback;
  final TextEditingController completionDateController;
  final ValueChanged<int> onDaySelected;
  final Future<void> Function() onPickCompletionDate;
  final ExerciseFeedbackDraft Function(
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

  Future<void> _showDayPickerBottomSheet(
    BuildContext context, {
    required ThemeData theme,
    required List<rust_api.ProgramDaySummary> days,
  }) async {
    await showModalBottomSheet<void>(
      context: context,
      showDragHandle: true,
      isScrollControlled: true,
      useSafeArea: true,
      builder: (context) {
        final width = MediaQuery.sizeOf(context).width;
        final isNarrow = width < 420;
        final textScale = MediaQuery.textScalerOf(
          context,
        ).scale(1.0).clamp(1.0, 1.4);
        final baseAspectRatio = isNarrow ? 4.4 : 3.9;

        return Padding(
          padding: const EdgeInsets.fromLTRB(16, 8, 16, 24),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                context.l10n.programDetailSelectDay,
                style: theme.textTheme.titleLarge,
              ),
              const SizedBox(height: 12),
              Flexible(
                child: GridView.builder(
                  padding: EdgeInsets.zero,
                  gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
                    crossAxisCount: isNarrow ? 1 : 2,
                    mainAxisSpacing: 10,
                    crossAxisSpacing: 10,
                    childAspectRatio: baseAspectRatio / textScale,
                  ),
                  itemCount: days.length,
                  itemBuilder: (context, index) {
                    final day = days[index];
                    final isSelected = selectedDay?.dayIndex == day.dayIndex;
                    if (day.isRestDay) {
                      return DecoratedBox(
                        decoration: BoxDecoration(
                          color: theme.colorScheme.surface.withValues(
                            alpha: 0.65,
                          ),
                          borderRadius: BorderRadius.circular(12),
                          border: Border.all(
                            color: theme.colorScheme.outlineVariant.withValues(
                              alpha: 0.55,
                            ),
                          ),
                        ),
                        child: Padding(
                          padding: const EdgeInsets.symmetric(
                            horizontal: 12,
                            vertical: 6,
                          ),
                          child: Row(
                            children: [
                              Icon(
                                Icons.self_improvement,
                                size: 18,
                                color: theme.textTheme.bodySmall?.color
                                    ?.withValues(alpha: 0.7),
                              ),
                              const SizedBox(width: 8),
                              Expanded(
                                child: Text(
                                  context.l10n.programDetailRestDayLabel(
                                    day.dayNumber,
                                  ),
                                  style: theme.textTheme.bodyMedium?.copyWith(
                                    color: theme.textTheme.bodySmall?.color
                                        ?.withValues(alpha: 0.8),
                                  ),
                                  maxLines: 1,
                                  overflow: TextOverflow.ellipsis,
                                ),
                              ),
                            ],
                          ),
                        ),
                      );
                    }

                    return InkWell(
                      onTap: () {
                        onDaySelected(day.dayIndex);
                        FocusScope.of(context).unfocus();
                        Navigator.of(context).pop();
                      },
                      borderRadius: BorderRadius.circular(12),
                      child: Container(
                        decoration: BoxDecoration(
                          borderRadius: BorderRadius.circular(12),
                          color: isSelected
                              ? theme.colorScheme.primary.withValues(
                                  alpha: 0.12,
                                )
                              : theme.colorScheme.surface.withValues(
                                  alpha: 0.65,
                                ),
                          border: Border.all(
                            color: isSelected
                                ? theme.colorScheme.primary.withValues(
                                    alpha: 0.85,
                                  )
                                : theme.colorScheme.outlineVariant.withValues(
                                    alpha: 0.55,
                                  ),
                          ),
                        ),
                        padding: const EdgeInsets.symmetric(
                          horizontal: 12,
                          vertical: 6,
                        ),
                        alignment: Alignment.centerLeft,
                        child: Row(
                          children: [
                            if (day.completedAt != null)
                              Icon(
                                Icons.check,
                                size: 18,
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
                                    context.l10n.programDetailDayLabel(
                                      day.dayNumber,
                                    ),
                                    style: theme.textTheme.bodyMedium?.copyWith(
                                      fontWeight: FontWeight.w600,
                                    ),
                                    maxLines: 1,
                                    overflow: TextOverflow.ellipsis,
                                  ),
                                  if (day.workoutName != null)
                                    Text(
                                      day.workoutName!,
                                      style: theme.textTheme.bodySmall
                                          ?.copyWith(
                                            color: theme
                                                .textTheme
                                                .bodySmall
                                                ?.color
                                                ?.withValues(alpha: 0.85),
                                          ),
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
                ),
              ),
            ],
          ),
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final isCompleted = selectedDay?.completedAt != null;

    return SectionCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (selectedProgram == null)
            Text(context.l10n.programDetailSelectProgram)
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
                  final selectedDayLabel = selectedDay != null
                      ? 'Day ${selectedDay!.dayNumber}'
                      : null;
                  final selectedWorkoutLabel = selectedDay?.workoutName;

                  return Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      Tooltip(
                        message: 'Change day',
                        child: Semantics(
                          button: true,
                          label: 'Change day',
                          child: InkWell(
                            onTap: () => _showDayPickerBottomSheet(
                              context,
                              theme: theme,
                              days: days,
                            ),
                            borderRadius: BorderRadius.circular(16),
                            child: Container(
                              padding: const EdgeInsets.symmetric(
                                horizontal: 14,
                                vertical: 12,
                              ),
                              decoration: BoxDecoration(
                                color: theme.colorScheme.surface.withValues(
                                  alpha: 0.65,
                                ),
                                borderRadius: BorderRadius.circular(16),
                                border: Border.all(
                                  color: theme.dividerColor.withValues(
                                    alpha: 0.18,
                                  ),
                                ),
                              ),
                              child: Row(
                                children: [
                                  if (selectedDay?.completedAt != null)
                                    Icon(
                                      Icons.verified,
                                      size: 18,
                                      color: theme.colorScheme.primary,
                                    )
                                  else
                                    Icon(
                                      Icons.calendar_today,
                                      size: 18,
                                      color: theme.textTheme.bodySmall?.color
                                          ?.withValues(alpha: 0.8),
                                    ),
                                  const SizedBox(width: 10),
                                  Expanded(
                                    child: Column(
                                      crossAxisAlignment:
                                          CrossAxisAlignment.start,
                                      mainAxisSize: MainAxisSize.min,
                                      children: [
                                        Text(
                                          selectedDayLabel ?? 'Choose a day',
                                          style: theme.textTheme.titleMedium
                                              ?.copyWith(
                                                fontWeight: FontWeight.w700,
                                              ),
                                        ),
                                        if (selectedWorkoutLabel != null)
                                          Text(
                                            selectedWorkoutLabel,
                                            style: theme.textTheme.bodySmall
                                                ?.copyWith(
                                                  color: theme
                                                      .textTheme
                                                      .bodySmall
                                                      ?.color
                                                      ?.withValues(alpha: 0.85),
                                                ),
                                            maxLines: 1,
                                            overflow: TextOverflow.ellipsis,
                                          )
                                        else
                                          Text(
                                            '${days.where((d) => !d.isRestDay).length} training days',
                                            style: theme.textTheme.bodySmall
                                                ?.copyWith(
                                                  color: theme
                                                      .textTheme
                                                      .bodySmall
                                                      ?.color
                                                      ?.withValues(alpha: 0.85),
                                                ),
                                          ),
                                      ],
                                    ),
                                  ),
                                  Icon(
                                    Icons.expand_more,
                                    color: theme.textTheme.bodySmall?.color
                                        ?.withValues(alpha: 0.8),
                                  ),
                                ],
                              ),
                            ),
                          ),
                        ),
                      ),
                    ],
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
                    (exercise) => ExerciseFeedbackCard(
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
                    labelText: context.l10n.programDetailCompletionDateLabel,
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
                        isCompleted
                            ? context.l10n.programDetailSave
                            : context.l10n.programDetailSaveAsCompleted,
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
                        child: Text(context.l10n.programDetailMarkAsNotCompleted),
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
