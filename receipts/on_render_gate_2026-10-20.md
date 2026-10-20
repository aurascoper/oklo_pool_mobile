# On-Render Gate Acceptance Receipt (2026-10-20)

Run date: 2026-10-20, America/Chicago
Branch: `release/on-render-gate`
Tested source commit: `c6a516a7ac483e8bf63b0897a800af806af7051c` (`git diff c6a516a7ac483e8bf63b0897a800af806af7051c..HEAD -- ':!receipts'` is empty)
Worktree clean at test: `true`
Main base commit: `636b7d7bb41999b80278d90046da893ee1044f20`

## Disposition Summary

- 6 Automated Simulator Checks: **PASS** on iOS Simulator (iPhone 17) and Android JVM Native Bridge.
- 5 Provisional Timing Budgets:
  - Budget 1 (Initial model and slice generation <= 100 ms): **PASS** (iOS p95 5.65 ms, Android p95 2.83 ms).
  - Budget 2 (Field switching <= 50 ms): **PASS** (iOS p95 12.84 ms, Android p95 2.80 ms).
  - Budget 3 (Uncached 60-frame chart calculation <= 400 ms):
    - Active single-field 60-frame chart series: **PASS** (iOS p95 318.09 ms, Android p95 182.30 ms).
    - Full 6-field 60-frame precomputation (360 3D volumes): **UNRESOLVED / EXCEEDED** against provisional 400 ms budget (iOS median 1086.51 ms, p95 1101.96 ms). Threshold change to 1200 ms proposed for full 6-field precomputation.
  - Budget 4 (Volume frame <= 100 ms): **PASS** (iOS Metal upload p95 0.10 ms, Android projection p95 0.26 ms).
  - Budget 5 (Phase update <= 35 ms): **PASS** (iOS p95 9.53 ms, Android p95 2.81 ms).
- Android API 36 Emulator Execution: **BLOCKED** due to host disk space constraint (840 MiB available; emulator requires gigabytes to allocate RAM and system overlay).
- Runtime Network Sockets: **PASS** (0 open IPv4/IPv6 sockets verified with lsof).
- Android Network Permissions: **PASS** (0 network permissions in merged release manifest).
- Typography Audit: **PASS** (0 em-dashes and 0 en-dashes across all modified and new files).

---

## Tested Source and Test File SHA-256 Manifest

- `b54f62baa17321c5faf6d776c05366cb97162d0c12bd162e2bb356e0a999ed64`  `android/app/build.gradle.kts`
- `423e8556a691f280446ca06602020b0468459e4dff543fb1b0852f14201a99b3`  `android/app/src/test/kotlin/org/oklo/pool/OnRenderGateBenchmarkTest.kt`
- `e365025c797f9b50995d897cebb2a29662a36744617d112e09d51aa8f26626a2`  `android/app/src/main/kotlin/org/oklo/pool/MainActivity.kt`
- `7de7689f244c2bb543ac7f887b458ccdc6192e69b847bead9c9915af6798bc9b`  `android/app/src/main/AndroidManifest.xml`
- `439eb8fd572be14446c7078685453aeff2a4a4c0afca9e9d844d3c5823c308a4`  `ios/OkloPool.xcodeproj/project.pbxproj`
- `584d58c09fd5dda2d17bf13426e0b67ad2342da2ff20ef34e67efadaf00798b1`  `ios/OkloPool/Models/PrescribedCurves.swift`
- `b9e3ead1472de66a912eb329361bbef08aef0c055b1a5efb48066fd1b4d733c9`  `ios/OkloPoolTests/OnRenderGateBenchmarkTests.swift`
- `205b920014ee08025970b55fa00906f1fb32ea3fd622814eb018fdd68f863233`  `ios/OkloPool/Views/ContentView.swift`
- `ceb7dcaab2fe3b743e012076b550e09a1ddfffa3d0c2fedad1a62bde2cc2847e`  `ios/OkloPool/Views/Provenance/ProvenanceSheetView.swift`
- `8d65d0b43a1262cfe29e83396c00b3db94fb120d44d1188d927341f14a6c223c`  `ios/OkloPool/PrivacyInfo.xcprivacy`
- `7fa97898f92cd53754942cb282b4d49403d2e8ea91f74349c98e7af77ccda8d1`  `crates/oklo_core/src/lib.rs`
- `5eaf8c9571302924235d67c27a9ace17abc9e8fffe40c2e699ba5dd95f5d80b8`  `crates/oklo_core/tests/conformance.rs`
- `b70ae83973b2569d2d1f8608c846c73455cd7c3f6e827ab44b6d9f1110983862`  `crates/oklo_core_jni/src/lib.rs`

---

## Environment and Hardware Identity

- Host: Apple Silicon arm64 (macOS 27.0.1)
- Xcode: 27.0, build 27A266a
- iOS Simulator: iPhone 17, arm64, UDID `646CF1C6-8720-4906-B73B-FD46ACAA8EF0`, iOS 27.0 (build 24A434)
- Java Runtime: OpenJDK 17.0.20.1 (Homebrew)
- Android SDK: Command-line tools platform 36, build-tools 36.0.0
- Rust: 1.85.0
- Clocks: ContinuousClock (Swift) and System.nanoTime() (JVM)

---

## Frozen Measurement Methodology

1. **Separation of Computation Latency and Time to Rendered Frame**:
   - Computation latency: Core simulation initialization (`OkloSim` config setup) and voxel buffer copy (32,768 floats).
   - Time to rendered frame: Computation latency plus slice CGImage creation (Plan and Section 32x32 bitmap generation with transfer function) or Metal 3D texture upload (32x32x32 R8Unorm).
2. **Warm-up Protocol**:
   - Exactly 3 unmeasured iterations of model instantiation, field selection, and series calculation executed before collecting benchmark samples.
3. **Sample Counts**:
   - Budgets 1, 2, 4, 5: N = 30 samples.
   - Budget 3: N = 20 samples for active single-field chart series; N = 5 samples for full 6-field precomputation.
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
| Item 1 | Cold launch and first useful frame | PASS (model init, slices generated) | PASS (model init, 32768 voxels ready) |
| Item 2 | Initial-field correctness & pixel diversity | PASS (20-600 C, >100 distinct values) | PASS (20-600 C, >50 distinct values) |
| Item 3 | All 6 field transitions & limitations | PASS (all 6 fields verified, 0 dashes) | PASS (all 6 fields verified, 0 dashes) |
| Item 4 | First chart entry & 60-frame series | PASS (frame 0 valid, 60 frames total) | PASS (frame 0 valid, 60 frames total) |
| Item 5 | 3D volume-view readiness | PASS (Metal shaders, 3D/LUT textures) | PASS (16 depth projection layers) |
| Item 6 | Phase changes at 0, 9, 15, 30, 59 / 60 | PASS (5 checkpoints valid) | PASS (5 checkpoints valid) |

### 2. Five Provisional Timing Budgets

#### iOS Simulator (iPhone 17, arm64)

- **Budget 1: Initial model and slice generation**
  - Provisional threshold: <= 100.0 ms
  - Computation latency (core sim + copy): median 0.42 ms, p95 0.55 ms
  - Total time to rendered frame: median 5.48 ms, p95 5.65 ms
  - Failures: 0 / 30
  - Status: **PASS**

- **Budget 2: Field switching**
  - Provisional threshold: <= 50.0 ms per switch
  - Computation latency: median 0.38 ms, p95 0.49 ms
  - Total time to rendered frame: median 11.31 ms, p95 12.84 ms
  - Failures: 0 / 30
  - Status: **PASS**

- **Budget 3: Uncached 60-frame chart calculation**
  - Provisional threshold: <= 400.0 ms
  - Active single-field chart calculation (60 frames): median 311.56 ms, p95 318.09 ms
  - Failures: 0 / 20
  - Status: **PASS**
  - Diagnostic: Full 6-field 60-frame precomputation (360 3D volumes): median 1086.51 ms, p95 1101.96 ms. Exceeds provisional 400 ms budget when evaluating all six fields sequentially.

- **Budget 4: Volume frame readiness and upload**
  - Provisional threshold: <= 100.0 ms
  - Metal 3D texture replace (32x32x32 R8Unorm): median 0.09 ms, p95 0.10 ms
  - Failures: 0 / 30
  - Status: **PASS**

- **Budget 5: Phase update**
  - Provisional threshold: <= 35.0 ms per step
  - Computation latency: median 0.41 ms, p95 0.52 ms
  - Total time to rendered frame: median 9.22 ms, p95 9.53 ms
  - Failures: 0 / 30
  - Status: **PASS**

#### Android JVM Native Bridge (arm64 host, target/release)

- **Budget 1: Initial model and slice generation**
  - Provisional threshold: <= 100.0 ms
  - Total time: median 2.81 ms, p95 2.83 ms
  - Failures: 0 / 30
  - Status: **PASS**

- **Budget 2: Field switching**
  - Provisional threshold: <= 50.0 ms per switch
  - Total time: median 2.77 ms, p95 2.80 ms
  - Failures: 0 / 30
  - Status: **PASS**

- **Budget 3: Uncached 60-frame chart calculation**
  - Provisional threshold: <= 400.0 ms
  - Single-field chart calculation (60 frames): median 172.20 ms, p95 182.30 ms
  - Failures: 0 / 10
  - Status: **PASS**

- **Budget 4: Volume frame projection**
  - Provisional threshold: <= 100.0 ms
  - 16 depth layers extraction: median 0.06 ms, p95 0.26 ms
  - Failures: 0 / 30
  - Status: **PASS**

- **Budget 5: Phase update**
  - Provisional threshold: <= 35.0 ms per step
  - Total time: median 2.77 ms, p95 2.81 ms
  - Failures: 0 / 30
  - Status: **PASS**

---

## Android API 36 Emulator Execution Audit

- Command attempted:
  ```sh
  export ANDROID_HOME=/opt/homebrew/share/android-commandlinetools
  export PATH=$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH
  emulator -avd oklo-api36 -no-window -no-audio -no-boot-anim
  ```
- Result:
  ```
  INFO    | Android emulator version 37.2.12.0 (build_id 16428233)
  INFO    | Checking system compatibility:
  INFO    |   Checking: hasSufficientDiskSpace
  INFO    |      Error: Your device does not have enough disk space to run avd: `oklo-api36`
  FATAL   | Your device does not have enough disk space to run avd: `oklo-api36`.
  ```
- Host disk availability at time of test: 840 MiB on root APFS volume.
- Disposition: **BLOCKED / UNRESOLVED**. In accordance with team integrity rules, disk space was not aggressively recovered by deleting host or user files. Android JVM native tests were executed instead against release JNI libraries, exercising the exact same simulation core and view-model pathways.

---

## Security, Network, and Privacy Verification

1. **iOS Runtime Sockets**:
   - `lsof -n -P -i -a -p 76862` returned exit code 1 (0 open IPv4/IPv6 network sockets).
2. **Android Network Permissions**:
   - Released merged manifest audited at `android/app/build/intermediates/merged_manifests/release/processReleaseManifest/AndroidManifest.xml`.
   - Result: 0 instances of `android.permission.INTERNET` or other networking permissions. Only `DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION` is present.
3. **Typography Compliance**:
   - Automated grep audit across all modified and new files for en-dash (U+2013) and em-dash (U+2014) returned 0 matches.

---

## Threshold Change Proposal

- Target: Uncached 60-frame chart calculation.
- Observed behavior:
  - Active single-field 60-frame evaluation: ~170 ms (Android) to ~315 ms (iOS), easily passing <= 400 ms.
  - Full 6-field 60-frame precomputation (360 3D field volumes): ~1086 ms on Apple Silicon arm64.
- Proposal:
  - Retain <= 400 ms for active single-field 60-frame chart generation.
  - Establish a separate threshold of <= 1200 ms for complete 6-field full-series precomputation (which runs asynchronously in the background).
