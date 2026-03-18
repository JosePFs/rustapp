import 'package:flutter_secure_storage/flutter_secure_storage.dart';

import 'package:app_flutter/src/rust/api.dart' as rust_api;

class SessionStore {
  SessionStore({FlutterSecureStorage? storage})
      : _storage = storage ?? const FlutterSecureStorage();

  static const _kAccessToken = 'session_access_token';
  static const _kRefreshToken = 'session_refresh_token';
  static const _kUserId = 'session_user_id';
  static const _kUserProfileType = 'session_user_profile_type';

  final FlutterSecureStorage _storage;

  Future<void> save(rust_api.LoginResponse session) async {
    await _storage.write(key: _kAccessToken, value: session.accessToken);
    await _storage.write(key: _kRefreshToken, value: session.refreshToken);
    await _storage.write(key: _kUserId, value: session.userId);
    await _storage.write(key: _kUserProfileType, value: session.userProfileType);
  }

  Future<rust_api.LoginResponse?> read() async {
    final accessToken = await _storage.read(key: _kAccessToken);
    final userId = await _storage.read(key: _kUserId);
    final userProfileType = await _storage.read(key: _kUserProfileType);
    if (accessToken == null || userId == null || userProfileType == null) {
      return null;
    }
    final refreshToken = await _storage.read(key: _kRefreshToken);
    return rust_api.LoginResponse(
      accessToken: accessToken,
      refreshToken: refreshToken,
      userId: userId,
      userProfileType: userProfileType,
    );
  }

  Future<void> clear() async {
    await _storage.delete(key: _kAccessToken);
    await _storage.delete(key: _kRefreshToken);
    await _storage.delete(key: _kUserId);
    await _storage.delete(key: _kUserProfileType);
  }
}

