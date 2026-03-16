#!/usr/bin/env bash

set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <output-dir>" >&2
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
REPO_DIR="$(cd "$APP_DIR/.." && pwd)"
OUTPUT_DIR="$1"
SOURCE_LIB="$REPO_DIR/target/release/libmobile_bridge_frb.so"
RUST_API="$REPO_DIR/mobile-bridge-frb/src/api.rs"
GENERATED="$REPO_DIR/mobile-bridge-frb/src/frb_generated.rs"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is required to build the Rust bridge." >&2
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

cargo build -p mobile-bridge-frb --release --manifest-path "$REPO_DIR/Cargo.toml"

mkdir -p "$OUTPUT_DIR"
cp "$SOURCE_LIB" "$OUTPUT_DIR/libmobile_bridge_frb.so"
