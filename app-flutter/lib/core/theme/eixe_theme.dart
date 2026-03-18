import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'eixe_colors.dart';
import 'eixe_typography.dart';

class EixeTheme {
  EixeTheme._();

  static ThemeData light({required ColorScheme baseScheme}) {
    final brandScheme = ColorScheme.fromSeed(
      seedColor: EixeColors.brandGreen,
      brightness: Brightness.light,
    );
    final scheme = _normalizeScheme(
      baseScheme: baseScheme,
      brandScheme: brandScheme,
      brightness: Brightness.light,
    );
    return _buildThemeData(scheme);
  }

  static ThemeData dark({required ColorScheme baseScheme}) {
    final brandScheme = ColorScheme.fromSeed(
      seedColor: EixeColors.brandGreen,
      brightness: Brightness.dark,
    );
    final scheme =
        _normalizeScheme(
          baseScheme: baseScheme,
          brandScheme: brandScheme,
          brightness: Brightness.dark,
        ).copyWith(
          primary: EixeColors.brandGreenDark,
          onPrimary: EixeColors.onGreenContainer,
          primaryContainer: EixeColors.greenContainerDark,
          onPrimaryContainer: EixeColors.greenContainer,
          tertiary: EixeColors.actionAmberDark,
          onTertiary: EixeColors.onAmberContainer,
          tertiaryContainer: EixeColors.amberContainerDark,
          onTertiaryContainer: EixeColors.amberContainer,
        );
    return _buildThemeData(scheme);
  }

  // ── Scheme ──────────────────────────────────────────────────────────────────

  static ColorScheme _normalizeScheme({
    required ColorScheme baseScheme,
    required ColorScheme brandScheme,
    required Brightness brightness,
  }) {
    final surface = EixeColors.warmSurface(baseScheme.surface);
    final surfaceContainerHighest = EixeColors.warmSurface(
      baseScheme.surfaceContainerHighest,
    );

    return baseScheme.copyWith(
      brightness: brightness,
      primary: EixeColors.brandGreen,
      onPrimary: brandScheme.onPrimary,
      primaryContainer: EixeColors.greenContainer,
      onPrimaryContainer: EixeColors.onGreenContainer,
      tertiary: EixeColors.actionAmber,
      onTertiary:
          ThemeData.estimateBrightnessForColor(EixeColors.actionAmber) ==
              Brightness.dark
          ? Colors.white
          : Colors.black,
      tertiaryContainer: EixeColors.amberContainer,
      onTertiaryContainer: EixeColors.onAmberContainer,
      secondary: baseScheme.secondary,
      onSecondary: baseScheme.onSecondary,
      surface: surface,
      surfaceContainerHighest: surfaceContainerHighest,
      outline: baseScheme.outline,
      outlineVariant: baseScheme.outlineVariant,
      error: EixeColors.error,
      onError: EixeColors.onError,
      errorContainer: EixeColors.errorContainer,
      onErrorContainer: EixeColors.onErrorContainer,
    );
  }

  // ── Builder ─────────────────────────────────────────────────────────────────

  static ThemeData _buildThemeData(ColorScheme scheme) {
    final isLight = scheme.brightness == Brightness.light;
    final border = EixeColors.hairlineBorder(scheme);

    const radius8 = BorderRadius.all(Radius.circular(8));
    const radius12 = BorderRadius.all(Radius.circular(12));
    const radius16 = BorderRadius.all(Radius.circular(16));

    SystemChrome.setSystemUIOverlayStyle(
      isLight
          ? SystemUiOverlayStyle.dark.copyWith(
              statusBarColor: Colors.transparent,
              systemNavigationBarColor: scheme.surface,
            )
          : SystemUiOverlayStyle.light.copyWith(
              statusBarColor: Colors.transparent,
              systemNavigationBarColor: scheme.surface,
            ),
    );

    final base = ThemeData(
      useMaterial3: true,
      colorScheme: scheme,
      scaffoldBackgroundColor: scheme.surface,

      // ── AppBar ────────────────────────────────────────────────────────────
      appBarTheme: AppBarTheme(
        elevation: 0,
        scrolledUnderElevation: 1,
        backgroundColor: scheme.surface,
        foregroundColor: scheme.onSurface,
        shadowColor: scheme.shadow.withValues(alpha: 0.08),
        surfaceTintColor: Colors.transparent,
        iconTheme: IconThemeData(color: scheme.onSurface, size: 22),
        actionsIconTheme: IconThemeData(color: scheme.onSurface, size: 22),
        systemOverlayStyle: isLight
            ? SystemUiOverlayStyle.dark
            : SystemUiOverlayStyle.light,
      ),

      // ── Divider ───────────────────────────────────────────────────────────
      dividerTheme: DividerThemeData(space: 1, thickness: 1, color: border),

      // ── Card ──────────────────────────────────────────────────────────────
      cardTheme: CardThemeData(
        elevation: 0,
        color: scheme.surfaceContainerLow,
        margin: EdgeInsets.zero,
        shape: RoundedRectangleBorder(
          borderRadius: radius12,
          side: BorderSide(color: border),
        ),
      ),

      // ── ListTile ──────────────────────────────────────────────────────────
      listTileTheme: ListTileThemeData(
        contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 4),
        minLeadingWidth: 16,
        shape: RoundedRectangleBorder(borderRadius: radius12),
        tileColor: Colors.transparent,
        selectedTileColor: scheme.primaryContainer.withValues(alpha: 0.5),
        iconColor: scheme.onSurfaceVariant,
      ),

      // ── Input ─────────────────────────────────────────────────────────────
      inputDecorationTheme: InputDecorationTheme(
        filled: true,
        fillColor: scheme.surfaceContainerLowest,
        border: OutlineInputBorder(
          borderRadius: radius12,
          borderSide: BorderSide(color: border),
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: radius12,
          borderSide: BorderSide(color: border),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: radius12,
          borderSide: BorderSide(color: scheme.primary, width: 1.8),
        ),
        errorBorder: OutlineInputBorder(
          borderRadius: radius12,
          borderSide: BorderSide(color: scheme.error),
        ),
        focusedErrorBorder: OutlineInputBorder(
          borderRadius: radius12,
          borderSide: BorderSide(color: scheme.error, width: 1.8),
        ),
        contentPadding: const EdgeInsets.symmetric(
          horizontal: 14,
          vertical: 14,
        ),
        prefixIconColor: scheme.onSurfaceVariant,
        suffixIconColor: scheme.onSurfaceVariant,
      ),

      // ── Buttons ───────────────────────────────────────────────────────────
      filledButtonTheme: FilledButtonThemeData(
        style: FilledButton.styleFrom(
          backgroundColor: scheme.primary,
          foregroundColor: scheme.onPrimary,
          disabledBackgroundColor: scheme.onSurface.withValues(alpha: 0.12),
          shape: const RoundedRectangleBorder(borderRadius: radius8),
          padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 16),
          elevation: 0,
        ),
      ),
      outlinedButtonTheme: OutlinedButtonThemeData(
        style: OutlinedButton.styleFrom(
          foregroundColor: scheme.primary,
          side: BorderSide(color: scheme.primary.withValues(alpha: 0.6)),
          shape: const RoundedRectangleBorder(borderRadius: radius8),
          padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 16),
        ),
      ),
      textButtonTheme: TextButtonThemeData(
        style: TextButton.styleFrom(
          foregroundColor: scheme.primary,
          shape: const RoundedRectangleBorder(borderRadius: radius8),
          padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 12),
        ),
      ),
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          backgroundColor: scheme.surfaceContainerHigh,
          foregroundColor: scheme.onSurface,
          elevation: 0,
          shape: const RoundedRectangleBorder(borderRadius: radius8),
          padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 16),
        ),
      ),

      // ── Chip ──────────────────────────────────────────────────────────────
      chipTheme: ChipThemeData(
        backgroundColor: scheme.surfaceContainerLow,
        selectedColor: scheme.primaryContainer,
        disabledColor: scheme.onSurface.withValues(alpha: 0.08),
        side: BorderSide(color: border),
        shape: const RoundedRectangleBorder(
          borderRadius: BorderRadius.all(Radius.circular(8)),
        ),
        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      ),

      // ── NavigationBar ─────────────────────────────────────────────────────
      navigationBarTheme: NavigationBarThemeData(
        backgroundColor: scheme.surfaceContainer,
        indicatorColor: scheme.primaryContainer,
        iconTheme: WidgetStateProperty.resolveWith((states) {
          if (states.contains(WidgetState.selected)) {
            return IconThemeData(color: scheme.onPrimaryContainer, size: 22);
          }
          return IconThemeData(
            color: scheme.onSurfaceVariant.withValues(alpha: 0.8),
            size: 22,
          );
        }),
        elevation: 0,
        shadowColor: Colors.transparent,
        surfaceTintColor: Colors.transparent,
      ),

      // ── SnackBar ──────────────────────────────────────────────────────────
      snackBarTheme: SnackBarThemeData(
        behavior: SnackBarBehavior.fixed,
        backgroundColor: scheme.inverseSurface,
        contentTextStyle: TextStyle(color: scheme.onInverseSurface),
        actionTextColor: scheme.inversePrimary,
        shape: const RoundedRectangleBorder(
          borderRadius: BorderRadius.vertical(top: Radius.circular(12)),
        ),
      ),

      // ── BottomSheet ───────────────────────────────────────────────────────
      bottomSheetTheme: BottomSheetThemeData(
        backgroundColor: scheme.surfaceContainerLow,
        modalBackgroundColor: scheme.surfaceContainerLow,
        shape: const RoundedRectangleBorder(
          borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
        ),
        dragHandleColor: scheme.onSurfaceVariant.withValues(alpha: 0.4),
        dragHandleSize: const Size(36, 4),
        elevation: 0,
        showDragHandle: true,
      ),

      // ── Dialog ────────────────────────────────────────────────────────────
      dialogTheme: DialogThemeData(
        backgroundColor: scheme.surfaceContainerLow,
        surfaceTintColor: Colors.transparent,
        elevation: 0,
        shape: const RoundedRectangleBorder(borderRadius: radius16),
      ),

      // ── FAB ───────────────────────────────────────────────────────────────
      floatingActionButtonTheme: FloatingActionButtonThemeData(
        backgroundColor: scheme.primary,
        foregroundColor: scheme.onPrimary,
        elevation: 2,
        shape: const RoundedRectangleBorder(borderRadius: radius16),
      ),

      // ── Switch ────────────────────────────────────────────────────────────
      switchTheme: SwitchThemeData(
        thumbColor: WidgetStateProperty.resolveWith((states) {
          if (states.contains(WidgetState.selected)) return Colors.white;
          return scheme.onSurfaceVariant;
        }),
        trackColor: WidgetStateProperty.resolveWith((states) {
          if (states.contains(WidgetState.selected)) return scheme.primary;
          return scheme.surfaceContainerHighest;
        }),
      ),

      // ── Checkbox ──────────────────────────────────────────────────────────
      checkboxTheme: CheckboxThemeData(
        fillColor: WidgetStateProperty.resolveWith((states) {
          if (states.contains(WidgetState.selected)) return scheme.primary;
          return Colors.transparent;
        }),
        checkColor: WidgetStateProperty.all(Colors.white),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(4)),
        side: BorderSide(color: scheme.outline, width: 1.5),
      ),

      // ── ProgressIndicator ─────────────────────────────────────────────────
      progressIndicatorTheme: ProgressIndicatorThemeData(
        color: scheme.primary,
        linearTrackColor: scheme.primaryContainer,
        circularTrackColor: scheme.primaryContainer,
        linearMinHeight: 6,
        borderRadius: BorderRadius.circular(8),
      ),

      // ── TabBar ────────────────────────────────────────────────────────────
      tabBarTheme: TabBarThemeData(
        labelColor: scheme.primary,
        unselectedLabelColor: scheme.onSurfaceVariant,
        indicatorColor: scheme.primary,
        indicatorSize: TabBarIndicatorSize.label,
        dividerColor: border,
      ),
    );

    return base.copyWith(
      textTheme: EixeTypography.build(base.textTheme),
      appBarTheme: base.appBarTheme.copyWith(
        titleTextStyle: EixeTypography.build(
          base.textTheme,
        ).titleLarge?.copyWith(color: scheme.onSurface),
      ),
    );
  }
}
