import 'package:flutter/material.dart';

class SectionHeader extends StatelessWidget {
  const SectionHeader({required this.title, this.subtitle, super.key});

  final String title;
  final String? subtitle;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 4),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(title, style: theme.textTheme.titleLarge),
          if (subtitle != null) ...[
            const SizedBox(height: 4),
            Text(
              subtitle!,
              style: theme.textTheme.bodyMedium?.copyWith(
                color: theme.textTheme.bodySmall?.color?.withValues(alpha: 0.8),
              ),
            ),
          ],
        ],
      ),
    );
  }
}

class SectionCard extends StatelessWidget {
  const SectionCard({required this.child, super.key});

  final Widget child;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final scheme = theme.colorScheme;
    final baseSurface = scheme.surfaceContainerHighest;
    final accentTint = scheme.secondary.withValues(alpha: 0.10);
    return DecoratedBox(
      decoration: BoxDecoration(
        color: Color.alphaBlend(accentTint, baseSurface),
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: scheme.outlineVariant.withValues(alpha: 0.55),
        ),
      ),
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: child,
      ),
    );
  }
}

