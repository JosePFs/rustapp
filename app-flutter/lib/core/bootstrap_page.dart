import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

import 'package:app_flutter/features/patient_home/patient_home_page.dart';
import 'package:app_flutter/shared/widgets/app_brand_title.dart';
import 'package:app_flutter/src/rust/api.dart' as rust_api;
import 'package:app_flutter/src/rust/frb_generated.dart';
import 'bridge_runtime_config.dart';

enum BootstrapStage { starting, readyForLogin, error }

class PatientAppBootstrapPage extends StatefulWidget {
  const PatientAppBootstrapPage({
    required this.bridgeConfig,
    this.autoInitializeBridge = true,
    super.key,
  });

  final BridgeRuntimeConfig bridgeConfig;
  final bool autoInitializeBridge;

  @override
  State<PatientAppBootstrapPage> createState() =>
      _PatientAppBootstrapPageState();
}

class _PatientAppBootstrapPageState extends State<PatientAppBootstrapPage> {
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();

  bool _bridgeInitialized = false;
  bool _bridgeRuntimeInitialized = false;
  bool _busy = false;
  String _status = 'Starting app...';
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
        _initializeBridge();
      });
    }
  }

  Future<void> _initializeBridge() async {
    if (_bridgeInitialized || _busy) {
      return;
    }

    if (!widget.bridgeConfig.isConfigured) {
      setState(() {
        _stage = BootstrapStage.error;
        _status =
            'Missing Supabase configuration. Pass SUPABASE_URL and SUPABASE_ANON_KEY with --dart-define.';
      });
      return;
    }

    if (!widget.autoInitializeBridge) {
      setState(() {
        _bridgeInitialized = true;
        _stage = BootstrapStage.readyForLogin;
        _status = 'Bridge ready. You can sign in now.';
      });
      return;
    }

    setState(() {
      _busy = true;
      _status = 'Initializing Rust bridge...';
    });

    try {
      await RustLib.init(externalLibrary: _bridgeLibrary());
      setState(() {
        _bridgeInitialized = true;
        _bridgeRuntimeInitialized = true;
        _stage = BootstrapStage.readyForLogin;
        _status = 'Bridge ready. You can sign in now.';
      });
    } catch (error) {
      setState(() {
        _stage = BootstrapStage.error;
        _status = 'Bridge initialization failed: $error';
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
        _status =
            'Missing Supabase configuration. Pass SUPABASE_URL and SUPABASE_ANON_KEY with --dart-define.';
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
      _status = 'Calling Rust login...';
      _patientPrograms = const [];
    });

    try {
      final loginResponse = await rust_api.login(
        request: rust_api.LoginRequest(
          email: _emailController.text.trim(),
          password: _passwordController.text,
        ),
        config: widget.bridgeConfig.toBridgeConfig(),
      );
      final patientPrograms = await _loadPatientPrograms(
        loginResponse.accessToken,
      );
      setState(() {
        _stage = BootstrapStage.readyForLogin;
        _loginResponse = loginResponse;
        _patientPrograms = patientPrograms;
        _status =
            'Signed in as ${loginResponse.userProfileType}. Loaded ${patientPrograms.length} program(s).';
      });
    } catch (error) {
      setState(() {
        _stage = BootstrapStage.error;
        _status = 'Rust call failed: $error';
      });
    } finally {
      if (mounted) {
        setState(() {
          _busy = false;
        });
      }
    }
  }

  Future<List<rust_api.PatientProgramSummary>> _loadPatientPrograms(
    String token,
  ) {
    return rust_api.getPatientPrograms(
      token: token,
      config: widget.bridgeConfig.toBridgeConfig(),
    );
  }

  Future<void> _submitDayFeedback(
    rust_api.SubmitDayFeedbackRequest request,
  ) async {
    final loginResponse = _loginResponse;
    if (loginResponse == null) {
      throw StateError('Not signed in.');
    }

    setState(() {
      _busy = true;
      _status = 'Saving feedback...';
    });

    try {
      await rust_api.submitDayFeedback(
        token: loginResponse.accessToken,
        request: request,
        config: widget.bridgeConfig.toBridgeConfig(),
      );
      final patientPrograms = await _loadPatientPrograms(
        loginResponse.accessToken,
      );
      if (mounted) {
        setState(() {
          _patientPrograms = patientPrograms;
          _status = 'Feedback saved.';
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

  Future<void> _updateDayCompletion(
    rust_api.UpdateDayCompletionRequest request,
  ) async {
    final loginResponse = _loginResponse;
    if (loginResponse == null) {
      throw StateError('Not signed in.');
    }

    setState(() {
      _busy = true;
      _status = 'Updating session state...';
    });

    try {
      await rust_api.updateDayCompletion(
        token: loginResponse.accessToken,
        request: request,
        config: widget.bridgeConfig.toBridgeConfig(),
      );
      final patientPrograms = await _loadPatientPrograms(
        loginResponse.accessToken,
      );
      if (mounted) {
        setState(() {
          _patientPrograms = patientPrograms;
          _status = request.completed
              ? 'Session marked as completed.'
              : 'Session marked as not completed.';
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
        onUpdateDayCompletion: _updateDayCompletion,
      );
    }

    return Scaffold(
      appBar: AppBar(title: const AppBrandTitle()),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          Text('Welcome back', style: theme.textTheme.titleMedium),
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
            decoration: const InputDecoration(
              labelText: 'Patient email',
              border: OutlineInputBorder(),
            ),
          ),
          const SizedBox(height: 12),
          TextField(
            controller: _passwordController,
            decoration: const InputDecoration(
              labelText: 'Password',
              border: OutlineInputBorder(),
            ),
            obscureText: true,
          ),
          const SizedBox(height: 16),
          Wrap(
            spacing: 12,
            runSpacing: 12,
            children: [
              FilledButton(
                onPressed: _busy ? null : _loginAndLoadPrograms,
                child: const Text('Sign in'),
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
      _status = 'Signed out. You can sign in again.';
    });
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
                'Starting Eixe Patient Front...',
                style: theme.textTheme.headlineSmall,
              ),
              const SizedBox(height: 12),
              Text(
                widget.bridgeConfig.isConfigured
                    ? 'Preparing the app and connecting the Rust core.'
                    : 'Missing runtime configuration. Add SUPABASE_URL and SUPABASE_ANON_KEY with --dart-define.',
              ),
              const SizedBox(height: 24),
              if (_busy) const CircularProgressIndicator(),
              if (!_busy) ...[
                FilledButton(
                  onPressed: () async {
                    await _initializeBridge();
                  },
                  child: const Text('Continue'),
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
                'Unable to start the app',
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
                child: const Text('Retry'),
              ),
            ],
          ),
        ),
      );
    }

    return null;
  }
}
