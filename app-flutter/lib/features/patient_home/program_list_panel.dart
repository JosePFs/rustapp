import 'package:flutter/material.dart';

import 'package:app_flutter/shared/widgets/section.dart';
import 'package:app_flutter/src/rust/api.dart' as rust_api;
import 'package:app_flutter/l10n/app_localizations_ext.dart';

class ProgramListPanel extends StatelessWidget {
  const ProgramListPanel({
    required this.programs,
    required this.selectedProgramId,
    required this.onProgramSelected,
    super.key,
  });

  final List<rust_api.PatientProgramSummary> programs;
  final String? selectedProgramId;
  final ValueChanged<String> onProgramSelected;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return SectionCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SectionHeader(
            title: context.l10n.programsYourProgramsTitle,
            subtitle: context.l10n.programsAssignedCount(programs.length),
          ),
          const SizedBox(height: 8),
          for (final program in programs)
            Container(
              margin: const EdgeInsets.only(top: 10),
              decoration: BoxDecoration(
                color: theme.colorScheme.surface,
                borderRadius: BorderRadius.circular(16),
                border: Border.all(
                  color: program.patientProgramId == selectedProgramId
                      ? theme.colorScheme.primary.withValues(alpha: 0.45)
                      : theme.colorScheme.outlineVariant.withValues(alpha: 0.55),
                ),
              ),
              child: ListTile(
                selected: program.patientProgramId == selectedProgramId,
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(16),
                ),
                title: Text(
                  program.programName,
                  style: theme.textTheme.titleMedium,
                ),
                subtitle: Padding(
                  padding: const EdgeInsets.only(top: 8),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        program.programDescription ??
                            context.l10n.programsNoDescription,
                        style: theme.textTheme.bodyMedium,
                      ),
                      const SizedBox(height: 10),
                      ClipRRect(
                        borderRadius: BorderRadius.circular(999),
                        child: LinearProgressIndicator(
                          minHeight: 10,
                          value: (program.progressPercent.clamp(0, 100) / 100)
                              .toDouble(),
                        ),
                      ),
                      const SizedBox(height: 6),
                      Row(
                        children: [
                          Text(
                            context.l10n.programsProgressLabel,
                            style: theme.textTheme.bodySmall,
                          ),
                          const Spacer(),
                          Text(
                            context.l10n.programsProgressPercent(
                              program.progressPercent.clamp(0, 100),
                            ),
                            style: theme.textTheme.bodySmall?.copyWith(
                              fontWeight: FontWeight.w600,
                            ),
                          ),
                        ],
                      ),
                      if (program.averageEffort != null ||
                          program.averagePain != null) ...[
                        const SizedBox(height: 6),
                        Text(
                          context.l10n.programsEffortPainSummary(
                            program.averageEffort?.toStringAsFixed(1) ?? '-',
                            program.averagePain?.toStringAsFixed(1) ?? '-',
                          ),
                          style: theme.textTheme.bodySmall,
                        ),
                      ],
                    ],
                  ),
                ),
                onTap: () => onProgramSelected(program.patientProgramId),
              ),
            ),
        ],
      ),
    );
  }
}
