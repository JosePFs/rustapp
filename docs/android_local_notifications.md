# Android local notifications

## Architecture

The `LocalNotificationService` trait (in `src/application/ports/local_notifications.rs`) is implemented on Android via JNI. On other platforms, `StubLocalNotificationService` provides a no-op implementation.

### Permission flow

1. App startup: `context.rs` calls `LocalNotificationService::request_permission()` (best-effort).
2. On Android 13+ (API 33): Triggers `Activity.requestPermissions()` via JNI.
3. On older Android: No-op (permission already in manifest).
4. Before showing: `show_now` checks `checkSelfPermission`; returns error key `"notification_permission_denied"` if not granted.

## Implemented

### `show_now`

`LocalNotificationService::show_now(id, title, body)` — fully working on Android via JNI.

- Creates a notification channel `"workouts"` on first use.
- Builds and shows the notification through `NotificationManager`.
- **Permission guard**: checks `checkSelfPermission("POST_NOTIFICATIONS")` before attempting to show. Returns `"notification_permission_denied"` error if not granted.
- Uses `android.R.drawable.ic_dialog_info` as the small icon.
- Testing: Use the "Probar notificación" button on the patient workout session view.

### `request_permission`

`LocalNotificationService::request_permission()` — requests `POST_NOTIFICATIONS` at runtime on Android 13+.

- Called automatically on app startup (`context.rs`, best-effort).
- On API < 33: no-op (permission not required).
- On API 33+: checks `checkSelfPermission` first; if not granted, calls `Activity.requestPermissions()` via JNI.

### Stub

On non-Android platforms, `StubLocalNotificationService` implements the same trait as a no-op.

## Permissions

### Manifest

Custom `AndroidManifest.xml` at project root, configured in `Dioxus.toml`:

```toml
[application]
android_manifest = "./AndroidManifest.xml"
```

Declared permissions:

- `android.permission.INTERNET`
- `android.permission.POST_NOTIFICATIONS`

### Runtime

On Android 13+ (API 33), `POST_NOTIFICATIONS` requires a runtime request. The app calls `Activity.requestPermissions()` via JNI on startup. If the user denies, `show_now` returns an error instead of silently failing.

## Scheduling (`schedule_at`)

**Not implemented** — returns `Ok(())` (no-op).

### Why

Scheduling via `AlarmManager` requires a `BroadcastReceiver` Kotlin/Java class compiled into the APK. Dioxus 0.7 has no stable mechanism to inject custom Kotlin/Java source into the generated Android project.

### Roadmap

1. **Dioxus 0.8** — `dx eject` (PR #4274) will give full Android project control. With an ejected project, we can add a Kotlin `BroadcastReceiver`, register it in the manifest, and schedule via `AlarmManager` + `PendingIntent` from Rust/JNI.
2. **Server-side FCM** — alternative approach: Supabase Edge Functions schedule a Firebase Cloud Messaging push at the desired time. No local `AlarmManager` needed, works cross-platform.

## IDE navigation and diagnostics (Linux)

The real Android implementation is in `notifications.rs` (compiled only for `target_os = "android"`). A **stub** in `notifications_stub.rs` is built on host so that:

- **rust-analyzer** can use the default (host) target: you get correct diagnostics and navigation without setting `cargo.target`.
- The `android` module is always present: you can open `notifications_stub.rs` and `notifications.rs` from the tree; the stub mirrors the public API so "Go to definition" works from `context.rs` on host.

To run checks for both host and Android, use the VS Code task **"cargo check (host + android)"** (or run `cargo check` and `cargo check --target aarch64-linux-android --features mobile --no-default-features`). For Android you need the NDK toolchain in `PATH` or set `CC_aarch64_linux_android`, `CXX_aarch64_linux_android`, `AR_aarch64_linux_android`.

## Build

- `jni` and `ndk-context` dependencies are conditional on `target_os = "android"` (see `Cargo.toml`).
- Build/serve: `cargo dev-android` or `dx serve --platform android`.
- Bundle: `dx bundle --platform android`.
