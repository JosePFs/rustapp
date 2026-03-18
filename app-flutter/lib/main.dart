import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:dynamic_color/dynamic_color.dart';

import 'core/bootstrap_page.dart';
import 'core/bridge_runtime_config.dart';
import 'core/locale_controller.dart';
import 'core/theme/eixe_theme.dart';
import 'core/theme/eixe_colors.dart';
import 'l10n/app_localizations.dart';

void main() {
  runApp(MyApp(bridgeConfig: BridgeRuntimeConfig.fromEnvironment()));
}

class MyApp extends StatefulWidget {
  const MyApp({
    required this.bridgeConfig,
    this.autoInitializeBridge = true,
    super.key,
  });

  final BridgeRuntimeConfig bridgeConfig;
  final bool autoInitializeBridge;

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  final _localeController = LocaleController();
  bool _localeLoaded = false;

  @override
  void initState() {
    super.initState();
    _loadLocale();
  }

  Future<void> _loadLocale() async {
    await _localeController.load();
    if (!mounted) return;
    setState(() {
      _localeLoaded = true;
    });
  }

  @override
  Widget build(BuildContext context) {
    return ValueListenableBuilder<Locale?>(
      valueListenable: _localeController.localeListenable,
      builder: (context, locale, _) {
        return DynamicColorBuilder(
          builder: (lightDynamic, darkDynamic) {
            final lightBase =
                lightDynamic ??
                ColorScheme.fromSeed(
                  seedColor: EixeColors.brandGreen,
                  brightness: Brightness.light,
                );
            final darkBase =
                darkDynamic ??
                ColorScheme.fromSeed(
                  seedColor: EixeColors.brandGreen,
                  brightness: Brightness.dark,
                );

            final theme = EixeTheme.light(baseScheme: lightBase);
            final darkTheme = EixeTheme.dark(baseScheme: darkBase);

            return MaterialApp(
              locale: locale,
              supportedLocales: const [
                Locale('en'),
                Locale('es'),
                Locale('gl'),
              ],
              localeResolutionCallback: (deviceLocale, supportedLocales) {
                if (locale != null) {
                  return locale;
                }
                final languageCode = deviceLocale?.languageCode.toLowerCase();
                for (final supported in supportedLocales) {
                  if (supported.languageCode == languageCode) {
                    return supported;
                  }
                }
                return const Locale('en');
              },
              localizationsDelegates: const [
                AppLocalizations.delegate,
                GlobalMaterialLocalizations.delegate,
                GlobalWidgetsLocalizations.delegate,
                GlobalCupertinoLocalizations.delegate,
              ],
              onGenerateTitle: (context) =>
                  AppLocalizations.of(context)?.appTitle ?? 'Eixe',
              theme: theme,
              darkTheme: darkTheme,
              themeMode: ThemeMode.system,
              home: PatientAppBootstrapPage(
                bridgeConfig: widget.bridgeConfig,
                autoInitializeBridge: widget.autoInitializeBridge,
                localeController: _localeController,
                localeLoaded: _localeLoaded,
              ),
            );
          },
        );
      },
    );
  }
}
