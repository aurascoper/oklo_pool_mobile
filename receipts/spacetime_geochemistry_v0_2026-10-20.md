# Verification Receipt: 4D Spatiotemporal Geochemistry Explorer (`spacetime_geochemistry_v0`)

- **Document ID:** `receipts/spacetime_geochemistry_v0_2026-10-20.md`
- **Date:** 2026-10-20
- **Requirement:** R10 (`spacetime_geochemistry_v0` - Stage 0 4D Data Contract and Stage 1 Synthetic Temporal Renderer Proof)
- **Branch:** `spacetime-geochemistry/v0-slice`
- **Baseline Commit (`main` parent):** `636b7d7bb41999b80278d90046da893ee1044f20`
- **Generator Provenance (`source_provenance`):** `scripts/generate_synthetic_4d_bundle.py` (`spacetime_geochemistry_v0_gen_0.2.0`, SHA-256 `d8a78d21351d8ee0c1c791adbec48e6e48c11e032ec93c6fb30eb91b815962e7`)
- **Tested Source Commit (`tested_source_commit`):** `23e95da70ed0d8e5971c29c87771b2f3ab2243e1`
- **Worktree Clean at Test:** `true` (`worktree_clean_at_test: true`)
- **Execution Context:** Local workstation execution (`execution_context: "local_workstation"`); GitHub Actions CI is not configured in this repository (`ci_pipeline_status: "not_configured_in_github_actions"`).
- **Commit Provenance Separation:** `baseline_commit` (`636b7d7bb41999b80278d90046da893ee1044f20`) records the `main` branch commit from which `spacetime-geochemistry/v0-slice` branched and is never used to claim that `636b7d7...` generated the R10 bundle. All Stage 0/1 validators, Node.js semantic tests, Rust conformance checks, text audits, and `1920x1080` headless browser screenshots were executed against the clean `tested_source_commit` (`23e95da70ed0d8e5971c29c87771b2f3ab2243e1`). This receipt and the companion JSON receipt are committed in the immediately following `receipt_head` commit without modifying any tested source or data files. `baseline_commit`, `tested_source_commit`, and `receipt_head` are distinct and never treated as interchangeable.
- **Scientific Status and Bookkeeping Disclaimer:** Passing viewer and data-contract tests establishes rendering and data-contract correctness only, not geological validity, radiation transport accuracy, or empirical calibration. In Stage 1 (`prescribed_analytically`), `analytical_bookkeeping_ledger` records illustrative analytical decay bookkeeping (`closes_by_algebraic_definition: true`, `is_finite_volume_conservation_test: false`) together with the non-zero discrete quadrature/boundary tail gap (`relative_quadrature_and_tail_gap`). Independent finite-volume PDE mass conservation is a Stage 2 gate requirement.
- **Overall Disposition:** `PASS`

---

## 1. Runtime Environment

- **Operating System:** `Darwin 27.0.0 arm64` (macOS Apple Silicon)
- **Python Runtime:** `Python 3.15.0`
- **Node.js Runtime:** `v26.11.1`
- **Rust Toolchain:** `rustc 1.97.1 (8bab26f4f 2026-07-14)` / `cargo 1.97.1 (c980f4866 2026-06-30)`
- **Headless Browser:** `Google Chrome 155.0.8059.85` (`--headless --window-size=1920,1080 --force-device-scale-factor=1`)
- **CI Status:** Local workstation verification only (`not_configured_in_github_actions`)

---

## 2. Stage 0: 4D Spatiotemporal Data Contract Verification

**Command:** `python3 scripts/validate_spacetime_4d.py`
**Result:** `PASS` (1 primary bundle + 23/23 negative-control rejection checks passed, exit code 0)

### 2.1 Verified Discrete Time States (`data/spacetime/synthetic_tracer_pulse_4d_v0.json`)

| State ID Anchor | `state_index` | `simulation_time` (`s`) | `state_derivation` | `criterial_status` | Center of Mass $(x, y, z)\text{ m}$ | Peak Value (`mol/m^3`) | Quadrature / Tail Gap (`relative_quadrature_and_tail_gap`) | `sha256_f32_le` (IEEE-754 Little-Endian Float32) |
| :--- | :---: | :---: | :--- | :---: | :---: | :---: | :---: | :--- |
| <a id="state_0"></a>`#state_0` | 0 | `0.0` | `prescribed_analytically` | `verified_accepted` | `(8.0000, 8.0000, 4.5296)` | `9.2574` | `0.000000` | `906e083f513e4d63d0e1ce2771439f15a4e1747c23f68469b5ad90d9e854290e` |
| <a id="state_1"></a>`#state_1` | 1 | `250.0` | `prescribed_analytically` | `verified_accepted` | `(8.3747, 7.7502, 6.1304)` | `6.8159` | `0.004363` | `279ddf8ae892310376a20b573afc7fd13f359a09fe8832f9f8b703bd10d7fa30` |
| <a id="state_2"></a>`#state_2` | 2 | `500.0` | `prescribed_analytically` | `verified_accepted` | `(8.7481, 7.5011, 7.7505)` | `5.0323` | `0.004180` | `8949b42e116ee75e40ffcf55c8524905ac62e31847b9e044ac004a1c53c1e7c9` |
| <a id="state_3"></a>`#state_3` | 3 | `750.0` | `prescribed_analytically` | `verified_accepted` | `(9.1175, 7.2541, 9.3643)` | `3.7364` | `0.001913` | `1fcb13a289ea3592e79188536280d6c05400b5f5fe516262ddf91f06e7be88d2` |
| <a id="state_4"></a>`#state_4` | 4 | `1000.0` | `prescribed_analytically` | `verified_accepted` | `(9.4789, 7.0110, 10.8976)` | `2.7921` | `0.010587` | `6cd8bc7c53a40702565a4d19904e6c240c48eeed5bafa1e3208bf3a598a3267f` |

### 2.2 Stage 0 Negative-Control Rejection Checks (`23/23 PASS`)

1. Reject non-monotonic `simulation_time` ($t_2 = 200.0 \le t_1 = 250.0$): `PASS`
2. Reject non-`"s"` `temporal_axis.time_units` (`"minutes" != "s"`): `PASS`
3. Reject invalid `fourth_dimension_symbol != "t"`: `PASS`
4. Reject voxel array length mismatch (`4095 != 4096`): `PASS`
5. Reject non-finite voxel value (`NaN` at voxel 128): `PASS`
6. Reject corrupted `sha256_f32_le` frame digest: `PASS`
7. Reject `state_derivation="interpolated_for_display"` inside stored states under `none_discrete_states_only`: `PASS`
8. Reject unauthorized `evidence_class="empirically_calibrated_model"`: `PASS`
9. Reject missing intermediate frame (`state_index` gap under `fail_closed_no_invented_states`): `PASS`
10. Reject discrete stored state with `run_receipt = null`: `PASS`
11. Reject mismatched `temporal_axis.time_bounds` vs state simulation times: `PASS`
12. Reject inverted `spatial_frame.domain_bounds_m` (`min >= max`): `PASS`
13. Reject inconsistent time step interval vs `temporal_axis.temporal_resolution_s` ($\Delta t = 350.0 \ne 250.0\text{ s}$): `PASS`
14. Reject false Stage 1 `is_finite_volume_conservation_test = true` claim: `PASS`
15. Reject `source_provenance` conflating `generator_version` with `baseline_commit` or prefix: `PASS`
16. Reject fabricated `0.0` quadrature/tail gap contradicting discrete voxel sum: `PASS`
17. Reject `source_provenance.generator_script_sha256` mismatch with generator script on disk: `PASS`
18. Reject mutated `min_value` contradicting hashed voxel buffer (`values` and `sha256_f32_le` unchanged): `PASS`
19. Reject mutated `max_value` contradicting hashed voxel buffer (`values` and `sha256_f32_le` unchanged): `PASS`
20. Reject mutated `mean_value` contradicting hashed voxel buffer (`values` and `sha256_f32_le` unchanged): `PASS`
21. Reject mutated `total_integrated_inventory_mol` contradicting hashed voxel buffer (`values` and `sha256_f32_le` unchanged): `PASS`
22. Reject mutated `center_of_mass_m` contradicting hashed voxel buffer (`values` and `sha256_f32_le` unchanged): `PASS`
23. Reject analytical surviving/decayed reference diverging from $M_0 \exp(-k_{\mathrm{decay}} t)$ despite algebraic closure (`M_surv + M_decay == M_0` and `gap == M_surv - M_ret`): `PASS`

---

## 3. Stage 1: Semantic and Frame-to-Receipt Test Suite

**Command:** `node tests/test_spacetime_4d_v0.js`
**Result:** `PASS` (`11/11` passed, `0` failed, exit code 0)

- **Test 1:** Fixed-step seconds time grid (`time_units: "s"`, uniform $\Delta t = 250.0\text{ s}$), separated `baseline_commit` vs `source_provenance`, `+k` up-axis frame, and strict `assert.deepStrictEqual` embedded bundle bit-parity -> `PASS`
- **Test 2:** `16x16x16` voxel dimensions, finite values, derived summary statistics (`min_value`, `max_value`, `mean_value`, `total_integrated_inventory_mol`, `center_of_mass_m`), and analytical decay reference checks ($M_0 e^{-k_{\mathrm{decay}} t}$ and $M_0 (1 - e^{-k_{\mathrm{decay}} t})$) -> `PASS`
- **Test 3:** Per-frame `Float32LE` SHA-256 checksums verified against Node crypto, independently pinned test manifest (`PINNED_EXPECTED_FRAME_DIGESTS_F32_LE`), and external receipt files (`spacetime_geochemistry_v0_verification.json` and `spacetime_geochemistry_v0_2026-10-20.md#state_k`) -> `PASS`
- **Test 4:** Changing time state mutates underlying `Float32Array`, independently recomputes 3D center of mass $[x_{\mathrm{cm}}, y_{\mathrm{cm}}, z_{\mathrm{cm}}]$ from each state's raw `Float32Array` buffer and `spatial_frame`, verifies monotonic upward motion along `+k`, and verifies agreement with `center_of_mass_m` -> `PASS`
- **Test 5:** Dual-time comparator $\Delta F(\mathbf{x}; t_A, t_B)$ zero-identity at $t_A = t_B$ and dipolar shift at $t_A \ne t_B$ -> `PASS`
- **Test 6:** Selected-voxel time series and criterial separation (`verified_accepted` vs `criterial_pruned` synthetic amplitude drift control) -> `PASS`
- **Test 7:** Missing or out-of-range frames and unauthorized fractional steps fail closed -> `PASS`
- **Test 8:** Display interpolation marks frame `"interpolated_for_display"` with `run_receipt: null` on fixed-step grids and rejects fractional interpolation on non-uniform time steps -> `PASS`
- **Test 9:** `verifyBundleIntegrity` and `selectTimeState` reject non-monotonic/non-uniform time steps, non-`"s"` `time_units`, `NaN`, bad SHA-256, calibrated claims, null `run_receipt`, bad `time_bounds`, bad `domain_bounds_m`, false Stage 1 finite-volume conservation claims, fabricated `0.0` quadrature gaps, conflated provenance, mismatched `criterial_pruned_branches` time points, mutated `min_value`/`max_value`/`mean_value`/`total_integrated_inventory_mol`/`center_of_mass_m`, and algebraically closed surviving/decayed references diverging from $M_0 \exp(-k_{\mathrm{decay}} t)$ -> `PASS`
- **Test 10:** 4D projection (`+k` up, $w = t$, extreme rotation angles), orthogonal slices, offline HTML, and zero em/en-dashes -> `PASS`
- **Test 11:** Interactive DOM harness (`?interp=1&state=1.5` policy/event banners, separated provenance readout, analytical bookkeeping readout, `Step -1`/`+1`, fail-closed invalid URL params, and PNG export header) -> `PASS`

---

## 4. Captured `1920x1080` Headless Verification Screenshots

| View Description | Path | Resolution | SHA-256 Digest | Status |
| :--- | :--- | :---: | :--- | :---: |
| State 0 ($t = 0.0\text{ s}$, lower zone $k = 5$) | `receipts/screenshots/r10_spacetime_t0.png` | `1920x1080` | `fdb1176b32771d93e64c9e7bfb90b8735be13c379388b558c911fc7eb533e306` | `PASS` |
| State 3 ($t = 750.0\text{ s}$, upper zone $k = 9$) | `receipts/screenshots/r10_spacetime_t750.png` | `1920x1080` | `3c5fea4079f6b7513848610ebe6243901bbf2ac05afd138c063589ba3a640009` | `PASS` |
| Dual-Time Comparison ($t_A = 0\text{ s}$ vs $t_B = 750\text{ s}$) | `receipts/screenshots/r10_spacetime_diff_t0_t750.png` | `1920x1080` | `1c94292ed8a3c277b8c16c371066b76dd7a8fec56ad12f271375659f69e1f766` | `PASS` |
| Display Interpolation Mode (`?interp=1&state=1.5`, $t = 375.0\text{ s}$) | `receipts/screenshots/r10_spacetime_interp_t375.png` | `1920x1080` | `90f12e415d879d0e6c1094a634374a88f197a02164d1934ddc30a64ccf4d97b1` | `PASS` |

---

## 5. Frozen v1 Rust Conformance and Typography Audit

- **Rust Conformance (`cargo test -p oklo_core --locked --test conformance`):** `7/7 PASS` (`b_sensitivity_to_gui_boundary_is_detectable`, `c_matches_recorded_snapshot`, `d_abi_round_trip`, `a_matches_python_export_at_recorded_boundary`, `f_series_frame_9_matches_fixture_and_cf3_header`, `g_temperature_isovalue_160_frames`, `e_step_keeps_phase_in_range`).
- **Text Audit (`bash scripts/text_audit.sh`):** `PASS` (`0` en dashes, `0` em dashes, `0` quarantined terms across the entire repository).
- **Branch Isolation:** Zero changes to `main`, PR #7 (`release/v1-materials`), PR #8 (`release/on-render-gate`), PR #9 (`planetary-explorer/v2-prototype`), or external research repositories (`aurascoper/Biofilms`, `aurascoper/rna_biofilm_toy`, `aurascoper/SobolevMacromolecule`).
