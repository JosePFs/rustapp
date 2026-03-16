import java.io.File
import java.util.Properties

plugins {
    id("com.android.application")
    id("kotlin-android")
    // The Flutter Gradle Plugin must be applied after the Android and Kotlin Gradle plugins.
    id("dev.flutter.flutter-gradle-plugin")
}

android {
    namespace = "org.eixe.patientfront"
    compileSdk = flutter.compileSdkVersion
    ndkVersion = flutter.ndkVersion

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }

    kotlinOptions {
        jvmTarget = JavaVersion.VERSION_11.toString()
    }

    defaultConfig {
        applicationId = "org.eixe.patientfront"
        minSdk = flutter.minSdkVersion
        targetSdk = flutter.targetSdkVersion
        versionCode = flutter.versionCode
        versionName = flutter.versionName
    }

    buildTypes {
        release {
            // TODO: Add your own signing config for the release build.
            // Signing with the debug keys for now, so `flutter run --release` works.
            signingConfig = signingConfigs.getByName("debug")
        }
    }
}

flutter {
    source = "../.."
}

val localProperties = Properties().apply {
    val localPropertiesFile = rootProject.file("local.properties")
    if (localPropertiesFile.exists()) {
        localPropertiesFile.inputStream().use(::load)
    }
}

val ndkDirectoryFromLocalProperties =
    localProperties.getProperty("sdk.dir")?.let { sdkDir ->
        File(sdkDir, "ndk/${android.ndkVersion}")
    }

val buildRustBridge by tasks.registering(Exec::class) {
    val scriptFile = File(projectDir, "../../scripts/build_rust_bridge_android.sh")

    inputs.file(scriptFile)
    inputs.file(File(projectDir, "../../../Cargo.toml"))
    inputs.dir(File(projectDir, "../../../mobile-bridge-frb/src"))
    outputs.dir(File(projectDir, "src/main/jniLibs"))

    workingDir = projectDir
    commandLine("bash", scriptFile.absolutePath)

    val ndkDir = ndkDirectoryFromLocalProperties
    if (ndkDir != null) {
        environment("ANDROID_NDK_HOME", ndkDir.absolutePath)
    }
}

tasks.named("preBuild").configure {
    dependsOn(buildRustBridge)
}
