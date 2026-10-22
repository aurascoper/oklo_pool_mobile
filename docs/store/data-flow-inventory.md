# Dependency and data-flow inventory

Review date: 2026-10-20
Scope: iOS app sources on `release/on-render-gate` at tested source commit `dd1ce73bea4716cd5a8161a56bdc8868fd792a01` (receipt head `fef07277ccb30826c12875e150c5d0405bb0be20`) and Android app sources on `release/on-render-gate`.
Status: source inspection, resolved production dependency graph, privacy manifest verification, and simulator/emulator runtime socket checks completed; physical-device signed release package verification remains an account-owner step.

## User inputs and data path

1. The app starts with built-in revision 0 fixture inputs and displays the persistent initial-screen disclaimer:
   "Revision-0 prescribed fields; not a physical simulation."
2. Field, phase, pulse-boundary, null-scenario, slice, and display controls update in-memory view state.
3. Each active field renders an adjacent limitation statement, and all six field limitation records are displayed in the Scope and Provenance view:
   - Temperature: Prescribed thermal profile; not coupled to hydrodynamic or convective cooling.
   - Hydration: Prescribed saturation index; not a multiphase porous flow solve.
   - H2 Concentration: Prescribed radiolysis proxy; not a reaction-diffusion kinetic solve.
   - 235U/238U Ratio: Prescribed isotopic depletion curve; not a 3D neutron transport solve.
   - Polymer accumulation index (arbitrary units): Prescribed heuristic field. The revision-0 fixture labels this quantity in umol/L, but no kinetic or thermodynamic basis for those concentration units is established. Values are not measured or predicted polymer concentrations.
   - Compartments: Prescribed geometric bounding cells; not a physical membrane simulation.
4. The Rust `oklo_core` library computes the prescribed field values locally in process memory.
5. SwiftUI/Metal (iOS) or Kotlin/Compose (Android) renders slices, charts, and the volume view on the device screen.

No account, upload, cloud sync, telemetry, or remote model endpoint exists in the reviewed app code.

## Direct and transitive dependencies

| Platform | Declared app dependencies | Data or network role found in source and dependency review |
| --- | --- | --- |
| iOS | Apple SwiftUI, Charts, MetalKit; local `OkloCoreKit` wrapping the Rust C ABI | No external Swift package dependency (`swift package show-dependencies` is empty). No analytics or advertising SDK is declared. Source search found no `URLSession`, `WKWebView`, socket, location, camera, photo, or `UserDefaults` persistence flow. `PrivacyInfo.xcprivacy` declares zero tracking and zero data collection. |
| Android | AndroidX Activity, Lifecycle, Compose UI/Foundation/Material3; Kotlin coroutines; local Rust JNI library (`liboklo_core_jni.so`) | No analytics or advertising SDK is declared. The app manifest does not request `INTERNET`, location, camera, contacts, media, or storage permissions. |
| Tests only | Swift XCTest; Android JUnit, AndroidX test, Espresso, Compose UI test | Used only in test targets (`OkloPoolTests`, `OkloPoolUITests`, `androidTest`); not packaged in release builds. |

The complete Android `releaseRuntimeClasspath` graph was captured with JDK 17 and Gradle 8.11.1; see [`android-release-runtime-dependencies.txt`](android-release-runtime-dependencies.txt), SHA-256 `c816dc9fef79ba0977aa1b111cf125e8b4e77fcfd1b4752a2ae2d33b752df190`. It contains only AndroidX, Kotlin standard library and coroutines, and Guava `listenablefuture`; no analytics, advertising, attribution, crash-reporting, or app networking library is present in the resolved production graph.

## Permissions and storage

- Android source manifest declares no user-data or network permission (`android.permission.INTERNET` is absent). The merged APK adds only the AndroidX-generated signature-only permission `org.oklo.pool.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION` for non-exported dynamic receivers.
- Android sets `allowBackup=false`.
- The iOS project declares no data-access usage description in its generated `Info.plist` settings, and bundles `PrivacyInfo.xcprivacy` in the `OkloPool` app target.
- Neither application persists user settings or state across launches.
- Both applications bundle a complete offline plain-text privacy policy readable without network connectivity.
- Explicit disclosure is documented in-app and in release materials:
  "URL remains unverified until the account owner confirms live hosting" for `https://aurascoper.github.io/oklo_pool_mobile/privacy-policy.html`.

## Runtime network and telemetry checks

1. **iOS Simulator (`iPhone 17`, iOS 27.0 `arm64`, PID `47404` and `76862`):**
   - While `OkloPool.app` was running and after exercising all six fields, 2D/3D views, chart, and provenance sheet, `lsof -n -P -i -a -p <PID>` returned exit code 1 confirming zero open `IPv4` or `IPv6` sockets (`ZERO_INET_SOCKETS`). See `receipts/ios_simulator_test_2026-10-19.txt` and `receipts/on_render_gate_2026-10-20.md`.
2. **Android Emulator (`oklo-api36`, Android 16 / API 36 `arm64-v8a`):**
   - In prior verification run, `adb shell dumpsys package org.oklo.pool` confirmed only `org.oklo.pool.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION` was granted and zero TCP/UDP sockets were open (`ACTIVE_INET_SOCKETS: 0`).
   - In 2026-10-20 On-Render Gate testing, fresh `oklo-api36` AVD startup was `BLOCKED / UNRESOLVED` due to host APFS disk space constraint (<1 GiB available). Android JVM native tests against release JNI libraries verified 18/18 checks.

## Account-owner blocker catalog before store submission

1. Register bundle ID `org.oklo.pool` in Apple Developer portal, configure provisioning profiles, and confirm live hosting of `https://aurascoper.github.io/oklo_pool_mobile/privacy-policy.html`.
2. Perform physical iOS and Android device validation on real hardware via TestFlight and Google Play Console internal testing.
3. Free host APFS disk space (<1 GiB currently free) to unblock `oklo-api36` AVD startup for automated emulator verification.
4. Perform release signing and submit through App Store Connect and Google Play Console.
