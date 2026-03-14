#!/usr/bin/env bash
# Copy project Android theme (status bar = app background) into the Dioxus-generated app.
# Run after: cargo dev-android or dx bundle --platform android

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SRC="$ROOT/android/app/src/main/res/values"
[[ -d "$SRC" ]] || { echo "Source $SRC not found." >&2; exit 1; }

# Prefer debug path; fallback to release
for segment in debug release; do
  DST="$ROOT/target/dx/rustapp/$segment/android/app/app/src/main/res/values"
  if [[ -d "$ROOT/target/dx/rustapp/$segment" ]]; then
    mkdir -p "$DST"
    cp -r "$SRC"/* "$DST/"
    echo "Applied theme to $DST"
    exit 0
  fi
done

echo "No target/dx/rustapp/{debug,release} found. Run a build first (e.g. cargo dev-android)." >&2
exit 1
