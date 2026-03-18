import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:webview_flutter/webview_flutter.dart';
import 'package:webview_flutter_android/webview_flutter_android.dart';
import 'package:webview_flutter_wkwebview/webview_flutter_wkwebview.dart';

import 'package:app_flutter/shared/utils/youtube.dart';

enum ExerciseVideoRenderMode { placeholder, youtubeIframe }

ExerciseVideoRenderMode chooseExerciseVideoRenderMode({
  required bool isTestEnv,
  required bool isAndroid,
  required bool isIos,
  required bool isLinux,
}) {
  if (isTestEnv) {
    return ExerciseVideoRenderMode.placeholder;
  }
  if (isAndroid || isIos) {
    return ExerciseVideoRenderMode.youtubeIframe;
  }
  return ExerciseVideoRenderMode.placeholder;
}

class ExerciseVideoPanel extends StatefulWidget {
  const ExerciseVideoPanel({
    required super.key,
    required this.exerciseId,
    required this.videoUrl,
  });

  final String exerciseId;
  final String videoUrl;

  @override
  State<ExerciseVideoPanel> createState() => _ExerciseVideoPanelState();
}

class _ExerciseVideoPanelState extends State<ExerciseVideoPanel> {
  late final String? _embedUrl;
  late final ExerciseVideoRenderMode _renderMode;
  late final Widget? _webViewWidget;

  @override
  void initState() {
    super.initState();
    _embedUrl = buildYouTubeEmbedUrl(widget.videoUrl);
    _renderMode = chooseExerciseVideoRenderMode(
      isTestEnv: Platform.environment.containsKey('FLUTTER_TEST'),
      isAndroid: Platform.isAndroid,
      isIos: Platform.isIOS,
      isLinux: Platform.isLinux,
    );

    if (_renderMode == ExerciseVideoRenderMode.youtubeIframe &&
        _embedUrl != null) {
      PlatformWebViewControllerCreationParams controllerParams =
          const PlatformWebViewControllerCreationParams();
      if (!kIsWeb && Platform.isAndroid) {
        controllerParams =
            AndroidWebViewControllerCreationParams.fromPlatformWebViewControllerCreationParams(
              controllerParams,
            );
      } else if (!kIsWeb && Platform.isIOS) {
        controllerParams =
            WebKitWebViewControllerCreationParams.fromPlatformWebViewControllerCreationParams(
              controllerParams,
              allowsInlineMediaPlayback: true,
              mediaTypesRequiringUserAction: const <PlaybackMediaTypes>{},
            );
      }

      final controller = WebViewController.fromPlatformCreationParams(
        controllerParams,
      );
      controller
        ..setNavigationDelegate(
          NavigationDelegate(
            onNavigationRequest: (request) async {
              if (shouldOpenEmbeddedVideoNavigationExternally(request.url)) {
                try {
                  await openExternalVideoUrl(request.url);
                } catch (_) {}
                return NavigationDecision.prevent;
              }
              return NavigationDecision.navigate;
            },
          ),
        )
        ..setJavaScriptMode(JavaScriptMode.unrestricted);

      if (!kIsWeb && Platform.isAndroid) {
        final androidController = controller.platform as AndroidWebViewController;
        androidController.setMediaPlaybackRequiresUserGesture(false);
      }

      controller.loadHtmlString(
        _buildYouTubeHtml(_embedUrl),
        baseUrl: 'https://org.eixe.patientfront/',
      );

      _webViewWidget = WebViewWidget(
        controller: controller,
        gestureRecognizers: {
          Factory<OneSequenceGestureRecognizer>(
            () => EagerGestureRecognizer(),
          ),
        },
      );
    } else {
      _webViewWidget = null;
    }
  }

  String _buildYouTubeHtml(String embedUrl) {
    return '''
<!DOCTYPE html>
<html>
<head>
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta name="referrer" content="strict-origin-when-cross-origin">
  <style>
    html, body { margin: 0; padding: 0; background: transparent; }
    iframe { border: 0; width: 100%; height: 100%; }
  </style>
</head>
<body>
  <iframe
    src="$embedUrl"
    allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture"
    allowfullscreen
  ></iframe>
</body>
</html>
''';
  }

  @override
  Widget build(BuildContext context) {
    switch (_renderMode) {
      case ExerciseVideoRenderMode.placeholder:
        return Container(
          height: 220,
          decoration: BoxDecoration(
            color: Theme.of(context).colorScheme.surfaceContainerHighest,
            borderRadius: BorderRadius.circular(12),
          ),
          alignment: Alignment.center,
          child: const Text('Exercise video'),
        );
      case ExerciseVideoRenderMode.youtubeIframe:
        if (_webViewWidget == null) {
          return const SizedBox.shrink();
        }
        return ClipRRect(
          borderRadius: BorderRadius.circular(12),
          child: SizedBox(
            height: 220,
            width: double.infinity,
            child: _webViewWidget,
          ),
        );
    }
  }
}

