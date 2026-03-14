use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use chrono::{DateTime, Utc};
use jni::objects::{JObject, JValue};
use jni::JNIEnv;
use ndk_context::android_context;

use crate::application::ports::LocalNotificationService;
use crate::domain::error::{DomainError, Result};

const CHANNEL_ID: &str = "workouts";
const CHANNEL_NAME: &str = "Reminders";

pub struct AndroidLocalNotifications;

impl AndroidLocalNotifications {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AndroidLocalNotifications {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalNotificationService for AndroidLocalNotifications {
    fn request_permission(&self) -> Result<()> {
        request_notification_permission().map_err(DomainError::Api)
    }

    fn show_now(&self, id: &str, title: &str, body: &str) -> Result<()> {
        show_notification_now(id, title, body).map_err(DomainError::Api)
    }

    fn schedule_at(&self, _id: &str, _title: &str, _body: &str, _at: DateTime<Utc>) -> Result<()> {
        // Scheduling requires AlarmManager + BroadcastReceiver (Kotlin/Java).
        Ok(())
    }
}

fn has_notification_permission() -> std::result::Result<bool, String> {
    let ctx = android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }
        .map_err(|e| format!("JavaVM::from_raw: {e}"))?;
    let mut env = vm
        .attach_current_thread()
        .map_err(|e| format!("attach_current_thread: {e}"))?;
    let activity = unsafe { JObject::from_raw(ctx.context() as jni::sys::jobject) };

    let sdk_int = env
        .get_static_field("android/os/Build$VERSION", "SDK_INT", "I")
        .map_err(|e| format!("SDK_INT: {e}"))?
        .i()
        .map_err(|e| format!("SDK_INT as i32: {e}"))?;

    // POST_NOTIFICATIONS permission was added in API 33 (Android 13)
    if sdk_int < 33 {
        return Ok(true);
    }

    let permission = env
        .new_string("android.permission.POST_NOTIFICATIONS")
        .map_err(|e| format!("new_string: {e}"))?;
    let permission_obj: JObject = permission.into();

    let check_result = env
        .call_method(
            &activity,
            "checkSelfPermission",
            "(Ljava/lang/String;)I",
            &[JValue::Object(&permission_obj)],
        )
        .map_err(|e| format!("checkSelfPermission: {e}"))?
        .i()
        .map_err(|e| format!("checkSelfPermission result: {e}"))?;

    Ok(check_result == 0)
}

fn request_notification_permission() -> std::result::Result<(), String> {
    if has_notification_permission()? {
        return Ok(());
    }

    let ctx = android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }
        .map_err(|e| format!("JavaVM::from_raw: {e}"))?;
    let mut env = vm
        .attach_current_thread()
        .map_err(|e| format!("attach_current_thread: {e}"))?;
    let activity = unsafe { JObject::from_raw(ctx.context() as jni::sys::jobject) };

    let permission = env
        .new_string("android.permission.POST_NOTIFICATIONS")
        .map_err(|e| format!("new_string: {e}"))?;
    let permission_obj: JObject = permission.into();

    let string_class = env
        .find_class("java/lang/String")
        .map_err(|e| format!("find_class String: {e}"))?;
    let permissions_array = env
        .new_object_array(1, string_class, &permission_obj)
        .map_err(|e| format!("new_object_array: {e}"))?;
    let permissions_array_obj: JObject = permissions_array.into();

    env.call_method(
        &activity,
        "requestPermissions",
        "([Ljava/lang/String;I)V",
        &[JValue::Object(&permissions_array_obj), JValue::Int(1001)],
    )
    .map_err(|e| format!("requestPermissions: {e}"))?;

    Ok(())
}

fn show_notification_now(id: &str, title: &str, body: &str) -> std::result::Result<(), String> {
    if !has_notification_permission()? {
        return Err("notification_permission_denied".to_string());
    }

    let ctx = android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }
        .map_err(|e| format!("JavaVM::from_raw: {e}"))?;
    let mut env = vm
        .attach_current_thread()
        .map_err(|e| format!("attach_current_thread: {e}"))?;
    let context = unsafe { JObject::from_raw(ctx.context() as jni::sys::jobject) };
    ensure_channel(&mut env, &context).map_err(|e| e.to_string())?;
    show_notification(&mut env, &context, id, title, body).map_err(|e| e.to_string())?;
    Ok(())
}

fn ensure_channel(env: &mut JNIEnv, context: &JObject) -> jni::errors::Result<()> {
    let notification_service = env.new_string("notification")?;
    let notif_svc_obj: JObject = notification_service.into();
    let nm_obj = env
        .call_method(
            context,
            "getSystemService",
            "(Ljava/lang/String;)Ljava/lang/Object;",
            &[JValue::Object(&notif_svc_obj)],
        )?
        .l()?;

    let channel_id = env.new_string(CHANNEL_ID)?;
    let channel_name = env.new_string(CHANNEL_NAME)?;
    let channel_id_obj: JObject = channel_id.into();
    let channel_name_obj: JObject = channel_name.into();
    let importance = 3i32; // NotificationManager.IMPORTANCE_DEFAULT

    let channel_class = env.find_class("android/app/NotificationChannel")?;
    let channel = env.new_object(
        channel_class,
        "(Ljava/lang/String;Ljava/lang/CharSequence;I)V",
        &[
            JValue::Object(&channel_id_obj),
            JValue::Object(&channel_name_obj),
            JValue::Int(importance),
        ],
    )?;

    env.call_method(
        &nm_obj,
        "createNotificationChannel",
        "(Landroid/app/NotificationChannel;)V",
        &[JValue::Object(&channel)],
    )?;
    Ok(())
}

fn show_notification(
    env: &mut JNIEnv,
    context: &JObject,
    id: &str,
    title: &str,
    body: &str,
) -> jni::errors::Result<()> {
    let notification_id = id_to_int(id);

    let notification_service = env.new_string("notification")?;
    let notif_svc_obj: JObject = notification_service.into();
    let nm_obj = env
        .call_method(
            context,
            "getSystemService",
            "(Ljava/lang/String;)Ljava/lang/Object;",
            &[JValue::Object(&notif_svc_obj)],
        )?
        .l()?;

    let channel_id = env.new_string(CHANNEL_ID)?;
    let channel_id_obj: JObject = channel_id.into();
    let builder_class = env.find_class("android/app/Notification$Builder")?;
    let builder = env.new_object(
        builder_class,
        "(Landroid/content/Context;Ljava/lang/String;)V",
        &[JValue::Object(context), JValue::Object(&channel_id_obj)],
    )?;

    let title_str = env.new_string(title)?;
    let body_str = env.new_string(body)?;
    let title_obj: JObject = title_str.into();
    let body_obj: JObject = body_str.into();

    env.call_method(
        &builder,
        "setContentTitle",
        "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;",
        &[JValue::Object(&title_obj)],
    )?;
    env.call_method(
        &builder,
        "setContentText",
        "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;",
        &[JValue::Object(&body_obj)],
    )?;

    let icon_res = get_small_icon_id(env)?;
    env.call_method(
        &builder,
        "setSmallIcon",
        "(I)Landroid/app/Notification$Builder;",
        &[JValue::Int(icon_res)],
    )?;

    let notification = env
        .call_method(&builder, "build", "()Landroid/app/Notification;", &[])?
        .l()?;

    env.call_method(
        &nm_obj,
        "notify",
        "(ILandroid/app/Notification;)V",
        &[JValue::Int(notification_id), JValue::Object(&notification)],
    )?;
    Ok(())
}

fn id_to_int(id: &str) -> i32 {
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    let h = hasher.finish();
    (h % (i32::MAX as u64)) as i32
}

fn get_small_icon_id(env: &mut JNIEnv) -> jni::errors::Result<i32> {
    let drawable_class = env.find_class("android/R$drawable")?;
    let icon_id = env
        .get_static_field(drawable_class, "ic_dialog_info", "I")?
        .i()?;
    Ok(icon_id)
}
