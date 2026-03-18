import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:shared_preferences/shared_preferences.dart';

class LocaleController {
  LocaleController();

  static const _prefsKey = 'app_locale';
  static const deviceKey = 'device';

  final ValueNotifier<Locale?> _locale = ValueNotifier<Locale?>(null);

  ValueListenable<Locale?> get localeListenable => _locale;
  Locale? get locale => _locale.value;

  Future<void> load() async {
    final prefs = await SharedPreferences.getInstance();
    final raw = prefs.getString(_prefsKey) ?? deviceKey;
    _locale.value = _localeFromPref(raw);
  }

  Future<void> setLocale(Locale? locale) async {
    _locale.value = locale;
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_prefsKey, _prefFromLocale(locale));
  }

  static String _prefFromLocale(Locale? locale) =>
      locale == null ? deviceKey : locale.languageCode;

  static Locale? _localeFromPref(String raw) {
    if (raw == deviceKey) {
      return null;
    }
    final normalized = raw.trim().toLowerCase();
    if (normalized.isEmpty) {
      return null;
    }
    return Locale(normalized);
  }
}

