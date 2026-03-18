import 'package:app_flutter/src/rust/api.dart' as rust_api;

class BridgeRuntimeConfig {
  const BridgeRuntimeConfig({
    required this.supabaseUrl,
    required this.supabaseAnonKey,
  });

  const BridgeRuntimeConfig.fromEnvironment()
    : supabaseUrl = const String.fromEnvironment('SUPABASE_URL'),
      supabaseAnonKey = const String.fromEnvironment('SUPABASE_ANON_KEY');

  final String supabaseUrl;
  final String supabaseAnonKey;

  bool get isConfigured =>
      supabaseUrl.trim().isNotEmpty && supabaseAnonKey.trim().isNotEmpty;

  rust_api.BridgeConfig toBridgeConfig() => rust_api.BridgeConfig(
    url: supabaseUrl.trim(),
    anonKey: supabaseAnonKey.trim(),
  );
}

