# oklo_pool_mobile

This is milestone M0 of a mobile viewer for the Oklo GUI conformance fixture. It holds a dependency-free Rust library, oklo_core, that reproduces revision 0 of the fixture's six fields from the equations in CONFORMANCE_FIXTURE_SPEC.md section 3. A hand-written C header exposes it, and a Swift package (ios/OkloCoreKit) links it on iOS and macOS. Android has a written JNI contract in docs/BRIDGING.md but no build, because no Android SDK or NDK is installed on this machine.

The fields are prescribed analytic functions of grid position and cycle phase. They are not a physical simulation, not a calibrated model, and not evidence that a prebiotic pool operated at Oklo. docs/PROVENANCE.md gives each field's definition record and the text for the app's assumptions and provenance drawer.

The source of truth is the frozen fixture in ~/.gemini/antigravity/scratch/oklo_gui_scratch, with CONFORMANCE_FIXTURE_SPEC.md at sha256 587745ede6c6997a298f8f0a4e01127823de1b7c606383cfa49b87db0cc60d0c. This repository reads that directory and never writes to it.

## Status on 2026-10-17

The record is receipts/m0_conformance_2026-10-17.txt. At the one case stored on disk (phase 0.15, active scenario, 32 x 32 x 32, pulse boundary 0.17), all six Rust fields are byte-identical to the MRC files that export_oklo_mrc.py wrote, in debug and release builds, on the first run of the code as written. Swift reads the same values through the C ABI on the macOS slice and matches them bit for bit. The iOS device and simulator slices build, but they have not been run, because no simulator runtime is installed. This is agreement with one implementation at one case. It is not the CF-2 check, which needs an independent reference generator and twenty cases.

oklo_sim_step does not work yet. It depends on advance_phase in crates/oklo_core/src/lib.rs, which is left for Hunter, because revision 0 has no time axis and the mapping from seconds to phase is a modelling decision. Until it is written, calling step aborts the process.

## Build and test

```sh
cargo test --offline -- --nocapture          # checks (a) to (d) against the fixture MRC files
scripts/build_xcframework.sh                 # ios-arm64, ios-arm64-simulator, macos-arm64
(cd ios/OkloCoreKit && swift test)           # Swift -> C ABI -> Rust, macOS slice
scripts/text_audit.sh                        # no en or em dashes, no quarantined terms
```

Set OKLO_FIXTURE_DIR to test against another copy of the fixture.

## Interactive viewer

[Oklo Fixture Viewer](https://claude.ai/artifact/39J73egtLdJJMwSizNmiQ9) shows the six fields at the tested case as a plan slice and a vertical section, with the R0-1 sensitivity map (temperature at b = 0.1667 minus b = 0.17, about 25.5 C at most). It hashes its data in the browser and compares the result with the spec's recorded hashes. The link is a private claude.ai artifact, so other people can open it only after it is shared with them from its Share menu. It shows only the tested case: there is no phase animation, because time steps are not defined yet.

The page source is viewer/index.html. To rebuild its data and view it locally:

```sh
cargo run --example dump_fields -- viewer/data 0.17
cargo run --example dump_fields -- viewer/data 0.1667
python3 -I scripts/pack_viewer_data.py viewer/data
(cd viewer && python3 -m http.server 8000)    # then open http://localhost:8000
```

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

## Not authorized in M0

No new physics: the finite-difference transport solvers, RNA polymerization kinetics, replicator dynamics and ARC puzzle layer in the original mobile prompt each need their own pre-registered specification first. Also not authorized: parameter fitting, installing the Android SDK or NDK or an iOS simulator runtime, choosing between 0.1667 and 0.17, making this repository public (it is a private GitHub repository by Hunter's decision), and any App Store or Google Play submission. Google Play currently requires new apps and updates to target Android 16 (API level 36), so the M0 Android contract targets that level.
