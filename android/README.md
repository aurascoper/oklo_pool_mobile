# Oklo Fixture Viewer for Android

This Android app renders the revision 0 six-field fixture through the Rust `oklo_core` library. It is a prescribed-function viewer, not a physical simulation.

## Requirements

- JDK 17
- Android SDK Platform 36 and Build Tools 36.0.0
- Android NDK 28.2.13676358
- Rust toolchain with `aarch64-linux-android` and `x86_64-linux-android` targets
- `cargo-ndk`
- Android Emulator with an API 36 Google APIs image for runtime and screenshot checks

Set `JAVA_HOME`, `ANDROID_HOME`, and `ANDROID_NDK_HOME` for the local installation. The committed wrapper pins Gradle 8.11.1.

## Build

From the repository root, build the two packaged JNI libraries:

```sh
scripts/build_android_native.sh
```

Then build and test the application:

```sh
cd android
./gradlew testDebugUnitTest
./gradlew assembleDebug
./gradlew bundleRelease
```

The native bridge uses integer handles backed by a synchronized registry. `OkloSim` serializes calls and makes close idempotent. Field arrays are copied with JNI `SetFloatArrayRegion`; the Kotlin layer never retains a pointer into Rust memory. ELF load segments are linked with 16 KB maximum page alignment.

## Runtime checks

Instrumented tests are in `app/src/androidTest`. Run them on API 36 arm64 and x86_64 emulators:

```sh
./gradlew connectedDebugAndroidTest
```

Capture store screenshots only from the running application. Runtime, accessibility, lifecycle, and physical-device checks remain separate evidence from build success.

## Data and limitations

The app does not request data access or network permissions. The merged manifest contains one AndroidX-generated signature-only permission for non-exported dynamic receivers. Inputs and rendering are implemented locally. Every displayed value comes from fixed analytic fixture functions. Labels preserve the revision 0 units, including fields whose original unit labels lack a derivation. Phase controls navigate prescribed values and do not model time-dependent physics. Network behavior has not yet been checked at runtime. See `../docs/PROVENANCE.md` for the full limitations record.
