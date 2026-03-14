# Android Notifications — Pending Work Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Complete the Android local notifications feature: runtime permission request for `POST_NOTIFICATIONS` (Android 13+), custom `AndroidManifest.xml`, and document the `schedule_at` roadmap.

**Architecture:** `schedule_at` cannot be implemented locally in Dioxus 0.7 because it requires a `BroadcastReceiver` (Kotlin class) and there is no stable way to inject custom Kotlin/Java into the generated Android project. Dioxus 0.8 will add `dx eject` for full project control. For now, `schedule_at` remains a no-op with clear documentation. The focus is on making `show_now` fully work on Android 13+ by requesting the `POST_NOTIFICATIONS` permission at runtime and declaring it in the manifest.

**Tech Stack:** Rust, JNI (jni 0.21), ndk-context, Dioxus 0.7, Android API 28+ (target), API 33+ (permissions)

---

## Task 1: Custom AndroidManifest with POST_NOTIFICATIONS permission

**Agent:** implementer

**Files:**
- Create: `AndroidManifest.xml` (project root)
- Modify: `Dioxus.toml`

**Step 1: Create base AndroidManifest.xml**

Create `AndroidManifest.xml` at the project root with the standard Dioxus-generated structure plus the `POST_NOTIFICATIONS` permission. The base template comes from Dioxus's internal `AndroidManifest.xml.hbs`:

```xml
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android">

    <uses-permission android:name="android.permission.INTERNET" />
    <uses-permission android:name="android.permission.POST_NOTIFICATIONS" />

    <application
        android:label="{{application.name}}"
        android:theme="@android:style/Theme.DeviceDefault.NoActionBar"
        android:hasCode="true"
        android:usesCleartextTraffic="true">

        <activity
            android:name="dev.dioxus.main.MainActivity"
            android:configChanges="orientation|screenSize|screenLayout|smallestScreenSize|keyboardHidden"
            android:windowSoftInputMode="adjustResize"
            android:exported="true">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
</manifest>
```

> **Note:** The `{{application.name}}` is a Handlebars placeholder used by the Dioxus CLI. If it doesn't work, replace with the literal app name `"rustapp"`. You may need to check the actual generated manifest first by running `dx serve --platform android` and inspecting the output under `target/dx/`.

**Step 2: Point Dioxus.toml to the custom manifest**

Add to `Dioxus.toml` under `[application]`:

```toml
android_manifest = "./AndroidManifest.xml"
```

**Step 3: Verify by rebuilding**

Run: `cargo dev-android`
Expected: Build succeeds, the APK includes the custom manifest with `POST_NOTIFICATIONS` permission.

**Step 4: Commit**

```bash
git add AndroidManifest.xml Dioxus.toml
git commit -m "feat: add custom AndroidManifest with POST_NOTIFICATIONS permission"
```

---

## Task 2: Runtime permission request from Rust/JNI

**Agent:** implementer

**Files:**
- Modify: `src/infrastructure/android/notifications.rs`
- Modify: `src/application/ports/local_notifications.rs` (add `request_permission` to trait)
- Modify: `src/application/ports/mod.rs` (export if needed)
- Modify: `src/infrastructure/android/mod.rs` (if splitting into submodules)

**Step 1: Add `request_permission` to the trait**

In `src/application/ports/local_notifications.rs`, add a method to the trait:

```rust
pub trait LocalNotificationService: Send + Sync {
    fn show_now(&self, id: &str, title: &str, body: &str) -> Result<()>;
    fn schedule_at(&self, id: &str, title: &str, body: &str, at: DateTime<Utc>) -> Result<()>;
    fn request_permission(&self) -> Result<()>;
}
```

Add a no-op implementation in `StubLocalNotificationService`:

```rust
fn request_permission(&self) -> Result<()> {
    Ok(())
}
```

**Step 2: Implement `request_permission` on Android**

In `src/infrastructure/android/notifications.rs`, implement the method:

```rust
fn request_permission(&self) -> Result<()> {
    request_notification_permission().map_err(DomainError::Api)
}
```

Add the JNI function:

```rust
fn request_notification_permission() -> std::result::Result<(), String> {
    // Only needed on API 33+ (Android 13+)
    let ctx = android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }
        .map_err(|e| format!("JavaVM::from_raw: {e}"))?;
    let mut env = vm
        .attach_current_thread()
        .map_err(|e| format!("attach_current_thread: {e}"))?;
    let activity = unsafe { JObject::from_raw(ctx.context() as jni::sys::jobject) };

    // Check SDK version: Build.VERSION.SDK_INT
    let version_class = env.find_class("android/os/Build$VERSION")
        .map_err(|e| e.to_string())?;
    let sdk_int = env.get_static_field(version_class, "SDK_INT", "I")
        .map_err(|e| e.to_string())?
        .i()
        .map_err(|e| e.to_string())?;

    if sdk_int < 33 {
        return Ok(()); // POST_NOTIFICATIONS not needed below API 33
    }

    // Check if already granted: context.checkSelfPermission(permission) == 0
    let perm_str = env.new_string("android.permission.POST_NOTIFICATIONS")
        .map_err(|e| e.to_string())?;
    let perm_obj: JObject = perm_str.into();
    let check_result = env.call_method(
        &activity,
        "checkSelfPermission",
        "(Ljava/lang/String;)I",
        &[JValue::Object(&perm_obj)],
    )
    .map_err(|e| e.to_string())?
    .i()
    .map_err(|e| e.to_string())?;

    if check_result == 0 {
        return Ok(()); // Already granted
    }

    // Request permission: activity.requestPermissions(new String[]{perm}, REQUEST_CODE)
    let string_class = env.find_class("java/lang/String")
        .map_err(|e| e.to_string())?;
    let perm_str2 = env.new_string("android.permission.POST_NOTIFICATIONS")
        .map_err(|e| e.to_string())?;
    let perm_array = env.new_object_array(1, string_class, perm_str2)
        .map_err(|e| e.to_string())?;
    let perm_array_obj: JObject = perm_array.into();

    env.call_method(
        &activity,
        "requestPermissions",
        "([Ljava/lang/String;I)V",
        &[JValue::Object(&perm_array_obj), JValue::Int(1001)],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
```

**Step 3: Call `request_permission` on app startup**

In `src/context.rs`, after creating the `AppContext`, call `request_permission` on the notifications service:

```rust
let local_notifications = local_notifications_impl();
let _ = local_notifications.request_permission(); // Best-effort
```

Alternatively, call it from the main app component in `src/main.rs` on first render.

**Step 4: Verify compilation**

Run: `cargo check --target aarch64-linux-android --features mobile --no-default-features` (with NDK env vars)
Expected: No errors.

**Step 5: Commit**

```bash
git add src/application/ports/local_notifications.rs src/infrastructure/android/notifications.rs src/context.rs
git commit -m "feat: request POST_NOTIFICATIONS permission at runtime on Android 13+"
```

---

## Task 3: Guard `show_now` with permission check

**Agent:** implementer

**Files:**
- Modify: `src/infrastructure/android/notifications.rs`

**Step 1: Add permission check before showing notification**

Extract a reusable `has_notification_permission` function and call it at the start of `show_notification_now`:

```rust
fn has_notification_permission() -> std::result::Result<bool, String> {
    let ctx = android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }
        .map_err(|e| format!("JavaVM::from_raw: {e}"))?;
    let mut env = vm
        .attach_current_thread()
        .map_err(|e| format!("attach_current_thread: {e}"))?;
    let activity = unsafe { JObject::from_raw(ctx.context() as jni::sys::jobject) };

    let version_class = env.find_class("android/os/Build$VERSION")
        .map_err(|e| e.to_string())?;
    let sdk_int = env.get_static_field(version_class, "SDK_INT", "I")
        .map_err(|e| e.to_string())?
        .i()
        .map_err(|e| e.to_string())?;

    if sdk_int < 33 {
        return Ok(true);
    }

    let perm_str = env.new_string("android.permission.POST_NOTIFICATIONS")
        .map_err(|e| e.to_string())?;
    let perm_obj: JObject = perm_str.into();
    let result = env.call_method(
        &activity,
        "checkSelfPermission",
        "(Ljava/lang/String;)I",
        &[JValue::Object(&perm_obj)],
    )
    .map_err(|e| e.to_string())?
    .i()
    .map_err(|e| e.to_string())?;

    Ok(result == 0)
}
```

In `show_notification_now`, add at the beginning:

```rust
if !has_notification_permission()? {
    return Err("POST_NOTIFICATIONS permission not granted".to_string());
}
```

**Step 2: Verify compilation**

Run: `cargo check --target aarch64-linux-android --features mobile --no-default-features`
Expected: No errors.

**Step 3: Commit**

```bash
git add src/infrastructure/android/notifications.rs
git commit -m "feat: guard show_now with POST_NOTIFICATIONS permission check"
```

---

## Task 4: Update documentation

**Agent:** docs

**Files:**
- Modify: `docs/android_local_notifications.md`

**Step 1: Update the doc to reflect current state**

Update sections:
- **Permissions**: Now handled — manifest declaration + runtime request. Document how it works.
- **Scheduling**: Keep as "not implemented in 0.7" but add a clear roadmap:
  - Dioxus 0.8 will bring `dx eject` → full Android project control → `AlarmManager` + `BroadcastReceiver`
  - Alternative: server-side scheduling via FCM (if the app goes that route)
- Remove any stale information.

**Step 2: Commit**

```bash
git add docs/android_local_notifications.md
git commit -m "docs: update Android notifications status and schedule_at roadmap"
```

---

## Summary

| Task | Agent | What it does |
|------|-------|--------------|
| 1 | implementer | Custom AndroidManifest + `POST_NOTIFICATIONS` in manifest + Dioxus.toml config |
| 2 | implementer | Runtime `requestPermissions` via JNI for Android 13+ |
| 3 | implementer | Guard `show_now` with permission check |
| 4 | docs | Update documentation with current state and roadmap |

## What stays as no-op (and why)

`schedule_at` remains `Ok(())` because:
- Requires a `BroadcastReceiver` Kotlin class compiled into the APK
- Dioxus 0.7 has no stable mechanism to inject custom Kotlin/Java source
- Dioxus 0.8 (`dx eject`, PR #4274) will unblock this
- Alternative: server-side FCM scheduling (separate feature, not part of this plan)
