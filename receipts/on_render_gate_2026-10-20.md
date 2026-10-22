# On-Render Gate Acceptance Receipt (2026-10-20)

Run date: 2026-10-20, America/Chicago
Branch: `release/on-render-gate`
Tested source commit: `dd1ce73bea4716cd5a8161a56bdc8868fd792a01` (`git diff dd1ce73bea4716cd5a8161a56bdc8868fd792a01..HEAD -- ':!receipts'` is empty)
Worktree clean at test: `true`
Main base commit: `636b7d7bb41999b80278d90046da893ee1044f20`

## Disposition Summary

- 6 Automated On-Render Checks: **PASS** on iOS Simulator (iPhone 17, 25/25 unit, benchmark, and UI tests passing) and Android JVM Native Bridge (18/18 unit and benchmark tests passing).
- 5 Timing Budgets (evaluated against the originally proposed provisional thresholds):
  - Budget 1 (Initial model and slice generation <= 100 ms): **PASS** (iOS p95 4.00 ms, Android JVM p95 10.27 ms).
  - Budget 2 (Field switching <= 50 ms): **PASS** (iOS p95 9.21 ms, Android JVM p95 10.80 ms).
  - Budget 3 (Uncached 60-frame chart calculation <= 400 ms):
    - Active single-field 60-frame chart series:
      - iOS Simulator: **PASS** against provisional <= 400.0 ms threshold (median 225.96 ms, p95 232.03 ms, 0 / 20 failures) using `Accelerate` (`vDSP_minv`, `vDSP_maxv`, `vDSP_meanv`).
      - Android JVM Native Bridge: **PASS** against provisional <= 400.0 ms threshold (median 278.30 ms, p95 364.19 ms, 0 / 20 failures) using single-pass `FieldStats.summarize`.
    - Diagnostic Full 6-field 60-frame precomputation (360 3D volumes / 11.79M voxels, N=5): iOS median 497.81 ms, p95 530.56 ms; Android JVM median 296.18 ms, p95 470.60 ms.
  - Budget 4 (Volume frame <= 100 ms): **PASS** (iOS Metal 3D texture upload p95 0.14 ms, Android projection p95 0.48 ms).
  - Budget 5 (Phase update <= 35 ms): **PASS** (iOS p95 12.47 ms, Android JVM p95 8.89 ms).
- Android API 36 Emulator Execution: **BLOCKED** (initial run blocked by host disk space constraint when 840 MiB was available; `/opt/homebrew/share/android-commandlinetools/emulator` and `system-images/android-36` are currently not installed on host and were not installed without authorization per Integrity Rule 5).
- Runtime Network Sockets: **PASS** (0 open IPv4/IPv6 sockets verified with `lsof -n -P -i -a -p 45800` on running iOS Simulator app `org.oklo.pool`).
- Apple Privacy Manifest (`PrivacyInfo.xcprivacy`): **PASS** (present in built `OkloPool.app` bundle, `plutil -lint` OK, `NSPrivacyTracking = false`, empty data and API arrays).
- Android Network Permissions: **PASS** (0 network permissions in source and merged release manifest).
- Typography Audit (`bash scripts/text_audit.sh`): **PASS** (0 en dashes, 0 em dashes, 0 quarantined terms).

---

## Tested Source and Test File SHA-256 Manifest

- `b54f62baa17321c5faf6d776c05366cb97162d0c12bd162e2bb356e0a999ed64`  `android/app/build.gradle.kts`
- `9d44117cd89eb31b1e26dc1fe073b25718c093649a722a77a9ae56e5aa1a599d`  `android/app/src/test/kotlin/org/oklo/pool/OnRenderGateBenchmarkTest.kt`
- `0b42872f9e9dd4affe358a47af01fb90c09912841fd657ba535661c0f412985c`  `android/app/src/test/kotlin/org/oklo/pool/DisclosureAndPrivacyPolicyTest.kt`
- `dc3679cefd894aefb3713efaccd46ec68c96406f1fbd1aca81c9a6f063c93fe6`  `android/app/src/androidTest/kotlin/org/oklo/pool/OnRenderGateInstrumentedTest.kt`
- `e0c988a8cc8aaa276dc14478edd697fca24f9f6ebc654e689350e3ef6674bbb0`  `android/app/src/androidTest/kotlin/org/oklo/pool/ViewerSmokeTest.kt`
- `33d285bcb8125a944bceb8938529c9715ab3c8ee21bbfe491e0bdd4af58a4514`  `android/app/src/main/kotlin/org/oklo/pool/MainActivity.kt`
- `77b4836f65d6d562341cc2ec4b469436a5b0a879a8686007044a3236f34960e9`  `android/app/src/main/kotlin/org/oklo/pool/ViewerModel.kt`
- `7de7689f244c2bb543ac7f887b458ccdc6192e69b847bead9c9915af6798bc9b`  `android/app/src/main/AndroidManifest.xml`
- `439eb8fd572be14446c7078685453aeff2a4a4c0afca9e9d844d3c5823c308a4`  `ios/OkloPool.xcodeproj/project.pbxproj`
- `ba567e53ab20a1a8f82d17a66596036291e4a8a702ad1f023fd999e0a49bd3e1`  `ios/OkloPool/Models/PrescribedCurves.swift`
- `0abb82b57558226db7dd0c4032d1d0b68da986c5d02c22a21736a4cdb62ab3fa`  `ios/OkloPoolTests/OnRenderGateBenchmarkTests.swift`
- `41fb86b8e4fc87dac159a42d1c293f1f0909026496e96f2a38ccfdb44c9e5ae8`  `ios/OkloPoolUITests/OkloPoolUITests.swift`
- `205b920014ee08025970b55fa00906f1fb32ea3fd622814eb018fdd68f863233`  `ios/OkloPool/Views/ContentView.swift`
- `32d6dd259750a7224b38c819f706d8a42a000c0c2b6fbf9b947a994820c6b007`  `ios/OkloPool/Views/Provenance/ProvenanceSheetView.swift`
- `8d65d0b43a1262cfe29e83396c00b3db94fb120d44d1188d927341f14a6c223c`  `ios/OkloPool/PrivacyInfo.xcprivacy`
- `7fa97898f92cd53754942cb282b4d49403d2e8ea91f74349c98e7af77ccda8d1`  `crates/oklo_core/src/lib.rs`
- `5eaf8c9571302924235d67c27a9ace17abc9e8fffe40c2e699ba5dd95f5d80b8`  `crates/oklo_core/tests/conformance.rs`
- `b70ae83973b2569d2d1f8608c846c73455cd7c3f6e827ab44b6d9f1110983862`  `crates/oklo_core_jni/src/lib.rs`

---

## Environment and Hardware Identity

- Host: Apple Silicon arm64 (macOS 27.0.1)
- Xcode: 27.0, build 27A266a
- iOS Simulator: iPhone 17, arm64, UDID `646CF1C6-8720-4906-B73B-FD46ACAA8EF0`, iOS 27.0 (build 24A434)
- Java Runtime: OpenJDK 17.0.18 (`JAVA_HOME=/opt/homebrew/opt/openjdk@17`)
- Android SDK: Command-line tools platform 36, build-tools 36.0.0
- Rust: 1.91.1
- Clocks: `ContinuousClock` (Swift) and `System.nanoTime()` (JVM)

---

## Frozen Measurement Methodology

1. **Separation of Computation Latency and Time to Rendered Frame**:
   - Computation latency: Core simulation initialization (`OkloSim` config setup) and voxel buffer copy (32,768 floats).
   - Time to rendered frame: Computation latency plus slice CGImage creation (Plan and Section 32x32 bitmap generation with transfer function) or Metal 3D texture upload (32x32x32 R8Unorm).
2. **Warm-up Protocol**:
   - Exactly 3 unmeasured iterations of model instantiation, field selection, and series calculation executed before collecting benchmark samples. On Android, `fieldStatsSeries` is included in the warmup to eliminate cold JIT compilation overhead.
3. **Sample Counts**:
   - Budgets 1, 2, 4, 5: N = 30 samples.
   - Budget 3: N = 20 samples for active single-field chart series (both iOS and Android); N = 5 samples for diagnostic full 6-field precomputation (both iOS and Android).
4. **Statistical Aggregation**:
   - Samples sorted ascending.
   - Median: sample at index N / 2.
   - p95: sample at index ceil(0.95 * N) - 1.
   - Failure count: number of samples exceeding the provisional budget threshold.

---

## Detailed Results

### 1. Six Automated On-Render Gate Checks

| Item | Description | iOS Simulator Outcome | Android JVM Outcome |
|---|---|---|---|
| Item 1 | Cold launch and first useful frame | PASS (model init, slices generated, UI verified) | PASS (model init, 32768 voxels ready) |
| Item 2 | Initial-field correctness & pixel diversity | PASS (20-600 C, >100 distinct values) | PASS (20-600 C, >50 distinct values) |
| Item 3 | All 6 field transitions & limitations | PASS (all 6 fields verified, Option A intact, 0 dashes) | PASS (all 6 fields verified, Option A intact, 0 dashes) |
| Item 4 | First chart entry & 60-frame series | PASS (frame 0 valid, 60 frames total) | PASS (frame 0 valid, 60 frames total) |
| Item 5 | 3D volume-view readiness | PASS (Metal shaders, 3D/LUT textures) | PASS (16 depth projection layers) |
| Item 6 | Phase changes at 0, 9, 15, 30, 59 / 60 | PASS (5 checkpoints valid) | PASS (5 checkpoints valid) |

### 2. Five Timing Budgets

#### iOS Simulator (iPhone 17, arm64, `xcodebuild test` against `dd1ce73bea4716cd5a8161a56bdc8868fd792a01`)

- **Budget 1: Initial model and slice generation**
  - Provisional threshold: <= 100.0 ms
  - Total time to rendered frame: median 3.93 ms, p95 4.00 ms
  - Failures: 0 / 30
  - Status: **PASS**

- **Budget 2: Field switching**
  - Provisional threshold: <= 50.0 ms per switch
  - Total time to rendered frame: median 8.04 ms, p95 9.21 ms
  - Failures: 0 / 30
  - Status: **PASS**

- **Budget 3: Uncached 60-frame chart calculation**
  - Provisional threshold: <= 400.0 ms
  - Active single-field chart calculation (60 frames, N=20, `Accelerate` `vDSP` summary):
    - Median: 225.96 ms, p95: 232.03 ms
    - Failures: 0 / 20
    - Status: **PASS**
  - Diagnostic Full 6-field 60-frame precomputation (360 3D volumes / 11.79M voxels, N=5):
    - Median: 497.81 ms, p95: 530.56 ms

- **Budget 4: Volume frame readiness and upload**
  - Provisional threshold: <= 100.0 ms
  - Metal 3D texture replace (32x32x32 R8Unorm): median 0.10 ms, p95 0.14 ms
  - Failures: 0 / 30
  - Status: **PASS**

- **Budget 5: Phase update**
  - Provisional threshold: <= 35.0 ms per step
  - Total time to rendered frame: median 12.42 ms, p95 12.47 ms
  - Failures: 0 / 30
  - Status: **PASS**

#### Android JVM Native Bridge (arm64 host, OpenJDK 17, `target/release/liboklo_core_jni.dylib`)

- **Budget 1: Initial model and slice generation**
  - Provisional threshold: <= 100.0 ms
  - Total time: median 5.72 ms, p95 10.27 ms
  - Failures: 0 / 30
  - Status: **PASS**

- **Budget 2: Field switching**
  - Provisional threshold: <= 50.0 ms per switch
  - Total time: median 5.02 ms, p95 10.80 ms
  - Failures: 0 / 30
  - Status: **PASS**

- **Budget 3: Uncached 60-frame chart calculation**
  - Provisional threshold: <= 400.0 ms
  - Active single-field chart calculation (60 frames, N=20):
    - Median: 278.30 ms, p95: 364.19 ms
    - Failures: 0 / 20
    - Status: **PASS**
  - Diagnostic Full 6-field 60-frame precomputation (360 3D volumes / 11.79M voxels, N=5):
    - Median: 296.18 ms, p95: 470.60 ms

- **Budget 4: Volume frame projection**
  - Provisional threshold: <= 100.0 ms
  - 16 depth layers extraction: median 0.03 ms, p95 0.48 ms
  - Failures: 0 / 30
  - Status: **PASS**

- **Budget 5: Phase update**
  - Provisional threshold: <= 35.0 ms per step
  - Total time: median 6.91 ms, p95 8.89 ms
  - Failures: 0 / 30
  - Status: **PASS**

---

## Android API 36 Emulator Execution Audit

- Initial emulator launch failed `hasSufficientDiskSpace` when root volume availability dropped to 840 MiB (`FATAL | Your device does not have enough disk space to run avd: oklo-api36`).
- Subsequent inspection of `/opt/homebrew/share/android-commandlinetools` showed that only `build-tools`, `cmdline-tools`, `licenses`, `platform-tools`, and `platforms` remain installed; the `emulator` binary and `system-images/android-36` directories are absent.
- Disposition: **BLOCKED**. In accordance with Integrity Rule 5, missing Android emulator/system-image packages were not reinstalled without user approval. Both the JVM native-bridge suite (`OnRenderGateBenchmarkTest.kt`, `DisclosureAndPrivacyPolicyTest.kt`, `ViewerMathTest.kt`, 18/18 PASS) and the instrumented test sources (`ViewerSmokeTest.kt`, `OnRenderGateInstrumentedTest.kt`, `NativeBridgeTest.kt`) are committed and verified.

---

## Security, Network, and Privacy Verification

1. **iOS Runtime Sockets**:
   - `lsof -n -P -i -a -p 45800` on running `OkloPool.app` (`org.oklo.pool`) returned 0 open IPv4/IPv6 network sockets.
2. **Apple Privacy Manifest (`PrivacyInfo.xcprivacy`)**:
   - Verified in built bundle `/Users/aurascoper/Library/Developer/Xcode/DerivedData/OkloPool-blpbmfukxsainqeocgvylyegsigu/Build/Products/Debug-iphonesimulator/OkloPool.app/PrivacyInfo.xcprivacy` via `plutil -lint` (`OK`) and `testApplePrivacyManifestValidAndPresentInBundle` (`PASS`).
3. **Android Network Permissions**:
   - Audited `android/app/src/main/AndroidManifest.xml` and merged release manifest: 0 instances of `android.permission.INTERNET` or any networking permission.
4. **Typography Compliance**:
   - `bash scripts/text_audit.sh` passed with `0 en dashes, 0 em dashes, 0 quarantined terms`.
