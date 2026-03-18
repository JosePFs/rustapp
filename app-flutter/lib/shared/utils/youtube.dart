import 'package:url_launcher/url_launcher.dart';

String? _extractYouTubeVideoId(String url) {
  final trimmed = url.trim();
  if (trimmed.isEmpty) {
    return null;
  }

  String? normalizedId(String? candidate) {
    if (candidate == null) {
      return null;
    }
    final match = RegExp(r'[_\-a-zA-Z0-9]{11}').firstMatch(candidate.trim());
    return match?.group(0);
  }

  final fromQuery = RegExp(r'[?&]v=([^&#?/]+)').firstMatch(trimmed);
  final queryId = normalizedId(fromQuery?.group(1));
  if (queryId != null) {
    return queryId;
  }

  final fromEmbed = RegExp(r'/embed/([^&#?/]+)').firstMatch(trimmed);
  final embedId = normalizedId(fromEmbed?.group(1));
  if (embedId != null) {
    return embedId;
  }

  final fromShort = RegExp(r'youtu\\.be/([^&#?/]+)').firstMatch(trimmed);
  final shortId = normalizedId(fromShort?.group(1));
  if (shortId != null) {
    return shortId;
  }

  final fromPath = RegExp(r'/(shorts|live)/([^&#?/]+)').firstMatch(trimmed);
  final pathId = normalizedId(fromPath?.group(2));
  if (pathId != null) {
    return pathId;
  }

  final lastSegment = normalizedId(
    trimmed.split('/').last.split('?').first.split('&').first,
  );
  if (lastSegment != null) {
    return lastSegment;
  }

  final fallbackId = normalizedId(trimmed);
  if (fallbackId != null) {
    return fallbackId;
  }

  return null;
}

String? buildYouTubeEmbedUrl(String url) {
  final videoId = _extractYouTubeVideoId(url);
  if (videoId == null) {
    return null;
  }

  return 'https://www.youtube.com/embed/$videoId';
}

Uri? _buildExternalVideoLaunchUri(String url) {
  final trimmed = url.trim();
  if (trimmed.isEmpty) {
    return null;
  }

  final withScheme = trimmed.contains('://') ? trimmed : 'https://$trimmed';
  final uri = Uri.tryParse(withScheme);
  if (uri == null || !uri.hasScheme || uri.host.isEmpty) {
    return null;
  }

  return uri;
}

Future<void> openExternalVideoUrl(String url) async {
  final uri = _buildExternalVideoLaunchUri(url);
  if (uri == null) {
    throw const FormatException('Invalid video URL.');
  }

  final launched = await launchUrl(uri, mode: LaunchMode.externalApplication);
  if (!launched) {
    throw StateError('Unable to open external video URL.');
  }
}

bool shouldOpenEmbeddedVideoNavigationExternally(String url) {
  final uri = _buildExternalVideoLaunchUri(url);
  if (uri == null) {
    return false;
  }

  final scheme = uri.scheme.toLowerCase();
  if (scheme != 'http' && scheme != 'https') {
    return true;
  }

  final host = uri.host.toLowerCase();
  final path = uri.path.toLowerCase();
  final isYouTubeHost =
      host == 'youtube.com' ||
      host == 'www.youtube.com' ||
      host == 'm.youtube.com' ||
      host == 'youtu.be';
  if (!isYouTubeHost) {
    return false;
  }

  if (path.contains('/embed/')) {
    return false;
  }

  return true;
}

