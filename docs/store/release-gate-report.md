# Oklo Fixture Viewer v1 release gate report

Date: 2026-10-19
Recommendation: **NOT STORE-SUBMISSION-READY**

## Branch and review status

| Workstream | Branch and commit | Evidence | Status |
| --- | --- | --- | --- |
| PR A conformance | `conformance/cf2-suite` at `b4ec76ac79e21f6d198234f8c280d094fc7e803f` | 20 cases, 120 field volumes; host Rust and Swift package pass. See conformance receipt on that branch. | Gate PASS; simulator and emulator targets blocked. |
| PR B iOS | `ios/v1-app` at `29f7573` | XCFramework generated locally; generic device, generic simulator, and simulator test bundles build; Swift package 5/5. | Build PASS; simulator tests and device smoke test blocked. |
| PR C Android | `android/v1-app` at `088e668` | `receipts/android_build_2026-10-19.md`; 5/5 unit tests; APK and AAB build. | Build PASS; emulator tests, screenshots, physical smoke test, and signing remain. |
| PR D release materials | `release/v1-materials` | This document and supporting drafts. | Prepared; PR link pending. |

Pull requests have not been created yet. Branches are local and unmerged. No branch is merged into `main`.

## Build evidence

### Android

Command from `android/`:

```sh
JAVA_HOME=/opt/homebrew/opt/openjdk@17/libexec/openjdk.jdk/Contents/Home \
ANDROID_HOME=/opt/homebrew/share/android-commandlinetools \
ANDROID_NDK_HOME=/opt/homebrew/share/android-commandlinetools/ndk/28.2.13676358 \
./gradlew testDebugUnitTest assembleAndroidTest assembleDebug bundleRelease
```

- Five unit tests passed.
- Instrumentation test APK compiled; it was not run.
- Debug APK SHA-256: `95e5537da6db4821915ab23e31172a654cf3d9b547c9ee075cddb1ae41dd771f`.
- Release AAB SHA-256: `fd7b3c5bc712fbad5bf9a2fb211438df29a6eb68a82451246aa5efe54a9d57c1`.
- Both native ABIs are packaged and 16 KB ELF aligned.
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

Static source and direct dependency review found no declared app network client, account flow, analytics SDK, ad SDK, or user-data permission. Android's merged manifest contains the AndroidX-generated signature-only dynamic receiver permission and no `INTERNET` permission. iOS does not declare a data-access usage description. These are source-level findings. Runtime network transmission and third-party runtime behavior remain **UNVERIFIED**. See the data-flow inventory and privacy drafts.

The Play build targets API 36. Google Play currently requires API 36 or higher for new apps and updates starting 2026-08-31. The Data safety form remains an account-owner task even if final testing supports a no-data answer. Apple's privacy disclosures and any applicable privacy manifest must be based on the final signed app and SDK review.

## Release blockers

- iOS simulator runtime unavailable to CoreSimulator; no UI/lifecycle execution or screenshots.
- Android emulator cannot allocate the required userdata storage; no instrumentation or ABI runtime execution.
- No physical-device smoke tests.
- Runtime network tests and full transitive dependency/package review remain.
- Android AAB is unsigned; no Apple distribution signing identity or provisioning was used.
- App Privacy and Data safety declarations are not complete.
- Privacy policy requires verified data practices, publisher approval, support contact, and public HTTPS hosting.
- Store listing, content ratings, account agreements, and screenshots remain account-owner tasks.
- No store listing, store submission, or public release has been made.

The scientific fixture remains prescribed functions, not physically validated reactor dynamics. The app must retain the provenance and limitations disclosures.
