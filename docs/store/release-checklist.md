# v1 release checklist

## Code, builds, and automated simulator/emulator verification

- [x] PR A (`conformance/cf2-suite`, [PR #4](https://github.com/aurascoper/oklo_pool_mobile/pull/4) at `3a2d41e6260d4dfe53b70c83be97861740ac6930`, tested source commit `ae56062a9f96fe12988a658dd89b3427e244bc9a`): 20-case CF-2 conformance suite passes across all 4 runnable targets (Host Rust `aarch64`, iOS `OkloCoreKit` macOS slice, iOS Simulator `iPhone 17` iOS 27.0 `arm64`, and Android Emulator `oklo-api36` API 36 `arm64-v8a`; Android `x86_64` JNI is build-verified and 16 KB page-aligned, runtime not tested on Apple Silicon).
- [x] PR B (`ios/v1-app`, [PR #5](https://github.com/aurascoper/oklo_pool_mobile/pull/5) at `49a5169328681e9145faacffcf6819d5572b0d9d`, tested source commit `8235b1a36e870dd0c0347501129802f546587acd`): Generic iOS device and simulator builds pass; Swift package tests pass (5/5); full `xcodebuild test` suite on `iPhone 17` (iOS 27.0 `arm64`) passes (**15/15 tests**: 11 unit tests including 20-case CF-2 verification and Option A ontology test + 4 UI tests).
- [x] PR C (`android/v1-app`, [PR #6](https://github.com/aurascoper/oklo_pool_mobile/pull/6) at `1dfde72db1f121ddb488c1daf041a7a5a0593ca2`, tested source commit `e46930c8ba9fd8eac59670798a46646a571fcd77`): Unit tests pass (**8/8**); debug APK, instrumentation APK, and unsigned release AAB build (`compileSdk=36`, `targetSdk=36`, 16 KB ELF load segment alignment on `arm64-v8a` and `x86_64`); full instrumentation suite on `oklo-api36` (Android 16 / API 36 `arm64-v8a`) passes (**12/12 tests**: 6 JNI bridge tests including 20-case CF-2 verification + 6 Compose UI smoke tests verifying Option A and the 60-frame phase scrubber); `x86_64` ABI is build-verified, runtime not tested.
- [x] Gate 3 controlled integration into `main` (`636b7d7bb41999b80278d90046da893ee1044f20`, tree `2e29d53091daf3488597bce4531b4cf8cfd178d0`) and post-merge verification across Rust workspace/conformance suites, `OkloCoreKit`, iOS 27.0 Simulator (`15/15`), Android JVM (`8/8`) and API 36 Emulator (`12/12`), 16 KB ELF page alignment, and text audit (`receipts/integrated_gate3_verification_2026-10-20.md` and `receipts/integrated_gate3_verification_2026-10-20.txt`, SHA-256 `01f3d5acc98bff2167d566cb28b152f9364f6c06cb87d10e14b39ef3235bdb39`).
- [x] On-Render Gate implementation (PR #8, branch `release/on-render-gate`, tested source commit `dd1ce73bea4716cd5a8161a56bdc8868fd792a01`, receipt head `fef07277ccb30826c12875e150c5d0405bb0be20`):
  - Persistent initial-screen statement `"Revision-0 prescribed fields; not a physical simulation."` rendered on launch across views.
  - Dynamic selected-field limitation statement rendered adjacent to the active canvas.
  - All six field limitation records included in in-app Scope and Provenance sheets:
    1. Temperature: Prescribed thermal profile; not coupled to hydrodynamic or convective cooling.
    2. Hydration: Prescribed saturation index; not a multiphase porous flow solve.
    3. H2 Concentration: Prescribed radiolysis proxy; not a reaction-diffusion kinetic solve.
    4. 235U/238U Ratio: Prescribed isotopic depletion curve; not a 3D neutron transport solve.
    5. Polymer accumulation index (arbitrary units): Prescribed heuristic field. The revision-0 fixture labels this quantity in umol/L, but no kinetic or thermodynamic basis for those concentration units is established. Values are not measured or predicted polymer concentrations.
    6. Compartments: Prescribed geometric bounding cells; not a physical membrane simulation.
  - Bundled offline full-text privacy policy accessible without network connectivity.
  - Explicit notice that external URL `https://aurascoper.github.io/oklo_pool_mobile/privacy-policy.html` remains unverified until confirmed live by account owner.
  - Apple `PrivacyInfo.xcprivacy` manifest bundled in app target and verified with `plutil -lint`.
  - Zero network permissions in Android manifest (`android.permission.INTERNET` absent).
  - Runtime socket inspection confirmed zero open sockets (`ZERO_INET_SOCKETS`).
- [x] On-Render Gate timing budgets and benchmarks documented in `receipts/on_render_gate_2026-10-20.md`:
  - Budget 1 (Initial model and slice generation <= 100 ms): PASS (iOS p95 4.00 ms, Android JVM p95 10.27 ms).
  - Budget 2 (Field switching <= 50 ms): PASS (iOS p95 9.21 ms, Android JVM p95 10.80 ms).
  - Budget 3 (Active single-field 60-frame chart calculation <= 400 ms): PASS (iOS `Accelerate` `vDSP` median 225.96 ms, p95 232.03 ms, 0/20 fails; Android JVM median 278.30 ms, p95 364.19 ms, 0/20 fails; diagnostic full 6-field 60-frame precomputation across 360 volumes: iOS median 497.81 ms, p95 530.56 ms; Android JVM median 296.18 ms, p95 470.60 ms).
  - Budget 4 (Volume frame readiness <= 100 ms): PASS (iOS p95 0.14 ms, Android JVM p95 0.48 ms).
  - Budget 5 (Phase update <= 35 ms): PASS (iOS p95 12.47 ms, Android JVM p95 8.89 ms).
- [x] Authentic screenshot verification in `docs/store/screenshots/`:
  - 4 iOS Simulator screenshots (`iPhone 17`, `1206x2622` PNG) and 5 Android Emulator screenshots (`oklo-api36`, `1080x2400` PNG). Zero mock placeholders.

## Blocker: Android API 36 Emulator startup

- [ ] Android `oklo-api36` AVD startup: **BLOCKED / UNRESOLVED**. Host machine has <1 GiB free disk space (measured at 840 MiB on root APFS volume), which prevents AVD RAM and disk overlay allocation. Resolving this requires the account owner to free 5 to 10 GiB of disk space. Android JVM native tests against release JNI libraries verified 18/18 checks in the interim.

## Account-owner blocker catalog before store submission

The following items are external prerequisites that require account-owner credentials, provisioned physical hardware, or manual store console configuration:

- [ ] 1. **App Store Connect and Signing Configuration:**
  - Register bundle identifier `org.oklo.pool` in Apple Developer account.
  - Generate distribution signing certificate and provisioning profile.
  - Configure App Store Connect app record, store listing metadata, age rating questionnaire, and support URL.
  - Publish `docs/store/privacy-policy.html` at `https://aurascoper.github.io/oklo_pool_mobile/privacy-policy.html` and confirm live hosting.
- [ ] 2. **Physical Device Smoke Tests:**
  - Install and verify signed `.ipa` build on physical iPhone/iPad hardware via TestFlight or Apple Developer installation.
  - Install and verify signed `.aab` build on physical Android phone/tablet hardware via Google Play Console internal testing.
- [ ] 3. **Host Disk Space Remediation:**
  - Clear at least 5 to 10 GiB of host disk space to allow `oklo-api36` AVD startup and complete execution of instrumented emulator tests.
- [ ] 4. **Release Code Signing:**
  - Sign `app-release.aab` with the production Google Play upload keystore.
  - Sign `OkloPool.app` with the production Apple Developer distribution certificate.
- [ ] 5. **Store Console Submission:**
  - Enter prepared Apple App Privacy disclosures in App Store Connect.
  - Enter prepared Google Play Data safety responses in Google Play Console.
  - Upload authentic screenshots, app icons, and final descriptions.
  - Submit apps for store review under account-owner authority.

**Overall status:** **NOT STORE-SUBMISSION-READY** until all account-owner blockers and physical-device validations are completed.
