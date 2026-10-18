# oklo_pool_mobile

This is milestone M0 of a mobile viewer for the Oklo GUI conformance fixture. It holds a dependency-free Rust library, oklo_core, that reproduces revision 0 of the fixture's six fields from the equations in CONFORMANCE_FIXTURE_SPEC.md section 3. A hand-written C header exposes it, and a Swift package (ios/OkloCoreKit) links it on iOS and macOS. Android has a written JNI contract in docs/BRIDGING.md but no build, because no Android SDK or NDK is installed on this machine.

The fields are prescribed analytic functions of grid position and cycle phase. They are not a physical simulation, not a calibrated model, and not evidence that a prebiotic pool operated at Oklo. docs/PROVENANCE.md gives each field's definition record and the text for the app's assumptions and provenance drawer.

The source of truth is the frozen fixture in ~/.gemini/antigravity/scratch/oklo_gui_scratch, with CONFORMANCE_FIXTURE_SPEC.md at sha256 587745ede6c6997a298f8f0a4e01127823de1b7c606383cfa49b87db0cc60d0c. This repository reads that directory and never writes to it.

## Status on 2026-10-17

The record is receipts/m0_conformance_2026-10-17.txt. At the one case stored on disk (phase 0.15, active scenario, 32 x 32 x 32, pulse boundary 0.17), all six Rust fields are byte-identical to the MRC files that export_oklo_mrc.py wrote, in debug and release builds, on the first run of the code as written. Swift reads the same values through the C ABI on the macOS slice and matches them bit for bit. The iOS device and simulator slices build, but they have not been run, because no simulator runtime is installed. This is agreement with one implementation at one case. It is not the CF-2 check, which needs an independent reference generator and twenty cases.

oklo_sim_step follows the contract adopted on 2026-10-17 (advance_phase in crates/oklo_core/src/lib.rs): forward steps wrap at the end of the cycle into [0, 1), negative and non-finite steps are errors that change nothing, and the configured cycle_period_s is the only time scale. crates/oklo_core/tests/step.rs covers it, through the Rust API and the C ABI, and the Swift package tests it through the bridge. Stepping is navigation along prescribed functions of phase, not dynamics, and the 3 h period it usually runs with is inherited, not validated.

## Build and test

```sh
cargo test --offline -- --nocapture          # checks (a) to (d) against the fixture MRC files
scripts/build_xcframework.sh                 # ios-arm64, ios-arm64-simulator, macos-arm64
(cd ios/OkloCoreKit && swift test)           # Swift -> C ABI -> Rust, macOS slice
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

Revision 0 has no flow. docs/specs/DARCY_FLOW_PREREG_v1.md is Darcy pre-registration v1: steady, single-phase, isothermal Darcy flow on the fixture geometry, with every parameter, boundary condition, solver setting and pass criterion declared in advance. It is frozen (sha256 recorded in receipts/darcy_prereg_v1_freeze.txt) and not approved for implementation; it authorizes no solver code, no parameter fitting and no claim about actual Oklo conditions. Its versioning is separate from the fixture's scientific-model revision 1. docs/specs/DARCY_FLOW_PREREG_v1_AMENDMENT_A1.md, also frozen (receipts/darcy_prereg_v1_amendment_A1_freeze.txt), defines the thermal Péclet screening diagnostic, Pe = ρ_w c_p,w |q| L / λ_m at the 0.5 m voxel scale, with λ_m for water-saturated sandstone from Robertson 1988 (USGS OFR 88-441), and records the porosity decisions. It changes nothing in v1. docs/specs/DARCY_FLOW_PREREG_v1_AMENDMENT_A2.md, frozen (receipts/darcy_prereg_v1_amendment_A2_freeze.txt), separates the two roles of porosity: the thermal-property span that bounds λ_m and so enters Pe, and the zone flow porosities that enter only pore velocity. It changes no number. Implementation of v1 with A1 and A2 is still not approved.

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
