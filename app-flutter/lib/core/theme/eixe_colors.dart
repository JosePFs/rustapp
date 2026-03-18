import 'package:flutter/material.dart';

class EixeColors {
  EixeColors._();

  // ── Brand ──────────────────────────────────────────────────────────────────
  static const brandGreen = Color(0xFF35B339);
  static const actionAmber = Color(0xFFE89320);

  // ── Green tonal ramp ───────────────────────────────────────────────────────
  static const greenContainer = Color(0xFFDDF5DD);
  static const onGreenContainer = Color(0xFF0A3D0C);

  // ── Amber tonal ramp ───────────────────────────────────────────────────────
  static const amberContainer = Color(0xFFFFF0D6);
  static const onAmberContainer = Color(0xFF4A2E00);

  // ── Dark variants ──────────────────────────────────────────────────────────
  static const brandGreenDark = Color(0xFF6FD973);
  static const actionAmberDark = Color(0xFFFFB74D);
  static const greenContainerDark = Color(0xFF1B5E1F);
  static const amberContainerDark = Color(0xFF5D3E00);

  // ── Warm tint ──────────────────────────────────────────────────────────────
  static const warmPeach = Color(0xFFFFDFCD);

  static Color warmSurface(Color baseSurface) {
    final tint = warmPeach.withValues(alpha: 0.08);
    return Color.alphaBlend(tint, baseSurface);
  }

  // ── Error ──────────────────────────────────────────────────────────────────
  static const error = Color(0xFFBA1A1A);
  static const onError = Colors.white;
  static const errorContainer = Color(0xFFFFDAD6);
  static const onErrorContainer = Color(0xFF410002);

  // ── Helpers ────────────────────────────────────────────────────────────────
  static Color hairlineBorder(ColorScheme scheme) =>
      scheme.outlineVariant.withValues(alpha: 0.55);
}
