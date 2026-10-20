# Android v1 build and runtime receipt

Run date: 2026-10-19, America/Chicago
Branch: `android/v1-app`
Tested source commit: `30bbe03bdffe80d09a5242508d4b5fab19289c04` (`git diff 30bbe03bdffe80d09a5242508d4b5fab19289c04..HEAD -- ':!receipts'` is empty)
Worktree clean at test: `true`
Main base commit: `041d1bab092553798dec6f7b84df1edfbe417ab4`

## Tested source and test file SHA-256 manifest

- `0778b716e142fdee3079906d35aae643e72b1313d6a90b8cf8d8b1aa33e90313`  `crates/oklo_core_jni/Cargo.toml`
- `b70ae83973b2569d2d1f8608c846c73455cd7c3f6e827ab44b6d9f1110983862`  `crates/oklo_core_jni/src/lib.rs`
- `49fa402ead90247f7bf9e45c7a637783c1344ab5e662bb4a8a49013a24b7c003`  `android/app/build.gradle.kts`
- `7de7689f244c2bb543ac7f887b458ccdc6192e69b847bead9c9915af6798bc9b`  `android/app/src/main/AndroidManifest.xml`
- `6e1a26f36378217a79287187c754aad3d15c1ebf2837be987e7859a659e76249`  `android/app/src/main/kotlin/org/oklo/pool/MainActivity.kt`
- `cef3d2595ef08d94b904260383c76b09e9e35e3ac64e8dd885e4e3078a12799f`  `android/app/src/main/kotlin/org/oklo/pool/ViewerModel.kt`
- `8f79a71691f8b0693079079a0a97bef5d02ee73391dea0fbf58ff43272ffcd7f`  `android/app/src/main/kotlin/org/oklo/pool/bridge/OkloCoreNative.kt`
- `0f035385c57ffae9cb9299bef4b4b113c65903da9e342e9fee6f476f0b920f64`  `android/app/src/test/kotlin/org/oklo/pool/ViewerMathTest.kt`
- `9c4bfa3dccfce2f67b8b70f9b68ff9a9b670351d52ed7c19239dd8fbb200e059`  `android/app/src/androidTest/kotlin/org/oklo/pool/ViewerSmokeTest.kt`
- `217ddd1af3fa050dfa57e7ae9a185d73375fa92708d4cd8618e8b889befcf529`  `android/app/src/androidTest/kotlin/org/oklo/pool/bridge/NativeBridgeTest.kt`

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
./gradlew testDebugUnitTest assembleDebug assembleAndroidTest bundleRelease
```

Result: `BUILD SUCCESSFUL`. Seven unit tests passed, zero failed. The instrumentation APK compiled and packaged cleanly. Unit tests cover phase advancement and wraparound, negative step rejection, pulse boundary selection, null-scenario phase invariance, continuous-field summaries, and categorical zone-share summaries.

The Android chart separates prescribed pulse/desiccation drivers from field values so the curves use independent axes. Continuous fields show a mean trace and min/max envelope; the categorical compartment field shows zone 3 and zone 4 voxel shares rather than a categorical mean.

Artifacts:

- Debug APK: `android/app/build/outputs/apk/debug/app-debug.apk`, SHA-256 `00a62e9445eed1cf52fcafb64193aca1fbf0d0cc6f81de92ddf3193f35db08d2`
- Instrumentation APK: `android/app/build/outputs/apk/androidTest/debug/app-debug-androidTest.apk`, SHA-256 `440d57d4777406eec8c5c637bb99bed782c46ddad422b05eb1b0597d862aa53e`
- Release AAB: `android/app/build/outputs/bundle/release/app-release.aab`, SHA-256 `2eb46e9e0c0215bad255dc2f90cdb760584492cbaffb61e32f3e6970cc6db1b6`

The APK reports package `org.oklo.pool`, version `1.0.0`, min API 26, target API 36, and native ABIs `arm64-v8a` and `x86_64`. The merged manifest contains only the AndroidX-generated signature-only permission `org.oklo.pool.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION`; it does not request `INTERNET` or user data access.

The release AAB is unsigned. Build success does not make it suitable for store upload without release signing keys.

## Emulator runtime verification (`arm64-v8a`, API 36)

Target: Android Emulator `oklo-api36` (`system-images;android-36;google_apis;arm64-v8a`, Android 16 / API 36, `1080x2400` at 420 dpi).
Command used:

```sh
adb install -r android/app/build/outputs/apk/debug/app-debug.apk
adb install -r android/app/build/outputs/apk/androidTest/debug/app-debug-androidTest.apk
adb shell am instrument -w -r org.oklo.pool.test/androidx.test.runner.AndroidJUnitRunner
```

Result: `PASS` (`OK (11 tests)`, 0 failures, 46.945 s).
Verbatim log: `receipts/android_emulator_test_2026-10-19.txt` (SHA-256 `627bfea9dd43f2488add185f7984ca75d29c384dfe8a2ae4900ff68c7d9d1e0c`).

Passing test methods:

1. `org.oklo.pool.ViewerSmokeTest#allSixFieldsCanBeSelected` - `PASS`
2. `org.oklo.pool.ViewerSmokeTest#provenanceSheetOpensAndDisplaysLimitations` - `PASS`
3. `org.oklo.pool.ViewerSmokeTest#pulseAndNullControlsHaveAccessibleLabelsAndUpdate` - `PASS`
4. `org.oklo.pool.ViewerSmokeTest#planSectionVolumeAndChartModesRenderLabels` - `PASS`
5. `org.oklo.pool.ViewerSmokeTest#playbackStopsWhenActivityLeavesForeground` - `PASS`
6. `org.oklo.pool.bridge.NativeBridgeTest#abiVersionAndSixFieldsLoad` - `PASS`
7. `org.oklo.pool.bridge.NativeBridgeTest#shortDestinationIsRejected` - `PASS`
8. `org.oklo.pool.bridge.NativeBridgeTest#closeIsIdempotentAtWrapperBoundary` - `PASS`
9. `org.oklo.pool.bridge.NativeBridgeTest#cf2TwentyCaseMatrixMatchesReference` - `PASS` (all 20 CF-2 cases and 120 3D field volumes matched reference SHA-256 digests bit-for-bit over JNI on `arm64-v8a`)
10. `org.oklo.pool.bridge.NativeBridgeTest#invalidConfigIsRejected` - `PASS`
11. `org.oklo.pool.bridge.NativeBridgeTest#steppingWrapsAndRejectsNegativeTime` - `PASS`

Runtime network check:

- While `org.oklo.pool` was running in the foreground (`PID 4165`), `adb shell dumpsys package org.oklo.pool` confirmed only `org.oklo.pool.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION` is declared or granted (no `android.permission.INTERNET`), and `adb shell ss -tupn` confirmed zero active TCP/UDP sockets (`ACTIVE_INET_SOCKETS: 0`).
- Five `1080x2400` PNG screenshots were captured from the running emulator across Plan, Section, Volume, Chart, and Scope/Provenance views.
- Android `x86_64` ABI is **build-verified, runtime not tested**: `x86_64/liboklo_core_jni.so` (SHA-256 `7ed74f69a8f6dd0e60aadf9b37dae136e26331229d6dc955081e0592fb0c71f2`) is compiled, packaged, and verified at 16 KB ELF `PT_LOAD` alignment, while `x86_64` emulator runtime execution is not tested (`BLOCKED` on the Apple Silicon `arm64` host).
- Physical-device testing is `NOT RUN`; no physical Android device and release signing setup were connected in this session.

## Limitations

These results establish buildability, unit-test outcomes, `arm64-v8a` API 36 emulator installation, JNI 20-case CF-2 conformance, Compose UI smoke tests, lifecycle background-pause behavior, and runtime absence of network permissions/sockets. They do not replace physical-device testing or release signing. Values remain the prescribed revision 0 fixture functions documented in `docs/PROVENANCE.md`.
