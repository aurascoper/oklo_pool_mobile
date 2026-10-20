# v1 release checklist

## Code and builds

- [x] PR A conformance branch exists and its review gate is recorded as PASS.
- [x] PR B iOS app branch exists and its review gate is recorded as PASS.
- [x] PR C Android app and JNI bridge branch committed at `088e668`.
- [x] Android unit tests pass; debug APK and release AAB build.
- [x] Generic iOS device and simulator builds pass after generating the local XCFramework.
- [x] iOS app, unit-test, and UI-test bundles compile for generic simulator.
- [x] Swift package tests pass: 5 tests.
- [ ] iOS simulator unit and UI tests execute on a booted runtime.
- [ ] Android instrumentation and ABI runtime tests execute on arm64 and x86_64.
- [ ] Physical-device smoke tests.
- [ ] Runtime network capture on both platforms.
- [ ] Accessibility, lifecycle, performance, and screenshot review on actual app screens.

## Signing and store preparation

- [ ] Release Android AAB signed with account-owner release key and signature verified.
- [ ] iOS archive signed with account-owner distribution identity and provisioning.
- [ ] Final dependency and SDK inventory reviewed from signed artifacts.
- [ ] Privacy policy completed, approved, and hosted at a public HTTPS URL.
- [ ] Google Play Data safety form completed by account owner.
- [ ] Apple App Privacy disclosures completed by account owner.
- [ ] Store ratings, listing, support details, icon, and actual app screenshots entered.
- [ ] Developer-program agreements reviewed and accepted by account owner.
- [ ] Store submission and publication approved by account owner.

No store listing has been created. No store form, developer agreement, or submission has been completed. Do not call this release STORE-SUBMISSION-READY until every blocking item is closed with evidence.
