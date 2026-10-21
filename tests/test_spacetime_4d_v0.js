#!/usr/bin/env node
/**
 * Stage 1 Semantic & Data-Contract Verification Suite (test_spacetime_4d_v0.js)
 *
 * Tests Requirement R10 (spacetime_geochemistry_v0) Stage 0 and Stage 1 invariants:
 * - Strict monotonic fixed-step seconds time grid (time_units = "s", dt = temporal_resolution_s)
 * - Separated baseline_commit vs R10 source_provenance (generator_script_sha256 verified on disk)
 * - Per-frame Float32LE SHA-256 checksum verification AND external receipt / pinned-manifest cross-check
 * - Illustrative analytical bookkeeping ledger (closes_by_algebraic_definition: true, is_finite_volume_conservation_test: false)
 * - Semantic proof that changing time mutates the underlying 3D Float32Array and shifts z_cm(+k)
 * - Dual-time comparison dF(x; t_A, t_B) identity and dipolar shift
 * - Selected-voxel time series and criterial separation (verified_accepted vs criterial_pruned)
 * - Missing-frame fail-closed policy (fail_closed_no_invented_states)
 * - Display interpolation explicit labeling ("interpolated_for_display", run_receipt: null)
 * - Negative-control rejection of corrupted/invalid bundles
 * - Zero em-dashes, zero en-dashes, zero external network dependencies
 */
"use strict";

const assert = require("assert");
const crypto = require("crypto");
const fs = require("fs");
const path = require("path");

const S4D = require("../viewer/js/spacetime_4d.js");

const REPO_ROOT = path.resolve(__dirname, "..");
const BUNDLE_PATH = path.join(REPO_ROOT, "data", "spacetime", "synthetic_tracer_pulse_4d_v0.json");
const SCHEMA_PATH = path.join(REPO_ROOT, "data", "spacetime", "schema_4d_v0.json");
const GENERATOR_PATH = path.join(REPO_ROOT, "scripts", "generate_synthetic_4d_bundle.py");
const RECEIPT_JSON_PATH = path.join(REPO_ROOT, "receipts", "spacetime_geochemistry_v0_verification.json");
const RECEIPT_MD_PATH = path.join(REPO_ROOT, "receipts", "spacetime_geochemistry_v0_2026-10-20.md");

// Independently pinned expected Float32LE SHA-256 frame digests for Stage 1 verification (Finding 3)
const PINNED_EXPECTED_FRAME_DIGESTS_F32_LE = [
  "906e083f513e4d63d0e1ce2771439f15a4e1747c23f68469b5ad90d9e854290e",
  "279ddf8ae892310376a20b573afc7fd13f359a09fe8832f9f8b703bd10d7fa30",
  "8949b42e116ee75e40ffcf55c8524905ac62e31847b9e044ac004a1c53c1e7c9",
  "1fcb13a289ea3592e79188536280d6c05400b5f5fe516262ddf91f06e7be88d2",
  "6cd8bc7c53a40702565a4d19904e6c240c48eeed5bafa1e3208bf3a598a3267f"
];

function nodeFloat32Sha256(values) {
  const f32 = new Float32Array(values.length);
  for (let i = 0; i < values.length; i++) {
    f32[i] = values[i];
  }
  const buf = Buffer.from(f32.buffer, f32.byteOffset, f32.byteLength);
  return crypto.createHash("sha256").update(buf).digest("hex");
}

function clone(obj) {
  return JSON.parse(JSON.stringify(obj));
}

function independentRecomputeVoxelSummary(valuesBuffer, spatialFrame) {
  const [nx, ny, nz] = spatialFrame.dimensions_voxels;
  const [xMin, xMax] = spatialFrame.domain_bounds_m.x;
  const [yMin, yMax] = spatialFrame.domain_bounds_m.y;
  const [zMin, zMax] = spatialFrame.domain_bounds_m.z_up;
  const dx = (xMax - xMin) / nx;
  const dy = (yMax - yMin) / ny;
  const dz = (zMax - zMin) / nz;
  const dV = dx * dy * dz;

  let minV = Infinity;
  let maxV = -Infinity;
  let sumV = 0.0;
  let sx = 0.0;
  let sy = 0.0;
  let sz = 0.0;

  for (let k = 0; k < nz; k++) {
    const z = zMin + (k + 0.5) * dz;
    for (let j = 0; j < ny; j++) {
      const y = yMin + (j + 0.5) * dy;
      for (let i = 0; i < nx; i++) {
        const x = xMin + (i + 0.5) * dx;
        const v = valuesBuffer[i + nx * (j + ny * k)];
        if (v < minV) minV = v;
        if (v > maxV) maxV = v;
        sumV += v;
        sx += x * v;
        sy += y * v;
        sz += z * v;
      }
    }
  }

  return {
    min_value: minV,
    max_value: maxV,
    mean_value: sumV / (nx * ny * nz),
    total_integrated_inventory_mol: sumV * dV,
    center_of_mass_m: [sx / sumV, sy / sumV, sz / sumV]
  };
}

function runSuite() {
  const diskBundle = JSON.parse(fs.readFileSync(BUNDLE_PATH, "utf8"));
  const schema = JSON.parse(fs.readFileSync(SCHEMA_PATH, "utf8"));
  assert.strictEqual(schema.title, "SpacetimeFieldBundleV0");

  const results = [];
  function test(name, fn) {
    try {
      fn();
      console.log("[PASS] " + name);
      results.push({ name, passed: true });
    } catch (err) {
      console.error("[FAIL] " + name + " -> " + err.message);
      results.push({ name, passed: false, error: err.message });
    }
  }

  // Test 1: Temporal axis fixed-step seconds grid, provenance separation, +k up-axis frame, and embedded bundle parity
  test("Test 1: Fixed-step seconds time grid, separated baseline/generator provenance, +k up-axis, and embedded bundle parity", () => {
    const check = S4D.verifyBundleIntegrity(diskBundle);
    assert.strictEqual(check.valid, true);
    assert.strictEqual(check.state_count, 5);
    assert.strictEqual(check.voxel_count, 4096);

    assert.strictEqual(diskBundle.temporal_axis.fourth_dimension_symbol, "t");
    assert.strictEqual(diskBundle.temporal_axis.time_units, "s");
    assert.strictEqual(diskBundle.temporal_axis.temporal_resolution_s, 250.0);
    assert.strictEqual(diskBundle.spatial_frame.up_axis, "+k");
    assert.strictEqual(diskBundle.spatial_frame.geometry_provenance, "schematic");

    // Finding 1: verify baseline_commit is separated from R10 source_provenance and generator SHA-256 matches script on disk
    assert.strictEqual(diskBundle.baseline_commit, "636b7d7bb41999b80278d90046da893ee1044f20");
    assert.strictEqual(diskBundle.source_provenance.baseline_commit, diskBundle.baseline_commit);
    assert.strictEqual(diskBundle.source_provenance.target_branch, "spacetime-geochemistry/v0-slice");
    assert.notStrictEqual(diskBundle.source_provenance.generator_version, diskBundle.baseline_commit);
    const genScriptSha = crypto.createHash("sha256").update(fs.readFileSync(GENERATOR_PATH)).digest("hex");
    assert.strictEqual(
      diskBundle.source_provenance.generator_script_sha256,
      genScriptSha,
      "source_provenance.generator_script_sha256 must match scripts/generate_synthetic_4d_bundle.py on disk"
    );

    const times = diskBundle.states.map((s) => s.simulation_time);
    assert.deepStrictEqual(times, [0.0, 250.0, 500.0, 750.0, 1000.0]);
    for (let i = 1; i < times.length; i++) {
      assert(times[i] > times[i - 1], "Time steps must be strictly increasing");
      assert(
        Math.abs((times[i] - times[i - 1]) - diskBundle.temporal_axis.temporal_resolution_s) < 1e-6,
        "Adjacent time steps must match temporal_axis.temporal_resolution_s"
      );
    }

    const embedded = S4D.buildEmbeddedSyntheticBundle();
    S4D.verifyBundleIntegrity(embedded);
    assert.deepStrictEqual(
      embedded,
      diskBundle,
      "Embedded synthetic bundle must be 100% deep-strict-equal to disk JSON bundle"
    );
  });

  // Test 2: 3D field dimensions, finite values, non-zero variance, derived summary agreement, and analytical decay reference checks
  test("Test 2: 16x16x16 voxel dimensions, finite values, derived summary statistics, and analytical decay reference checks", () => {
    assert.deepStrictEqual(diskBundle.spatial_frame.dimensions_voxels, [16, 16, 16]);
    const kDecay = diskBundle.governing_parameters.decay_rate_s_inv;
    const m0 = independentRecomputeVoxelSummary(diskBundle.states[0].values, diskBundle.spatial_frame)
      .total_integrated_inventory_mol;
    for (let s = 0; s < diskBundle.states.length; s++) {
      const st = diskBundle.states[s];
      assert.strictEqual(st.voxel_count, 4096);
      assert.strictEqual(st.values.length, 4096);
      for (let i = 0; i < st.values.length; i++) {
        const v = st.values[i];
        assert(Number.isFinite(v), "Voxel value must be finite");
        assert(v >= 0.0, "Synthetic concentration must be non-negative");
      }
      const recomputed = independentRecomputeVoxelSummary(st.values, diskBundle.spatial_frame);
      assert(recomputed.max_value - recomputed.min_value > 2.0, "State " + s + " must have non-trivial spatial variance");
      assert(Math.abs(st.min_value - recomputed.min_value) < 1e-6, "min_value must match voxel buffer");
      assert(Math.abs(st.max_value - recomputed.max_value) < 1e-6, "max_value must match voxel buffer");
      assert(Math.abs(st.mean_value - recomputed.mean_value) < 1e-6, "mean_value must match voxel buffer");
      assert(
        Math.abs(st.total_integrated_inventory_mol - recomputed.total_integrated_inventory_mol) < 1e-4,
        "total_integrated_inventory_mol must match integrated voxel buffer"
      );
      for (let c = 0; c < 3; c++) {
        assert(
          Math.abs(st.center_of_mass_m[c] - recomputed.center_of_mass_m[c]) < 1e-5,
          "center_of_mass_m[" + c + "] must match voxel buffer"
        );
      }

      // Finding 2 & derived reference check: Verify illustrative analytical bookkeeping ledger against M0*exp(-k_decay*t)
      const ledger = st.analytical_bookkeeping_ledger;
      assert(ledger && typeof ledger === "object", "State " + s + " must include analytical_bookkeeping_ledger");
      assert.strictEqual(ledger.closes_by_algebraic_definition, true);
      assert.strictEqual(ledger.is_finite_volume_conservation_test, false);
      assert(ledger.analytical_bookkeeping_note.includes("not test finite-volume numerical conservation"));

      const expectedSurviving = m0 * Math.exp(-kDecay * st.simulation_time);
      const expectedDecayed = m0 * (1.0 - Math.exp(-kDecay * st.simulation_time));
      const expectedGap = expectedSurviving - recomputed.total_integrated_inventory_mol;
      assert(
        Math.abs(ledger.analytical_surviving_reference_mol - expectedSurviving) < 1e-4,
        "State " + s + " analytical_surviving_reference_mol must equal M0*exp(-k_decay*t)"
      );
      assert(
        Math.abs(ledger.analytical_decayed_reference_mol - expectedDecayed) < 1e-4,
        "State " + s + " analytical_decayed_reference_mol must equal M0*(1 - exp(-k_decay*t))"
      );
      assert(
        Math.abs(ledger.unmodeled_boundary_and_quadrature_gap_mol - expectedGap) < 1e-4,
        "State " + s + " unmodeled_boundary_and_quadrature_gap_mol must equal M0*exp(-k*t) - M_retained"
      );
      if (s > 0) {
        assert(
          ledger.relative_quadrature_and_tail_gap > 0.0 && ledger.relative_quadrature_and_tail_gap < 0.02,
          "State " + s + " must record a realistic non-zero discrete quadrature/boundary tail gap"
        );
      }
    }
  });

  // Test 3: Per-frame Float32LE SHA-256 checksums AND independent external receipt / pinned manifest verification (Finding 3)
  test("Test 3: Per-frame Float32LE checksums verified against Node crypto, pinned manifest, and external receipt files", () => {
    const receiptJson = JSON.parse(fs.readFileSync(RECEIPT_JSON_PATH, "utf8"));
    const receiptMd = fs.readFileSync(RECEIPT_MD_PATH, "utf8");
    assert.strictEqual(receiptJson.state_frame_digests_f32_le.length, 5);

    const hashes = new Set();
    for (let s = 0; s < diskBundle.states.length; s++) {
      const st = diskBundle.states[s];
      const jsSha = S4D.sha256Float32LE(st.values);
      const nodeSha = nodeFloat32Sha256(st.values);
      assert.strictEqual(jsSha, nodeSha, "Pure-JS SHA-256 must match Node crypto SHA-256");
      assert.strictEqual(jsSha, st.sha256_f32_le, "Recomputed SHA-256 must match bundle metadata checksum");
      assert.strictEqual(
        jsSha,
        PINNED_EXPECTED_FRAME_DIGESTS_F32_LE[s],
        "Frame digest must match independently pinned test manifest at state " + s
      );
      assert.strictEqual(
        jsSha,
        receiptJson.state_frame_digests_f32_le[s].sha256_f32_le,
        "Frame digest must match external verification receipt JSON record at state " + s
      );
      const anchor = "#state_" + s;
      assert(st.run_receipt.endsWith(anchor), "State run_receipt must reference " + anchor);
      assert(
        receiptMd.includes('id="state_' + s + '"') && receiptMd.includes(jsSha),
        "External Markdown receipt must contain anchor #state_" + s + " and frame digest " + jsSha
      );
      hashes.add(jsSha);
    }
    assert.strictEqual(hashes.size, 5, "All 5 time states must have distinct SHA-256 digests");
  });

  // Test 4: Semantic time selection proof (changing time changes underlying Float32Array & shifts independently recomputed z_cm upward)
  test("Test 4: Changing time state mutates underlying Float32Array and shifts independently recomputed z_cm monotonically along +k", () => {
    const sel0 = S4D.selectTimeState(diskBundle, 0, { allowDisplayInterpolation: false });
    const sel2 = S4D.selectTimeState(diskBundle, 2, { allowDisplayInterpolation: false });
    const sel4 = S4D.selectTimeState(diskBundle, 4, { allowDisplayInterpolation: false });

    assert(sel0.float32Values instanceof Float32Array);
    assert(sel2.float32Values instanceof Float32Array);
    assert(sel4.float32Values instanceof Float32Array);

    assert.notStrictEqual(sel0.sha256_f32_le, sel2.sha256_f32_le);
    assert.notStrictEqual(sel2.sha256_f32_le, sel4.sha256_f32_le);

    // Independently recompute 3D center of mass and summary statistics from each state's raw Float32Array buffer
    let prevRecomputedZ = -Infinity;
    let prevRecomputedMax = Infinity;
    for (let s = 0; s < 5; s++) {
      const sel = S4D.selectTimeState(diskBundle, s, { allowDisplayInterpolation: false });
      assert.strictEqual(sel.is_simulated_state, true);
      assert.strictEqual(sel.state_derivation, "prescribed_analytically");
      assert.strictEqual(sel.criterial_status, "verified_accepted");

      const recomputed = independentRecomputeVoxelSummary(sel.float32Values, diskBundle.spatial_frame);
      for (let c = 0; c < 3; c++) {
        assert(
          Math.abs(sel.center_of_mass_m[c] - recomputed.center_of_mass_m[c]) < 1e-5,
          "Stored center_of_mass_m[" + c + "] at state " + s + " must match center of mass recomputed from Float32Array"
        );
      }
      assert(Math.abs(sel.min_value - recomputed.min_value) < 1e-6);
      assert(Math.abs(sel.max_value - recomputed.max_value) < 1e-6);
      assert(Math.abs(sel.mean_value - recomputed.mean_value) < 1e-6);
      assert(Math.abs(sel.total_integrated_inventory_mol - recomputed.total_integrated_inventory_mol) < 1e-4);

      const zCmRecomputed = recomputed.center_of_mass_m[2];
      assert(
        zCmRecomputed > prevRecomputedZ,
        "Independently recomputed vertical center of mass z_cm must strictly increase along +k"
      );
      assert(
        recomputed.max_value < prevRecomputedMax,
        "Independently recomputed peak concentration must attenuate under diffusion and decay"
      );
      prevRecomputedZ = zCmRecomputed;
      prevRecomputedMax = recomputed.max_value;
    }

    // Verify specific voxel (8, 8, 5) vs (8, 8, 11) change in opposite directions over time
    const idxLower = S4D.voxelIndex(8, 8, 5, 16, 16);
    const idxUpper = S4D.voxelIndex(8, 8, 11, 16, 16);
    assert(sel0.float32Values[idxLower] > sel4.float32Values[idxLower]);
    assert(sel4.float32Values[idxUpper] > sel0.float32Values[idxUpper]);
  });

  // Test 5: Dual-time comparison dF(x; t_A, t_B) identity and dipolar shift
  test("Test 5: Dual-time comparator zero-identity at t_A=t_B and dipolar shift at t_A!=t_B", () => {
    const sameDiff = S4D.computeDualTimeComparison(diskBundle, 2, 2);
    assert.strictEqual(sameDiff.l2_norm, 0.0);
    assert.strictEqual(sameDiff.linf_norm, 0.0);
    assert.deepStrictEqual(sameDiff.center_of_mass_shift_m, [0, 0, 0]);

    const shiftDiff = S4D.computeDualTimeComparison(diskBundle, 0, 3);
    assert.strictEqual(shiftDiff.delta_time_s, 750.0);
    assert(shiftDiff.l2_norm > 50.0, "Expected substantial L2 difference between t=0s and t=750s");
    assert(shiftDiff.min_diff < -8.0, "Expected strong negative depletion at initial pulse center");
    assert(shiftDiff.max_diff > 3.0, "Expected positive accumulation in upper zone");
    assert(shiftDiff.center_of_mass_shift_m[2] > 4.5, "Expected >4.5m upward center-of-mass shift");
  });

  // Test 6: Selected-voxel time series and criterial separation (verified_accepted vs criterial_pruned)
  test("Test 6: Selected-voxel time series and criterial separation (verified_accepted vs criterial_pruned)", () => {
    const tsLower = S4D.extractVoxelTimeSeries(diskBundle, 8, 8, 5);
    const tsUpper = S4D.extractVoxelTimeSeries(diskBundle, 8, 8, 11);

    assert.strictEqual(tsLower.verified_series.length, 5);
    assert.strictEqual(tsUpper.verified_series.length, 5);
    assert(tsLower.verified_series[0].value > tsLower.verified_series[4].value);
    assert(tsUpper.verified_series[4].value > tsUpper.verified_series[0].value);

    for (const pt of tsLower.verified_series) {
      assert.strictEqual(pt.criterial_status, "verified_accepted");
    }

    assert.strictEqual(tsLower.criterial_pruned_series.length, 1);
    const prunedBranch = tsLower.criterial_pruned_series[0];
    assert.strictEqual(prunedBranch.branch_id, "synthetic_amplitude_drift_control");
    assert.strictEqual(prunedBranch.criterial_status, "criterial_pruned");
    assert.strictEqual(prunedBranch.max_relative_analytical_drift, 0.38);
    assert(prunedBranch.prune_reason.includes("not finite-volume PDE conservation"));
    assert.strictEqual(prunedBranch.series.length, 5);
    assert(
      Math.abs(prunedBranch.series[4].value - tsLower.verified_series[4].value * 1.38) < 1e-6,
      "Pruned branch at t=1000s must reflect +38% synthetic amplitude drift"
    );

    // Non-probe voxel (6, 7, 9) must derive pruned value from recorded tp.relative_analytical_drift
    const customDriftBundle = clone(diskBundle);
    customDriftBundle.criterial_pruned_branches[0].time_points[4].relative_analytical_drift = 0.21;
    const tsNonProbe = S4D.extractVoxelTimeSeries(customDriftBundle, 6, 7, 9);
    assert(
      Math.abs(
        tsNonProbe.criterial_pruned_series[0].series[4].value -
          tsNonProbe.verified_series[4].value * 1.21
      ) < 1e-6,
      "Non-probe voxel pruned series must use recorded tp.relative_analytical_drift dynamically"
    );

    // Out-of-bounds voxel coordinates must throw
    assert.throws(() => S4D.extractVoxelTimeSeries(diskBundle, -1, 8, 5), /out of bounds/);
    assert.throws(() => S4D.extractVoxelTimeSeries(diskBundle, 8, 16, 5), /out of bounds/);
  });

  // Test 7: Missing-frame fail-closed behavior (fail_closed_no_invented_states)
  test("Test 7: Missing or out-of-range frames and unauthorized fractional steps fail closed", () => {
    assert.throws(
      () => S4D.selectTimeState(diskBundle, -1, { allowDisplayInterpolation: false }),
      /out of bounds/
    );
    assert.throws(
      () => S4D.selectTimeState(diskBundle, 5, { allowDisplayInterpolation: false }),
      /out of bounds/
    );
    assert.throws(
      () => S4D.selectTimeState(diskBundle, NaN, { allowDisplayInterpolation: false }),
      /finite number/
    );
    assert.throws(
      () => S4D.selectTimeState(diskBundle, 1.5, { allowDisplayInterpolation: false }),
      /none_discrete_states_only/
    );

    const gappedBundle = clone(diskBundle);
    gappedBundle.states[2].state_index = 99;
    assert.throws(
      () => S4D.selectTimeState(gappedBundle, 2, { allowDisplayInterpolation: false }),
      /Missing discrete state/
    );
  });

  // Test 8: Display interpolation explicit labeling ("interpolated_for_display") and fixed-step grid consistency
  test("Test 8: Display interpolation marks frame 'interpolated_for_display' with run_receipt: null on fixed-step grids", () => {
    const interpSel = S4D.selectTimeState(diskBundle, 1.5, { allowDisplayInterpolation: true });
    assert.strictEqual(interpSel.state_derivation, "interpolated_for_display");
    assert.strictEqual(interpSel.is_simulated_state, false);
    assert.strictEqual(interpSel.run_receipt, null);
    assert.strictEqual(interpSel.analytical_bookkeeping_ledger, null);
    assert.strictEqual(interpSel.sha256_f32_le, "unverified_display_interpolation");
    assert.strictEqual(interpSel.simulation_time, 375.0);
    assert(interpSel.display_warning && interpSel.display_warning.includes("INTERPOLATED FOR DISPLAY ONLY"));

    const st1 = S4D.selectTimeState(diskBundle, 1, { allowDisplayInterpolation: false });
    const st2 = S4D.selectTimeState(diskBundle, 2, { allowDisplayInterpolation: false });
    const probeIdx = S4D.voxelIndex(8, 8, 7, 16, 16);
    const expectedMid = Math.fround(0.5 * st1.float32Values[probeIdx] + 0.5 * st2.float32Values[probeIdx]);
    assert(Math.abs(interpSel.float32Values[probeIdx] - expectedMid) < 1e-6);

    // Finding 4: Non-uniform dt bundle (e.g., t = [0, 100, 250, 750, 1000] s with temporal_resolution_s = 250.0) must be rejected by both verifyBundleIntegrity and selectTimeState
    const nonUniformBundle = clone(diskBundle);
    const nonUniformTimes = [0.0, 100.0, 250.0, 750.0, 1000.0];
    for (let i = 0; i < 5; i++) {
      nonUniformBundle.states[i].simulation_time = nonUniformTimes[i];
      nonUniformBundle.temporal_axis.event_markers[i].simulation_time = nonUniformTimes[i];
    }
    assert.throws(
      () => S4D.verifyBundleIntegrity(nonUniformBundle),
      /Inconsistent time step/
    );
    assert.throws(
      () => S4D.selectTimeState(nonUniformBundle, 1.5, { allowDisplayInterpolation: true }),
      /Inconsistent time step interval for display interpolation/
    );
  });

  // Test 9: Negative-control rejection of corrupted/invalid bundles
  test("Test 9: verifyBundleIntegrity and selectTimeState reject corrupted time, provenance, voxel summaries (min/max/mean/inventory/CoM), and decay references", () => {
    // 9a. Non-monotonic time
    const bNonMono = clone(diskBundle);
    bNonMono.states[3].simulation_time = 400.0;
    assert.throws(() => S4D.verifyBundleIntegrity(bNonMono), /Non-monotonic simulation_time/);

    // 9b. Non-'s' time_units (Finding 4)
    const bBadUnits = clone(diskBundle);
    bBadUnits.temporal_axis.time_units = "minutes";
    assert.throws(() => S4D.verifyBundleIntegrity(bBadUnits), /time_units must equal 's'/);

    // 9c. Invalid 4th dimension symbol
    const bBad4D = clone(diskBundle);
    bBad4D.temporal_axis.fourth_dimension_symbol = "w_distortion";
    assert.throws(() => S4D.verifyBundleIntegrity(bBad4D), /fourth_dimension_symbol/);

    // 9d. Voxel length mismatch
    const bShortVoxels = clone(diskBundle);
    bShortVoxels.states[0].values.pop();
    assert.throws(() => S4D.verifyBundleIntegrity(bShortVoxels), /values length mismatch/);

    // 9e. Non-finite voxel value
    const bNan = clone(diskBundle);
    bNan.states[1].values[42] = NaN;
    assert.throws(() => S4D.verifyBundleIntegrity(bNan), /Non-finite voxel/);

    // 9f. Corrupted SHA-256 digest
    const bBadSha = clone(diskBundle);
    bBadSha.states[4].sha256_f32_le = "f".repeat(64);
    assert.throws(() => S4D.verifyBundleIntegrity(bBadSha), /SHA-256 checksum mismatch/);

    // 9g. Unauthorized empirically_calibrated_model claim
    const bCalib = clone(diskBundle);
    bCalib.evidence_class = "empirically_calibrated_model";
    assert.throws(() => S4D.verifyBundleIntegrity(bCalib), /empirically_calibrated_model/);

    // 9h. Stored interpolated_for_display under none_discrete_states_only
    const bStoredInterp = clone(diskBundle);
    bStoredInterp.states[2].state_derivation = "interpolated_for_display";
    assert.throws(() => S4D.verifyBundleIntegrity(bStoredInterp), /interpolated_for_display/);

    // 9i. Discrete state missing run_receipt
    const bMissingReceipt = clone(diskBundle);
    bMissingReceipt.states[1].run_receipt = null;
    assert.throws(() => S4D.verifyBundleIntegrity(bMissingReceipt), /non-empty run_receipt/);

    // 9j. Mismatched temporal_axis.time_bounds
    const bBadTimeBounds = clone(diskBundle);
    bBadTimeBounds.temporal_axis.time_bounds = [0.0, 500.0];
    assert.throws(() => S4D.verifyBundleIntegrity(bBadTimeBounds), /time_bounds/);

    // 9k. Inverted spatial_frame.domain_bounds_m
    const bBadDomainBounds = clone(diskBundle);
    bBadDomainBounds.spatial_frame.domain_bounds_m.x = [16.0, 0.0];
    assert.throws(() => S4D.verifyBundleIntegrity(bBadDomainBounds), /domain_bounds_m/);

    // 9l. False Stage 1 finite-volume conservation claim (Finding 2)
    const bFalseConservation = clone(diskBundle);
    bFalseConservation.states[0].analytical_bookkeeping_ledger.is_finite_volume_conservation_test = true;
    assert.throws(() => S4D.verifyBundleIntegrity(bFalseConservation), /is_finite_volume_conservation_test must be false/);

    // 9m. Fabricated zero quadrature/tail gap contradicting discrete voxel sum (Finding 2)
    const bFakeZeroGap = clone(diskBundle);
    bFakeZeroGap.states[1].analytical_bookkeeping_ledger.unmodeled_boundary_and_quadrature_gap_mol = 0.0;
    bFakeZeroGap.states[1].analytical_bookkeeping_ledger.relative_quadrature_and_tail_gap = 0.0;
    assert.throws(() => S4D.verifyBundleIntegrity(bFakeZeroGap), /contradicts M_surv - M_retained/);

    // 9n. Conflated baseline_commit or short prefix in source_provenance / v2_compatibility (Finding 1)
    const bConflatedProv = clone(diskBundle);
    bConflatedProv.source_provenance.generator_version = bConflatedProv.baseline_commit.slice(0, 7);
    assert.throws(() => S4D.verifyBundleIntegrity(bConflatedProv), /generator_version must not conflate/);

    const bBadV2 = clone(diskBundle);
    bBadV2.v2_compatibility.epistemic_record.source_revision = bBadV2.baseline_commit;
    assert.throws(() => S4D.verifyBundleIntegrity(bBadV2), /source_revision must be distinct/);

    // 9o. Mismatched criterial_pruned_branches time_points length
    const bBadPruned = clone(diskBundle);
    bBadPruned.criterial_pruned_branches[0].time_points.pop();
    assert.throws(() => S4D.verifyBundleIntegrity(bBadPruned), /time_points length must match/);

    // 9p. Mutated min_value without changing values or sha256_f32_le
    const bBadMin = clone(diskBundle);
    bBadMin.states[1].min_value = 0.25;
    assert.throws(() => S4D.verifyBundleIntegrity(bBadMin), /min_value .* does not match recomputed voxel minimum/);
    assert.throws(() => S4D.selectTimeState(bBadMin, 1, { allowDisplayInterpolation: false }), /min_value mismatch/);

    // 9q. Mutated max_value without changing values or sha256_f32_le
    const bBadMax = clone(diskBundle);
    bBadMax.states[2].max_value = 9.99;
    assert.throws(() => S4D.verifyBundleIntegrity(bBadMax), /max_value .* does not match recomputed voxel maximum/);
    assert.throws(() => S4D.selectTimeState(bBadMax, 2, { allowDisplayInterpolation: false }), /max_value mismatch/);

    // 9r. Mutated mean_value without changing values or sha256_f32_le
    const bBadMean = clone(diskBundle);
    bBadMean.states[3].mean_value = 0.75;
    assert.throws(() => S4D.verifyBundleIntegrity(bBadMean), /mean_value .* does not match recomputed voxel mean/);
    assert.throws(() => S4D.selectTimeState(bBadMean, 3, { allowDisplayInterpolation: false }), /mean_value mismatch/);

    // 9s. Mutated total_integrated_inventory_mol without changing values or sha256_f32_le
    const bBadInventory = clone(diskBundle);
    bBadInventory.states[1].total_integrated_inventory_mol = 500.0;
    assert.throws(
      () => S4D.verifyBundleIntegrity(bBadInventory),
      /total_integrated_inventory_mol .* does not match recomputed integrated voxel inventory/
    );
    assert.throws(
      () => S4D.selectTimeState(bBadInventory, 1, { allowDisplayInterpolation: false }),
      /total_integrated_inventory_mol mismatch/
    );

    // 9t. Mutated center_of_mass_m without changing values or sha256_f32_le
    const bBadCom = clone(diskBundle);
    bBadCom.states[4].center_of_mass_m = [8.0, 8.0, 2.5];
    assert.throws(
      () => S4D.verifyBundleIntegrity(bBadCom),
      /center_of_mass_m\[0\] .* does not match recomputed voxel center of mass/
    );
    assert.throws(
      () => S4D.selectTimeState(bBadCom, 4, { allowDisplayInterpolation: false }),
      /center_of_mass_m mismatch/
    );

    // 9u. Mutated analytical surviving/decayed reference diverging from M0*exp(-k_decay*t) while keeping algebraic closure
    const bBadDecayRef = clone(diskBundle);
    const led2 = bBadDecayRef.states[2].analytical_bookkeeping_ledger;
    const m0 = led2.initial_discrete_inventory_mol;
    const ret = led2.retained_discrete_inventory_mol;
    const fakeSurv = Number((m0 * 0.95).toFixed(6));
    led2.analytical_surviving_reference_mol = fakeSurv;
    led2.analytical_decayed_reference_mol = Number((m0 - fakeSurv).toFixed(6));
    led2.unmodeled_boundary_and_quadrature_gap_mol = Number((fakeSurv - ret).toFixed(6));
    led2.relative_quadrature_and_tail_gap = Number((Math.abs(fakeSurv - ret) / m0).toFixed(6));
    assert.throws(
      () => S4D.verifyBundleIntegrity(bBadDecayRef),
      /analytical_surviving_reference_mol .* does not match M0 \* exp\(-k_decay \* simulation_time\)/
    );
  });

  // Test 10: 4D projection (including extreme hyperplane angles), orthogonal slices, offline HTML, and typography audit
  test("Test 10: 4D projection (+k up, w=t, extreme angles), orthogonal slices, offline HTML, and zero em/en-dashes", () => {
    const pBottom = S4D.project4DSpatiotemporalPoint(8, 8, 2, 0.0, { angleXT: 0, angleZT: 0, yaw: 0, pitch: 0 });
    const pTop = S4D.project4DSpatiotemporalPoint(8, 8, 14, 0.0, { angleXT: 0, angleZT: 0, yaw: 0, pitch: 0 });
    assert(pTop.screenY > pBottom.screenY, "+k upward coordinate must project to higher vertical screenY");
    assert.strictEqual(pTop.fourthDimensionSymbol, "t");

    const pT0 = S4D.project4DSpatiotemporalPoint(8, 8, 8, 0.0, { angleXT: 0.45, angleZT: 0.25 });
    const pT1 = S4D.project4DSpatiotemporalPoint(8, 8, 8, 1.0, { angleXT: 0.45, angleZT: 0.25 });
    assert.notStrictEqual(pT0.screenX, pT1.screenX, "4D X-t hyperplane rotation must separate t=0 and t=1");

    // Extreme hyperplane rotation angles (-1.2, +1.2 rad)
    for (const [xt, zt] of [[-1.2, 1.2], [1.2, -1.2]]) {
      for (const [x, y, z, tN] of [[0, 0, 0, 0], [16, 16, 16, 1], [8, 8, 9.5, 0.75]]) {
        const pExt = S4D.project4DSpatiotemporalPoint(x, y, z, tN, { angleXT: xt, angleZT: zt });
        assert(Number.isFinite(pExt.screenX) && Number.isFinite(pExt.screenY) && Number.isFinite(pExt.depth));
        assert(pExt.scale4D > 0.5 && pExt.scale4D < 3.0, "4D perspective scale must remain bounded and positive");
      }
    }

    const sel0 = S4D.selectTimeState(diskBundle, 0, { allowDisplayInterpolation: false });
    const sliceXY = S4D.extractOrthogonalSlice(sel0.float32Values, [16, 16, 16], "XY", 5);
    const sliceXZ = S4D.extractOrthogonalSlice(sel0.float32Values, [16, 16, 16], "XZ", 8);
    const sliceYZ = S4D.extractOrthogonalSlice(sel0.float32Values, [16, 16, 16], "YZ", 8);
    assert.strictEqual(sliceXY.data.length, 256);
    assert.strictEqual(sliceXZ.data.length, 256);
    assert.strictEqual(sliceYZ.data.length, 256);

    const filesToAudit = [
      "docs/specs/SPACETIME_GEOCHEMISTRY_v0.md",
      "data/spacetime/schema_4d_v0.json",
      "data/spacetime/synthetic_tracer_pulse_4d_v0.json",
      "scripts/generate_synthetic_4d_bundle.py",
      "scripts/validate_spacetime_4d.py",
      "viewer/js/spacetime_4d.js",
      "viewer/spacetime_explorer_4d.html",
      "tests/test_spacetime_4d_v0.js"
    ];

    const q1 = "radiotrophic" + " methanogens";
    const q2 = "free-will" + " octane";

    for (const rel of filesToAudit) {
      const text = fs.readFileSync(path.join(REPO_ROOT, rel), "utf8");
      assert(!text.includes("\u2013"), "En-dash found in " + rel);
      assert(!text.includes("\u2014"), "Em-dash found in " + rel);
      assert(!text.toLowerCase().includes(q1), "Quarantined term 1 found in " + rel);
      assert(!text.toLowerCase().includes(q2), "Quarantined term 2 found in " + rel);
    }

    const htmlText = fs.readFileSync(path.join(REPO_ROOT, "viewer/spacetime_explorer_4d.html"), "utf8");
    assert(!/<script[^>]+src=["']https?:/i.test(htmlText), "HTML must not load external HTTP/HTTPS scripts");
    assert(!/<link[^>]+href=["']https?:/i.test(htmlText), "HTML must not load external HTTP/HTTPS stylesheets/fonts");
  });

  // Test 11: Interactive DOM harness test for viewer/spacetime_explorer_4d.html
  test("Test 11: Interactive DOM harness verifies ?interp=1&state=1.5, separated provenance, analytical bookkeeping readout, Step -1/+1, and Export PNG labels", () => {
    const vm = require("vm");
    const htmlText = fs.readFileSync(path.join(REPO_ROOT, "viewer/spacetime_explorer_4d.html"), "utf8");
    const inlineScripts = [...htmlText.matchAll(/<script>([\s\S]*?)<\/script>/g)];
    assert.strictEqual(inlineScripts.length, 1, "Expected 1 inline controller script in spacetime_explorer_4d.html");
    const controllerCode = inlineScripts[0][1];

    function runViewerInDom(queryStr) {
      const elements = {};
      const stateBtns = [0, 1, 2, 3, 4].map((idx) => {
        const btn = {
          className: "btn",
          listeners: {},
          getAttribute: (attr) => (attr === "data-state-btn" ? String(idx) : null),
          addEventListener: (ev, fn) => {
            btn.listeners[ev] = fn;
          }
        };
        return btn;
      });
      const exportedCanvases = [];
      const downloadLinks = [];

      function makeElem(id) {
        if (!elements[id]) {
          elements[id] = {
            id,
            width: 560,
            height: 390,
            value: "0",
            step: "1",
            max: "4",
            checked: false,
            textContent: "",
            innerHTML: "",
            className: "",
            style: {},
            listeners: {},
            addEventListener(ev, fn) {
              this.listeners[ev] = fn;
            },
            trigger(ev, eventObj) {
              if (this.listeners[ev]) {
                this.listeners[ev](eventObj || { currentTarget: this });
              }
            },
            getContext() {
              const calls = [];
              this._ctxCalls = calls;
              return {
                fillStyle: "",
                strokeStyle: "",
                lineWidth: 1,
                font: "",
                fillRect: () => {},
                strokeRect: () => {},
                beginPath: () => {},
                moveTo: () => {},
                lineTo: () => {},
                stroke: () => {},
                fill: () => {},
                arc: () => {},
                setLineDash: () => {},
                save: () => {},
                restore: () => {},
                translate: () => {},
                rotate: () => {},
                drawImage: () => {},
                fillText(txt, x, y) {
                  calls.push({ text: String(txt), fillStyle: this.fillStyle, x, y });
                }
              };
            },
            toDataURL() {
              return "data:image/png;base64,SYNTHETIC";
            },
            click() {
              downloadLinks.push({ download: this.download, href: this.href });
            }
          };
        }
        return elements[id];
      }

      const sandbox = {
        window: {
          Spacetime4D: S4D,
          location: { search: queryStr }
        },
        document: {
          getElementById: (id) => makeElem(id),
          querySelectorAll: (sel) => (sel === "[data-state-btn]" ? stateBtns : []),
          createElement: (tag) => {
            const el = makeElem("created_" + tag + "_" + exportedCanvases.length);
            if (tag === "canvas") exportedCanvases.push(el);
            return el;
          }
        },
        URLSearchParams,
        Float32Array,
        Math,
        Number,
        String,
        Object,
        Array,
        parseInt,
        parseFloat,
        setInterval: (fn) => {
          sandbox._intervalFn = fn;
          return 101;
        },
        clearInterval: () => {
          sandbox._intervalCleared = true;
        }
      };
      vm.runInNewContext(controllerCode, sandbox);
      return { elements, stateBtns, exportedCanvases, downloadLinks, sandbox };
    }

    // 11a. Query ?interp=1&state=1.5 synchronizes policy label, event banner, SHA-256 box, and PNG export
    const domInterp = runViewerInDom("?interp=1&state=1.5");
    assert.strictEqual(
      domInterp.elements["lbl-policy-mode"].textContent,
      "Policy: explicit_display_only_when_toggled | Display Interpolation Active"
    );
    assert.strictEqual(domInterp.elements["badge-state-derivation"].textContent, "DERIVATION: INTERPOLATED_FOR_DISPLAY");
    assert.strictEqual(
      domInterp.elements["meta-sha256-box"].textContent,
      "unverified_display_interpolation (run_receipt = null)"
    );
    assert(domInterp.elements["meta-event-banner"].innerHTML.includes("Interpolated Display Frame (Not Simulated)"));
    assert.strictEqual(
      domInterp.elements["meta-baseline-commit"].textContent,
      "636b7d7bb41999b80278d90046da893ee1044f20"
    );
    assert(
      domInterp.elements["meta-source-provenance"].textContent.includes("spacetime_geochemistry_v0_gen_0.2.0")
    );

    // Click Export 1920x1080 PNG while on interpolated state 1.5
    domInterp.elements["btn-export-png"].trigger("click");
    assert.strictEqual(domInterp.exportedCanvases.length, 1);
    const interpPngTexts = domInterp.exportedCanvases[0]._ctxCalls;
    assert(
      interpPngTexts.some(
        (c) => c.fillStyle === "#fbbf24" && c.text.includes("DISPLAY INTERPOLATION ONLY (NOT SIMULATED)")
      ),
      "Exported PNG on interpolated state must stamp amber warning header"
    );
    assert.strictEqual(
      domInterp.downloadLinks[0].download,
      "spacetime_geochemistry_v0_interpolated_display_1920x1080.png"
    );

    // Step -1 from fractional state 1.5 must land on discrete State 1 (not 0) and display ALGEBRAIC (GAP 0.44%)
    domInterp.elements["btn-step-prev"].trigger("click");
    assert.strictEqual(domInterp.elements["active-time-readout"].textContent, "STATE 1 / 4 | t = 250.0 s");
    assert.strictEqual(
      domInterp.elements["badge-state-derivation"].textContent,
      "DERIVATION: PRESCRIBED_ANALYTICALLY"
    );
    assert.strictEqual(
      domInterp.elements["ledger-residual"].textContent,
      "ALGEBRAIC (GAP 0.44%)"
    );

    // Click Export 1920x1080 PNG on discrete State 1 -> must stamp green verified header
    domInterp.elements["btn-export-png"].trigger("click");
    const discPngTexts = domInterp.exportedCanvases[1]._ctxCalls;
    assert(
      discPngTexts.some(
        (c) => c.fillStyle === "#10b981" && c.text.includes("Stage 0/1 Verified Synthetic 4D Tracer Proof | State: 1")
      ),
      "Exported PNG on discrete state must stamp emerald green verified header"
    );

    // 11b. Unauthorized fractional ?state=1.5 without interp=1 surfaces fail-closed error banner
    const domRejected = runViewerInDom("?state=1.5");
    assert.strictEqual(domRejected.elements["top-disclaimer-bar"].className, "disclaimer-bar error-active");
    assert(domRejected.elements["disclaimer-text"].innerHTML.includes("FAIL-CLOSED CONTRACT REJECTION:"));
    assert.strictEqual(domRejected.elements["active-time-readout"].textContent, "STATE 0 / 4 | t = 0.0 s");
  });

  const failed = results.filter((r) => !r.passed);
  console.log(
    "\nSummary: " + (results.length - failed.length) + "/" + results.length + " semantic tests passed."
  );
  if (failed.length > 0) {
    process.exit(1);
  }
}

runSuite();
