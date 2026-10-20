# Oklo Fixture Viewer v1 release gate report

Date: 2026-10-19
Recommendation: **NOT STORE-SUBMISSION-READY** (All repository engineering, 20-case CF-2 cross-platform conformance, native iOS and Android builds, simulator and emulator UI/instrumentation suites, runtime network audits, and authentic screenshots are **COMPLETE and PASSING**; final readiness is gated only on external account-owner tasks: physical-device smoke tests, production code-signing credentials, public HTTPS privacy-policy hosting, and store console submission.)

## Branch and pull request status

| Workstream | Branch, receipt head, and tested source commit | Evidence | Status |
| --- | --- | --- | --- |
| PR A conformance | [`conformance/cf2-suite` at `3a2d41e6260d4dfe53b70c83be97861740ac6930` (tested source commit `ae56062a9f96fe12988a658dd89b3427e244bc9a`)](https://github.com/aurascoper/oklo_pool_mobile/pull/4) | `receipts/cf2_conformance_2026-10-19.txt`, `receipts/cf2_matrix_2026-10-19.json`, `receipts/cf2_reference_outputs_sha256_2026-10-19.txt` | Open; **PASS across all 4 runnable targets** (Host Rust, Swift Package, iOS 27.0 Simulator, Android API 36 `arm64-v8a` Emulator; Android `x86_64` JNI build-verified, runtime not tested). |
| PR B iOS app | [`ios/v1-app` at `49a5169328681e9145faacffcf6819d5572b0d9d` (tested source commit `8235b1a36e870dd0c0347501129802f546587acd`)](https://github.com/aurascoper/oklo_pool_mobile/pull/5) | `receipts/ios_simulator_test_2026-10-19.md`, `.json`, `.txt` | Open; **PASS** (generic device + simulator builds pass; 5/5 Swift package tests pass; **15/15 simulator unit + UI tests pass** on `iPhone 17` iOS 27.0 `arm64` including Option A ontology test; `ZERO_INET_SOCKETS`). |
| PR C Android app | [`android/v1-app` at `f0b0b268f1b89a0c17c90b316fbf08aee6850f16` (tested source commit `a4673ec173a8da5c6ddf254edf9c576dbe500449`)](https://github.com/aurascoper/oklo_pool_mobile/pull/6) | `receipts/android_build_2026-10-19.md`, `receipts/android_emulator_test_2026-10-19.txt` | Open; **PASS** (7/7 unit tests pass; debug APK, androidTest APK, and unsigned release AAB build; **11/11 API 36 `arm64-v8a` emulator instrumented tests pass** including Option A verification; `x86_64` build-verified, runtime not tested; `ACTIVE_INET_SOCKETS: 0`). |
| PR D release materials | [`release/v1-materials`](https://github.com/aurascoper/oklo_pool_mobile/pull/7) | `README.md`, `docs/specs/FIELD_ONTOLOGY_AMENDMENT_A1.md`, `docs/store/*`, `docs/store/screenshots/*.png`, `receipts/ios_build_2026-10-19.md` | Open; merged with `ios/v1-app` (`49a5169`); includes updated project landing page (`README.md`), `FIELD_ONTOLOGY_AMENDMENT_A1.md`, 9 authentic simulator/emulator screenshots, privacy policy (MD + HTML), store drafts, and this report. |

All four pull requests remain open and unmerged in accordance with project governance rules.

## Test matrix by platform, architecture, and suite

| Target | Architecture | Suite / Command | Result | Raw receipt location |
| --- | --- | --- | --- | --- |
| Host Rust CF-2 suite | `aarch64-apple-darwin` | `cargo test --release -p conformance_cf2` | **PASS** (9/9 tests; 20 cases / 120 field volumes bit-identical to Python reference) | `receipts/cf2_conformance_2026-10-19.txt` (`conformance/cf2-suite`) |
| `OkloCoreKit` Swift Package | `aarch64-apple-darwin` | `(cd ios/OkloCoreKit && swift test)` | **PASS** (5/5 tests, 6 fields bit-identical) | `receipts/cf2_conformance_2026-10-19.txt` (`conformance/cf2-suite`) |
| iOS App & Unit/UI Simulator Suite (`iPhone 17`, iOS 27.0 `24A5297g`) | `aarch64-apple-ios-sim` | `xcodebuild test -project ios/OkloPool.xcodeproj -scheme OkloPool -destination 'platform=iOS Simulator,id=646CF1C6-8720-4906-B73B-FD46ACAA8EF0'` | **PASS** (**15/15 tests**: 11 unit tests including 20-case CF-2 matrix and Option A ontology test + 4 UI tests) | `receipts/ios_simulator_test_2026-10-19.txt` (SHA-256 `7897563507a351d1bb9b42660932749439d1c93b0e4090a6464d01128609884d`) |
| iOS Generic Device Build | `aarch64-apple-ios` | `xcodebuild -project ios/OkloPool.xcodeproj -scheme OkloPool -destination 'generic/platform=iOS' CODE_SIGNING_ALLOWED=NO build` | **PASS** (`BUILD SUCCEEDED`) | `receipts/ios_build_2026-10-19.md` |
| Android JVM Unit Suite | `aarch64` host JVM 17 | `./gradlew testDebugUnitTest` | **PASS** (**7/7 tests**) | `receipts/android_build_2026-10-19.md` (`android/v1-app`) |
| Android API 36 Emulator Instrumentation Suite (`oklo-api36`, Android 16) | `arm64-v8a` | `adb shell am instrument -w -r org.oklo.pool.test/androidx.test.runner.AndroidJUnitRunner` | **PASS** (**11/11 tests**: 6 JNI bridge tests including 20-case CF-2 matrix + 5 Compose UI smoke tests verifying Option A) | `receipts/android_emulator_test_2026-10-19.txt` on `android/v1-app` (SHA-256 `6f2c4c76af2add5cb8c6e393b46c3a7bd86b1e9e644185ab10fd797e3f4d4571`) |
| Android `x86_64` ABI | `x86_64` | `cargo ndk -t arm64-v8a -t x86_64 ...` + `./gradlew assembleDebug bundleRelease` | **BUILD-VERIFIED, RUNTIME NOT TESTED** (`liboklo_core_jni.so` built and 16 KB ELF aligned; `x86_64` emulator execution blocked on Apple Silicon `arm64` host) | `receipts/android_build_2026-10-19.md` (`android/v1-app`) |
| Physical iOS and Android devices | `arm64` / `arm64-v8a` | Physical device installation and smoke test | **NOT RUN** (no physical hardware or signing certificates attached) | Documented below |

## Build artifacts and SHA-256 digests

### Android (`android/v1-app` at `a4673ec`)

- `arm64-v8a/liboklo_core_jni.so` (16 KB ELF `PT_LOAD` aligned): `802c4aa33251cdda30a2a81ce672ecd611899a9ba8cf9dbc48ebef8b3b397092`
- `x86_64/liboklo_core_jni.so` (16 KB ELF `PT_LOAD` aligned): `7ed74f69a8f6dd0e60aadf9b37dae136e26331229d6dc955081e0592fb0c71f2`
- `android/app/build/outputs/apk/debug/app-debug.apk`: `fb61ab2b4bf19f7ca68084d35f0c21cf9d64621ce6609af31c2e043107100220`
- `android/app/build/outputs/apk/androidTest/debug/app-debug-androidTest.apk`: `e29be7cfaab94c6265ebadf3c283bb3e0fc6f135005d9f2576e74ac75590c35f`
- `android/app/build/outputs/bundle/release/app-release.aab` (unsigned): `1db6f02a4de373f32dc4c37da8ba4c82b08e89cade6b452398e0cebe056b201a`

### iOS (`ios/v1-app` at `8235b1a` / `release/v1-materials`)

- App icon (`1024x1024` PNG, `docs/store/assets/oklo-fixture-viewer-icon.png`): `96b4d6cc4b5b5e15f782312fab80632cf996e9bd5fccc8d05e57f3d844757f49`
- 4 iOS Simulator screenshots (`1206x2622` PNG) and 5 Android API 36 Emulator screenshots (`1080x2400` PNG): see [`docs/store/screenshots/README.md`](screenshots/README.md).

## Verified privacy and data-flow findings

1. **Source and dependency audit:**
   - iOS declares only Apple system frameworks (`SwiftUI`, `Charts`, `MetalKit`) and local `OkloCoreKit` (`swift package show-dependencies` has zero external dependencies).
   - Android `releaseRuntimeClasspath` (`docs/store/android-release-runtime-dependencies.txt`, SHA-256 `c816dc9fef79ba0977aa1b111cf125e8b4e77fcfd1b4752a2ae2d33b752df190`) contains only AndroidX, Kotlin stdlib/coroutines, and Guava `listenablefuture`. No networking, analytics, advertising, attribution, or crash-reporting library is present.
2. **Manifest and permission audit:**
   - Android merged manifest requests no `android.permission.INTERNET` and no user-data permissions (only the AndroidX-generated signature-only `org.oklo.pool.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION`), and sets `allowBackup=false`.
   - iOS `Info.plist` declares no sensitive resource usage descriptions.
3. **Runtime socket verification:**
   - iOS Simulator (`iPhone 17`, iOS 27.0, `PID 47404`): `lsof -n -P -i -a -p 47404` -> `ZERO_INET_SOCKETS`.
   - Android API 36 Emulator (`oklo-api36`, `PID 2193` / `UID 10213`): `adb shell dumpsys package org.oklo.pool` confirmed no `INTERNET` permission and `/proc/net/{tcp,tcp6,udp,udp6}` confirmed `ACTIVE_INET_SOCKETS: 0`.

## Remaining external prerequisites before store submission

1. **Physical-device smoke tests:** Install and verify the signed iOS and Android builds on physical hardware (`NOT RUN` in this session; no physical devices were connected).
2. **Production code signing:** Sign `app-release.aab` with the Google Play upload key and archive/sign `OkloPool` with an Apple Developer distribution certificate and provisioning profile.
3. **Public HTTPS privacy policy hosting:** Fill in the publisher name, effective date, and monitored support email in `docs/store/privacy-policy.html` / `docs/store/privacy-policy.md` and host it at a public HTTPS URL.
4. **Store console completion:** Enter the prepared App Store Connect and Google Play Console metadata, age-rating questionnaires, App Privacy / Data safety forms, app icons, and screenshots, and submit under the account owner's credentials.

The scientific fixture remains prescribed revision 0 analytic functions, not empirically validated reactor dynamics. Both apps present the full provenance and limitations disclosures in-app.
