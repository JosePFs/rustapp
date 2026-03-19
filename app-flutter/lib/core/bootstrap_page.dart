import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import 'package:app_flutter/features/patient_home/patient_home_page.dart';
import 'package:app_flutter/shared/widgets/app_brand_title.dart';
import 'package:app_flutter/core/locale_controller.dart';
import 'package:app_flutter/core/session_store.dart';
import 'package:app_flutter/l10n/app_localizations_ext.dart';
import 'package:app_flutter/src/rust/api.dart' as rust_api;
import 'package:app_flutter/src/rust/frb_generated.dart';
import 'bridge_runtime_config.dart';

enum BootstrapStage { starting, readyForLogin, error }

class _SessionBound<T> {
  const _SessionBound({required this.session, required this.value});

  final rust_api.LoginResponse session;
  final T value;
}

class PatientAppBootstrapPage extends StatefulWidget {
  const PatientAppBootstrapPage({
    required this.bridgeConfig,
    required this.localeController,
    required this.localeLoaded,
    this.autoInitializeBridge = true,
    super.key,
  });

  final BridgeRuntimeConfig bridgeConfig;
  final LocaleController localeController;
  final bool localeLoaded;
  final bool autoInitializeBridge;

  @override
  State<PatientAppBootstrapPage> createState() =>
      _PatientAppBootstrapPageState();
}

class _PatientAppBootstrapPageState extends State<PatientAppBootstrapPage> {
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();
  final _sessionStore = SessionStore();

  bool _bridgeInitialized = false;
  bool _bridgeRuntimeInitialized = false;
  bool _busy = false;
  bool _restoringSession = false;
  String _status = 'Starting app...';
  String? _loginErrorMessage;
  BootstrapStage _stage = BootstrapStage.starting;
  rust_api.LoginResponse? _loginResponse;
  List<rust_api.PatientProgramSummary> _patientPrograms = const [];

  ExternalLibrary? _bridgeLibrary() {
    if (kIsWeb) {
      return null;
    }

    if (Platform.isAndroid) {
      return ExternalLibrary.open('libmobile_bridge_frb.so');
    }

    if (Platform.isLinux) {
      final executableDir = File(Platform.resolvedExecutable).parent;
      final bundledLibrary = File(
        '${executableDir.path}/lib/libmobile_bridge_frb.so',
      );
      if (bundledLibrary.existsSync()) {
        return ExternalLibrary.open(bundledLibrary.path);
      }

      final repoReleaseLibrary = File(
        '${Directory.current.path}/../target/release/libmobile_bridge_frb.so',
      );
      if (repoReleaseLibrary.existsSync()) {
        return ExternalLibrary.open(repoReleaseLibrary.path);
      }

      return ExternalLibrary.open('libmobile_bridge_frb.so');
    }

    return null;
  }

  @override
  void initState() {
    super.initState();
    if (widget.autoInitializeBridge) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        _initializeBridge().then((_) {
          _tryRestoreSession();
        });
      });
    }
  }

  Future<void> _promoteSession(rust_api.LoginResponse session) async {
    await _sessionStore.save(session);
    if (!mounted) {
      _loginResponse = session;
      return;
    }
    setState(() {
      _loginResponse = session;
    });
  }

  bool _shouldClearStoredSession(Object error) {
    final text = error.toString();
    if (text.contains('Missing refresh token')) return true;
    // Refresh grant is rejected (token revoked/expired/etc).
    if (text.contains('Auth refresh failed: status 400')) return true;
    if (text.contains('Auth refresh failed: status 401')) return true;
    if (text.contains('Auth refresh failed: status 403')) return true;
    return false;
  }

  Future<void> _tryRestoreSession() async {
    if (_busy || !_bridgeInitialized || !widget.bridgeConfig.isConfigured) {
      return;
    }
    setState(() {
      _restoringSession = true;
    });
    final stored = await _sessionStore.read();
    if (!mounted || stored == null) {
      if (mounted) {
        setState(() {
          _restoringSession = false;
        });
      }
      return;
    }

    setState(() {
      _busy = true;
      _status = context.l10n.statusCallingLogin;
    });

    try {
      final result = await _loadPatientProgramsWithRefresh(stored);
      if (!mounted) return;
      if (result.session.accessToken != stored.accessToken ||
          result.session.refreshToken != stored.refreshToken) {
        await _promoteSession(result.session);
      } else {
        _loginResponse = stored;
      }
      setState(() {
        _patientPrograms = result.value;
        _stage = BootstrapStage.readyForLogin;
        _status = context.l10n.statusSignedInLoadedPrograms(
          result.session.userProfileType,
          result.value.length,
        );
      });
    } catch (error) {
      if (_shouldClearStoredSession(error)) {
        await _sessionStore.clear();
      }
      if (!mounted) return;
      setState(() {
        _loginResponse = null;
        _patientPrograms = const [];
        // Keep the app usable even if restore fails.
        _stage = BootstrapStage.readyForLogin;
        _status = context.l10n.errorRustCallFailed(error);
      });
    } finally {
      if (mounted) {
        setState(() {
          _busy = false;
          _restoringSession = false;
        });
      }
    }
  }

  Future<void> _initializeBridge() async {
    if (_bridgeInitialized || _busy) {
      return;
    }

    if (!widget.bridgeConfig.isConfigured) {
      setState(() {
        _stage = BootstrapStage.error;
        _status = context.l10n.errorMissingSupabaseConfig;
      });
      return;
    }

    if (!widget.autoInitializeBridge) {
      setState(() {
        _bridgeInitialized = true;
        _stage = BootstrapStage.readyForLogin;
        _status = context.l10n.statusBridgeReady;
      });
      return;
    }

    setState(() {
      _busy = true;
      _status = context.l10n.statusInitializingBridge;
    });

    try {
      await RustLib.init(externalLibrary: _bridgeLibrary());
      setState(() {
        _bridgeInitialized = true;
        _bridgeRuntimeInitialized = true;
        _stage = BootstrapStage.readyForLogin;
        _status = context.l10n.statusBridgeReady;
      });
    } catch (error) {
      setState(() {
        _stage = BootstrapStage.error;
        _status = context.l10n.errorBridgeInitFailed(error);
      });
    } finally {
      if (mounted) {
        setState(() {
          _busy = false;
        });
      }
    }
  }

  Future<void> _loginAndLoadPrograms() async {
    if (!widget.bridgeConfig.isConfigured) {
      setState(() {
        _stage = BootstrapStage.error;
        _status = context.l10n.errorMissingSupabaseConfig;
      });
      return;
    }

    if (!_bridgeInitialized) {
      await _initializeBridge();
      if (!_bridgeInitialized) {
        return;
      }
    }

    setState(() {
      _busy = true;
      _status = context.l10n.statusCallingLogin;
      _patientPrograms = const [];
      _loginErrorMessage = null;
    });

    try {
      final loginResponse = await rust_api.login(
        request: rust_api.LoginRequest(
          email: _emailController.text.trim(),
          password: _passwordController.text,
        ),
        config: widget.bridgeConfig.toBridgeConfig(),
      );
      final result = await _loadPatientProgramsWithRefresh(loginResponse);
      await _promoteSession(result.session);
      setState(() {
        _stage = BootstrapStage.readyForLogin;
        _patientPrograms = result.value;
        _status = context.l10n.statusSignedInLoadedPrograms(
          result.session.userProfileType,
          result.value.length,
        );
      });
    } catch (error) {
      final errorText = error.toString();
      final isWrongCredentials =
          errorText.contains('wrong_credentials') ||
          errorText.contains('wrong credentials') ||
          errorText.contains('Auth failed: status 400');
      setState(() {
        _stage = BootstrapStage.readyForLogin;
        _loginResponse = null;
        _loginErrorMessage = isWrongCredentials
            ? context.l10n.authLoginFailedWrongCredentials
            : context.l10n.authLoginFailedGeneric;
        _status = context.l10n.errorRustCallFailed(error);
      });
    } finally {
      if (mounted) {
        setState(() {
          _busy = false;
        });
      }
    }
  }

  bool _isAuthFailure(Object error) {
    final text = error.toString();
    return text.contains('status 401') ||
        text.contains('status 403') ||
        text.contains('Auth failed: status 401') ||
        text.contains('Auth failed: status 403');
  }

  Future<rust_api.LoginResponse> _refreshSessionOrThrow(
    rust_api.LoginResponse current,
  ) async {
    final refreshToken = current.refreshToken;
    if (refreshToken == null || refreshToken.trim().isEmpty) {
      throw StateError('Missing refresh token.');
    }
    return rust_api.refreshSession(
      refreshToken: refreshToken,
      config: widget.bridgeConfig.toBridgeConfig(),
    );
  }

  Future<_SessionBound<T>> _withAuthRetry<T>(
    rust_api.LoginResponse session,
    Future<T> Function(String accessToken) operation,
  ) async {
    try {
      final value = await operation(session.accessToken);
      return _SessionBound(session: session, value: value);
    } catch (error) {
      if (!_isAuthFailure(error)) rethrow;
      final refreshed = await _refreshSessionOrThrow(session);
      final value = await operation(refreshed.accessToken);
      return _SessionBound(session: refreshed, value: value);
    }
  }

  Future<_SessionBound<List<rust_api.PatientProgramSummary>>>
  _loadPatientProgramsWithRefresh(rust_api.LoginResponse session) {
    return _withAuthRetry(
      session,
      (token) => rust_api.getPatientPrograms(
        token: token,
        config: widget.bridgeConfig.toBridgeConfig(),
      ),
    );
  }

  Future<void> _submitDayFeedback(
    rust_api.MarkDayAsCompletedRequest request,
  ) async {
    final loginResponse = _loginResponse;
    if (loginResponse == null) {
      throw StateError('Not signed in.');
    }

    setState(() {
      _busy = true;
      _status = context.l10n.statusSavingFeedback;
    });

    try {
      final submitResult = await _withAuthRetry(
        loginResponse,
        (token) => rust_api.markDayAsCompleted(
          token: token,
          request: request,
          config: widget.bridgeConfig.toBridgeConfig(),
        ),
      );
      if (submitResult.session.accessToken != loginResponse.accessToken ||
          submitResult.session.refreshToken != loginResponse.refreshToken) {
        await _promoteSession(submitResult.session);
      }
      final programsResult = await _withAuthRetry(
        submitResult.session,
        (token) => rust_api.getPatientPrograms(
          token: token,
          config: widget.bridgeConfig.toBridgeConfig(),
        ),
      );
      if (programsResult.session.accessToken !=
              submitResult.session.accessToken ||
          programsResult.session.refreshToken !=
              submitResult.session.refreshToken) {
        await _promoteSession(programsResult.session);
      }
      if (mounted) {
        setState(() {
          _patientPrograms = programsResult.value;
          _status = context.l10n.statusSavingFeedback;
        });
      }
    } finally {
      if (mounted) {
        setState(() {
          _busy = false;
        });
      }
    }
  }

  Future<void> _updateDayAsUnCompleted(
    rust_api.MarkDayAsUncompletedRequest request,
  ) async {
    final loginResponse = _loginResponse;
    if (loginResponse == null) {
      throw StateError('Not signed in.');
    }

    setState(() {
      _busy = true;
      _status = context.l10n.statusUpdatingSessionState;
    });

    try {
      final updateResult = await _withAuthRetry(
        loginResponse,
        (token) => rust_api.markDayAsUncompleted(
          token: token,
          request: request,
          config: widget.bridgeConfig.toBridgeConfig(),
        ),
      );
      if (updateResult.session.accessToken != loginResponse.accessToken ||
          updateResult.session.refreshToken != loginResponse.refreshToken) {
        await _promoteSession(updateResult.session);
      }
      final programsResult = await _withAuthRetry(
        updateResult.session,
        (token) => rust_api.getPatientPrograms(
          token: token,
          config: widget.bridgeConfig.toBridgeConfig(),
        ),
      );
      if (programsResult.session.accessToken !=
              updateResult.session.accessToken ||
          programsResult.session.refreshToken !=
              updateResult.session.refreshToken) {
        await _promoteSession(programsResult.session);
      }
      if (mounted) {
        setState(() {
          _patientPrograms = programsResult.value;
          _status = request.workoutSessionId.isNotEmpty
              ? context.l10n.statusSessionMarkedCompleted
              : context.l10n.statusSessionMarkedNotCompleted;
        });
      }
    } finally {
      if (mounted) {
        setState(() {
          _busy = false;
        });
      }
    }
  }

  @override
  void dispose() {
    _emailController.dispose();
    _passwordController.dispose();
    if (_bridgeRuntimeInitialized) {
      RustLib.dispose();
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final bootstrapBody = _buildBootstrapBody(theme);
    if (bootstrapBody != null) {
      return bootstrapBody;
    }

    if (_loginResponse != null) {
      return PatientHomePage(
        loginResponse: _loginResponse!,
        patientPrograms: _patientPrograms,
        onSignOut: _signOut,
        onSubmitDayFeedback: _submitDayFeedback,
        onMarkDayAsUnCompleted: _updateDayAsUnCompleted,
        localeController: widget.localeController,
        localeLoaded: widget.localeLoaded,
      );
    }

    if (_restoringSession) {
      return Scaffold(
        appBar: AppBar(title: const AppBrandTitle()),
        body: Center(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                const CircularProgressIndicator(),
                const SizedBox(height: 16),
                Text(
                  context.l10n.statusStartingApp,
                  style: theme.textTheme.titleMedium,
                  textAlign: TextAlign.center,
                ),
              ],
            ),
          ),
        ),
      );
    }

    return Scaffold(
      appBar: AppBar(title: const AppBrandTitle()),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          Text(
            context.l10n.bootstrapWelcomeBack,
            style: theme.textTheme.titleMedium,
          ),
          const SizedBox(height: 16),
          if (kDebugMode)
            Text(
              widget.bridgeConfig.isConfigured
                  ? 'Supabase configuration loaded from Dart defines.'
                  : 'Supabase configuration missing. Use --dart-define for SUPABASE_URL and SUPABASE_ANON_KEY.',
            ),
          const SizedBox(height: 12),
          TextField(
            controller: _emailController,
            decoration: InputDecoration(
              labelText: context.l10n.authEmailLabel,
              hintText: context.l10n.authEmailHint,
              border: const OutlineInputBorder(),
            ),
          ),
          const SizedBox(height: 12),
          TextField(
            controller: _passwordController,
            decoration: InputDecoration(
              labelText: context.l10n.authPasswordLabel,
              hintText: context.l10n.authPasswordHint,
              border: const OutlineInputBorder(),
            ),
            obscureText: true,
            textInputAction: TextInputAction.done,
            onSubmitted: (_) {
              if (_busy) return;
              _loginAndLoadPrograms();
            },
          ),
          if (_loginErrorMessage != null) ...[
            const SizedBox(height: 12),
            DecoratedBox(
              decoration: BoxDecoration(
                color: theme.colorScheme.errorContainer.withValues(alpha: 0.55),
                borderRadius: BorderRadius.circular(12),
                border: Border.all(
                  color: theme.colorScheme.error.withValues(alpha: 0.35),
                ),
              ),
              child: Padding(
                padding: const EdgeInsets.symmetric(
                  horizontal: 12,
                  vertical: 10,
                ),
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Icon(
                      Icons.error_outline,
                      size: 18,
                      color: theme.colorScheme.error,
                    ),
                    const SizedBox(width: 10),
                    Expanded(
                      child: Text(
                        '${context.l10n.authLoginFailedTitle}\n$_loginErrorMessage',
                        style: theme.textTheme.bodyMedium?.copyWith(
                          color: theme.colorScheme.onErrorContainer,
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ],
          const SizedBox(height: 16),
          Wrap(
            spacing: 12,
            runSpacing: 12,
            children: [
              FilledButton(
                onPressed: _busy ? null : _loginAndLoadPrograms,
                child: Text(context.l10n.authSignIn),
              ),
            ],
          ),
          const SizedBox(height: 16),
          if (kDebugMode) Text(_status),
        ],
      ),
    );
  }

  void _signOut() {
    setState(() {
      _loginResponse = null;
      _patientPrograms = const [];
      _emailController.clear();
      _passwordController.clear();
      _status = context.l10n.statusSignedOut;
    });
    _sessionStore.clear();
  }

  Widget? _buildBootstrapBody(ThemeData theme) {
    if (_stage == BootstrapStage.starting) {
      return Scaffold(
        appBar: AppBar(title: const AppBrandTitle()),
        body: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                context.l10n.statusStartingApp,
                style: theme.textTheme.headlineSmall,
              ),
              const SizedBox(height: 12),
              Text(
                widget.bridgeConfig.isConfigured
                    ? context.l10n.statusInitializingBridge
                    : context.l10n.errorMissingSupabaseConfig,
              ),
              const SizedBox(height: 24),
              if (_busy) const CircularProgressIndicator(),
              if (!_busy) ...[
                FilledButton(
                  onPressed: () async {
                    await _initializeBridge();
                  },
                  child: Text(context.l10n.bootstrapContinue),
                ),
              ],
            ],
          ),
        ),
      );
    }

    if (_stage == BootstrapStage.error) {
      return Scaffold(
        appBar: AppBar(title: const AppBrandTitle()),
        body: Padding(
          padding: const EdgeInsets.all(24),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                context.l10n.bootstrapUnableToStartTitle,
                style: theme.textTheme.headlineSmall,
              ),
              const SizedBox(height: 12),
              Text(_status),
              const SizedBox(height: 24),
              FilledButton(
                onPressed: _busy
                    ? null
                    : () async {
                        setState(() {
                          _stage = BootstrapStage.starting;
                        });
                        await _initializeBridge();
                      },
                child: Text(context.l10n.bootstrapRetry),
              ),
            ],
          ),
        ),
      );
    }

    return null;
  }
}
