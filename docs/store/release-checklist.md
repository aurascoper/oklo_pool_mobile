# v1 release checklist

## Code, builds, and automated simulator/emulator verification

- [x] PR A (`conformance/cf2-suite`, [PR #4](https://github.com/aurascoper/oklo_pool_mobile/pull/4) at `3a2d41e6260d4dfe53b70c83be97861740ac6930`, tested source commit `ae56062a9f96fe12988a658dd89b3427e244bc9a`): 20-case CF-2 conformance suite passes across all 4 runnable targets (Host Rust `aarch64`, iOS `OkloCoreKit` macOS slice, iOS Simulator `iPhone 17` iOS 27.0 `arm64`, and Android Emulator `oklo-api36` API 36 `arm64-v8a`; Android `x86_64` JNI is build-verified and 16 KB page-aligned, runtime not tested on Apple Silicon).
- [x] PR B (`ios/v1-app`, [PR #5](https://github.com/aurascoper/oklo_pool_mobile/pull/5) at `49a5169328681e9145faacffcf6819d5572b0d9d`, tested source commit `8235b1a36e870dd0c0347501129802f546587acd`): Generic iOS device and simulator builds pass; Swift package tests pass (5/5); full `xcodebuild test` suite on `iPhone 17` (iOS 27.0 `arm64`) passes (**15/15 tests**: 11 unit tests including 20-case CF-2 verification and Option A ontology test + 4 UI tests).
- [x] PR C (`android/v1-app`, [PR #6](https://github.com/aurascoper/oklo_pool_mobile/pull/6) at `1dfde72db1f121ddb488c1daf041a7a5a0593ca2`, tested source commit `e46930c8ba9fd8eac59670798a46646a571fcd77`): Unit tests pass (**8/8**); debug APK, instrumentation APK, and unsigned release AAB build (`compileSdk=36`, `targetSdk=36`, 16 KB ELF load segment alignment on `arm64-v8a` and `x86_64`); full instrumentation suite on `oklo-api36` (Android 16 / API 36 `arm64-v8a`) passes (**12/12 tests**: 6 JNI bridge tests including 20-case CF-2 verification + 6 Compose UI smoke tests verifying Option A and the 60-frame phase scrubber); `x86_64` ABI is build-verified, runtime not tested.
- [x] Runtime network and permission checks completed on both booted platforms (iOS Simulator `PID 47404` `lsof`: `ZERO_INET_SOCKETS`; Android API 36 Emulator `PID 4103` / `UID 10213` `dumpsys package` + `/proc/net`: no `INTERNET` permission, `ACTIVE_INET_SOCKETS: 0`).
- [x] Accessibility, lifecycle (pause on background), legibility, and authentic screenshot capture completed on both platforms (4 iOS `1206x2622` PNGs and 5 Android `1080x2400` PNGs stored in `docs/store/screenshots/`).
- [ ] Physical-device smoke tests on a provisioned iPhone/iPad and physical Android handset/tablet.
- [ ] Android `x86_64` emulator runtime check on an `x86_64` host (build and 16 KB ELF alignment are verified; `x86_64` guest execution is blocked on the Apple Silicon `arm64` host).

## Signing and store submission (Account-owner tasks)

- [ ] Release Android AAB (`app-release.aab`) signed with account-owner upload key and verified with `jarsigner`/`apksigner`.
- [ ] iOS release archive (`.ipa`) signed with account-owner Apple Developer distribution certificate and provisioning profile.
- [ ] Final dependency and SDK manifest (`PrivacyInfo.xcprivacy` if required by App Store Connect upload check) confirmed on signed archives.
- [ ] Privacy policy (`docs/store/privacy-policy.md` / `docs/store/privacy-policy.html`) populated with publisher name, effective date, and monitored support email, and published at a public HTTPS URL.
- [ ] Google Play Data safety form completed in Play Console using `docs/store/google-play-data-safety-draft.md`.
- [ ] Apple App Privacy disclosures completed in App Store Connect using `docs/store/apple-app-privacy-draft.md`.
- [ ] Store age-rating questionnaires, store listings, support URL, app icon, and screenshots uploaded in App Store Connect and Google Play Console.
- [ ] Developer-program agreements accepted and final store submission triggered by the account owner.

No store listing has been created or submitted. Do not mark the release `STORE-SUBMISSION-READY` until the physical-device smoke tests, release signing, public HTTPS privacy-policy hosting, and store console entries above are completed.
