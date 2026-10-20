# Dependency and data-flow inventory

Review date: 2026-10-19
Scope: iOS app sources on `ios/v1-app` at tested source commit `2db67d4ce3526bb2968954fecdba6ade7a991b02` (receipt commit `4742bb57dab2ee175777c5991cca8c05a4f43080`) and Android app sources on `android/v1-app` at tested source commit `30bbe03bdffe80d09a5242508d4b5fab19289c04` (receipt commit `7edcd0ed58f233630ddf579d96dff859beeeb6c8`).
Status: source inspection, resolved production dependency graph, and simulator/emulator runtime socket checks completed; physical-device signed release package verification remains an account-owner step.

## User inputs and data path

1. The app starts with built-in revision 0 fixture inputs.
2. Field, phase, pulse-boundary, null-scenario, slice, and display controls update in-memory view state.
3. The Rust `oklo_core` library computes the prescribed field values locally in process memory.
4. SwiftUI/Metal (iOS) or Kotlin/Compose (Android) renders slices, charts, and the volume view on the device screen.

No account, upload, cloud sync, telemetry, or remote model endpoint exists in the reviewed app code.

## Direct and transitive dependencies

| Platform | Declared app dependencies | Data or network role found in source and dependency review |
| --- | --- | --- |
| iOS | Apple SwiftUI, Charts, MetalKit; local `OkloCoreKit` wrapping the Rust C ABI | No external Swift package dependency (`swift package show-dependencies` is empty). No analytics or advertising SDK is declared. Source search found no `URLSession`, `WKWebView`, socket, location, camera, photo, or `UserDefaults` persistence flow. |
| Android | AndroidX Activity, Lifecycle, Compose UI/Foundation/Material3; Kotlin coroutines; local Rust JNI library (`liboklo_core_jni.so`) | No analytics or advertising SDK is declared. The app manifest does not request `INTERNET`, location, camera, contacts, media, or storage permissions. |
| Tests only | Swift XCTest; Android JUnit, AndroidX test, Espresso, Compose UI test | Used only in test targets (`OkloPoolTests`, `OkloPoolUITests`, `androidTest`); not packaged in release builds. |

The complete Android `releaseRuntimeClasspath` graph was captured on 2026-10-19 with JDK 17 and Gradle 8.11.1; see [`android-release-runtime-dependencies.txt`](android-release-runtime-dependencies.txt), SHA-256 `c816dc9fef79ba0977aa1b111cf125e8b4e77fcfd1b4752a2ae2d33b752df190`. It contains only AndroidX, Kotlin standard library and coroutines, and Guava `listenablefuture`; no analytics, advertising, attribution, crash-reporting, or app networking library is present in the resolved production graph.

## Permissions and storage

- Android source manifest declares no user-data or network permission. The merged APK adds only the AndroidX-generated signature-only permission `org.oklo.pool.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION` for non-exported dynamic receivers.
- Android sets `allowBackup=false`.
- The iOS project declares no data-access usage description in its generated `Info.plist` settings.
- Neither application persists user settings or state across launches.

## Runtime network and telemetry checks

1. **iOS Simulator (`iPhone 17`, iOS 27.0 `arm64`, PID `47404`):**
   - While `OkloPool.app` was running and after exercising all six fields, 2D/3D views, chart, and provenance sheet, `lsof -n -P -i -a -p 47404` returned zero open `IPv4` or `IPv6` sockets (`ZERO_INET_SOCKETS`). See `receipts/ios_simulator_test_2026-10-19.txt`.
2. **Android Emulator (`oklo-api36`, Android 16 / API 36 `arm64-v8a`, PID `4165`):**
   - While `org.oklo.pool` was running in the foreground, `adb shell dumpsys package org.oklo.pool` confirmed that only `org.oklo.pool.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION` is declared or granted (no `android.permission.INTERNET` permission, so the Linux kernel denies `AF_INET`/`AF_INET6` socket creation for the app UID), and `adb shell ss -tupn` confirmed zero active TCP/UDP sockets (`ACTIVE_INET_SOCKETS: 0`). See `receipts/android_emulator_test_2026-10-19.txt` on `android/v1-app`.

## Evidence summary and remaining account-owner checks

- iOS simulator build, 14/14 test pass, and runtime socket check: `receipts/ios_simulator_test_2026-10-19.md` and `receipts/ios_simulator_test_2026-10-19.txt`.
- Android APK/AAB build, 7/7 unit test pass, 11/11 API 36 emulator instrumentation pass, and runtime socket/permission check: `receipts/android_build_2026-10-19.md` and `receipts/android_emulator_test_2026-10-19.txt` on `android/v1-app`.
- Physical-device smoke tests and final signed release binary inspection remain to be completed by the account owner prior to store submission.
