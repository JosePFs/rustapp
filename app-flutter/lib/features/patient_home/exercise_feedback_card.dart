import 'package:flutter/material.dart';

import 'package:app_flutter/src/rust/api.dart' as rust_api;
import 'package:app_flutter/l10n/app_localizations_ext.dart';
import 'exercise_video_panel.dart';
import 'patient_home_models.dart';

class ExerciseFeedbackCard extends StatelessWidget {
  const ExerciseFeedbackCard({
    required this.programId,
    required this.dayIndex,
    required this.exercise,
    required this.draft,
    required this.commentController,
    required this.onEffortChanged,
    required this.onPainChanged,
    required this.onCommentChanged,
    super.key,
  });

  final String programId;
  final int dayIndex;
  final rust_api.ExerciseInstructionSummary exercise;
  final ExerciseFeedbackDraft draft;
  final TextEditingController commentController;
  final void Function(String exerciseKey, int value) onEffortChanged;
  final void Function(String exerciseKey, int value) onPainChanged;
  final void Function(String exerciseKey, String value) onCommentChanged;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final exerciseKey = '$programId::$dayIndex::${exercise.exerciseId}';

    return Container(
      margin: const EdgeInsets.only(bottom: 16),
      decoration: BoxDecoration(
        color: theme.colorScheme.surface,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(color: theme.dividerColor.withValues(alpha: 0.18)),
      ),
      child: Padding(
        padding: const EdgeInsets.all(18),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              exercise.name,
              style: theme.textTheme.titleMedium?.copyWith(
                fontWeight: FontWeight.w700,
              ),
            ),
            if (exercise.description != null) ...[
              const SizedBox(height: 8),
              Text(
                exercise.description!,
                style: theme.textTheme.bodyMedium?.copyWith(
                  color: theme.textTheme.bodySmall?.color?.withValues(
                    alpha: 0.85,
                  ),
                ),
              ),
            ],
            const SizedBox(height: 16),
            Row(
              children: [
                Icon(
                  Icons.fitness_center,
                  size: 18,
                  color: theme.textTheme.bodySmall?.color?.withValues(
                    alpha: 0.8,
                  ),
                ),
                const SizedBox(width: 8),
                Text(
                  context.l10n.exerciseSetsReps(exercise.sets, exercise.reps),
                  style: theme.textTheme.bodyMedium?.copyWith(
                    fontWeight: FontWeight.w600,
                  ),
                ),
              ],
            ),
            if (exercise.videoUrl != null) ...[
              const SizedBox(height: 8),
              ExerciseVideoPanel(
                key: Key('exercise-video-${exercise.exerciseId}'),
                exerciseId: exercise.exerciseId,
                videoUrl: exercise.videoUrl!,
              ),
            ],
            const SizedBox(height: 16),
            Row(
              children: [
                Text(
                  context.l10n.exerciseEffortLabel,
                  style: theme.textTheme.bodyMedium?.copyWith(
                    fontWeight: FontWeight.w600,
                  ),
                ),
                const Spacer(),
                Text(
                  '${draft.effort}/10',
                  style: theme.textTheme.bodyMedium?.copyWith(
                    fontWeight: FontWeight.w900,
                  ),
                ),
              ],
            ),
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
            Row(
              children: [
                Text(
                  context.l10n.exercisePainLabel,
                  style: theme.textTheme.bodyMedium?.copyWith(
                    fontWeight: FontWeight.w600,
                  ),
                ),
                const Spacer(),
                Text(
                  '${draft.pain}/10',
                  style: theme.textTheme.bodyMedium?.copyWith(
                    fontWeight: FontWeight.w900,
                  ),
                ),
              ],
            ),
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
              maxLines: 6,
              keyboardType: TextInputType.multiline,
              decoration: InputDecoration(
                labelText: context.l10n.exerciseCommentLabel,
                hintText: context.l10n.exerciseCommentHint,
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
