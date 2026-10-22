# oklo_pool_mobile

`oklo_pool_mobile` is a cross-platform mobile and desktop viewer for the revision 0, six-field Oklo GUI conformance fixture. It holds a dependency-free Rust library (`crates/oklo_core`) that reproduces revision 0 of the fixture's six fields from the equations in `CONFORMANCE_FIXTURE_SPEC.md` section 3, a C ABI header (`crates/oklo_core/include/oklo_core.h`), a Swift package (`ios/OkloCoreKit`), a native iOS SwiftUI + Metal application (`ios/OkloPool`), a Rust JNI bridge (`crates/oklo_core_jni`), a native Android Kotlin/Compose application (`android/`) targeting API 36, an independent 20-case CF-2 conformance suite (`tests/conformance_cf2` and `scripts/cf2_reference.py`), and evidence-backed store release and privacy materials (`docs/store/`).

The fields are prescribed analytic functions of grid position and cycle phase. They are not a physical simulation, not a calibrated model, and not evidence that a prebiotic pool operated at Oklo. `docs/PROVENANCE.md` gives each field's definition record and the text for the app's assumptions and provenance drawer.

The source of truth is the frozen fixture in `~/.gemini/antigravity/scratch/oklo_gui_scratch`, with `CONFORMANCE_FIXTURE_SPEC.md` at sha256 `587745ede6c6997a298f8f0a4e01127823de1b7c606383cfa49b87db0cc60d0c`. This repository reads that directory and never writes to it.

## Cross-platform v1 mobile release status (2026-10-20)

Four review branches implement and verify the cross-platform v1 viewer around the frozen revision 0 fixture (PRs #4, #5, and #6 merged into `main` at `636b7d7bb41999b80278d90046da893ee1044f20` under Gate 3 authorization; integrated verification recorded in `receipts/integrated_gate3_verification_2026-10-20.md` and `receipts/integrated_gate3_verification_2026-10-20.txt` at SHA-256 `01f3d5acc98bff2167d566cb28b152f9364f6c06cb87d10e14b39ef3235bdb39`):

1. **CF-2 independent 20-case conformance (`conformance/cf2-suite`, [PR #4](https://github.com/aurascoper/oklo_pool_mobile/pull/4), merged at `1483cebd8c7e96f7188d3a0c79b91a38f64223a4`):**
   - Independent reference generator (`tests/conformance_cf2` and `scripts/cf2_reference.py`) derived directly from `CONFORMANCE_FIXTURE_SPEC.md` section 3 across 20 pinned cases (5 phases x 2 scenarios x 2 pulse boundaries = 120 3D field volumes).
   - **PASS** across all four runnable targets: Host Rust (`aarch64-apple-darwin`), `OkloCoreKit` Swift package (`aarch64-apple-darwin`), iOS 27.0 Simulator (`iPhone 17` `arm64`), and Android 16 / API 36 Emulator (`oklo-api36` `arm64-v8a`), matching the reference SHA-256 digests bit-for-bit (`0` mismatches). Android `x86_64` JNI is build-verified and 16 KB ELF aligned; `x86_64` runtime execution is not tested (blocked by the Apple Silicon `arm64` host architecture).
   - Receipts: `receipts/cf2_conformance_2026-10-19.txt`, `receipts/cf2_matrix_2026-10-19.json`, `receipts/cf2_reference_outputs_sha256_2026-10-19.txt` (tested source commit `ae56062a9f96fe12988a658dd89b3427e244bc9a`, receipt commit `3a2d41e6260d4dfe53b70c83be97861740ac6930`).
2. **Native iOS application (`ios/v1-app`, [PR #5](https://github.com/aurascoper/oklo_pool_mobile/pull/5), merged at `798870bcf097c5ee135bb1956e8b409505bd2957`):**
   - Complete SwiftUI + Metal application (`ios/OkloPool.xcodeproj`) rendering all six revision 0 fields (with field 4 presented as `Polymer accumulation index` in `arbitrary units` under `docs/specs/FIELD_ONTOLOGY_AMENDMENT_A1.md`), 2D Plan/Section slices, 3D Metal volume raymarcher, strip chart, phase transport, pulse boundary (`0.17` vs `0.1667`), null scenario, and in-app provenance sheet.
   - Generic iOS device and simulator builds succeed; **15 of 15 simulator tests pass** (11 unit tests including the 20-case CF-2 matrix and the Option A ontology metadata check + 4 UI tests) on `iPhone 17` (iOS 27.0 `arm64`); runtime `lsof` inspection confirmed zero open `IPv4`/`IPv6` sockets (`ZERO_INET_SOCKETS`).
   - Receipts: `receipts/ios_simulator_test_2026-10-19.md`, `receipts/ios_simulator_test_2026-10-19.json`, `receipts/ios_simulator_test_2026-10-19.txt` (tested source commit `8235b1a36e870dd0c0347501129802f546587acd`, receipt commit `49a5169328681e9145faacffcf6819d5572b0d9d`).
3. **Native Android application and JNI bridge (`android/v1-app`, [PR #6](https://github.com/aurascoper/oklo_pool_mobile/pull/6), reconciled at `e486f3c2eb8371232daeee045610e4c14404acc8` and merged at `636b7d7bb41999b80278d90046da893ee1044f20`):**
   - Complete Kotlin/Compose application (`android/`) and Rust JNI bridge (`crates/oklo_core_jni`) targeting API 36 (`compileSdk=36`, `targetSdk=36`, `minSdk=26`) with 16 KB ELF `PT_LOAD` alignment verified on both `arm64-v8a` and `x86_64`, presenting field 4 as `Polymer accumulation index` (`arbitrary units`) with the revision-0 `umol/L` caveat, and aligning the phase scrubber to the 60 discrete cycle frames (`0/60..59/60`).
   - **8 of 8 JVM unit tests pass** and **12 of 12 API 36 `arm64-v8a` emulator instrumented tests pass** (`NativeBridgeTest` including the 20-case CF-2 matrix + `ViewerSmokeTest` including the Option A and 60-frame phase scrubber checks); debug APK and unsigned release AAB build; runtime inspection confirmed no `android.permission.INTERNET` permission and zero open sockets (`ACTIVE_INET_SOCKETS: 0`).
   - Receipts: `receipts/android_build_2026-10-19.md`, `receipts/android_emulator_test_2026-10-19.txt` (tested source commit `e46930c8ba9fd8eac59670798a46646a571fcd77`, receipt commit `1dfde72db1f121ddb488c1daf041a7a5a0593ca2`), plus post-merge integrated verification in `receipts/integrated_gate3_verification_2026-10-20.md`.
4. **Store release materials and privacy audit (`release/v1-materials`, [PR #7](https://github.com/aurascoper/oklo_pool_mobile/pull/7)):**
   - Display-ontology amendment specification (`docs/specs/FIELD_ONTOLOGY_AMENDMENT_A1.md`), privacy policy (`docs/store/privacy-policy.md`, `docs/store/privacy-policy.html`), dependency and data-flow inventory (`docs/store/data-flow-inventory.md`), Apple App Privacy and Google Play Data safety drafts, store listing draft, app icons, 9 authentic simulator/emulator PNG screenshots (`docs/store/screenshots/`), post-merge integrated verification receipt (`receipts/integrated_gate3_verification_2026-10-20.md`), and the release gate report (`docs/store/release-gate-report.md`).
   - Reconciled with On-Render Gate implementation on `release/on-render-gate` (PR #8) and benchmark receipt `receipts/on_render_gate_2026-10-20.md`.
   - Overall release gate remains **NOT STORE-SUBMISSION-READY** until external account-owner prerequisites are completed: physical-device smoke tests on provisioned hardware, production Apple/Google distribution code signing, public HTTPS hosting confirmation of the privacy policy, and resolving host disk space for Android emulator AVD startup.

### Milestone A On-Render Gate and release reconciliation summary

1. **Persistent initial-screen statement and disclaimers:**
   - Both apps render a persistent initial-screen disclaimer visible across all views on launch:
     `"Revision-0 prescribed fields; not a physical simulation."`
   - Active field limitation statements are rendered dynamically adjacent to the visualization canvas.

2. **Field limitation records for all six fields:**
   The in-app Scope and Provenance views document the complete six-field limitation catalog:
   - **Temperature:** Prescribed thermal profile; not coupled to hydrodynamic or convective cooling.
   - **Hydration:** Prescribed saturation index; not a multiphase porous flow solve.
   - **H2 Concentration:** Prescribed radiolysis proxy; not a reaction-diffusion kinetic solve.
   - **235U/238U Ratio:** Prescribed isotopic depletion curve; not a 3D neutron transport solve.
   - **Polymer accumulation index (arbitrary units):** Prescribed heuristic field. The revision-0 fixture labels this quantity in umol/L, but no kinetic or thermodynamic basis for those concentration units is established. Values are not measured or predicted polymer concentrations.
   - **Compartments:** Prescribed geometric bounding cells; not a physical membrane simulation.

3. **Privacy Policy and manifest audit:**
   - Offline bundled full-text privacy policy accessible in-app with zero network requirements.
   - Local computation only: zero tracking, zero networking permissions, zero open sockets (`ZERO_INET_SOCKETS`).
   - Explicit disclosure: proposed URL `https://aurascoper.github.io/oklo_pool_mobile/privacy-policy.html` remains unverified until confirmed live by the account owner.
   - Apple `PrivacyInfo.xcprivacy` included in app target (Milestone A1 PR #8), declaring empty collection/tracking arrays.

4. **Timing budgets and benchmark methodology:**
   - Frozen measurement methodology documented in `receipts/on_render_gate_2026-10-20.md`: separating computation latency from time to rendered frame, 3 unmeasured warmup cycles, ascending order percentiles.
   - Budget 1 (Initial model and slice generation <= 100 ms): PASS (iOS p95 3.99 ms, Android p95 2.83 ms).
   - Budget 2 (Field switching <= 50 ms): PASS (iOS p95 9.27 ms, Android p95 2.85 ms).
   - Budget 3a (Active single-field 60-frame chart calculation): nominal runs pass within 315 ms (iOS) and 172 ms (Android); under iOS simulator test suite CPU contention p95 reaches 435 to 445 ms. Threshold revision proposed: `<= 500 ms`.
   - Budget 3b (Full 6-field 60-frame precomputation, 360 3D volumes / 11.79M voxels): iOS simulator median 1086 to 1398 ms, p95 1450 to 1500 ms; Android median 208 ms. Threshold revision proposed: `<= 1500 ms` (recommended `<= 1600 ms` headroom).
   - Budget 4 (Volume frame readiness <= 100 ms): PASS (iOS p95 0.19 ms, Android p95 0.25 ms).
   - Budget 5 (Phase update <= 35 ms): PASS (iOS p95 9.34 ms, Android p95 2.91 ms).

5. **Android emulator startup status:**
   - Status: **BLOCKED / UNRESOLVED**. Host machine has <1 GiB free disk space (840 MiB on root APFS volume), blocking `oklo-api36` AVD memory and overlay allocation. Android JVM native tests against release JNI libraries verified 18/18 checks.

6. **Account-owner blocker catalog before store submission:**
   - App Store Connect account configuration, bundle ID registration, signing certificates, and live hosting confirmation of privacy policy.
   - Physical iOS and Android device validation (Apple Developer account, TestFlight, Google Play Console internal testing).
   - Android Emulator host disk space: host machine has <1 GiB free disk space, blocking `oklo-api36` AVD startup (status: BLOCKED / UNRESOLVED per Launch Gates).

7. **Authentic screenshots:**
   - Nine authentic PNG screenshots (4 iOS Simulator at `1206x2622`, 5 Android Emulator at `1080x2400`) verified in `docs/store/screenshots/`. Zero mock placeholders.

## Historical M0 status on 2026-10-17

The record is receipts/m0_conformance_2026-10-17.txt. At the one case stored on disk (phase 0.15, active scenario, 32 x 32 x 32, pulse boundary 0.17), all six Rust fields are byte-identical to the MRC files that export_oklo_mrc.py wrote, in debug and release builds, on the first run of the code as written. Swift reads the same values through the C ABI on the macOS slice and matches them bit for bit. The iOS device and simulator slices build, but they have not been run, because no simulator runtime is installed. This is agreement with one implementation at one case. It is not the CF-2 check, which needs an independent reference generator and twenty cases.

oklo_sim_step follows the contract adopted on 2026-10-17 (advance_phase in crates/oklo_core/src/lib.rs): forward steps wrap at the end of the cycle into [0, 1), negative and non-finite steps are errors that change nothing, and the configured cycle_period_s is the only time scale. crates/oklo_core/tests/step.rs covers it, through the Rust API and the C ABI, and the Swift package tests it through the bridge. Stepping is navigation along prescribed functions of phase, not dynamics, and the 3 h period it usually runs with is inherited, not validated.

## Build and test

```sh
cargo test --offline -- --nocapture          # M0 and core checks against the fixture MRC files
cargo test --release -p conformance_cf2      # 20-case CF-2 independent reference conformance suite
scripts/build_xcframework.sh                 # ios-arm64, ios-arm64-simulator, macos-arm64
(cd ios/OkloCoreKit && swift test)           # Swift -> C ABI -> Rust, macOS slice
xcodebuild test -project ios/OkloPool.xcodeproj -scheme OkloPool \
  -destination 'platform=iOS Simulator,name=iPhone 17' CODE_SIGNING_ALLOWED=NO
scripts/build_android_native.sh              # arm64-v8a and x86_64 JNI libraries (16 KB page-aligned)
(cd android && ./gradlew testDebugUnitTest assembleDebug assembleAndroidTest bundleRelease)
scripts/text_audit.sh                        # no en or em dashes, no quarantined terms
```

Set OKLO_FIXTURE_DIR to test against another copy of the fixture.

## Interactive viewer

[Oklo Fixture Viewer](https://claude.ai/artifact/39J73egtLdJJMwSizNmiQ9) shows one field at a time over one cycle: a 3D volume, plan and section slices, play and scrub over 60 frames (3 minutes each under the 3 h period), a pulse-boundary toggle, a null-scenario toggle, and a strip chart of the field and of the prescribed pulse and desiccation curves. The page computes every value with crates/oklo_web, which is oklo_core compiled to WebAssembly, so the browser runs the same engine the tests check. On load it checks frame 9 against the spec's recorded hashes, and it can check all 720 frames against the native Rust series. The series samples prescribed functions of phase; it is not simulated dynamics, and no flow is modelled. The link is a private claude.ai artifact, so other people can open it only after it is shared from its Share menu.

To rebuild the viewer's files and view it locally:

```sh
cargo build --release --target wasm32-unknown-unknown -p oklo_web
cargo run --release --example export_series -- out/series/b0.17 0.17
cargo run --release --example export_series -- out/series/b0.1667 0.1667
node scripts/wasm_parity.mjs                   # writes viewer/data/native_hashes.json
cp target/wasm32-unknown-unknown/release/oklo_web.wasm viewer/data/
cp out/probes/chimerax_frame9.json viewer/data/   # from the ChimeraX probe below
(cd viewer && python3 -m http.server 8000)     # then open http://localhost:8000
```

## ChimeraX, one cycle side by side

chimerax/oklo_cycle.cxc opens the six fields as 60-frame volume series from out/series/b0.17, names each with its unit as revision 0 labels it, gives each one display style, and tiles them in two rows of three so nothing overlaps. In ChimeraX, run `open chimerax/oklo_cycle.cxc`, then `vseries play #1-6 loop true`. The tile step needs the ChimeraX window; headless ChimeraX cannot run it. chimerax/probe_frame9.py is the CF-4 check: run after opening the series, it hashes ChimeraX's frame 9 matrices and records step and origin on all three axes in out/probes/chimerax_frame9.json.

## Flow

Revision 0 has no flow. docs/specs/DARCY_FLOW_PREREG_v1.md is Darcy pre-registration v1: steady, single-phase, isothermal Darcy flow on the fixture geometry, with every parameter, boundary condition, solver setting and pass criterion declared in advance. It is frozen (sha256 recorded in receipts/darcy_prereg_v1_freeze.txt) and not approved for implementation; it authorizes no solver code, no parameter fitting and no claim about actual Oklo conditions. Its versioning is separate from the fixture's scientific-model revision 1. docs/specs/DARCY_FLOW_PREREG_v1_AMENDMENT_A1.md, also frozen (receipts/darcy_prereg_v1_amendment_A1_freeze.txt), defines the thermal Péclet screening diagnostic, Pe = ρ_w c_p,w |q| L / λ_m at the 0.5 m voxel scale, with λ_m for water-saturated sandstone from Robertson 1988 (USGS OFR 88-441), and records the porosity decisions. It changes nothing in v1. docs/specs/DARCY_FLOW_PREREG_v1_AMENDMENT_A2.md, frozen (receipts/darcy_prereg_v1_amendment_A2_freeze.txt), separates the two roles of porosity: the thermal-property span that bounds λ_m and so enters Pe, and the zone flow porosities that enter only pore velocity. It changes no number. docs/specs/DARCY_FLOW_PREREG_v1_AMENDMENT_A3.md, frozen before any solver code (receipts/darcy_prereg_v1_amendment_A3_freeze.txt), corrects the sign of the conservation residual: the physical residual is A h - b, the negative of the solver residual b - A h; no tolerance or check changes. Implementation of v1 with A1 to A3 is approved, bounded to the declared hypothetical numerical experiment.

The implementation is crates/oklo_core/src/darcy.rs, with checks in crates/oklo_core/tests/darcy.rs and the run in crates/oklo_core/examples/darcy_run.rs. Results are in receipts/darcy_v1_run_2026-10-18.json, and the verbatim test log is in receipts/darcy_v1_tests_2026-10-18.txt. Two pre-registered checks fail and are recorded as failures, with nothing tuned: D3a's conjugate-gradient flux error (2.58e-10 against 1e-10, while the direct solve passes at 3.1e-13), and D1's global balance in configuration ratio_1 (1.68e-10 against 1e-10). Both show that the pre-registered CG stopping rule, 1e-12 relative to ||b||, does not by itself guarantee those two tolerances. D1 (every other part and configuration), the zero-flow control, D2 (orders 2.0021 and 2.0005), D3b, D4 and D5 pass, and all eleven configurations converge. The flow is a hypothetical numerical experiment on the fixture geometry, not the hydrogeology of Oklo. Pore velocities use unsourced flow porosities except in the core.

Amendment A4 (docs/specs/DARCY_FLOW_PREREG_v1_AMENDMENT_A4.md, frozen before any rerun) tightened only the CG rule, to 1e-13, and reran the whole suite as a separate verification experiment; v1's receipt and log are unchanged and v1's two failures remain v1's result. Under A4 (receipts/darcy_v1_A4_run_2026-10-18.json and receipts/darcy_v1_A4_tests_2026-10-18.txt), D1 passes in all eleven configurations, including ratio_1 (global balance 7.0e-13), and every check that passed under v1 still passes. D3a still fails with the identical CG flux error, 2.58e-10. A post-hoc diagnostic, not pre-registered (receipts/darcy_v1_A4_posthoc_D3a_diagnostic.txt), shows why: CG already reaches a residual of 5.8e-14 in nine iterations under both rules. D3a measures flux at the bottom face beside the most permeable layer, where a head error near 1e-13 becomes a relative flux error near 1e-9. The metric is not changed after the fact, so D3a stays failed. Select the rule with OKLO_DARCY_RULE=v1 or a4 for the tests, and as the last argument of the darcy_run example.

The A4 experiment is closed (2026-10-18). D3a stays failed under v1 and under A4, and both results stand as recorded. No A5 is planned. If one is ever written, it would be a separately pre-registered comparison of a named alternative solver method, frozen before any code or run, with D3a's bottom-face flux metric and its 1e-10 threshold kept unchanged as the reference. It would not be a change of metric or tolerance.

## Uranium reduction-transport column and timeline

docs/specs/URANIUM_REDOX_TRANSPORT_PREREG_v1.md is a frozen pre-registration (receipts/uranium_prereg_v1_freeze.txt) for a synthetic one-dimensional U(VI) reduction-transport column: advection-dispersion with linear U(VI) sorption, a stationary organic zone with an effective first-order immobilization to a U(IV)-bearing solid, a mass ledger, two analytic benchmarks (Ogata-Banks on its own semi-infinite problem, and a finite-domain steady state), and controls including negative tests. Its geological motivation (Gauthier-Lafaye et al. 1996 and 2002) does not validate its kinetics, and every rate, dispersion and site input is a declared assumption or an EPA literature sensitivity bound. It is not a reconstruction of Oklo, and implementation was not approved when it was frozen.

Amendment A1 (docs/specs/URANIUM_REDOX_TRANSPORT_PREREG_v1_AMENDMENT_A1.md, frozen before any code, receipts/uranium_prereg_v1_amendment_A1_freeze.txt) supplements v1 without editing it. It bounds the time step so that the scheme provably keeps 0 ≤ C ≤ C0, makes every output time a whole number of steps, pins the grid rule's floating-point evaluation, adds a non-negativity check and a temporal-accuracy check, and makes control C2 sound. It is frozen together with a machine-readable parameter manifest, and it follows two review records in docs/research (an evidence review, stated as incomplete, and a numerical-method review).

Hunter approved implementation on 2026-10-18, strictly as a numerical implementation of frozen v1 plus A1, on branch uranium-column-v1 and not merged into main. The code is crates/oklo_core/src/uranium.rs, with the checks in crates/oklo_core/tests/uranium.rs and the sweep in crates/oklo_core/examples/uranium_run.rs. The recorded run at commit 0af5af4 is receipts/uranium_v1_A1_run_2026-10-18.json, with the verbatim test log in receipts/uranium_v1_A1_tests_2026-10-18.txt (29 of 29 tests pass, exit status 0) and the output-file hashes in receipts/uranium_v1_A1_outputs_sha256_2026-10-18.txt. Every pre-registered check passed. The nine grid sizes match A1.4. B1's spatial orders are 2.0022 and 2.0006, T1's temporal order is 2.0000, and B2's orders are 2.0007 and 2.0002, all inside the frozen band of 1.8 to 2.2. The ledger closes within 4.7e-12 of cumulative inflow in all 144 runs, against a limit of 1e-10. N1 held at every step of every uncorrupted run. C1 kept S exactly zero in the 36 runs without reaction, and C2 detected both corrupted variants in all 36 configurations, with the uncorrupted model triggering neither rule. One implementation choice is recorded in the code and in commit 2ff4b01: the cell Péclet number is evaluated as Pe_s / N, which equals v dx / D_eff in exact arithmetic, because evaluated the other way the six pairs that sit exactly on the limit of 2 come out at 2.0000000000000004. These results show that the code implements the declared synthetic equations to the tested tolerances. They do not validate the assumed rates, sorption values or organic zone, and the column is not a reconstruction of Oklo or the Franceville Basin. To reproduce:

```sh
cargo test --release --offline -p oklo_core --test uranium
cargo run --release --offline -p oklo_core --example uranium_run -- out/uranium_v1_A1 receipt.json meta.json
```

data/timeline/paleo_intervals_v1.json is a frozen, source-labelled set of intervals (receipts/timeline_v1_freeze.txt): two separately attributed definitions of the Great Oxidation Event (Gumsley et al. 2017; the 2.43 to 2.22 Ga framing of Craig et al. 2023), the Oklo reaction date of 1.95 +/- 0.04 Ga (Gauthier-Lafaye et al. 1996), and one study's estimate of eukaryote origins (Craig et al. 2023). Date separations are labelled as derived, never as a GOE-to-Oklo lag.

The original Oklo GUI (index.html in the fixture directory) is not linked here. It uses the 0.1667 boundary, so it disagrees with these tested values by up to 25.5 C, and it loads a development-only CDN script (findings R0-1 and R0-9).

## M0 calibration and conformance policy

Adopted from Hunter on 2026-10-17.

1. The revision 0 equations, parameter values, operation order, and conformance fixtures are frozen inputs.
2. Conformance runs may measure discrepancies but may not optimize parameters, select thresholds, or refit equations to reduce those discrepancies.
3. An implementation fix must identify the violated specification clause and retain a before/after test receipt.
4. A discrepancy that cannot be resolved from the frozen specification must be reported as unresolved, not silently calibrated away.
5. Any scientific parameter adjustment, alternative pulse-boundary choice, or model revision requires a versioned proposal, explicit authorization, and a separate evidence record.
6. Calibration, when authorized later, must use designated calibration data and a documented procedure. Validation data must remain independent of parameter fitting and model selection.

The pulse boundary is a required configuration value with no default. The tests use 0.17 when comparing with the stored files because the spec records that the exporter which wrote them uses 0.17. The 0.1667 run is a sensitivity diagnostic showing that the comparison can detect finding R0-1. It is not a calibration result and does not endorse either value.

## Governance and scope boundaries

No new physics is added by the mobile release branches: the finite-difference transport solvers beyond frozen Darcy v1/A1-A4 and frozen Uranium v1/A1, RNA polymerization kinetics, replicator dynamics, and ARC puzzle layer in the original mobile prompt each need their own pre-registered specification first. Parameter fitting, choosing between `0.1667` and `0.17`, and making this repository public remain unauthorized without explicit approval. While the iOS Simulator and Android SDK/NDK/Emulator prerequisites were approved on 2026-10-19 for v1 build and runtime verification, live App Store or Google Play submission remains gated on account-owner physical-device testing, release code signing, public HTTPS privacy-policy hosting, and explicit user authorization. Google Play requires new apps and updates to target Android 16 (API level 36), which the Android app targets (`compileSdk=36`, `targetSdk=36`).
