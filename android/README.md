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

Instrumented tests are in `app/src/androidTest`. Run them on an API 36 emulator:

```sh
./gradlew connectedDebugAndroidTest
```

Or install the built APKs directly and run via `adb`:

```sh
adb install -r app/build/outputs/apk/debug/app-debug.apk
adb install -r app/build/outputs/apk/androidTest/debug/app-debug-androidTest.apk
adb shell am instrument -w -r org.oklo.pool.test/androidx.test.runner.AndroidJUnitRunner
```

On the API 36 `arm64-v8a` emulator (`oklo-api36`), all 11 instrumented tests (6 native JNI bridge tests including the 20-case CF-2 matrix and 5 Compose UI smoke tests) passed; see `../receipts/android_build_2026-10-19.md` and `../receipts/android_emulator_test_2026-10-19.txt`.

## Data and limitations

The app does not request data access or network permissions. The merged manifest contains one AndroidX-generated signature-only permission for non-exported dynamic receivers, and runtime inspection on API 36 confirmed zero network permissions and zero open sockets. Inputs and rendering are implemented locally. Every displayed value comes from fixed analytic fixture functions. Under the Option A display-ontology amendment, field 4 (`polymer_yield`) is presented as `Polymer accumulation index` (`arbitrary units`), with the in-app provenance sheet explaining that revision 0 labels the quantity in `umol/L` without a kinetic or thermodynamic basis. Phase controls navigate prescribed values and do not model time-dependent physics. See `../docs/PROVENANCE.md` for the full limitations record.
