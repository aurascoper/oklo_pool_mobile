# Gate 3 Integrated Verification Receipt (2026-10-20)

- **Status:** `PASS` across all integrated host, iOS 27.0 Simulator, and Android API 36 (`arm64-v8a`) Emulator suites; `NOT STORE-SUBMISSION-READY` pending physical-device smoke tests, production code signing, public HTTPS privacy-policy hosting, and account-owner store console submission.
- **Integrated `main` commit SHA:** `636b7d7bb41999b80278d90046da893ee1044f20`
- **Integrated Git tree SHA:** `2e29d53091daf3488597bce4531b4cf8cfd178d0`
- **Worktree clean at test:** `true`
- **Verbatim console log:** `receipts/integrated_gate3_verification_2026-10-20.txt` (SHA-256: `01f3d5acc98bff2167d566cb28b152f9364f6c06cb87d10e14b39ef3235bdb39`)

---

## 1. Gate 3 Controlled Merge Lineage on `main`

1. **Base `main` prior to Gate 3:** `041d1bab092553798dec6f7b84df1edfbe417ab4`
2. **[PR #4 (`conformance/cf2-suite`)](https://github.com/aurascoper/oklo_pool_mobile/pull/4):**
   - Branch head: `3a2d41e6260d4dfe53b70c83be97861740ac6930` (tested clean source commit `ae56062a9f96fe12988a658dd89b3427e244bc9a`)
   - Merge commit on `main`: `1483cebd8c7e96f7188d3a0c79b91a38f64223a4`
3. **[PR #5 (`ios/v1-app`)](https://github.com/aurascoper/oklo_pool_mobile/pull/5):**
   - Branch head: `49a5169328681e9145faacffcf6819d5572b0d9d` (tested clean source commit `8235b1a36e870dd0c0347501129802f546587acd`)
   - Merge commit on `main`: `798870bcf097c5ee135bb1956e8b409505bd2957`
4. **[PR #6 (`android/v1-app`)](https://github.com/aurascoper/oklo_pool_mobile/pull/6):**
   - Pre-reconciliation branch head: `1dfde72db1f121ddb488c1daf041a7a5a0593ca2` (tested clean source commit `e46930c8ba9fd8eac59670798a46646a571fcd77`)
   - Combined Cargo workspace reconciliation commit on `android/v1-app`: `e486f3c2eb8371232daeee045610e4c14404acc8` (reconciled `Cargo.toml` and `Cargo.lock` containing `crates/oklo_core`, `crates/oklo_web`, `crates/oklo_core_jni`, and `tests/conformance_cf2`; verified with `cargo ndk`, `./gradlew testDebugUnitTest assembleDebug assembleAndroidTest bundleRelease`, and `12/12` API 36 emulator tests prior to merging)
   - Merge commit on `main`: `636b7d7bb41999b80278d90046da893ee1044f20`

All pre-integration receipts (`receipts/cf2_*_2026-10-19.*`, `receipts/ios_*_2026-10-19.*`, and `receipts/android_*_2026-10-19.*`) are preserved unmodified as historical evidence.

---

## 2. Integrated Verification Matrix at `636b7d7bb41999b80278d90046da893ee1044f20`

| Suite / Check | Target / Environment | Command | Result | Details |
| :--- | :--- | :--- | :--- | :--- |
| **CF-2 Rust Conformance (`conformance_cf2`)** | macOS `aarch64-apple-darwin` (`rustc 1.93.0`) | `cargo test -p conformance_cf2 --locked` | **PASS (9/9)** | 4 unit tests + 5 integration tests (`20/20` CF-2 cases, `120` 3D volumes, Rust API + C ABI + Python bitwise identity). |
| **M0 Conformance (`oklo_core --test conformance`)** | macOS `aarch64-apple-darwin` | `cargo test -p oklo_core --test conformance --locked` | **PASS (7/7)** | Matches Python export, recorded snapshot, C ABI round-trip, phase stepping, CF-3 header, and 160-frame temperature isovalue. |
| **Phase Step Contract (`oklo_core --test step`)** | macOS `aarch64-apple-darwin` | `cargo test -p oklo_core --test step --locked` | **PASS (10/10)** | Wrapping, error handling, and 10,000-step bounds invariance. |
| **Uranium Column (`oklo_core --test uranium`)** | macOS `aarch64-apple-darwin` | `cargo test -p oklo_core --test uranium --locked` | **PASS (29/29)** | Full 144-run factorial sweep, spatial/temporal convergence, mass ledger closure, N1 positivity, and C1/C2 controls (`547.71 s`). |
| **JNI & Web Crates (`oklo_core_jni`, `oklo_web`)** | macOS `aarch64-apple-darwin` | `cargo test -p oklo_core_jni --locked && cargo test -p oklo_web --locked` | **PASS** | Compiles cleanly in combined workspace with `--locked`. |
| **Independent Python CF-2 Reference** | Python 3 (`macOS arm64`) | `python3 scripts/cf2_reference.py --verify` | **PASS (6/6 fields)** | Case 6 SHA-256 digests match frozen spec snapshot. |
| **Android Native JNI Build & 16 KB ELF Page Alignment** | NDK `28.2.13676358` (`arm64-v8a` + `x86_64`) | `bash scripts/build_android_native.sh` + `llvm-readelf -l` | **PASS** | `arm64-v8a` (`802c4aa3...`) and `x86_64` (`7ed74f69...`) both have `0x4000` (16 KB) `LOAD` alignment. |
| **iOS XCFramework + Swift Package (`OkloCoreKit`)** | macOS / iOS / iOS Simulator (`aarch64`) | `bash scripts/build_xcframework.sh && (cd ios/OkloCoreKit && swift test)` | **PASS (5 suites / 10 cases)** | Bit-for-bit match across all 6 fields, ABI version, step wrapping, and zone counts. |
| **iOS Simulator App (`OkloPool`)** | `iPhone 17` (`646CF1C6-8720-4906-B73B-FD46ACAA8EF0`, iOS 27.0 `arm64`, Xcode 27.0) | `xcodebuild test -project ios/OkloPool.xcodeproj -scheme OkloPool ...` | **PASS (15/15)** | `11/11` unit tests (including 20-case CF-2 matrix and Option A polymer metadata) + `4/4` UI tests; `lsof` on PID `86150` confirmed `0` active network sockets (`ZERO_INET_SOCKETS`). |
| **Android JVM Unit Tests & Bundles** | OpenJDK `17.0.18`, Gradle `8.11.1`, AGP `8.7.3` (`compileSdk 36`, `targetSdk 36`) | `./gradlew testDebugUnitTest assembleDebug assembleAndroidTest bundleRelease --no-daemon` | **PASS (8/8 unit tests)** | `app-debug.apk` (`eda97a9b...`), `app-debug-androidTest.apk` (`6970390f...`), `app-release.aab` (`f36f8f44...`); includes 60-frame phase scrubber (`0/60..59/60`) regression test. |
| **Android API 36 Emulator Instrumentation** | `oklo-api36` (`sdk_gphone64_arm64`, Android 16 / API 36, `arm64-v8a`) | `adb -s emulator-5554 shell am instrument -w -r org.oklo.pool.test/androidx.test.runner.AndroidJUnitRunner` | **PASS (12/12)** | `6/6` `ViewerSmokeTest` (including `phaseScrubberDisplaysFrame60AtMaxAndCannotProducePhaseOne`) + `6/6` `NativeBridgeTest` (including 20-case CF-2 matrix); UID `10213` (`PID 2145`) verified `ACTIVE_INET_SOCKETS: 0` and zero `INTERNET` permission. |
| **Text Audit** | Repository-wide | `bash scripts/text_audit.sh` | **PASS** | `0 en dashes, 0 em dashes, 0 quarantined terms`. |

> Note on research-track Darcy module: `crates/oklo_core/tests/darcy.rs` is part of the separate groundwater Darcy research track (requiring `OKLO_DARCY_RULE=v1` or `a4`, where `d3a_layers_in_series` is a known open research diagnostic). It is not invoked by the revision-0 six-field GUI conformance fixture (`oklo_sim_*` C ABI).

---

## 3. Open Limitations and Store-Submission Blockers

1. **Android `x86_64` Emulator Runtime:** `x86_64` native library (`liboklo_core_jni.so`, SHA-256 `7ed74f69a8f6dd0e60aadf9b37dae136e26331229d6dc955081e0592fb0c71f2`) is compiled, 16 KB ELF page-aligned (`0x4000`), and packaged into both `app-debug.apk` and `app-release.aab`, but **runtime not tested** on this Apple Silicon (`arm64`) host.
2. **Physical-Device Smoke Tests:** Not run on physical iPhone/iPad or physical Android hardware (**BLOCKED** on hardware provisioning and developer signing).
3. **Production Code Signing:** iOS archive requires an Apple Developer distribution certificate and provisioning profile; Android `app-release.aab` requires an upload/release keystore rather than debug signing.
4. **Public HTTPS Privacy-Policy Hosting & Store Console Submission:** `docs/store/privacy-policy.html` and `docs/store/privacy-policy.md` must be hosted at a permanent public HTTPS URL, and the prepared App Store Connect and Google Play Console forms must be completed by the account owner.
