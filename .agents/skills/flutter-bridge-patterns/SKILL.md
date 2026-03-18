---
name: flutter-bridge-patterns
description: Use this whenever you are integrating Flutter with Rust via flutter_rust_bridge (FRB) in this repo (e.g., editing `mobile-bridge-frb/src/api.rs`, `mobile-bridge-frb/src/frb_generated.rs`, `app-flutter/lib/src/rust/*`, FRB build scripts, or calling `RustLib.init`). Helps you add/propagate DTO fields safely, regenerate bindings, and avoid common FRB failure modes (stale generated files, version/hash mismatch, library stem drift, Android JNI libs/NDK wiring, and platform limitations like web).
---

## Purpose

This skill captures the **project-specific** conventions for Flutter ↔ Rust integration using **flutter_rust_bridge**.

Use it when you:

- Add/change a Rust DTO or exported function and need the Flutter side updated.
- See errors that look like stale generated bindings (`frb_generated.rs` / `frb_generated.dart`) or codegen version/hash mismatches.
- Touch native library loading (`libmobile_bridge_frb.so`) or bootstrapping (`RustLib.init`).
- Need to understand where FRB lives in this monorepo and the “correct” workflow here.

## Repo layout (FRB-relevant)

- **Rust bridge crate**: `mobile-bridge-frb/`
  - Source of truth for exported API: `mobile-bridge-frb/src/api.rs` (FRB input is `crate::api`)
  - Generated Rust glue: `mobile-bridge-frb/src/frb_generated.rs` (included from `mobile-bridge-frb/src/lib.rs`)
- **Flutter generated bindings**: `app-flutter/lib/src/rust/`
  - `api.dart` (generated DTOs + wrappers)
  - `frb_generated.dart` (+ `.io.dart` / `.web.dart`)
- **Codegen/build scripts**: `app-flutter/scripts/build_rust_bridge_*.sh`
- **Android native loading**: `app-flutter/android/app/src/main/kotlin/.../MainActivity.kt` (`System.loadLibrary("mobile_bridge_frb")`)
- **Linux desktop wiring**: `app-flutter/linux/CMakeLists.txt` (custom target that builds/copies `libmobile_bridge_frb.so`)

## Canonical workflow: add a new field end-to-end

When you add/change data surfaced to Flutter:

1. **Application/use-case layer (Rust)**
   - Add the field to the use-case result struct (e.g., `MobilePatientProgram` in `application/src/use_cases/...`).
   - Compute/populate it in the use-case.

2. **Bridge DTO (Rust)**
   - Add the field to the FRB DTO in `mobile-bridge-frb/src/api.rs` (e.g., `PatientProgramSummary`).
   - Map from the application result into the bridge DTO.
   - Keep exported function signatures FRB-friendly (prefer primitives, `Option<T>`, `Vec<T>`, strings; errors as `Result<T, String>` in this repo).

3. **Regenerate FRB bindings**
   - Run the repo’s generator script (preferred) or the standard command used in scripts:
     - `flutter_rust_bridge_codegen generate --rust-root ... --rust-input crate::api --dart-root ... --dart-output ...`
   - Regeneration is expected whenever DTOs/functions change; don’t hand-edit generated files.
   - If codegen appears to “skip” unexpectedly, touch/save `mobile-bridge-frb/src/api.rs` (it’s the timestamp the scripts compare against) or run codegen explicitly.

4. **Flutter usage**
   - Update app code to read the new fields from the generated Dart DTOs.
   - Update widget tests if they assert DTO constructors/text that now changed.

## Stable ordering with concurrency (pattern)

If you fetch multiple program payloads concurrently and the UI expects stable order:

- Keep the authoritative order from the upstream list API.
- When using unordered concurrency (e.g. `buffer_unordered`), carry an `order_index` and **sort the final list** before returning it.

## Examples (from this repo)

### Example: exported Rust API uses FRB-friendly types + String errors

```17:80:/home/jose/Workspace/eixe/rustapp/mobile-bridge-frb/src/api.rs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(request: LoginRequest, config: BridgeConfig) -> Result<LoginResponse, String> {
    let use_case = MobileLoginUseCase::<NativeApi>::new(backend(config));
    let result = use_case
        .execute(LoginUseCaseArgs {
            credentials: Credentials::from(&request.email, &request.password),
        })
        .await
        .map_err(|error| error.to_string())?;

    Ok(LoginResponse {
        access_token: result.session.access_token().to_string(),
        user_id: result.session.user_id().to_string(),
        user_profile_type: match result.user_profile_type {
            UserProfileType::Specialist => "specialist".to_string(),
            UserProfileType::Patient => "patient".to_string(),
        },
    })
}
```

### Example: Flutter bootstraps FRB by loading the native library and calling `RustLib.init`

```1474:1534:/home/jose/Workspace/eixe/rustapp/app-flutter/lib/main.dart
ExternalLibrary? _bridgeLibrary() {
  if (kIsWeb) {
    return null;
  }

  if (Platform.isAndroid) {
    return ExternalLibrary.open('libmobile_bridge_frb.so');
  }

  if (Platform.isLinux) {
    final executableDir = File(Platform.resolvedExecutable).parent;
    final bundledLibrary = File('${executableDir.path}/lib/libmobile_bridge_frb.so');
    if (bundledLibrary.existsSync()) {
      return ExternalLibrary.open(bundledLibrary.path);
    }
    // ... repo fallback, then name fallback ...
    return ExternalLibrary.open('libmobile_bridge_frb.so');
  }

  return null;
}
```

### Example: build scripts regenerate bindings from `crate::api`

```23:37:/home/jose/Workspace/eixe/rustapp/app-flutter/scripts/build_rust_bridge_linux.sh
flutter_rust_bridge_codegen generate \
  --rust-root "$REPO_DIR/mobile-bridge-frb" \
  --rust-input crate::api \
  --dart-root "$REPO_DIR/app-flutter" \
  --dart-output "$REPO_DIR/app-flutter/lib/src/rust"
```

## Common pitfalls (and what to do instead)

- **Stale generated files** (`frb_generated.rs` or Dart bindings don’t reflect your Rust changes)
  - Fix: regenerate bindings using the repo scripts. Avoid editing generated files manually.
  - Note: this repo’s scripts commonly compare timestamps against `mobile-bridge-frb/src/api.rs`; changes elsewhere may not trigger regeneration unless `api.rs` also changes.

- **Codegen/hash mismatch errors**
  - Cause: generated Rust and generated Dart not produced by the same FRB/codegen run/version.
  - Fix: regenerate both sides together; ensure you’re running the same `flutter_rust_bridge_codegen` version.

- **Library stem drift**
  - If the stem changes, update _all_ places consistently:
    - Android `System.loadLibrary("<stem>")`
    - Dart loader config (`stem`)
    - scripts + file paths (`lib<stem>.so`)
  - Symptom: runtime init/load failures that look like “could not load dynamic library” / “symbol not found”.

- **Web limitations**
  - FRB does not “just work” on web unless you intentionally support it; treat web as a separate target with separate constraints.
  - Symptom: build succeeds but runtime throws when the generated web stub is hit or when a native-only call is made.

- **Flattened error typing**
  - This repo uses `Result<T, String>` for FRB functions. If you need structured errors, you must add a structured error DTO and return it (don’t silently encode data into arbitrary strings).

- **Android double-loading suspicion**
  - This repo loads the library in Kotlin (`System.loadLibrary`) and also opens it in Dart via `ExternalLibrary.open(...)`.
  - If you see flakey load issues on Android, check for stem/name mismatches and consider consolidating loading in one place (but do so intentionally, not accidentally).

## What I will do when you invoke this skill

- Identify the “source of truth” Rust type/function you’re changing.
- List the required propagation steps (use-case → bridge DTO → codegen → Flutter usage/tests).
- Point to the exact regeneration command/script used in this repo.
- Suggest the minimal, consistent changes that keep FRB generated code in sync across platforms.
