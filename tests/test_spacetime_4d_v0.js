#!/usr/bin/env node
/**
 * Stage 1 Semantic & Data-Contract Verification Suite (test_spacetime_4d_v0.js)
 *
 * Tests Requirement R10 (spacetime_geochemistry_v0) Stage 0 and Stage 1 invariants:
 * - Strict monotonic time ordering and explicit units
 * - Bit-exact IEEE-754 little-endian Float32Array SHA-256 verification
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

  // Test 1: Temporal axis, spatial coordinate frame, and embedded bundle parity
  test("Test 1: Temporal axis monotonicity, +k up-axis frame, and embedded bundle bit-parity", () => {
    const check = S4D.verifyBundleIntegrity(diskBundle);
    assert.strictEqual(check.valid, true);
    assert.strictEqual(check.state_count, 5);
    assert.strictEqual(check.voxel_count, 4096);

    assert.strictEqual(diskBundle.temporal_axis.fourth_dimension_symbol, "t");
    assert.strictEqual(diskBundle.temporal_axis.time_units, "s");
    assert.strictEqual(diskBundle.spatial_frame.up_axis, "+k");
    assert.strictEqual(diskBundle.spatial_frame.geometry_provenance, "schematic");

    const times = diskBundle.states.map((s) => s.simulation_time);
    assert.deepStrictEqual(times, [0.0, 250.0, 500.0, 750.0, 1000.0]);
    for (let i = 1; i < times.length; i++) {
      assert(times[i] > times[i - 1], "Time steps must be strictly increasing");
    }

    const embedded = S4D.buildEmbeddedSyntheticBundle();
    S4D.verifyBundleIntegrity(embedded);
    for (let s = 0; s < 5; s++) {
      assert.strictEqual(
        embedded.states[s].sha256_f32_le,
        diskBundle.states[s].sha256_f32_le,
        "Embedded bundle SHA-256 must match disk JSON at state " + s
      );
    }
  });

  // Test 2: 3D field dimensions, finite values, non-zero variance, and conservation closure
  test("Test 2: 16x16x16 voxel dimensions, finite values, non-zero variance, and ledger closure", () => {
    assert.deepStrictEqual(diskBundle.spatial_frame.dimensions_voxels, [16, 16, 16]);
    for (let s = 0; s < diskBundle.states.length; s++) {
      const st = diskBundle.states[s];
      assert.strictEqual(st.voxel_count, 4096);
      assert.strictEqual(st.values.length, 4096);
      let minV = Infinity;
      let maxV = -Infinity;
      for (let i = 0; i < st.values.length; i++) {
        const v = st.values[i];
        assert(Number.isFinite(v), "Voxel value must be finite");
        assert(v >= 0.0, "Synthetic concentration must be non-negative");
        if (v < minV) minV = v;
        if (v > maxV) maxV = v;
      }
      assert(maxV - minV > 2.0, "State " + s + " must have non-trivial spatial variance");
      assert(
        st.conservation_ledger.relative_mass_balance_residual < 1e-12,
        "State " + s + " closed mass balance residual must be < 1e-12"
      );
    }
  });

  // Test 3: Per-frame IEEE-754 little-endian Float32Array SHA-256 verification
  test("Test 3: Bit-exact Float32LE SHA-256 digests verified against Node crypto and mutually distinct", () => {
    const hashes = new Set();
    for (let s = 0; s < diskBundle.states.length; s++) {
      const st = diskBundle.states[s];
      const jsSha = S4D.sha256Float32LE(st.values);
      const nodeSha = nodeFloat32Sha256(st.values);
      assert.strictEqual(jsSha, nodeSha, "Pure-JS SHA-256 must match Node crypto SHA-256");
      assert.strictEqual(jsSha, st.sha256_f32_le, "Recomputed SHA-256 must match recorded receipt digest");
      hashes.add(jsSha);
    }
    assert.strictEqual(hashes.size, 5, "All 5 time states must have distinct SHA-256 digests");
  });

  // Test 4: Semantic time selection proof (changing time changes underlying Float32Array & shifts z_cm upward)
  test("Test 4: Changing time state mutates underlying Float32Array and shifts z_cm monotonically along +k", () => {
    const sel0 = S4D.selectTimeState(diskBundle, 0, { allowDisplayInterpolation: false });
    const sel2 = S4D.selectTimeState(diskBundle, 2, { allowDisplayInterpolation: false });
    const sel4 = S4D.selectTimeState(diskBundle, 4, { allowDisplayInterpolation: false });

    assert(sel0.float32Values instanceof Float32Array);
    assert(sel2.float32Values instanceof Float32Array);
    assert(sel4.float32Values instanceof Float32Array);

    assert.notStrictEqual(sel0.sha256_f32_le, sel2.sha256_f32_le);
    assert.notStrictEqual(sel2.sha256_f32_le, sel4.sha256_f32_le);

    // Verify monotonic upward center-of-mass motion along +k across all 5 states
    let prevZ = -Infinity;
    let prevMax = Infinity;
    for (let s = 0; s < 5; s++) {
      const sel = S4D.selectTimeState(diskBundle, s, { allowDisplayInterpolation: false });
      assert.strictEqual(sel.is_simulated_state, true);
      assert.strictEqual(sel.state_derivation, "prescribed_analytically");
      assert.strictEqual(sel.criterial_status, "verified_accepted");
      const zCm = sel.center_of_mass_m[2];
      assert(zCm > prevZ, "Vertical center of mass z_cm must strictly increase along +k");
      assert(sel.max_value < prevMax, "Peak concentration must attenuate under diffusion and decay");
      prevZ = zCm;
      prevMax = sel.max_value;
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
    assert.strictEqual(prunedBranch.criterial_status, "criterial_pruned");
    assert.strictEqual(prunedBranch.max_relative_mass_balance_residual, 0.38);
    assert.strictEqual(prunedBranch.series.length, 5);
    assert(
      Math.abs(prunedBranch.series[4].value - tsLower.verified_series[4].value * 1.38) < 1e-6,
      "Pruned branch at t=1000s must reflect +38% uncompensated mass leak"
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

  // Test 8: Display interpolation explicit labeling ("interpolated_for_display")
  test("Test 8: Display interpolation marks frame 'interpolated_for_display' with run_receipt: null", () => {
    const interpSel = S4D.selectTimeState(diskBundle, 1.5, { allowDisplayInterpolation: true });
    assert.strictEqual(interpSel.state_derivation, "interpolated_for_display");
    assert.strictEqual(interpSel.is_simulated_state, false);
    assert.strictEqual(interpSel.run_receipt, null);
    assert.strictEqual(interpSel.sha256_f32_le, "unverified_display_interpolation");
    assert.strictEqual(interpSel.simulation_time, 375.0);
    assert(interpSel.display_warning && interpSel.display_warning.includes("INTERPOLATED FOR DISPLAY ONLY"));

    const st1 = S4D.selectTimeState(diskBundle, 1, { allowDisplayInterpolation: false });
    const st2 = S4D.selectTimeState(diskBundle, 2, { allowDisplayInterpolation: false });
    const probeIdx = S4D.voxelIndex(8, 8, 7, 16, 16);
    const expectedMid = Math.fround(0.5 * st1.float32Values[probeIdx] + 0.5 * st2.float32Values[probeIdx]);
    assert(Math.abs(interpSel.float32Values[probeIdx] - expectedMid) < 1e-6);
  });

  // Test 9: Negative-control rejection of corrupted/invalid bundles
  test("Test 9: verifyBundleIntegrity rejects non-monotonic time, NaN, bad SHA-256, and calibrated claims", () => {
    // 9a. Non-monotonic time
    const bNonMono = clone(diskBundle);
    bNonMono.states[3].simulation_time = 400.0;
    assert.throws(() => S4D.verifyBundleIntegrity(bNonMono), /Non-monotonic simulation_time/);

    // 9b. Empty time_units
    const bEmptyUnits = clone(diskBundle);
    bEmptyUnits.temporal_axis.time_units = "";
    assert.throws(() => S4D.verifyBundleIntegrity(bEmptyUnits), /time_units/);

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
    assert.throws(() => S4D.verifyBundleIntegrity(bBadSha), /SHA-256 mismatch/);

    // 9g. Unauthorized empirically_calibrated_model claim
    const bCalib = clone(diskBundle);
    bCalib.evidence_class = "empirically_calibrated_model";
    assert.throws(() => S4D.verifyBundleIntegrity(bCalib), /empirically_calibrated_model/);

    // 9h. Stored interpolated_for_display under none_discrete_states_only
    const bStoredInterp = clone(diskBundle);
    bStoredInterp.states[2].state_derivation = "interpolated_for_display";
    assert.throws(() => S4D.verifyBundleIntegrity(bStoredInterp), /interpolated_for_display/);
  });

  // Test 10: 4D projection, orthogonal slices, offline independence, and typography audit
  test("Test 10: 4D projection (+k up, w=t), orthogonal slices, offline HTML, and zero em/en-dashes", () => {
    const pBottom = S4D.project4DSpatiotemporalPoint(8, 8, 2, 0.0, { angleXT: 0, angleZT: 0, yaw: 0, pitch: 0 });
    const pTop = S4D.project4DSpatiotemporalPoint(8, 8, 14, 0.0, { angleXT: 0, angleZT: 0, yaw: 0, pitch: 0 });
    assert(pTop.screenY > pBottom.screenY, "+k upward coordinate must project to higher vertical screenY");
    assert.strictEqual(pTop.fourthDimensionSymbol, "t");

    const pT0 = S4D.project4DSpatiotemporalPoint(8, 8, 8, 0.0, { angleXT: 0.45, angleZT: 0.25 });
    const pT1 = S4D.project4DSpatiotemporalPoint(8, 8, 8, 1.0, { angleXT: 0.45, angleZT: 0.25 });
    assert.notStrictEqual(pT0.screenX, pT1.screenX, "4D X-t hyperplane rotation must separate t=0 and t=1");

    const sel0 = S4D.selectTimeState(diskBundle, 0, { allowDisplayInterpolation: false });
    const sliceXY = S4D.extractOrthogonalSlice(sel0.float32Values, [16, 16, 16], "XY", 5);
    const sliceXZ = S4D.extractOrthogonalSlice(sel0.float32Values, [16, 16, 16], "XZ", 8);
    assert.strictEqual(sliceXY.data.length, 256);
    assert.strictEqual(sliceXZ.data.length, 256);

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

  const failed = results.filter((r) => !r.passed);
  console.log(
    "\nSummary: " + (results.length - failed.length) + "/" + results.length + " semantic tests passed."
  );
  if (failed.length > 0) {
    process.exit(1);
  }
}

runSuite();
