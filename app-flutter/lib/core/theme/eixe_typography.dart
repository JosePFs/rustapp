import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

class EixeTypography {
  EixeTypography._();

  static TextTheme build(TextTheme base) {
    return base.copyWith(
      // ── Display ─────────────────────────────────────────────────────────
      displayLarge: GoogleFonts.atkinsonHyperlegible(
        textStyle: base.displayLarge,
        fontSize: 56,
        fontWeight: FontWeight.w600,
        letterSpacing: -1.5,
        height: 1.1,
      ),
      displayMedium: GoogleFonts.atkinsonHyperlegible(
        textStyle: base.displayMedium,
        fontSize: 44,
        fontWeight: FontWeight.w600,
        letterSpacing: -0.5,
        height: 1.15,
      ),
      displaySmall: GoogleFonts.atkinsonHyperlegible(
        textStyle: base.displaySmall,
        fontSize: 36,
        fontWeight: FontWeight.w600,
        letterSpacing: -0.3,
        height: 1.2,
      ),

      // ── Headline ────────────────────────────────────────────────────────
      headlineLarge: GoogleFonts.atkinsonHyperlegible(
        textStyle: base.headlineLarge,
        fontSize: 34,
        fontWeight: FontWeight.w700,
        letterSpacing: -0.5,
        height: 1.2,
      ),
      headlineMedium: GoogleFonts.atkinsonHyperlegible(
        textStyle: base.headlineMedium,
        fontSize: 28,
        fontWeight: FontWeight.w700,
        letterSpacing: -0.3,
        height: 1.25,
      ),
      headlineSmall: GoogleFonts.atkinsonHyperlegible(
        textStyle: base.headlineSmall,
        fontSize: 24,
        fontWeight: FontWeight.w700,
        letterSpacing: -0.2,
        height: 1.3,
      ),

      // ── Title ───────────────────────────────────────────────────────────
      titleLarge: GoogleFonts.atkinsonHyperlegible(
        textStyle: base.titleLarge,
        fontSize: 20,
        fontWeight: FontWeight.w700,
        letterSpacing: -0.1,
        height: 1.3,
      ),
      titleMedium: GoogleFonts.atkinsonHyperlegible(
        textStyle: base.titleMedium,
        fontSize: 18,
        fontWeight: FontWeight.w700,
        letterSpacing: 0.05,
        height: 1.35,
      ),
      titleSmall: GoogleFonts.atkinsonHyperlegible(
        textStyle: base.titleSmall,
        fontSize: 15,
        fontWeight: FontWeight.w600,
        letterSpacing: 0.05,
        height: 1.4,
      ),

      // ── Body ────────────────────────────────────────────────────────────
      bodyLarge: GoogleFonts.sourceSans3(
        textStyle: base.bodyLarge,
        fontSize: 17,
        fontWeight: FontWeight.w400,
        letterSpacing: 0,
        height: 1.5,
      ),
      bodyMedium: GoogleFonts.sourceSans3(
        textStyle: base.bodyMedium,
        fontSize: 15,
        fontWeight: FontWeight.w400,
        letterSpacing: 0,
        height: 1.5,
      ),
      bodySmall: GoogleFonts.sourceSans3(
        textStyle: base.bodySmall,
        fontSize: 13,
        fontWeight: FontWeight.w400,
        letterSpacing: 0.1,
        height: 1.5,
      ),

      // ── Label ───────────────────────────────────────────────────────────
      labelLarge: GoogleFonts.sourceSans3(
        textStyle: base.labelLarge,
        fontSize: 15,
        fontWeight: FontWeight.w700,
        letterSpacing: 0.1,
        height: 1.4,
      ),
      labelMedium: GoogleFonts.sourceSans3(
        textStyle: base.labelMedium,
        fontSize: 13,
        fontWeight: FontWeight.w600,
        letterSpacing: 0.4,
        height: 1.4,
      ),
      labelSmall: GoogleFonts.sourceSans3(
        textStyle: base.labelSmall,
        fontSize: 11,
        fontWeight: FontWeight.w500,
        letterSpacing: 0.5,
        height: 1.4,
      ),
    );
  }
}
