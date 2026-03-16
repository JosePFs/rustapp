#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
REPO_DIR="$(cd "$APP_DIR/.." && pwd)"
JNI_DIR="$APP_DIR/android/app/src/main/jniLibs"
MANIFEST_PATH="$REPO_DIR/mobile-bridge-frb/Cargo.toml"
RUST_API="$REPO_DIR/mobile-bridge-frb/src/api.rs"
GENERATED="$REPO_DIR/mobile-bridge-frb/src/frb_generated.rs"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is required to build the Rust bridge." >&2
  exit 1
fi

if ! command -v cargo-ndk >/dev/null 2>&1; then
  echo "cargo-ndk is required. Install it with: cargo install cargo-ndk" >&2
  exit 1
fi

if [[ -z "${ANDROID_NDK_HOME:-}" && -z "${ANDROID_NDK_ROOT:-}" ]]; then
  echo "ANDROID_NDK_HOME or ANDROID_NDK_ROOT must be set for Android bridge builds." >&2
  exit 1
fi

if [[ ! -f "$GENERATED" ]] || [[ "$RUST_API" -nt "$GENERATED" ]]; then
  echo "API changed, regenerating bridge..."
  flutter_rust_bridge_codegen generate \
    --rust-root "$REPO_DIR/mobile-bridge-frb" \
    --rust-input crate::api \
    --dart-root "$REPO_DIR/app-flutter" \
    --dart-output "$REPO_DIR/app-flutter/lib/src/rust"
else
  echo "Bridge up to date, skipping generate."
fi

rustup target add \
  aarch64-linux-android \
  armv7-linux-androideabi \
  x86_64-linux-android >/dev/null

rm -rf "$JNI_DIR"
mkdir -p "$JNI_DIR"

cargo ndk \
  -o "$JNI_DIR" \
  --manifest-path "$MANIFEST_PATH" \
  -t armeabi-v7a \
  -t arm64-v8a \
  -t x86_64 \
  build --release
