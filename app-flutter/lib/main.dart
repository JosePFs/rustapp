import 'package:flutter/material.dart';

import 'core/bootstrap_page.dart';
import 'core/bridge_runtime_config.dart';

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
    const brandSeedColor = Color(0xFF35B339);
    final baseScheme = ColorScheme.fromSeed(seedColor: brandSeedColor);
    return MaterialApp(
      title: 'Eixe Patient Front',
      theme: ThemeData(
        useMaterial3: true,
        colorScheme: baseScheme,
        scaffoldBackgroundColor: baseScheme.surface,
        dividerTheme: const DividerThemeData(space: 1, thickness: 1),
        cardTheme: CardThemeData(
          elevation: 0,
          color: baseScheme.surface,
          margin: EdgeInsets.zero,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(12),
            side: const BorderSide(color: Color(0x14000000)),
          ),
        ),
        listTileTheme: const ListTileThemeData(
          contentPadding: EdgeInsets.symmetric(horizontal: 16, vertical: 6),
          minLeadingWidth: 16,
        ),
        inputDecorationTheme: InputDecorationTheme(
          border: const OutlineInputBorder(),
          enabledBorder: OutlineInputBorder(
            borderRadius: const BorderRadius.all(Radius.circular(12)),
            borderSide: BorderSide(color: Colors.black.withValues(alpha: 0.10)),
          ),
          focusedBorder: const OutlineInputBorder(
            borderRadius: BorderRadius.all(Radius.circular(12)),
            borderSide: BorderSide(color: brandSeedColor, width: 1.6),
          ),
          contentPadding: const EdgeInsets.symmetric(
            horizontal: 14,
            vertical: 14,
          ),
        ),
        textTheme: const TextTheme(
          headlineSmall: TextStyle(fontSize: 26, fontWeight: FontWeight.w700),
          titleLarge: TextStyle(fontSize: 20, fontWeight: FontWeight.w700),
          titleMedium: TextStyle(fontSize: 18, fontWeight: FontWeight.w600),
          bodyLarge: TextStyle(fontSize: 17),
          bodyMedium: TextStyle(fontSize: 16),
          bodySmall: TextStyle(fontSize: 15),
          labelLarge: TextStyle(fontSize: 16, fontWeight: FontWeight.w600),
        ),
        filledButtonTheme: FilledButtonThemeData(
          style: FilledButton.styleFrom(
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(8),
            ),
            padding: const EdgeInsets.symmetric(horizontal: 18, vertical: 16),
          ),
        ),
        outlinedButtonTheme: OutlinedButtonThemeData(
          style: OutlinedButton.styleFrom(
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(8),
            ),
            padding: const EdgeInsets.symmetric(horizontal: 18, vertical: 16),
          ),
        ),
        textButtonTheme: TextButtonThemeData(
          style: TextButton.styleFrom(
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(8),
            ),
            padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 12),
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
