# Android v1 build receipt

Run date: 2026-10-19, America/Chicago
Branch: `android/v1-app`
Base: `041d1bab092553798dec6f7b84df1edfbe417ab4`

## Toolchain

- macOS arm64 host
- JDK 17.0.20.1 from Homebrew
- Android SDK Platform 36, Build Tools 36.0.0, Emulator 37.2.12
- Android NDK 28.2.13676358
- Rust Android targets: `aarch64-linux-android`, `x86_64-linux-android`
- Gradle wrapper 8.11.1, Android Gradle Plugin 8.9.2, Kotlin 2.0.21
- `compileSdk=36`, `targetSdk=36`, `minSdk=26`

## Native builds

Command used from the repository root:

```sh
ANDROID_HOME=/opt/homebrew/share/android-commandlinetools \
ANDROID_NDK_HOME=/opt/homebrew/share/android-commandlinetools/ndk/28.2.13676358 \
RUSTFLAGS='-C link-arg=-Wl,-z,max-page-size=16384' \
cargo ndk -t arm64-v8a -t x86_64 -o android/app/src/main/jniLibs \
build --release -p oklo_core_jni
```

Both JNI libraries were produced and packaged. ELF load segments for both ABIs were inspected with `llvm-readobj --program-headers`; every `PT_LOAD` alignment was 16384 bytes.

SHA-256:

- `arm64-v8a/liboklo_core_jni.so`: `802c4aa33251cdda30a2a81ce672ecd611899a9ba8cf9dbc48ebef8b3b397092`
- `x86_64/liboklo_core_jni.so`: `7ed74f69a8f6dd0e60aadf9b37dae136e26331229d6dc955081e0592fb0c71f2`

## Android build and unit tests

Command used from `android/`:

```sh
JAVA_HOME=/opt/homebrew/opt/openjdk@17/libexec/openjdk.jdk/Contents/Home \
ANDROID_HOME=/opt/homebrew/share/android-commandlinetools \
ANDROID_NDK_HOME=/opt/homebrew/share/android-commandlinetools/ndk/28.2.13676358 \
./gradlew testDebugUnitTest assembleAndroidTest assembleDebug bundleRelease
```

Result: `BUILD SUCCESSFUL`. Seven unit tests passed, zero failed. The instrumentation APK compiled successfully. Tests cover phase advancement and wraparound, negative step rejection, pulse boundary selection, null-scenario phase invariance, continuous-field summaries, and categorical zone-share summaries.

The latest Android chart revision separates prescribed pulse/desiccation drivers from field values so the curves use independent axes. Continuous fields show a mean trace and min/max envelope; the categorical compartment field shows zone 3 and zone 4 voxel shares rather than a categorical mean. Two unit tests cover the share calculation and ensure continuous fields report no categorical shares.

Artifacts:

- Debug APK: `android/app/build/outputs/apk/debug/app-debug.apk`, SHA-256 `92224fc11ca7c58077ea6eade630e141ab360b49d9cbf8bbb8b51b6bf7ebdf3d`
- Instrumentation APK: `android/app/build/outputs/apk/androidTest/debug/app-debug-androidTest.apk`, SHA-256 `443fea9353a3dce98f25cccc7be51533cc5c39944ba27e8f2fee0682143dcde9`
- Release AAB: `android/app/build/outputs/bundle/release/app-release.aab`, SHA-256 `6af2123768b8c1a3672e7219f625c9a48890d26de710a37bfb4c736e05b01159`

The APK reports package `org.oklo.pool`, version `1.0.0`, min API 26, target API 36, and native ABIs `arm64-v8a` and `x86_64`. The merged manifest contains only the AndroidX-generated signature-only permission `org.oklo.pool.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION`; it does not request `INTERNET` or user data access. This is a source and manifest inspection, not a runtime network test.

The release AAB is unsigned. Build success does not make it suitable for store upload.

## Runtime status

Instrumentation tests, emulator UI verification, screenshots, and native runtime loading remain `BLOCKED`. The API 36 arm64 Google APIs image installed, but the host had 2052.19 MB free when the emulator checked for its required 7372.80 MB userdata image. A manual AVD and smaller partition setting did not bypass this emulator storage requirement. No device test passed, and no store screenshot was captured.

Physical-device testing is `NOT RUN`; no device and signing setup were available in this session.

## Limitations

These results establish buildability and unit-test outcomes only. They do not establish successful installation, Android runtime behavior, ABI runtime conformance, accessibility quality, lifecycle correctness, absence of runtime network traffic, store compliance, or scientific validity. Values remain the prescribed revision 0 fixture functions documented in `docs/PROVENANCE.md`.
