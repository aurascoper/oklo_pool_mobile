# Oklo Fixture Viewer v1 release gate report

Date: 2026-10-19
Recommendation: **NOT STORE-SUBMISSION-READY**

## Branch and review status

| Workstream | Branch and commit | Evidence | Status |
| --- | --- | --- | --- |
| PR A conformance | [`conformance/cf2-suite` at `b4ec76ac79e21f6d198234f8c280d094fc7e803f`](https://github.com/aurascoper/oklo_pool_mobile/pull/4) | 20 cases, 120 field volumes; host Rust and Swift package pass. See conformance receipt on that branch. | Open; gate PASS; simulator and emulator targets blocked. |
| PR B iOS | [`ios/v1-app` at `29f75739bddd4a42f24924eb1a2fac7f92736097`](https://github.com/aurascoper/oklo_pool_mobile/pull/5) | XCFramework generated locally; generic device, generic simulator, and simulator test bundles build; Swift package 5/5. | Open; build PASS; simulator tests and device smoke test blocked. |
| PR C Android | [`android/v1-app` at `11f9561287418b31cc291004dcaceeb9917039c9`](https://github.com/aurascoper/oklo_pool_mobile/pull/6) | `receipts/android_build_2026-10-19.md`; 7/7 unit tests; APK, instrumentation APK, and AAB build. | Open; build PASS; emulator tests, screenshots, physical smoke test, and signing remain. |
| PR D release materials | [`release/v1-materials`](https://github.com/aurascoper/oklo_pool_mobile/pull/7) | This document and supporting drafts; targets `ios/v1-app`. | Open; prepared; requires PR B and Android PR C evidence. |

All four requested pull requests are open. PR D is based on PR B and targets `ios/v1-app`; after PR B and PR C are approved and merged, reconcile PR D against `main`. No branch is merged into `main`.

GitHub status was rechecked on 2026-10-19. All four requested pull requests remain open; no review decisions or commit status checks are recorded. PR D compares seven commits ahead and zero behind its base.

## Build evidence

### Android

Command from `android/`:

```sh
JAVA_HOME=/opt/homebrew/opt/openjdk@17/libexec/openjdk.jdk/Contents/Home \
ANDROID_HOME=/opt/homebrew/share/android-commandlinetools \
ANDROID_NDK_HOME=/opt/homebrew/share/android-commandlinetools/ndk/28.2.13676358 \
./gradlew testDebugUnitTest assembleAndroidTest assembleDebug bundleRelease
```

- Seven unit tests passed.
- Instrumentation test APK compiled, including UI checks for all six fields, four visualization modes, and stopping playback on background; tests were not run.
- Continuous fields chart mean and min/max envelope separately from prescribed drivers; the compartment chart plots zone 3 and zone 4 shares.
- Debug APK SHA-256: `92224fc11ca7c58077ea6eade630e141ab360b49d9cbf8bbb8b51b6bf7ebdf3d`.
- Instrumentation APK SHA-256: `2b46e34d2995956ab02d4585e76a744849594ff6a6124b75bdf7a14b20184dc3`.
- Release AAB SHA-256: `6af2123768b8c1a3672e7219f625c9a48890d26de710a37bfb4c736e05b01159`.
- Both native ABIs are packaged and 16 KB ELF aligned.
- The full resolved Android release runtime graph is captured in `docs/store/android-release-runtime-dependencies.txt`; it shows AndroidX, Kotlin, and Guava support libraries, with no app networking, analytics, ads, attribution, or crash-reporting SDK.
- The AAB is unsigned. It is not ready for Play upload.
- Emulator startup was blocked: userdata needed 7372.80 MB; 2052.19 MB was available.

### iOS

Commands from the repository root:

```sh
bash scripts/build_xcframework.sh
xcodebuild -project ios/OkloPool.xcodeproj -scheme OkloPool \
  -destination 'generic/platform=iOS' CODE_SIGNING_ALLOWED=NO build
xcodebuild -project ios/OkloPool.xcodeproj -scheme OkloPool \
  -destination 'generic/platform=iOS Simulator' CODE_SIGNING_ALLOWED=NO build
xcodebuild -project ios/OkloPool.xcodeproj -scheme OkloPool \
  -destination 'generic/platform=iOS Simulator' ARCHS=arm64 \
  CODE_SIGNING_ALLOWED=NO build-for-testing
(cd ios/OkloCoreKit && swift test)
```

- Generic device and simulator builds passed.
- App, unit-test, and UI-test bundles compiled for simulator.
- Swift package tests passed: 5 tests, including six field comparisons.
- `xcrun simctl list runtimes` reports no registered iOS runtime; simulator tests and screenshots were not run.
- Physical-device testing and distribution signing were not available.

## Privacy evidence and limits

Static source and resolved dependency review found no declared app network client, account flow, analytics SDK, ad SDK, or user-data permission. The Android release graph is captured; the iOS Swift package graph has no external dependencies. Android's merged manifest contains the AndroidX-generated signature-only dynamic receiver permission and no `INTERNET` permission. iOS does not declare a data-access usage description. These are package and source findings. Runtime network transmission and operating-system behavior remain **UNVERIFIED**. See the data-flow inventory and privacy drafts.

The Play build targets API 36. Google Play currently requires API 36 or higher for new apps and updates starting 2026-08-31. The Data safety form remains an account-owner task even if final testing supports a no-data answer. Apple's privacy disclosures and any applicable privacy manifest must be based on the final signed app and SDK review.

## Release blockers

- iOS simulator runtime unavailable to CoreSimulator; no UI/lifecycle execution or screenshots.
- Android emulator cannot allocate the required userdata storage; no instrumentation or ABI runtime execution.
- No physical-device smoke tests.
- Runtime network tests and final signed-package/privacy-manifest review remain.
- Android AAB is unsigned; no Apple distribution signing identity or provisioning was used.
- App Privacy and Data safety declarations are not complete.
- Privacy policy requires verified data practices, publisher approval, support contact, and public HTTPS hosting.
- Store listing, content ratings, account agreements, and screenshots remain account-owner tasks.
- No store listing, store submission, or public release has been made.

The scientific fixture remains prescribed functions, not physically validated reactor dynamics. The app must retain the provenance and limitations disclosures.
