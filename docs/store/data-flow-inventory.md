# Dependency and data-flow inventory

Review date: 2026-10-19
Scope: app sources on `ios/v1-app` at `29f7573` and Android implementation on `android/v1-app` at `11f9561`.
Status: source inspection and resolved production dependency graph captured; runtime network checks remain blocked.

## User inputs and data path

1. The app starts with built-in revision 0 fixture inputs.
2. Field, phase, pulse-boundary, null-scenario, slice, and display controls update in-memory view state.
3. The Rust `oklo_core` library computes the prescribed field values.
4. SwiftUI/Metal or Kotlin/Compose renders slices, charts, and the volume view.

No account, upload, cloud sync, or remote model endpoint is declared in the reviewed app code. This does not establish what the installed apps or operating systems do at runtime.

## Direct dependencies

| Platform | Declared app dependencies | Data or network role found in source review |
| --- | --- | --- |
| iOS | Apple SwiftUI, Charts, MetalKit; local `OkloCoreKit` wrapping the Rust C ABI | No analytics or advertising SDK is declared. The app source search found no URL session, web view, endpoint, location, camera, photo, or user-defaults flow. |
| Android | AndroidX Activity, Lifecycle, Compose UI/Foundation/Material3; Kotlin coroutines; local Rust JNI library | No analytics or advertising SDK is declared. The app manifest does not request `INTERNET`, location, camera, contacts, media, or storage permissions. |
| Tests only | Swift XCTest; Android JUnit, AndroidX test, Espresso, Compose UI test | Not packaged as production app features. |

The resolved iOS Swift package has no external package dependencies. The complete Android `releaseRuntimeClasspath` graph was captured on 2026-10-19 with JDK 17 and Gradle 8.11.1; see [`android-release-runtime-dependencies.txt`](android-release-runtime-dependencies.txt), SHA-256 `c816dc9fef79ba0977aa1b111cf125e8b4e77fcfd1b4752a2ae2d33b752df190`. It contains AndroidX, Kotlin standard library and coroutines, and Guava `listenablefuture`; no analytics, advertising, attribution, crash-reporting, or app networking library is present in that resolved production graph. This is package inventory evidence, not runtime behavior evidence. Final signed-package and any store-required SDK-manifest review remain.

## Permissions and storage

- Android source manifest declares no user-data or network permission. The merged APK adds the AndroidX-generated signature-only permission `org.oklo.pool.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION` for dynamic receivers.
- Android sets `allowBackup=false`.
- The iOS project has no declared data-access usage description in its generated Info.plist settings.
- Runtime storage behavior and operating-system telemetry have not been measured.

## Network and telemetry

Static inspection found no app API endpoint, networking library, analytics SDK, ad SDK, or tracking code in the reviewed app sources and direct dependency declarations. No runtime network capture has been completed on either platform. Therefore network transmission, telemetry, collection, sharing, and tracking status are **UNVERIFIED at runtime** and must not be reported as zero.

## Evidence and verification gaps

- Android APK manifest and package architecture inspection: `receipts/android_build_2026-10-19.md`.
- Android instrumentation test APK compiled, including all-field selection, four view modes, and stop-on-background checks, but tests were not run because no usable Android emulator could be started.
- iOS generic device and simulator builds and `build-for-testing` succeeded. The simulator UI tests did not execute because CoreSimulator lists no usable iOS runtime.
- Physical-device tests were not run.
- Store declarations remain drafts. The store account owner must review the signed release build and enter the disclosures.
