# Verification Receipt: 4D Spatiotemporal Geochemistry Explorer (`spacetime_geochemistry_v0`)

- **Document ID:** `receipts/spacetime_geochemistry_v0_2026-10-20.md`
- **Date:** 2026-10-20
- **Requirement:** R10 (`spacetime_geochemistry_v0` - Stage 0 4D Data Contract and Stage 1 Synthetic Temporal Renderer Proof)
- **Branch:** `spacetime-geochemistry/v0-slice`
- **Baseline Commit (`main`):** `636b7d7bb41999b80278d90046da893ee1044f20`
- **Tested Source Commit:** `91c66e2ab7590fa01465d16a2bd3b7b996b32498`
- **Worktree Clean at Test:** `true` (`worktree_clean_at_test: true`)
- **Commit Provenance Separation:** All Stage 0/1 validators, Node.js semantic tests, Rust conformance checks, text audits, and `1920x1080` headless browser screenshots were executed against the clean `tested_source_commit` (`91c66e2ab7590fa01465d16a2bd3b7b996b32498`). This receipt and the companion JSON receipt are committed in the immediately following `receipt_head` commit without modifying any tested source or data files. `tested_source_commit` and `receipt_head` are distinct and never treated as interchangeable.
- **Scientific Status Disclaimer:** Passing viewer and data-contract tests establishes rendering and data-contract correctness only, not geological validity, radiation transport accuracy, or empirical calibration.
- **Overall Disposition:** `PASS`

---

## 1. Runtime Environment

- **Operating System:** `Darwin 27.0.0 arm64` (macOS Apple Silicon)
- **Python Runtime:** `Python 3.15.0`
- **Node.js Runtime:** `v26.11.1`
- **Rust Toolchain:** `rustc 1.97.1 (8bab26f4f 2026-07-14)` / `cargo 1.97.1 (c980f4866 2026-06-30)`
- **Headless Browser:** `Google Chrome 155.0.8059.85` (`--headless --window-size=1920,1080 --force-device-scale-factor=1`)

---

## 2. Stage 0: 4D Spatiotemporal Data Contract Verification

**Command:** `python3 scripts/validate_spacetime_4d.py`
**Result:** `PASS` (1 primary bundle + 9/9 negative-control rejection checks passed, exit code 0)

### 2.1 Verified Discrete Time States (`data/spacetime/synthetic_tracer_pulse_4d_v0.json`)

| State ID Anchor | `state_index` | `simulation_time` (`s`) | `state_derivation` | `criterial_status` | Center of Mass $(x, y, z)\text{ m}$ | Peak Value (`mol/m^3`) | `sha256_f32_le` (IEEE-754 Little-Endian Float32) |
| :--- | :---: | :---: | :--- | :--- | :---: | :---: | :--- |
| <a id="state_0"></a>`#state_0` | 0 | `0.0` | `prescribed_analytically` | `verified_accepted` | `(8.0000, 8.0000, 4.5296)` | `9.2574` | `906e083f513e4d63d0e1ce2771439f15a4e1747c23f68469b5ad90d9e854290e` |
| <a id="state_1"></a>`#state_1` | 1 | `250.0` | `prescribed_analytically` | `verified_accepted` | `(8.3749, 7.7501, 6.1304)` | `6.8159` | `279ddf8ae892310376a20b573afc7fd13f359a09fe8832f9f8b703bd10d7fa30` |
| <a id="state_2"></a>`#state_2` | 2 | `500.0` | `prescribed_analytically` | `verified_accepted` | `(8.7495, 7.5003, 7.7505)` | `5.0323` | `8949b42e116ee75e40ffcf55c8524905ac62e31847b9e044ac004a1c53c1e7c9` |
| <a id="state_3"></a>`#state_3` | 3 | `750.0` | `prescribed_analytically` | `verified_accepted` | `(9.1235, 7.2509, 9.3643)` | `3.7364` | `1fcb13a289ea3592e79188536280d6c05400b5f5fe516262ddf91f06e7be88d2` |
| <a id="state_4"></a>`#state_4` | 4 | `1000.0` | `prescribed_analytically` | `verified_accepted` | `(9.4959, 7.0025, 10.8976)` | `2.7921` | `6cd8bc7c53a40702565a4d19904e6c240c48eeed5bafa1e3208bf3a598a3267f` |

### 2.2 Stage 0 Negative-Control Rejection Checks

1. Reject non-monotonic `simulation_time` ($t_2 = 200.0 \le t_1 = 250.0$): `PASS`
2. Reject empty/whitespace `temporal_axis.time_units`: `PASS`
3. Reject invalid `fourth_dimension_symbol != "t"`: `PASS`
4. Reject voxel array length mismatch (`4095 != 4096`): `PASS`
5. Reject non-finite voxel value (`NaN` at voxel 128): `PASS`
6. Reject corrupted `sha256_f32_le` frame digest: `PASS`
7. Reject `state_derivation="interpolated_for_display"` inside stored states under `none_discrete_states_only`: `PASS`
8. Reject unauthorized `evidence_class="empirically_calibrated_model"`: `PASS`
9. Reject missing intermediate frame (`state_index` gap under `fail_closed_no_invented_states`): `PASS`

---

## 3. Stage 1: Semantic and Frame-to-Receipt Test Suite

**Command:** `node tests/test_spacetime_4d_v0.js`
**Result:** `PASS` (`10/10` passed, `0` failed, exit code 0)

- **Test 1:** Temporal axis monotonicity, `+k` up-axis frame, and embedded bundle bit-parity -> `PASS`
- **Test 2:** `16x16x16` voxel dimensions, finite values, non-zero variance, and ledger closure (`< 1e-12`) -> `PASS`
- **Test 3:** Bit-exact `Float32LE` SHA-256 digests verified against Node crypto and mutually distinct -> `PASS`
- **Test 4:** Changing time state mutates underlying `Float32Array` and shifts $z_{\mathrm{cm}}$ monotonically along `+k` -> `PASS`
- **Test 5:** Dual-time comparator $\Delta F(\mathbf{x}; t_A, t_B)$ zero-identity at $t_A = t_B$ and dipolar shift at $t_A \ne t_B$ -> `PASS`
- **Test 6:** Selected-voxel time series and criterial separation (`verified_accepted` vs `criterial_pruned`) -> `PASS`
- **Test 7:** Missing or out-of-range frames and unauthorized fractional steps fail closed -> `PASS`
- **Test 8:** Display interpolation marks frame `"interpolated_for_display"` with `run_receipt: null` -> `PASS`
- **Test 9:** `verifyBundleIntegrity` rejects non-monotonic time, `NaN`, bad SHA-256, and calibrated claims -> `PASS`
- **Test 10:** 4D projection (`+k` up, $w = t$), orthogonal slices, offline HTML, and zero em/en-dashes -> `PASS`

---

## 4. Captured `1920x1080` Headless Verification Screenshots

| View Description | Path | Resolution | SHA-256 Digest | Status |
| :--- | :--- | :---: | :--- | :---: |
| State 0 ($t = 0.0\text{ s}$, lower zone $k = 5$) | `receipts/screenshots/r10_spacetime_t0.png` | `1920x1080` | `2c9d12b08598fad5a138ba075f22ded33c50f0ed5a10130d5383f2d309ea889b` | `PASS` |
| State 3 ($t = 750.0\text{ s}$, upper zone $k = 9$) | `receipts/screenshots/r10_spacetime_t750.png` | `1920x1080` | `8db67e60d2367509763414b48947121c1bc26e282221882218f14545d46da5a1` | `PASS` |
| Dual-Time Comparison ($t_A = 0\text{ s}$ vs $t_B = 750\text{ s}$) | `receipts/screenshots/r10_spacetime_diff_t0_t750.png` | `1920x1080` | `ad231fcc593e8fce2b2bb9006a1f02796e95c330809b97b074e8332eaeca1f9b` | `PASS` |

---

## 5. Frozen v1 Rust Conformance and Typography Audit

- **Rust Conformance (`cargo test -p oklo_core --locked --test conformance`):** `7/7 PASS` (`b_sensitivity_to_gui_boundary_is_detectable`, `c_matches_recorded_snapshot`, `d_abi_round_trip`, `a_matches_python_export_at_recorded_boundary`, `f_series_frame_9_matches_fixture_and_cf3_header`, `g_temperature_isovalue_160_frames`, `e_step_keeps_phase_in_range`).
- **Text Audit (`bash scripts/text_audit.sh`):** `PASS` (`0` en dashes, `0` em dashes, `0` quarantined terms across the entire repository).
- **Branch Isolation:** Zero changes to `main`, PR #7 (`release/v1-materials`), PR #8 (`release/on-render-gate`), PR #9 (`planetary-explorer/v2-prototype`), or external research repositories (`aurascoper/Biofilms`, `aurascoper/rna_biofilm_toy`, `aurascoper/SobolevMacromolecule`).
