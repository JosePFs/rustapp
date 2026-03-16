# Front Flutter

Patient-facing Flutter application for the Eixe monorepo. This app consumes the Rust core through `flutter_rust_bridge` and loads the native bridge library from `mobile-bridge-frb`.

## Project Layout

- `lib/main.dart`: patient app bootstrap shell with splash, bridge initialization, and login flow backed by Dart defines.
- `lib/src/rust/`: generated Dart bindings from `flutter_rust_bridge`.
- `scripts/build_rust_bridge_android.sh`: builds Android `.so` libraries into `android/app/src/main/jniLibs`.
- `scripts/build_rust_bridge_linux.sh`: builds the Linux `.so` and copies it into the Flutter Linux bundle.

## Prerequisites

### Common

- Flutter 3.35+
- Rust toolchain
- `flutter_rust_bridge_codegen` 2.11.1

Install the FRB generator:

```bash
cargo install flutter_rust_bridge_codegen --version 2.11.1
```

### Android

- Android SDK
- Android NDK
- `cargo-ndk`

Install `cargo-ndk`:

```bash
cargo install cargo-ndk
```

The Android build expects `ANDROID_NDK_HOME` or `ANDROID_NDK_ROOT` to be available. The Gradle integration also tries to derive the NDK path from `android/local.properties`.

### Linux

- Flutter Linux desktop dependencies
- A working native Linux toolchain

## Rust Bridge

The Rust bridge lives in `../mobile-bridge-frb`.

- Android uses `libmobile_bridge_frb.so` bundled through `jniLibs`.
- Linux copies `libmobile_bridge_frb.so` into the packaged app `lib/` directory.
- `lib/main.dart` loads the native library explicitly on Android and Linux before calling `RustLib.init()` during app startup.

## Configuration

The Flutter app expects Supabase settings via Dart defines:

```bash
--dart-define=SUPABASE_URL=https://YOUR_PROJECT.supabase.co
--dart-define=SUPABASE_ANON_KEY=your_anon_key
```

This keeps the runtime configuration on the Flutter side, which is the natural entrypoint for mobile app environment-specific values.

## Regenerate Flutter Rust Bridge Bindings

Run from the monorepo root:

```bash
flutter_rust_bridge_codegen generate \
  --rust-root mobile-bridge-frb \
  --rust-input crate::api \
  --dart-root app-flutter \
  --dart-output app-flutter/lib/src/rust
```

Use this whenever you change public APIs in `mobile-bridge-frb/src/api.rs`, but `build_rust_bridge_linux.sh` and `build_rust_bridge_android.s` already generates when changes detected.

## Running

## Recommended Development Flow

- Use `flutter run -d linux` for fast day-to-day UI iteration, layout tweaks, and general flow work while developing on Linux.
- Use Android or iOS devices/emulators to validate the real mobile behavior before considering a change done.
- Treat Linux desktop as a developer convenience target, not as the reference runtime for platform-specific behavior such as embedded media playback.

### Linux

The Linux CMake build invokes `scripts/build_rust_bridge_linux.sh` automatically.

```bash
cd /app-flutter
flutter run -d linux \
  --dart-define=SUPABASE_URL=https://YOUR_PROJECT.supabase.co \
  --dart-define=SUPABASE_ANON_KEY=your_anon_key
```

### Android

The Android Gradle build invokes `scripts/build_rust_bridge_android.sh` automatically before `preBuild`.

```bash
cd app-flutter
flutter run -d 'mobile device id' \
  --dart-define=SUPABASE_URL=https://YOUR_PROJECT.supabase.co \
  --dart-define=SUPABASE_ANON_KEY=your_anon_key
```

## Manual Rust Bridge Builds

### Linux

```bash
cd app-flutter
./scripts/build_rust_bridge_linux.sh build/rust_bridge
```

### Android

```bash
cd app-flutter
./scripts/build_rust_bridge_android.sh
```

## Verification

Recommended checks:

```bash
cd rustapp
cargo check --workspace
cargo build -p mobile-bridge-frb --release

cd app-flutter
flutter analyze
flutter test
```

## Notes

- If you rename the Rust bridge crate or output library stem, update:
  - `mobile-bridge-frb`
  - `app-flutter/lib/src/rust/frb_generated.dart`
  - `app-flutter/android/app/src/main/kotlin/.../MainActivity.kt`
  - `app-flutter/scripts/`
