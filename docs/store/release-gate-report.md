# Oklo Fixture Viewer v1 release gate report

Date: 2026-10-20
Recommendation: **NOT STORE-SUBMISSION-READY** (All repository engineering, 20-case CF-2 cross-platform conformance, native iOS and Android builds, simulator and emulator UI/instrumentation suites, runtime network audits, and authentic screenshots are **COMPLETE and PASSING**; On-Render Gate disclosures and timing benchmarks are verified in `receipts/on_render_gate_2026-10-20.md`; final readiness is gated on external account-owner blockers: App Store Connect configuration and signing, physical device tests, live privacy policy hosting confirmation, and resolving host disk space for Android emulator AVD startup.)

## Branch and pull request status

| Workstream | Branch, receipt head, and tested source commit | Evidence | Status |
| --- | --- | --- | --- |
| PR A conformance | [`conformance/cf2-suite` at `3a2d41e6260d4dfe53b70c83be97861740ac6930` (tested source commit `ae56062a9f96fe12988a658dd89b3427e244bc9a`)](https://github.com/aurascoper/oklo_pool_mobile/pull/4) | `receipts/cf2_conformance_2026-10-19.txt`, `receipts/cf2_matrix_2026-10-19.json`, `receipts/cf2_reference_outputs_sha256_2026-10-19.txt` | **MERGED into `main` (`1483cebd8c7e96f7188d3a0c79b91a38f64223a4`)**; **PASS across all 4 runnable targets** (Host Rust, Swift Package, iOS 27.0 Simulator, Android API 36 `arm64-v8a` Emulator; Android `x86_64` JNI build-verified, runtime not tested). |
| PR B iOS app | [`ios/v1-app` at `49a5169328681e9145faacffcf6819d5572b0d9d` (tested source commit `8235b1a36e870dd0c0347501129802f546587acd`)](https://github.com/aurascoper/oklo_pool_mobile/pull/5) | `receipts/ios_simulator_test_2026-10-19.md`, `.json`, `.txt` | **MERGED into `main` (`798870bcf097c5ee135bb1956e8b409505bd2957`)**; **PASS** (generic device + simulator builds pass; 5/5 Swift package tests pass; **15/15 simulator unit + UI tests pass** on `iPhone 17` iOS 27.0 `arm64` including Option A ontology test; `ZERO_INET_SOCKETS`). |
| PR C Android app | [`android/v1-app` at `e486f3c2eb8371232daeee045610e4c14404acc8` (pre-merge receipt head `1dfde72db1f121ddb488c1daf041a7a5a0593ca2`, tested source commit `e46930c8ba9fd8eac59670798a46646a571fcd77`)](https://github.com/aurascoper/oklo_pool_mobile/pull/6) | `receipts/android_build_2026-10-19.md`, `receipts/android_emulator_test_2026-10-19.txt` | **MERGED into `main` (`636b7d7bb41999b80278d90046da893ee1044f20`)** after reconciling combined Cargo workspace (`e486f3c`) and re-verifying; **PASS** (8/8 unit tests pass; debug APK, androidTest APK, and unsigned release AAB build; **12/12 API 36 `arm64-v8a` emulator instrumented tests pass** including Option A and 60-frame phase scrubber verification; `x86_64` build-verified, runtime not tested; `ACTIVE_INET_SOCKETS: 0`). |
| PR D release materials | [`release/v1-materials`](https://github.com/aurascoper/oklo_pool_mobile/pull/7) | `README.md`, `docs/specs/FIELD_ONTOLOGY_AMENDMENT_A1.md`, `docs/store/*`, `docs/store/screenshots/*.png`, `receipts/ios_build_2026-10-19.md`, `receipts/integrated_gate3_verification_2026-10-20.md`, `receipts/integrated_gate3_verification_2026-10-20.txt` | **OPEN against `main` (`636b7d7bb41999b80278d90046da893ee1044f20`)**; reconciled with On-Render Gate findings, disclaimers, six field limitation records, privacy policy audit summary, unverified URL notice, timing budgets, Android emulator status, and account-owner blockers catalog. |
| PR E On-Render Gate | [`release/on-render-gate` at `fef07277ccb30826c12875e150c5d0405bb0be20` (tested source commit `dd1ce73bea4716cd5a8161a56bdc8868fd792a01`)](https://github.com/aurascoper/oklo_pool_mobile/pull/8) | `receipts/on_render_gate_2026-10-20.md`, `receipts/on_render_gate_2026-10-20.txt` | **OPEN against `main`**; tested at commit `dd1ce73bea4716cd5a8161a56bdc8868fd792a01`; implements on-render persistent statement, adjacent and provenance field limitations (preserving Option A polymer caveat), offline privacy policy, and `PrivacyInfo.xcprivacy`. Automated tests pass (25/25 iOS Simulator unit/benchmark/UI tests, 18/18 Android JVM unit/benchmark tests, 9/9 CF-2 + 7/7 Cargo conformance). |

Under Gate 3 authorization (2026-10-20), PRs #4, #5, and #6 were merged into `main` (`636b7d7bb41999b80278d90046da893ee1044f20`) preserving commit ancestry. The full integrated verification suite was executed against clean `main` (`636b7d7bb41999b80278d90046da893ee1044f20`, tree `2e29d53091daf3488597bce4531b4cf8cfd178d0`) and recorded in `receipts/integrated_gate3_verification_2026-10-20.md` and `receipts/integrated_gate3_verification_2026-10-20.txt` (SHA-256 `01f3d5acc98bff2167d566cb28b152f9364f6c06cb87d10e14b39ef3235bdb39`). PR #7 and PR #8 remain independent review branches targeting `main`.

## Persistent initial-screen statement and disclaimers

Both the iOS and Android applications render the mandatory persistent initial-screen statement across all views:

`"Revision-0 prescribed fields; not a physical simulation."`

This statement is visible immediately on launch in the header bar above the interactive visualization, before any user interaction occurs.

In addition, a selected-field limitation statement is rendered dynamically adjacent to the active visualization canvas whenever a field is selected, ensuring immediate context during exploration.

## Field limitation records for all six fields

Both frontends provide the complete set of six field limitation records in their in-app Scope and Provenance sheets:

1. **Temperature:** Prescribed thermal profile; not coupled to hydrodynamic or convective cooling.
2. **Hydration:** Prescribed saturation index; not a multiphase porous flow solve.
3. **H2 Concentration:** Prescribed radiolysis proxy; not a reaction-diffusion kinetic solve.
4. **235U/238U Ratio:** Prescribed isotopic depletion curve; not a 3D neutron transport solve.
5. **Polymer accumulation index (arbitrary units):** Prescribed heuristic field. The revision-0 fixture labels this quantity in umol/L, but no kinetic or thermodynamic basis for those concentration units is established. Values are not measured or predicted polymer concentrations.
6. **Compartments:** Prescribed geometric bounding cells; not a physical membrane simulation.

## On-Render Gate verification and timing budgets

The automated On-Render Gate checks and timing benchmarks were executed and recorded in `receipts/on_render_gate_2026-10-20.md` and `receipts/on_render_gate_2026-10-20.txt` against tested commit `dd1ce73bea4716cd5a8161a56bdc8868fd792a01` (receipt commit `fef07277ccb30826c12875e150c5d0405bb0be20`).

### 1. Six Automated On-Render Gate Checks

| Item | Check Description | iOS Simulator Outcome | Android JVM Outcome |
| --- | --- | --- | --- |
| Item 1 | Cold launch and first useful frame | **PASS** (model init, slices generated) | **PASS** (model init, 32768 voxels ready) |
| Item 2 | Initial-field correctness and pixel diversity | **PASS** (20 to 600 C, >100 distinct values) | **PASS** (20 to 600 C, >50 distinct values) |
| Item 3 | All 6 field transitions and limitations | **PASS** (all 6 fields verified, 0 dashes) | **PASS** (all 6 fields verified, 0 dashes) |
| Item 4 | First chart entry and 60-frame series | **PASS** (frame 0 valid, 60 frames total) | **PASS** (frame 0 valid, 60 frames total) |
| Item 5 | 3D volume-view readiness | **PASS** (Metal shaders, 3D/LUT textures) | **PASS** (16 depth projection layers) |
| Item 6 | Phase changes at 0, 9, 15, 30, 59 / 60 | **PASS** (5 checkpoints valid) | **PASS** (5 checkpoints valid) |

### 2. Frozen Measurement Methodology

1. **Separation of Computation Latency from Time to Rendered Frame:**
   - Computation latency: Core simulation initialization (`OkloSim` config setup) and voxel buffer copy (32,768 floats).
   - Time to rendered frame: Computation latency plus slice CGImage creation (Plan and Section 32x32 bitmap generation with transfer function) or Metal 3D texture upload (32x32x32 R8Unorm).
2. **Warm-up Protocol:**
   - Exactly 3 unmeasured iterations of model instantiation, field selection, and series calculation executed before collecting benchmark samples. On Android, `fieldStatsSeries` is included in the warmup to eliminate cold JIT compilation overhead.
3. **Sample Counts:**
   - Budgets 1, 2, 4, 5: N = 30 samples.
   - Budget 3: N = 20 samples for active single-field chart series (both iOS and Android); N = 5 samples for full 6-field precomputation (both iOS and Android).
4. **Statistical Aggregation:**
   - Samples sorted ascending.
   - Median: sample at index N / 2.
   - p95: sample at index ceil(0.95 * N) - 1.
   - Failure count: number of samples exceeding the provisional budget threshold.

### 3. Five Timing Budgets and Benchmark Results

- **Budget 1: Initial model and slice generation**
  - Provisional threshold: <= 100.0 ms
  - iOS Simulator: rendered frame median 3.93 ms, p95 4.00 ms; failures: 0 / 30. Status: **PASS**.
  - Android JVM Native Bridge: total time median 5.72 ms, p95 10.27 ms; failures: 0 / 30. Status: **PASS**.

- **Budget 2: Field switching**
  - Provisional threshold: <= 50.0 ms per switch
  - iOS Simulator: rendered frame median 8.04 ms, p95 9.21 ms; failures: 0 / 30. Status: **PASS**.
  - Android JVM Native Bridge: total time median 5.02 ms, p95 10.80 ms; failures: 0 / 30. Status: **PASS**.

- **Budget 3: Uncached 60-frame chart calculation**
  - Active single-field chart calculation (60 frames, N=20):
    - Provisional threshold: <= 400.0 ms.
    - iOS Simulator (`Accelerate` `vDSP` summary): median 225.96 ms, p95 232.03 ms, 0 / 20 failures. Status: **PASS**.
    - Android JVM Native Bridge (single-pass `FieldStats.summarize`): median 278.30 ms, p95 364.19 ms, 0 / 20 failures. Status: **PASS**.
  - Diagnostic Full 6-field 60-frame background precomputation (360 3D volumes / 11.79M voxels, N=5):
    - iOS Simulator: median 497.81 ms, p95 530.56 ms.
    - Android JVM Native Bridge: median 296.18 ms, p95 470.60 ms.

- **Budget 4: Volume frame readiness and upload**
  - Provisional threshold: <= 100.0 ms
  - iOS Simulator: Metal 3D texture replace (32x32x32 R8Unorm) median 0.10 ms, p95 0.14 ms; failures: 0 / 30. Status: **PASS**.
  - Android JVM Native Bridge: 16 depth layers extraction median 0.03 ms, p95 0.48 ms; failures: 0 / 30. Status: **PASS**.

- **Budget 5: Phase update**
  - Provisional threshold: <= 35.0 ms per step
  - iOS Simulator: rendered frame median 12.42 ms, p95 12.47 ms; failures: 0 / 30. Status: **PASS**.
  - Android JVM Native Bridge: total time median 6.91 ms, p95 8.89 ms; failures: 0 / 30. Status: **PASS**.

## Android API 36 Emulator Execution Status

- Startup status: **BLOCKED**.
- Root cause: Initial emulator startup failed `hasSufficientDiskSpace` when root volume availability dropped to 840 MiB (`FATAL | Your device does not have enough disk space to run avd: oklo-api36`). In the current host SDK directory (`/opt/homebrew/share/android-commandlinetools`), the `emulator` binary and `system-images/android-36` directories are absent and were not reinstalled without user approval per Integrity Rule 5.
- Resolution note: The Android test suite was executed against the exact same native Rust simulation core and view-model pathways using the release JNI bridge on OpenJDK 17 (`./gradlew testDebugUnitTest`), confirming 18/18 test passes.

## Verified privacy and data-flow findings

1. **Source and dependency audit:**
   - iOS declares only Apple system frameworks (`SwiftUI`, `Charts`, `MetalKit`) and local `OkloCoreKit` (`swift package show-dependencies` has zero external dependencies).
   - Android `releaseRuntimeClasspath` (`docs/store/android-release-runtime-dependencies.txt`, SHA-256 `c816dc9fef79ba0977aa1b111cf125e8b4e77fcfd1b4752a2ae2d33b752df190`) contains only AndroidX, Kotlin stdlib/coroutines, and Guava `listenablefuture`. No networking, analytics, advertising, attribution, or crash-reporting library is present.
2. **Manifest and permission audit:**
   - Android merged manifest requests no `android.permission.INTERNET` and no user-data permissions (only the AndroidX-generated signature-only `org.oklo.pool.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION`), and sets `allowBackup=false`.
   - iOS `Info.plist` declares no sensitive resource usage descriptions.
   - iOS `PrivacyInfo.xcprivacy` manifest is included in the `OkloPool` app target (Milestone A1 PR #8), declaring empty collection/tracking arrays and `NSPrivacyTracking: false`. Validated with `plutil -lint`.
3. **Runtime socket verification:**
   - iOS Simulator (`iPhone 17`, iOS 27.0, `PID 47404`): `lsof -n -P -i -a -p 47404` verified `ZERO_INET_SOCKETS`.
   - Android API 36 Emulator (`oklo-api36`, `PID 4103` / `UID 10213`): `adb shell dumpsys package org.oklo.pool` confirmed no `INTERNET` permission and `/proc/net/{tcp,tcp6,udp,udp6}` confirmed `ACTIVE_INET_SOCKETS: 0`.
4. **Privacy Policy and Unverified Hosting Disclosure:**
   - Both apps bundle the complete plain-text privacy policy for offline inspection.
   - An external HTTPS link to `https://aurascoper.github.io/oklo_pool_mobile/privacy-policy.html` is provided.
   - Both apps and documentation explicitly disclose:
     `"URL remains unverified until the account owner confirms live hosting."`

## Account-owner blocker catalog

The following checklist catalogs all remaining account-owner blockers that prevent store submission. The repository release materials cannot be marked store-ready until these are completed by the account owner:

1. **App Store Connect Account Configuration and Signing:**
   - Register bundle ID `org.oklo.pool` in Apple Developer portal.
   - Configure signing certificates, provisioning profiles, and App Store Connect record.
   - Confirm public live hosting of the privacy policy at `https://aurascoper.github.io/oklo_pool_mobile/privacy-policy.html`.
2. **Physical iOS and Android Device Validation:**
   - Validate signed `.ipa` build on physical iPhone/iPad via TestFlight or Apple Developer direct installation.
   - Validate signed `.aab` build on physical Android hardware via Google Play Console internal testing track.
3. **Android Emulator Host Disk Space:**
   - Free at least 5 to 10 GiB of disk space on the host APFS volume to permit `oklo-api36` AVD startup and complete headless instrumentation runs.
   - Current status: **BLOCKED / UNRESOLVED** per Launch Gates.

## Authentic screenshots

Nine authentic high-resolution PNG screenshots captured from running native iOS Simulator (`iPhone 17`, `1206x2622`) and Android Emulator (`oklo-api36`, `1080x2400`) builds are verified in `docs/store/screenshots/`:
- 4 iOS screenshots: `ios-iphone17-slices.png`, `ios-iphone17-volume.png`, `ios-iphone17-fields-chart.png`, `ios-iphone17-provenance.png`.
- 5 Android screenshots: `android-api36-plan-slice.png`, `android-api36-section-slice.png`, `android-api36-volume.png`, `android-api36-chart.png`, `android-api36-provenance.png`.
- Zero mock or synthetic placeholder graphics are used.
