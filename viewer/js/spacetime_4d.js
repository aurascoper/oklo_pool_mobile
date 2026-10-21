/**
 * Stage 1 Spatiotemporal 4D Data Verifier, State Selector, Dual-Time Comparator,
 * Orthogonal Slicer, and 4D Hyperplane Projector (spacetime_geochemistry_v0).
 *
 * Compatible with both browser (window.Spacetime4D) and Node.js CommonJS.
 * Strictly zero em-dashes and zero en-dashes.
 */
(function (root, factory) {
  if (typeof module === "object" && module.exports) {
    module.exports = factory();
  } else {
    root.Spacetime4D = factory();
  }
})(typeof self !== "undefined" ? self : this, function () {
  "use strict";

  var ALLOWED_EVIDENCE_CLASSES = [
    "geographic_observation_or_locator",
    "sourced_geological_record",
    "prescribed_analytic_field",
    "numerically_verified_synthetic_model",
    "hypothesis_or_unvalidated_model",
    "empirically_calibrated_model"
  ];

  var ALLOWED_STATE_DERIVATIONS = [
    "computed",
    "measured",
    "prescribed_analytically",
    "interpolated_for_display"
  ];

  // Pure-JS SHA-256 implementation for synchronous verification in browser (file://) and Node.js
  var SHA256_K = new Uint32Array([
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
  ]);

  function rotr(x, n) {
    return (x >>> n) | (x << (32 - n));
  }

  function sha256Bytes(bytes) {
    var len = bytes.length;
    var bitLenHi = Math.floor((len * 8) / 0x100000000);
    var bitLenLo = (len * 8) >>> 0;
    var padLen = ((len + 9 + 63) & ~63);
    var buf = new Uint8Array(padLen);
    buf.set(bytes, 0);
    buf[len] = 0x80;
    var view = new DataView(buf.buffer);
    view.setUint32(padLen - 8, bitLenHi, false);
    view.setUint32(padLen - 4, bitLenLo, false);

    var h0 = 0x6a09e667;
    var h1 = 0xbb67ae85;
    var h2 = 0x3c6ef372;
    var h3 = 0xa54ff53a;
    var h4 = 0x510e527f;
    var h5 = 0x9b05688c;
    var h6 = 0x1f83d9ab;
    var h7 = 0x5be0cd19;

    var w = new Uint32Array(64);

    for (var offset = 0; offset < padLen; offset += 64) {
      for (var i = 0; i < 16; i++) {
        w[i] = view.getUint32(offset + i * 4, false);
      }
      for (var j = 16; j < 64; j++) {
        var s0 = rotr(w[j - 15], 7) ^ rotr(w[j - 15], 18) ^ (w[j - 15] >>> 3);
        var s1 = rotr(w[j - 2], 17) ^ rotr(w[j - 2], 19) ^ (w[j - 2] >>> 10);
        w[j] = (w[j - 16] + s0 + w[j - 7] + s1) >>> 0;
      }

      var a = h0;
      var b = h1;
      var c = h2;
      var d = h3;
      var e = h4;
      var f = h5;
      var g = h6;
      var h = h7;

      for (var t = 0; t < 64; t++) {
        var S1 = rotr(e, 6) ^ rotr(e, 11) ^ rotr(e, 25);
        var ch = (e & f) ^ (~e & g);
        var temp1 = (h + S1 + ch + SHA256_K[t] + w[t]) >>> 0;
        var S0 = rotr(a, 2) ^ rotr(a, 13) ^ rotr(a, 22);
        var maj = (a & b) ^ (a & c) ^ (b & c);
        var temp2 = (S0 + maj) >>> 0;

        h = g;
        g = f;
        f = e;
        e = (d + temp1) >>> 0;
        d = c;
        c = b;
        b = a;
        a = (temp1 + temp2) >>> 0;
      }

      h0 = (h0 + a) >>> 0;
      h1 = (h1 + b) >>> 0;
      h2 = (h2 + c) >>> 0;
      h3 = (h3 + d) >>> 0;
      h4 = (h4 + e) >>> 0;
      h5 = (h5 + f) >>> 0;
      h6 = (h6 + g) >>> 0;
      h7 = (h7 + h) >>> 0;
    }

    function hex32(val) {
      return ("00000000" + (val >>> 0).toString(16)).slice(-8);
    }

    return (
      hex32(h0) +
      hex32(h1) +
      hex32(h2) +
      hex32(h3) +
      hex32(h4) +
      hex32(h5) +
      hex32(h6) +
      hex32(h7)
    );
  }

  function sha256Float32LE(valuesArray) {
    var count = valuesArray.length;
    var ab = new ArrayBuffer(count * 4);
    var dv = new DataView(ab);
    for (var i = 0; i < count; i++) {
      dv.setFloat32(i * 4, valuesArray[i], true);
    }
    return sha256Bytes(new Uint8Array(ab));
  }

  function voxelIndex(i, j, k, nx, ny) {
    return i + nx * (j + ny * k);
  }

  function computeCenterOfMass(valuesArray, nx, ny, nz, domainBoundsM) {
    var xMin = 0.0;
    var xMax = nx;
    var yMin = 0.0;
    var yMax = ny;
    var zMin = 0.0;
    var zMax = nz;
    if (domainBoundsM && typeof domainBoundsM === "object") {
      if (Array.isArray(domainBoundsM.x) && domainBoundsM.x.length === 2) {
        xMin = domainBoundsM.x[0];
        xMax = domainBoundsM.x[1];
      }
      if (Array.isArray(domainBoundsM.y) && domainBoundsM.y.length === 2) {
        yMin = domainBoundsM.y[0];
        yMax = domainBoundsM.y[1];
      }
      if (Array.isArray(domainBoundsM.z_up) && domainBoundsM.z_up.length === 2) {
        zMin = domainBoundsM.z_up[0];
        zMax = domainBoundsM.z_up[1];
      }
    }
    var dx = (xMax - xMin) / nx;
    var dy = (yMax - yMin) / ny;
    var dz = (zMax - zMin) / nz;
    var total = 0.0;
    var sx = 0.0;
    var sy = 0.0;
    var sz = 0.0;
    for (var k = 0; k < nz; k++) {
      var z = zMin + (k + 0.5) * dz;
      for (var j = 0; j < ny; j++) {
        var y = yMin + (j + 0.5) * dy;
        for (var i = 0; i < nx; i++) {
          var x = xMin + (i + 0.5) * dx;
          var v = valuesArray[voxelIndex(i, j, k, nx, ny)];
          total += v;
          sx += x * v;
          sy += y * v;
          sz += z * v;
        }
      }
    }
    if (total <= 0.0) {
      return [
        Number((0.5 * (xMin + xMax)).toFixed(6)),
        Number((0.5 * (yMin + yMax)).toFixed(6)),
        Number((0.5 * (zMin + zMax)).toFixed(6))
      ];
    }
    return [
      Number((sx / total).toFixed(6)),
      Number((sy / total).toFixed(6)),
      Number((sz / total).toFixed(6))
    ];
  }

  function computeVoxelSummaryStatistics(valuesArray, spatialFrame) {
    var dims = spatialFrame.dimensions_voxels;
    var nx = dims[0];
    var ny = dims[1];
    var nz = dims[2];
    var dBounds = spatialFrame.domain_bounds_m;
    var xMin = dBounds.x[0];
    var xMax = dBounds.x[1];
    var yMin = dBounds.y[0];
    var yMax = dBounds.y[1];
    var zMin = dBounds.z_up[0];
    var zMax = dBounds.z_up[1];
    var dx = (xMax - xMin) / nx;
    var dy = (yMax - yMin) / ny;
    var dz = (zMax - zMin) / nz;
    var voxelVolumeM3 = dx * dy * dz;

    var minV = Infinity;
    var maxV = -Infinity;
    var sumV = 0.0;
    var sx = 0.0;
    var sy = 0.0;
    var sz = 0.0;

    for (var k = 0; k < nz; k++) {
      var z = zMin + (k + 0.5) * dz;
      for (var j = 0; j < ny; j++) {
        var y = yMin + (j + 0.5) * dy;
        var baseIdx = nx * (j + ny * k);
        for (var i = 0; i < nx; i++) {
          var x = xMin + (i + 0.5) * dx;
          var v = Math.fround(valuesArray[baseIdx + i]);
          if (v < minV) minV = v;
          if (v > maxV) maxV = v;
          sumV += v;
          sx += x * v;
          sy += y * v;
          sz += z * v;
        }
      }
    }

    var voxelCount = nx * ny * nz;
    var com =
      sumV > 0.0
        ? [
            Number((sx / sumV).toFixed(6)),
            Number((sy / sumV).toFixed(6)),
            Number((sz / sumV).toFixed(6))
          ]
        : [
            Number((0.5 * (xMin + xMax)).toFixed(6)),
            Number((0.5 * (yMin + yMax)).toFixed(6)),
            Number((0.5 * (zMin + zMax)).toFixed(6))
          ];

    return {
      min_value: minV,
      max_value: maxV,
      mean_value: sumV / voxelCount,
      voxel_sum: sumV,
      voxel_volume_m3: voxelVolumeM3,
      total_integrated_inventory_mol: sumV * voxelVolumeM3,
      center_of_mass_m: com
    };
  }

  // Deterministic synchronous fallback generator matching scripts/generate_synthetic_4d_bundle.py
  function buildEmbeddedSyntheticBundle() {
    var nx = 16;
    var ny = 16;
    var nz = 16;
    var voxelCount = nx * ny * nz;
    var c0 = 10.0;
    var sigma0 = 1.8;
    var diff = 0.0012;
    var kDecay = 0.0004;
    var x0 = 8.0;
    var y0 = 8.0;
    var z0 = 4.5;
    var ux = 0.0015;
    var uy = -0.001;
    var uz = 0.0065;
    var timeSteps = [0.0, 250.0, 500.0, 750.0, 1000.0];

    var eventDefs = [
      [
        "t0_injection_pulse",
        "initial_condition",
        "Initial 3D Gaussian tracer pulse centered at z = 4.50 m (+k upward axis)."
      ],
      [
        "t1_lower_conduit_ascent",
        "advection_diffusion_step",
        "Upward +k advection to z_c = 6.125 m with isotropic diffusive spreading."
      ],
      [
        "t2_mid_column_transit",
        "advection_diffusion_step",
        "Mid-domain transit at z_c = 7.75 m; peak amplitude attenuated by diffusion and decay."
      ],
      [
        "t3_upper_zone_entry",
        "advection_diffusion_step",
        "Upper compartment entry at z_c = 9.375 m; per-frame float32 checksum recorded."
      ],
      [
        "t4_late_plume_dispersion",
        "terminal_benchmark_state",
        "Late-stage dispersed plume at z_c = 11.00 m; discrete voxel quadrature inventory and analytical decay bookkeeping recorded."
      ]
    ];

    var states = [];
    var prunedPoints = [];
    var eventMarkers = [];
    var initialMassMol = null;

    for (var sIdx = 0; sIdx < timeSteps.length; sIdx++) {
      var tS = timeSteps[sIdx];
      var sigmaSq = sigma0 * sigma0 + 2.0 * diff * tS;
      var ampScale = Math.pow((sigma0 * sigma0) / sigmaSq, 1.5);
      var decayFactor = Math.exp(-kDecay * tS);
      var prefactor = c0 * ampScale * decayFactor;

      var xc = x0 + ux * tS;
      var yc = y0 + uy * tS;
      var zc = z0 + uz * tS;

      var f32 = new Float32Array(voxelCount);
      var values = new Array(voxelCount);
      var retainedMass = 0.0;
      var minV = Infinity;
      var maxV = -Infinity;

      for (var k = 0; k < nz; k++) {
        var z = k + 0.5;
        var dz2 = (z - zc) * (z - zc);
        for (var j = 0; j < ny; j++) {
          var y = j + 0.5;
          var dy2 = (y - yc) * (y - yc);
          for (var i = 0; i < nx; i++) {
            var x = i + 0.5;
            var dx2 = (x - xc) * (x - xc);
            var r2 = dx2 + dy2 + dz2;
            var rawVal = prefactor * Math.exp(-r2 / (2.0 * sigmaSq));
            var qVal = Math.fround(rawVal);
            var flatIdx = voxelIndex(i, j, k, nx, ny);
            f32[flatIdx] = qVal;
            values[flatIdx] = qVal;
            retainedMass += qVal;
            if (qVal < minV) minV = qVal;
            if (qVal > maxV) maxV = qVal;
          }
        }
      }

      if (initialMassMol === null) {
        initialMassMol = retainedMass;
      }

      var expectedSurviving = initialMassMol * Math.exp(-kDecay * tS);
      var cumulativeDecayed = initialMassMol * (1.0 - Math.exp(-kDecay * tS));
      var quadratureGap = expectedSurviving - retainedMass;
      var relQuadratureGap = Math.abs(quadratureGap) / initialMassMol;

      var com = computeCenterOfMass(values, nx, ny, nz);
      var shaHex = sha256Float32LE(f32);

      eventMarkers.push({
        state_index: sIdx,
        simulation_time: tS,
        label: eventDefs[sIdx][0] + ": " + eventDefs[sIdx][2],
        event_type: eventDefs[sIdx][1]
      });

      states.push({
        state_index: sIdx,
        simulation_time: tS,
        time_units: "s",
        state_derivation: "prescribed_analytically",
        criterial_status: "verified_accepted",
        is_simulated_state: true,
        run_receipt: "receipts/spacetime_geochemistry_v0_2026-10-20.md#state_" + sIdx,
        voxel_count: voxelCount,
        sha256_f32_le: shaHex,
        min_value: minV,
        max_value: maxV,
        mean_value: retainedMass / voxelCount,
        total_integrated_inventory_mol: retainedMass,
        center_of_mass_m: com,
        analytical_bookkeeping_ledger: {
          initial_discrete_inventory_mol: Number(initialMassMol.toFixed(6)),
          retained_discrete_inventory_mol: Number(retainedMass.toFixed(6)),
          analytical_surviving_reference_mol: Number(expectedSurviving.toFixed(6)),
          analytical_decayed_reference_mol: Number(cumulativeDecayed.toFixed(6)),
          unmodeled_boundary_and_quadrature_gap_mol: Number(quadratureGap.toFixed(6)),
          relative_quadrature_and_tail_gap: Number(relQuadratureGap.toFixed(6)),
          closes_by_algebraic_definition: true,
          is_finite_volume_conservation_test: false,
          analytical_bookkeeping_note:
            "Illustrative analytical bookkeeping only: M_surv = M0*exp(-k*t) and M_decay = M0*(1-exp(-k*t)) close by algebraic definition, while M_surv - M_retained reflects unmodeled Gaussian tail mass outside [0,16m]^3 and 1m voxel midpoint quadrature. Does not test finite-volume numerical conservation (deferred to Stage 2)."
        },
        values: values
      });

      var driftMult = 1.0 + 0.38 * (tS / 1000.0);
      var driftFrac = Number((0.38 * (tS / 1000.0)).toFixed(6));
      prunedPoints.push({
        state_index: sIdx,
        simulation_time: tS,
        center_of_mass_m: [
          Number((com[0] + 0.12 * (tS / 1000.0)).toFixed(6)),
          Number((com[1] - 0.15 * (tS / 1000.0)).toFixed(6)),
          Number((com[2] - 0.85 * (tS / 1000.0)).toFixed(6))
        ],
        probe_voxel_8_8_5_value: values[voxelIndex(8, 8, 5, nx, ny)] * driftMult,
        probe_voxel_8_8_11_value: values[voxelIndex(8, 8, 11, nx, ny)] * driftMult,
        relative_analytical_drift: driftFrac
      });
    }

    return {
      schema_version: "spacetime_4d_v0",
      bundle_id: "synthetic_tracer_pulse_4d_v0",
      model_id: "spacetime_geochemistry_v0",
      model_version: "0.1.0",
      specification_version: "docs/specs/SPACETIME_GEOCHEMISTRY_v0.md#v0.1.0",
      evidence_class: "numerically_verified_synthetic_model",
      source_repository: "https://github.com/aurascoper/oklo_pool_mobile",
      baseline_commit: "636b7d7bb41999b80278d90046da893ee1044f20",
      source_provenance: {
        target_branch: "spacetime-geochemistry/v0-slice",
        baseline_commit: "636b7d7bb41999b80278d90046da893ee1044f20",
        generator_script: "scripts/generate_synthetic_4d_bundle.py",
        generator_version: "spacetime_geochemistry_v0_gen_0.2.0",
        generator_script_sha256: "d8a78d21351d8ee0c1c791adbec48e6e48c11e032ec93c6fb30eb91b815962e7",
        tested_source_commit_policy: "recorded_in_external_verification_receipt"
      },
      run_id: "synth_adv_diff_decay_16x16x16_5state_v0",
      run_receipt: "receipts/spacetime_geochemistry_v0_2026-10-20.md",
      parameter_set_id: "synthetic_advection_diffusion_decay_v0_default",
      field_name: "synthetic_tracer_concentration",
      field_units: "mol_m3_synthetic_tracer",
      uncertainty:
        "Synthetic analytical benchmark sampled on a 1m voxel grid; IEEE-754 float32 quantization residual < 1e-6; analytical bookkeeping closes by algebraic identity (not a finite-volume PDE conservation test).",
      known_limitations:
        "Stage 1 synthetic 3D advection-diffusion-decay pulse on a schematic 16m cube; not a geological, hydrothermal, or radiation-transport simulation.",
      scientific_status_disclaimer:
        "Passing viewer and data-contract tests establishes rendering and data-contract correctness only, not geological validity, radiation transport accuracy, or empirical calibration.",
      spatial_frame: {
        coordinate_system: "cartesian_local_16m_cube",
        geometry_provenance: "schematic",
        dimensions_voxels: [nx, ny, nz],
        voxel_order: "x_fastest_then_y_then_k_up",
        domain_bounds_m: {
          x: [0.0, 16.0],
          y: [0.0, 16.0],
          z_up: [0.0, 16.0]
        },
        up_axis: "+k"
      },
      temporal_axis: {
        fourth_dimension_symbol: "t",
        time_units: "s",
        state_count: timeSteps.length,
        temporal_resolution_s: 250.0,
        time_bounds: [timeSteps[0], timeSteps[timeSteps.length - 1]],
        strict_monotonic_ordering: true,
        missing_frame_policy: "fail_closed_no_invented_states",
        interpolation_policy: "none_discrete_states_only",
        initial_condition_summary:
          "3D Gaussian tracer pulse centered at (8.0, 8.0, 4.5) m with sigma_0 = 1.8 m and peak C_0 = 10.0 mol_m3_synthetic_tracer at t = 0.0 s.",
        event_markers: eventMarkers
      },
      governing_parameters: {
        equation_summary:
          "C(x,y,z,t) = C0 * (sigma0^2 / sigma(t)^2)^(3/2) * exp(-k_decay * t) * exp(-((x-xc(t))^2 + (y-yc(t))^2 + (z-zc(t))^2) / (2 * sigma(t)^2))",
        initial_peak_concentration_mol_m3: c0,
        initial_sigma_m: sigma0,
        diffusion_coefficient_m2_s: diff,
        decay_rate_s_inv: kDecay,
        advection_velocity_m_s: [ux, uy, uz],
        initial_center_m: [x0, y0, z0]
      },
      v2_compatibility: {
        schema_target: "https://aurascoper.github.io/oklo_pool_mobile/schemas/epistemic_record_v1.json",
        epistemic_record: {
          model_id: "spacetime_geochemistry_v0",
          model_version: "0.1.0",
          evidence_class: "numerically_verified_synthetic_model",
          source_repository: "https://github.com/aurascoper/oklo_pool_mobile",
          baseline_commit: "636b7d7bb41999b80278d90046da893ee1044f20",
          source_revision: "spacetime-geochemistry/v0-slice@spacetime_geochemistry_v0_gen_0.2.0",
          specification_version: "docs/specs/SPACETIME_GEOCHEMISTRY_v0.md#v0.1.0",
          run_receipt: "receipts/spacetime_geochemistry_v0_2026-10-20.md",
          parameter_set_id: "synthetic_advection_diffusion_decay_v0_default",
          units: "mol_m3_synthetic_tracer; time_units=s",
          coordinate_system: "cartesian_local_16m_cube",
          uncertainty:
            "Synthetic analytical benchmark sampled on a 1m voxel grid; IEEE-754 float32 quantization residual < 1e-6; analytical bookkeeping closes by algebraic identity (not a finite-volume PDE conservation test).",
          known_limitations:
            "Stage 1 synthetic 3D advection-diffusion-decay pulse on a schematic 16m cube; not a geological, hydrothermal, or radiation-transport simulation."
        }
      },
      states: states,
      criterial_pruned_branches: [
        {
          branch_id: "synthetic_amplitude_drift_control",
          label: "Negative Control: Synthetic +38% Amplitude-Drift Candidate (Pruned)",
          criterial_status: "criterial_pruned",
          prune_reason:
            "Synthetic negative-control candidate perturbed by an artificial +38% amplitude drift multiplier (1 + 0.38 * t / 1000 s) at t = 1000 s, exceeding the Stage 1 analytical reference envelope gate (0.38 > 1.0e-4). Demonstrates visual criterial separation only, not finite-volume PDE conservation.",
          max_relative_analytical_drift: 0.38,
          criterial_threshold_exceeded: 0.0001,
          time_points: prunedPoints
        }
      ]
    };
  }

  function verifyBundleIntegrity(bundle) {
    if (!bundle || typeof bundle !== "object") {
      throw new Error("SpacetimeContractError: Bundle must be a non-null object");
    }
    if (bundle.schema_version !== "spacetime_4d_v0") {
      throw new Error("SpacetimeContractError: schema_version must be 'spacetime_4d_v0'");
    }
    var requiredStrings = [
      "bundle_id",
      "model_id",
      "model_version",
      "specification_version",
      "source_repository",
      "baseline_commit",
      "run_id",
      "parameter_set_id",
      "field_name",
      "field_units",
      "uncertainty",
      "known_limitations",
      "scientific_status_disclaimer"
    ];
    for (var s = 0; s < requiredStrings.length; s++) {
      var key = requiredStrings[s];
      if (typeof bundle[key] !== "string" || bundle[key].trim().length === 0) {
        throw new Error("SpacetimeContractError: Missing or empty required string '" + key + "'");
      }
    }
    if (!/^[0-9a-f]{7,40}$/.test(bundle.baseline_commit)) {
      throw new Error("SpacetimeContractError: baseline_commit must be a lowercase hex commit hash");
    }

    var prov = bundle.source_provenance;
    if (!prov || typeof prov !== "object") {
      throw new Error("SpacetimeContractError: Missing source_provenance object");
    }
    var provStrings = [
      "target_branch",
      "baseline_commit",
      "generator_script",
      "generator_version",
      "generator_script_sha256",
      "tested_source_commit_policy"
    ];
    for (var pIdx = 0; pIdx < provStrings.length; pIdx++) {
      var pKey = provStrings[pIdx];
      if (typeof prov[pKey] !== "string" || prov[pKey].trim().length === 0) {
        throw new Error("SpacetimeContractError: Missing or empty source_provenance." + pKey);
      }
    }
    if (prov.baseline_commit !== bundle.baseline_commit) {
      throw new Error("SpacetimeContractError: source_provenance.baseline_commit mismatch with baseline_commit");
    }
    if (
      prov.generator_version === bundle.baseline_commit ||
      prov.generator_version.indexOf(bundle.baseline_commit.slice(0, 7)) === 0 ||
      /^[0-9a-f]{7,40}$/.test(prov.generator_version)
    ) {
      throw new Error(
        "SpacetimeContractError: source_provenance.generator_version must not conflate generator revision with baseline_commit or a raw commit hash"
      );
    }
    if (!/^[0-9a-f]{64}$/.test(prov.generator_script_sha256)) {
      throw new Error("SpacetimeContractError: source_provenance.generator_script_sha256 must be a 64-hex SHA-256");
    }

    if (ALLOWED_EVIDENCE_CLASSES.indexOf(bundle.evidence_class) === -1) {
      throw new Error("SpacetimeContractError: Invalid evidence_class '" + bundle.evidence_class + "'");
    }
    if (bundle.evidence_class === "empirically_calibrated_model") {
      throw new Error(
        "SpacetimeContractError: Unauthorized evidence_class 'empirically_calibrated_model' in Stage 0/1 synthetic slice"
      );
    }

    var spatial = bundle.spatial_frame;
    if (!spatial || typeof spatial !== "object") {
      throw new Error("SpacetimeContractError: Missing spatial_frame");
    }
    if (spatial.up_axis !== "+k") {
      throw new Error("SpacetimeContractError: spatial_frame.up_axis must be '+k'");
    }
    if (spatial.voxel_order !== "x_fastest_then_y_then_k_up") {
      throw new Error("SpacetimeContractError: spatial_frame.voxel_order must be 'x_fastest_then_y_then_k_up'");
    }
    if (
      spatial.geometry_provenance !== "schematic" &&
      spatial.geometry_provenance !== "surveyed_or_sourced"
    ) {
      throw new Error("SpacetimeContractError: Invalid spatial_frame.geometry_provenance");
    }
    var dims = spatial.dimensions_voxels;
    if (
      !Array.isArray(dims) ||
      dims.length !== 3 ||
      !Number.isInteger(dims[0]) ||
      !Number.isInteger(dims[1]) ||
      !Number.isInteger(dims[2]) ||
      dims[0] <= 0 ||
      dims[1] <= 0 ||
      dims[2] <= 0
    ) {
      throw new Error("SpacetimeContractError: spatial_frame.dimensions_voxels must be 3 positive integers");
    }
    var expectedVoxels = dims[0] * dims[1] * dims[2];

    var dBounds = spatial.domain_bounds_m;
    if (!dBounds || typeof dBounds !== "object") {
      throw new Error("SpacetimeContractError: Missing spatial_frame.domain_bounds_m");
    }
    var axes = ["x", "y", "z_up"];
    for (var aIdx = 0; aIdx < axes.length; aIdx++) {
      var ax = dBounds[axes[aIdx]];
      if (
        !Array.isArray(ax) ||
        ax.length !== 2 ||
        typeof ax[0] !== "number" ||
        typeof ax[1] !== "number" ||
        !Number.isFinite(ax[0]) ||
        !Number.isFinite(ax[1]) ||
        ax[0] >= ax[1]
      ) {
        throw new Error(
          "SpacetimeContractError: spatial_frame.domain_bounds_m." + axes[aIdx] + " must have finite min < max"
        );
      }
    }

    var gov = bundle.governing_parameters;
    if (!gov || typeof gov !== "object") {
      throw new Error("SpacetimeContractError: Missing governing_parameters");
    }
    if (typeof gov.equation_summary !== "string" || gov.equation_summary.trim().length === 0) {
      throw new Error("SpacetimeContractError: governing_parameters.equation_summary must be a non-empty string");
    }
    var govNums = [
      "initial_peak_concentration_mol_m3",
      "initial_sigma_m",
      "diffusion_coefficient_m2_s",
      "decay_rate_s_inv"
    ];
    for (var gIdx = 0; gIdx < govNums.length; gIdx++) {
      var gKey = govNums[gIdx];
      if (typeof gov[gKey] !== "number" || !Number.isFinite(gov[gKey])) {
        throw new Error("SpacetimeContractError: governing_parameters." + gKey + " must be a finite number");
      }
    }
    if (gov.decay_rate_s_inv < 0) {
      throw new Error("SpacetimeContractError: governing_parameters.decay_rate_s_inv must be >= 0");
    }
    var kDecay = gov.decay_rate_s_inv;
    var govVecs = ["advection_velocity_m_s", "initial_center_m"];
    for (var gvIdx = 0; gvIdx < govVecs.length; gvIdx++) {
      var gv = gov[govVecs[gvIdx]];
      if (
        !Array.isArray(gv) ||
        gv.length !== 3 ||
        typeof gv[0] !== "number" ||
        typeof gv[1] !== "number" ||
        typeof gv[2] !== "number" ||
        !Number.isFinite(gv[0]) ||
        !Number.isFinite(gv[1]) ||
        !Number.isFinite(gv[2])
      ) {
        throw new Error("SpacetimeContractError: governing_parameters." + govVecs[gvIdx] + " must be 3 finite numbers");
      }
    }

    var temporal = bundle.temporal_axis;
    if (!temporal || typeof temporal !== "object") {
      throw new Error("SpacetimeContractError: Missing temporal_axis");
    }
    if (temporal.fourth_dimension_symbol !== "t") {
      throw new Error("SpacetimeContractError: temporal_axis.fourth_dimension_symbol must equal 't'");
    }
    if (temporal.time_units !== "s") {
      throw new Error("SpacetimeContractError: temporal_axis.time_units must equal 's' in spacetime_4d_v0");
    }
    if (
      typeof temporal.temporal_resolution_s !== "number" ||
      !Number.isFinite(temporal.temporal_resolution_s) ||
      temporal.temporal_resolution_s <= 0
    ) {
      throw new Error("SpacetimeContractError: temporal_axis.temporal_resolution_s must be a finite positive number");
    }
    if (temporal.strict_monotonic_ordering !== true) {
      throw new Error("SpacetimeContractError: temporal_axis.strict_monotonic_ordering must be true");
    }
    if (temporal.missing_frame_policy !== "fail_closed_no_invented_states") {
      throw new Error(
        "SpacetimeContractError: temporal_axis.missing_frame_policy must be 'fail_closed_no_invented_states'"
      );
    }
    if (
      temporal.interpolation_policy !== "none_discrete_states_only" &&
      temporal.interpolation_policy !== "explicit_display_only_when_toggled"
    ) {
      throw new Error("SpacetimeContractError: Invalid temporal_axis.interpolation_policy");
    }
    if (
      typeof temporal.initial_condition_summary !== "string" ||
      temporal.initial_condition_summary.trim().length === 0
    ) {
      throw new Error("SpacetimeContractError: temporal_axis.initial_condition_summary must be a non-empty string");
    }

    var states = bundle.states;
    if (!Array.isArray(states) || states.length < 2) {
      throw new Error("SpacetimeContractError: states must be an array with at least 2 time states");
    }
    if (states.length !== temporal.state_count) {
      throw new Error("SpacetimeContractError: states.length does not match temporal_axis.state_count");
    }

    var prevTime = null;
    var seenHashes = {};
    var initialIntegratedInventory = 0.0;

    for (var idx = 0; idx < states.length; idx++) {
      var st = states[idx];
      if (!st || typeof st !== "object") {
        throw new Error("SpacetimeContractError: states[" + idx + "] must be an object");
      }
      if (st.state_index !== idx) {
        throw new Error(
          "SpacetimeContractError: Missing or out-of-order state_index at states[" +
            idx +
            "] (got " +
            st.state_index +
            ")"
        );
      }
      if (typeof st.simulation_time !== "number" || !Number.isFinite(st.simulation_time)) {
        throw new Error("SpacetimeContractError: states[" + idx + "].simulation_time must be finite");
      }
      if (prevTime !== null) {
        if (st.simulation_time <= prevTime) {
          throw new Error(
            "SpacetimeContractError: Non-monotonic simulation_time at states[" +
              idx +
              "]: " +
              st.simulation_time +
              " <= " +
              prevTime
          );
        }
        var dtStep = st.simulation_time - prevTime;
        if (Math.abs(dtStep - temporal.temporal_resolution_s) > 1e-6) {
          throw new Error(
            "SpacetimeContractError: Inconsistent time step at states[" +
              idx +
              "]: dt=" +
              dtStep +
              " != temporal_axis.temporal_resolution_s=" +
              temporal.temporal_resolution_s
          );
        }
      }
      prevTime = st.simulation_time;

      if (st.time_units !== "s") {
        throw new Error("SpacetimeContractError: states[" + idx + "].time_units must equal 's'");
      }
      if (ALLOWED_STATE_DERIVATIONS.indexOf(st.state_derivation) === -1) {
        throw new Error("SpacetimeContractError: Invalid state_derivation at states[" + idx + "]");
      }
      if (
        temporal.interpolation_policy === "none_discrete_states_only" &&
        st.state_derivation === "interpolated_for_display"
      ) {
        throw new Error(
          "SpacetimeContractError: Stored states cannot be 'interpolated_for_display' under none_discrete_states_only"
        );
      }
      if (st.state_derivation === "interpolated_for_display") {
        if (st.run_receipt !== null) {
          throw new Error("SpacetimeContractError: interpolated_for_display state must have run_receipt = null");
        }
        if (st.is_simulated_state !== false) {
          throw new Error("SpacetimeContractError: interpolated_for_display state must have is_simulated_state = false");
        }
      } else {
        if (st.is_simulated_state !== true) {
          throw new Error("SpacetimeContractError: Discrete state at states[" + idx + "] must have is_simulated_state = true");
        }
        if (typeof st.run_receipt !== "string" || st.run_receipt.trim().length === 0) {
          throw new Error("SpacetimeContractError: Discrete state at states[" + idx + "] must have a non-empty run_receipt");
        }
      }
      if (st.criterial_status !== "verified_accepted") {
        throw new Error(
          "SpacetimeContractError: Primary states[" + idx + "] must have criterial_status='verified_accepted'"
        );
      }

      var ledger = st.analytical_bookkeeping_ledger;
      if (!ledger || typeof ledger !== "object") {
        throw new Error("SpacetimeContractError: Missing analytical_bookkeeping_ledger at states[" + idx + "]");
      }
      if (ledger.closes_by_algebraic_definition !== true) {
        throw new Error(
          "SpacetimeContractError: states[" + idx + "].analytical_bookkeeping_ledger.closes_by_algebraic_definition must be true"
        );
      }
      if (ledger.is_finite_volume_conservation_test !== false) {
        throw new Error(
          "SpacetimeContractError: states[" + idx + "].analytical_bookkeeping_ledger.is_finite_volume_conservation_test must be false in Stage 1"
        );
      }
      if (
        typeof ledger.analytical_bookkeeping_note !== "string" ||
        ledger.analytical_bookkeeping_note.trim().length === 0
      ) {
        throw new Error(
          "SpacetimeContractError: states[" + idx + "].analytical_bookkeeping_ledger.analytical_bookkeeping_note must be non-empty"
        );
      }
      var ledgerNumFields = [
        "initial_discrete_inventory_mol",
        "retained_discrete_inventory_mol",
        "analytical_surviving_reference_mol",
        "analytical_decayed_reference_mol",
        "unmodeled_boundary_and_quadrature_gap_mol",
        "relative_quadrature_and_tail_gap"
      ];
      for (var lf = 0; lf < ledgerNumFields.length; lf++) {
        var lfVal = ledger[ledgerNumFields[lf]];
        if (typeof lfVal !== "number" || !Number.isFinite(lfVal)) {
          throw new Error(
            "SpacetimeContractError: states[" + idx + "].analytical_bookkeeping_ledger." + ledgerNumFields[lf] + " must be finite"
          );
        }
      }
      if (ledger.relative_quadrature_and_tail_gap < 0) {
        throw new Error(
          "SpacetimeContractError: states[" + idx + "].analytical_bookkeeping_ledger.relative_quadrature_and_tail_gap must be >= 0"
        );
      }

      if (st.voxel_count !== expectedVoxels) {
        throw new Error("SpacetimeContractError: states[" + idx + "].voxel_count mismatch");
      }
      if (!Array.isArray(st.values) || st.values.length !== expectedVoxels) {
        throw new Error("SpacetimeContractError: states[" + idx + "].values length mismatch");
      }
      for (var v = 0; v < expectedVoxels; v++) {
        var val = st.values[v];
        if (typeof val !== "number" || !Number.isFinite(val) || !Number.isFinite(Math.fround(val))) {
          throw new Error(
            "SpacetimeContractError: Non-finite voxel value at states[" + idx + "].values[" + v + "]"
          );
        }
      }

      // Independently recompute derived summary statistics from hashed voxel array and spatial_frame
      var recomputed = computeVoxelSummaryStatistics(st.values, spatial);
      var sumNumFields = [
        "min_value",
        "max_value",
        "mean_value",
        "total_integrated_inventory_mol"
      ];
      for (var sf = 0; sf < sumNumFields.length; sf++) {
        var sfKey = sumNumFields[sf];
        if (typeof st[sfKey] !== "number" || !Number.isFinite(st[sfKey])) {
          throw new Error("SpacetimeContractError: states[" + idx + "]." + sfKey + " must be a finite number");
        }
      }
      if (Math.abs(st.min_value - recomputed.min_value) > 1e-6) {
        throw new Error(
          "SpacetimeContractError: states[" +
            idx +
            "].min_value (" +
            st.min_value +
            ") does not match recomputed voxel minimum (" +
            recomputed.min_value +
            ")"
        );
      }
      if (Math.abs(st.max_value - recomputed.max_value) > 1e-6) {
        throw new Error(
          "SpacetimeContractError: states[" +
            idx +
            "].max_value (" +
            st.max_value +
            ") does not match recomputed voxel maximum (" +
            recomputed.max_value +
            ")"
        );
      }
      if (Math.abs(st.mean_value - recomputed.mean_value) > 1e-6) {
        throw new Error(
          "SpacetimeContractError: states[" +
            idx +
            "].mean_value (" +
            st.mean_value +
            ") does not match recomputed voxel mean (" +
            recomputed.mean_value +
            ")"
        );
      }
      if (Math.abs(st.total_integrated_inventory_mol - recomputed.total_integrated_inventory_mol) > 1e-5) {
        throw new Error(
          "SpacetimeContractError: states[" +
            idx +
            "].total_integrated_inventory_mol (" +
            st.total_integrated_inventory_mol +
            ") does not match recomputed integrated voxel inventory (" +
            recomputed.total_integrated_inventory_mol +
            ")"
        );
      }
      if (
        !Array.isArray(st.center_of_mass_m) ||
        st.center_of_mass_m.length !== 3 ||
        typeof st.center_of_mass_m[0] !== "number" ||
        typeof st.center_of_mass_m[1] !== "number" ||
        typeof st.center_of_mass_m[2] !== "number" ||
        !Number.isFinite(st.center_of_mass_m[0]) ||
        !Number.isFinite(st.center_of_mass_m[1]) ||
        !Number.isFinite(st.center_of_mass_m[2])
      ) {
        throw new Error("SpacetimeContractError: states[" + idx + "].center_of_mass_m must be 3 finite numbers");
      }
      for (var cAxis = 0; cAxis < 3; cAxis++) {
        if (Math.abs(st.center_of_mass_m[cAxis] - recomputed.center_of_mass_m[cAxis]) > 1e-5) {
          throw new Error(
            "SpacetimeContractError: states[" +
              idx +
              "].center_of_mass_m[" +
              cAxis +
              "] (" +
              st.center_of_mass_m[cAxis] +
              ") does not match recomputed voxel center of mass (" +
              recomputed.center_of_mass_m[cAxis] +
              ")"
          );
        }
      }

      var stateIntegratedInventory = recomputed.total_integrated_inventory_mol;
      if (idx === 0) {
        initialIntegratedInventory = stateIntegratedInventory;
      }
      if (
        ledger.initial_discrete_inventory_mol <= 0 ||
        Math.abs(ledger.initial_discrete_inventory_mol - initialIntegratedInventory) > 1e-5
      ) {
        throw new Error(
          "SpacetimeContractError: states[" + idx + "].analytical_bookkeeping_ledger.initial_discrete_inventory_mol mismatch with state 0 integrated inventory"
        );
      }
      if (Math.abs(ledger.retained_discrete_inventory_mol - stateIntegratedInventory) > 1e-5) {
        throw new Error(
          "SpacetimeContractError: states[" + idx + "].analytical_bookkeeping_ledger.retained_discrete_inventory_mol mismatch with discrete integrated inventory"
        );
      }
      var expectedSurvRef = initialIntegratedInventory * Math.exp(-kDecay * st.simulation_time);
      var expectedDecRef = initialIntegratedInventory * (1.0 - Math.exp(-kDecay * st.simulation_time));
      if (Math.abs(ledger.analytical_surviving_reference_mol - expectedSurvRef) > 1e-5) {
        throw new Error(
          "SpacetimeContractError: states[" +
            idx +
            "].analytical_bookkeeping_ledger.analytical_surviving_reference_mol (" +
            ledger.analytical_surviving_reference_mol +
            ") does not match M0 * exp(-k_decay * simulation_time) (" +
            expectedSurvRef.toFixed(6) +
            ")"
        );
      }
      if (Math.abs(ledger.analytical_decayed_reference_mol - expectedDecRef) > 1e-5) {
        throw new Error(
          "SpacetimeContractError: states[" +
            idx +
            "].analytical_bookkeeping_ledger.analytical_decayed_reference_mol (" +
            ledger.analytical_decayed_reference_mol +
            ") does not match M0 * (1 - exp(-k_decay * simulation_time)) (" +
            expectedDecRef.toFixed(6) +
            ")"
        );
      }
      if (
        Math.abs(
          ledger.analytical_surviving_reference_mol +
            ledger.analytical_decayed_reference_mol -
            ledger.initial_discrete_inventory_mol
        ) > 1e-5
      ) {
        throw new Error(
          "SpacetimeContractError: states[" + idx + "].analytical_bookkeeping_ledger surviving + decayed does not close algebraically"
        );
      }
      var expectedGapMol = ledger.analytical_surviving_reference_mol - ledger.retained_discrete_inventory_mol;
      if (Math.abs(ledger.unmodeled_boundary_and_quadrature_gap_mol - expectedGapMol) > 1e-5) {
        throw new Error(
          "SpacetimeContractError: states[" + idx + "].analytical_bookkeeping_ledger.unmodeled_boundary_and_quadrature_gap_mol contradicts M_surv - M_retained"
        );
      }
      var expectedRelGap = Math.abs(expectedGapMol) / ledger.initial_discrete_inventory_mol;
      if (Math.abs(ledger.relative_quadrature_and_tail_gap - expectedRelGap) > 1e-5) {
        throw new Error(
          "SpacetimeContractError: states[" + idx + "].analytical_bookkeeping_ledger.relative_quadrature_and_tail_gap contradicts |M_surv - M_retained| / M0"
        );
      }

      var computedSha = sha256Float32LE(st.values);
      if (computedSha !== st.sha256_f32_le) {
        throw new Error(
          "SpacetimeContractError: Per-frame Float32LE SHA-256 checksum mismatch at states[" +
            idx +
            "]: expected " +
            st.sha256_f32_le +
            ", computed " +
            computedSha
        );
      }
      if (seenHashes[computedSha]) {
        throw new Error("SpacetimeContractError: Duplicate frame SHA-256 across distinct time states");
      }
      seenHashes[computedSha] = true;
    }

    var tBounds = temporal.time_bounds;
    if (
      !Array.isArray(tBounds) ||
      tBounds.length !== 2 ||
      tBounds[0] !== states[0].simulation_time ||
      tBounds[1] !== states[states.length - 1].simulation_time
    ) {
      throw new Error(
        "SpacetimeContractError: temporal_axis.time_bounds must match [states[0].simulation_time, states[M-1].simulation_time]"
      );
    }

    var evMarkers = temporal.event_markers;
    if (!Array.isArray(evMarkers) || evMarkers.length !== states.length) {
      throw new Error("SpacetimeContractError: temporal_axis.event_markers must be an array matching states.length");
    }
    for (var emIdx = 0; emIdx < evMarkers.length; emIdx++) {
      var em = evMarkers[emIdx];
      if (
        !em ||
        typeof em !== "object" ||
        em.state_index !== emIdx ||
        em.simulation_time !== states[emIdx].simulation_time ||
        typeof em.label !== "string" ||
        em.label.trim().length === 0 ||
        typeof em.event_type !== "string" ||
        em.event_type.trim().length === 0
      ) {
        throw new Error("SpacetimeContractError: Invalid temporal_axis.event_markers[" + emIdx + "]");
      }
    }

    var v2Comp = bundle.v2_compatibility;
    if (
      !v2Comp ||
      typeof v2Comp !== "object" ||
      v2Comp.schema_target !== "https://aurascoper.github.io/oklo_pool_mobile/schemas/epistemic_record_v1.json" ||
      !v2Comp.epistemic_record ||
      typeof v2Comp.epistemic_record !== "object"
    ) {
      throw new Error("SpacetimeContractError: Missing or invalid v2_compatibility.epistemic_record");
    }
    var v2Ep = v2Comp.epistemic_record;
    if (
      v2Ep.evidence_class !== bundle.evidence_class ||
      v2Ep.coordinate_system !== spatial.coordinate_system ||
      v2Ep.baseline_commit !== bundle.baseline_commit
    ) {
      throw new Error("SpacetimeContractError: v2_compatibility.epistemic_record metadata mismatch");
    }
    if (
      typeof v2Ep.source_revision !== "string" ||
      v2Ep.source_revision.trim().length === 0 ||
      v2Ep.source_revision === bundle.baseline_commit ||
      v2Ep.source_revision.indexOf(bundle.baseline_commit.slice(0, 7)) === 0
    ) {
      throw new Error(
        "SpacetimeContractError: v2_compatibility.epistemic_record.source_revision must be distinct from baseline_commit"
      );
    }

    if (!Array.isArray(bundle.criterial_pruned_branches) || bundle.criterial_pruned_branches.length < 1) {
      throw new Error("SpacetimeContractError: criterial_pruned_branches must contain at least 1 control branch");
    }
    for (var b = 0; b < bundle.criterial_pruned_branches.length; b++) {
      var br = bundle.criterial_pruned_branches[b];
      if (!br || br.criterial_status !== "criterial_pruned") {
        throw new Error("SpacetimeContractError: Invalid criterial_pruned_branches[" + b + "]");
      }
      if (
        typeof br.branch_id !== "string" ||
        br.branch_id.trim().length === 0 ||
        typeof br.label !== "string" ||
        br.label.trim().length === 0 ||
        typeof br.prune_reason !== "string" ||
        br.prune_reason.trim().length === 0
      ) {
        throw new Error("SpacetimeContractError: criterial_pruned_branches[" + b + "] missing required string fields");
      }
      if (
        typeof br.max_relative_analytical_drift !== "number" ||
        !Number.isFinite(br.max_relative_analytical_drift) ||
        typeof br.criterial_threshold_exceeded !== "number" ||
        !Number.isFinite(br.criterial_threshold_exceeded) ||
        br.criterial_threshold_exceeded <= 0 ||
        br.max_relative_analytical_drift <= br.criterial_threshold_exceeded
      ) {
        throw new Error(
          "SpacetimeContractError: criterial_pruned_branches[" + b + "] must exceed positive criterial threshold"
        );
      }
      if (!Array.isArray(br.time_points) || br.time_points.length !== states.length) {
        throw new Error(
          "SpacetimeContractError: criterial_pruned_branches[" + b + "].time_points length must match states.length"
        );
      }
      for (var tpIdx = 0; tpIdx < br.time_points.length; tpIdx++) {
        var tp = br.time_points[tpIdx];
        if (!tp || tp.state_index !== tpIdx || tp.simulation_time !== states[tpIdx].simulation_time) {
          throw new Error(
            "SpacetimeContractError: criterial_pruned_branches[" + b + "].time_points[" + tpIdx + "] time grid mismatch"
          );
        }
        if (
          !Array.isArray(tp.center_of_mass_m) ||
          tp.center_of_mass_m.length !== 3 ||
          typeof tp.center_of_mass_m[0] !== "number" ||
          typeof tp.center_of_mass_m[1] !== "number" ||
          typeof tp.center_of_mass_m[2] !== "number" ||
          !Number.isFinite(tp.center_of_mass_m[0]) ||
          !Number.isFinite(tp.center_of_mass_m[1]) ||
          !Number.isFinite(tp.center_of_mass_m[2]) ||
          typeof tp.probe_voxel_8_8_5_value !== "number" ||
          !Number.isFinite(tp.probe_voxel_8_8_5_value) ||
          typeof tp.probe_voxel_8_8_11_value !== "number" ||
          !Number.isFinite(tp.probe_voxel_8_8_11_value) ||
          typeof tp.relative_analytical_drift !== "number" ||
          !Number.isFinite(tp.relative_analytical_drift)
        ) {
          throw new Error(
            "SpacetimeContractError: criterial_pruned_branches[" + b + "].time_points[" + tpIdx + "] invalid numeric values"
          );
        }
      }
    }

    return {
      valid: true,
      bundle_id: bundle.bundle_id,
      state_count: states.length,
      voxel_count: expectedVoxels
    };
  }

  function selectTimeState(bundle, stateIndexOrPos, options) {
    var opts = options || {};
    if (!bundle || !Array.isArray(bundle.states) || bundle.states.length < 2) {
      throw new Error("SpacetimeContractError: Invalid bundle passed to selectTimeState");
    }
    if (!bundle.temporal_axis || bundle.temporal_axis.time_units !== "s") {
      throw new Error("SpacetimeContractError: temporal_axis.time_units must equal 's' in selectTimeState");
    }
    if (typeof stateIndexOrPos !== "number" || !Number.isFinite(stateIndexOrPos)) {
      throw new Error("SpacetimeContractError: Requested state index must be a finite number");
    }
    var maxIdx = bundle.states.length - 1;
    if (stateIndexOrPos < 0 || stateIndexOrPos > maxIdx) {
      throw new Error(
        "SpacetimeContractError: Requested state index " +
          stateIndexOrPos +
          " out of bounds [0, " +
          maxIdx +
          "] (missing_frame_policy: fail_closed_no_invented_states)"
      );
    }

    var dims = bundle.spatial_frame.dimensions_voxels;
    var expectedVoxels = dims[0] * dims[1] * dims[2];
    var rounded = Math.round(stateIndexOrPos);
    var isDiscrete = Math.abs(stateIndexOrPos - rounded) < 1e-9;

    if (isDiscrete) {
      var st = bundle.states[rounded];
      if (!st || st.state_index !== rounded) {
        throw new Error(
          "SpacetimeContractError: Missing discrete state at index " +
            rounded +
            " (fail_closed_no_invented_states)"
        );
      }
      if (st.time_units !== "s") {
        throw new Error("SpacetimeContractError: states[" + rounded + "].time_units must equal 's'");
      }
      if (!Array.isArray(st.values) || st.values.length !== expectedVoxels) {
        throw new Error("SpacetimeContractError: Corrupted voxel array at state " + rounded);
      }
      var f32 = new Float32Array(expectedVoxels);
      for (var i = 0; i < expectedVoxels; i++) {
        var v = st.values[i];
        if (typeof v !== "number" || !Number.isFinite(v) || !Number.isFinite(Math.fround(v))) {
          throw new Error("SpacetimeContractError: Non-finite voxel in state " + rounded);
        }
        f32[i] = v;
      }
      var actualSha = sha256Float32LE(f32);
      if (actualSha !== st.sha256_f32_le) {
        throw new Error(
          "SpacetimeContractError: Frame SHA-256 checksum verification failed at state " +
            rounded +
            " (expected " +
            st.sha256_f32_le +
            ", got " +
            actualSha +
            ")"
        );
      }
      var recomputed = computeVoxelSummaryStatistics(f32, bundle.spatial_frame);
      if (typeof st.min_value !== "number" || Math.abs(st.min_value - recomputed.min_value) > 1e-6) {
        throw new Error("SpacetimeContractError: states[" + rounded + "].min_value mismatch with voxel buffer");
      }
      if (typeof st.max_value !== "number" || Math.abs(st.max_value - recomputed.max_value) > 1e-6) {
        throw new Error("SpacetimeContractError: states[" + rounded + "].max_value mismatch with voxel buffer");
      }
      if (typeof st.mean_value !== "number" || Math.abs(st.mean_value - recomputed.mean_value) > 1e-6) {
        throw new Error("SpacetimeContractError: states[" + rounded + "].mean_value mismatch with voxel buffer");
      }
      if (
        typeof st.total_integrated_inventory_mol !== "number" ||
        Math.abs(st.total_integrated_inventory_mol - recomputed.total_integrated_inventory_mol) > 1e-5
      ) {
        throw new Error(
          "SpacetimeContractError: states[" + rounded + "].total_integrated_inventory_mol mismatch with voxel buffer"
        );
      }
      if (
        !Array.isArray(st.center_of_mass_m) ||
        st.center_of_mass_m.length !== 3 ||
        Math.abs(st.center_of_mass_m[0] - recomputed.center_of_mass_m[0]) > 1e-5 ||
        Math.abs(st.center_of_mass_m[1] - recomputed.center_of_mass_m[1]) > 1e-5 ||
        Math.abs(st.center_of_mass_m[2] - recomputed.center_of_mass_m[2]) > 1e-5
      ) {
        throw new Error("SpacetimeContractError: states[" + rounded + "].center_of_mass_m mismatch with voxel buffer");
      }

      var stLedger = st.analytical_bookkeeping_ledger;
      if (
        !stLedger ||
        typeof stLedger !== "object" ||
        stLedger.closes_by_algebraic_definition !== true ||
        stLedger.is_finite_volume_conservation_test !== false
      ) {
        throw new Error(
          "SpacetimeContractError: Invalid analytical_bookkeeping_ledger flags at states[" + rounded + "]"
        );
      }
      if (
        !bundle.governing_parameters ||
        typeof bundle.governing_parameters.decay_rate_s_inv !== "number" ||
        !Number.isFinite(bundle.governing_parameters.decay_rate_s_inv) ||
        bundle.governing_parameters.decay_rate_s_inv < 0
      ) {
        throw new Error("SpacetimeContractError: Invalid governing_parameters.decay_rate_s_inv in selectTimeState");
      }
      var m0Recomputed =
        rounded === 0
          ? recomputed.total_integrated_inventory_mol
          : computeVoxelSummaryStatistics(bundle.states[0].values, bundle.spatial_frame).total_integrated_inventory_mol;
      var kDec = bundle.governing_parameters.decay_rate_s_inv;
      var expSurv = m0Recomputed * Math.exp(-kDec * st.simulation_time);
      var expDec = m0Recomputed * (1.0 - Math.exp(-kDec * st.simulation_time));
      var expGap = stLedger.analytical_surviving_reference_mol - stLedger.retained_discrete_inventory_mol;
      var expRelGap = Math.abs(expGap) / m0Recomputed;
      if (
        typeof stLedger.initial_discrete_inventory_mol !== "number" ||
        Math.abs(stLedger.initial_discrete_inventory_mol - m0Recomputed) > 1e-5 ||
        typeof stLedger.retained_discrete_inventory_mol !== "number" ||
        Math.abs(stLedger.retained_discrete_inventory_mol - recomputed.total_integrated_inventory_mol) > 1e-5 ||
        typeof stLedger.analytical_surviving_reference_mol !== "number" ||
        Math.abs(stLedger.analytical_surviving_reference_mol - expSurv) > 1e-5 ||
        typeof stLedger.analytical_decayed_reference_mol !== "number" ||
        Math.abs(stLedger.analytical_decayed_reference_mol - expDec) > 1e-5 ||
        Math.abs(
          stLedger.analytical_surviving_reference_mol +
            stLedger.analytical_decayed_reference_mol -
            stLedger.initial_discrete_inventory_mol
        ) > 1e-5 ||
        typeof stLedger.unmodeled_boundary_and_quadrature_gap_mol !== "number" ||
        Math.abs(stLedger.unmodeled_boundary_and_quadrature_gap_mol - expGap) > 1e-5 ||
        typeof stLedger.relative_quadrature_and_tail_gap !== "number" ||
        Math.abs(stLedger.relative_quadrature_and_tail_gap - expRelGap) > 1e-5
      ) {
        throw new Error(
          "SpacetimeContractError: states[" +
            rounded +
            "].analytical_bookkeeping_ledger mismatch with recomputed voxel inventory or exponential decay reference"
        );
      }

      return {
        state_index: st.state_index,
        simulation_time: st.simulation_time,
        time_units: st.time_units,
        state_derivation: st.state_derivation,
        criterial_status: st.criterial_status,
        is_simulated_state: true,
        run_receipt: st.run_receipt,
        voxel_count: st.voxel_count,
        sha256_f32_le: actualSha,
        min_value: recomputed.min_value,
        max_value: recomputed.max_value,
        mean_value: recomputed.mean_value,
        total_integrated_inventory_mol: recomputed.total_integrated_inventory_mol,
        center_of_mass_m: recomputed.center_of_mass_m.slice(),
        analytical_bookkeeping_ledger: Object.assign({}, st.analytical_bookkeeping_ledger),
        display_warning: null,
        float32Values: f32
      };
    }

    // Fractional time position requested
    if (!opts.allowDisplayInterpolation) {
      throw new Error(
        "SpacetimeContractError: Fractional state position " +
          stateIndexOrPos +
          " rejected under interpolation_policy='none_discrete_states_only' when allowDisplayInterpolation is false"
      );
    }

    var idxLow = Math.floor(stateIndexOrPos);
    var idxHigh = Math.ceil(stateIndexOrPos);
    var alpha = stateIndexOrPos - idxLow;
    var rawLow = bundle.states[idxLow];
    var rawHigh = bundle.states[idxHigh];
    var dtRes = bundle.temporal_axis.temporal_resolution_s;
    if (
      !rawLow ||
      !rawHigh ||
      typeof dtRes !== "number" ||
      !Number.isFinite(dtRes) ||
      dtRes <= 0 ||
      Math.abs((rawHigh.simulation_time - rawLow.simulation_time) - dtRes) > 1e-6
    ) {
      throw new Error(
        "SpacetimeContractError: Inconsistent time step interval for display interpolation between states " +
          idxLow +
          " and " +
          idxHigh +
          " (dt=" +
          (rawHigh && rawLow ? rawHigh.simulation_time - rawLow.simulation_time : NaN) +
          " != temporal_resolution_s=" +
          dtRes +
          ")"
      );
    }
    var stLow = selectTimeState(bundle, idxLow, { allowDisplayInterpolation: false });
    var stHigh = selectTimeState(bundle, idxHigh, { allowDisplayInterpolation: false });

    var interpF32 = new Float32Array(expectedVoxels);
    for (var k = 0; k < expectedVoxels; k++) {
      interpF32[k] = (1.0 - alpha) * stLow.float32Values[k] + alpha * stHigh.float32Values[k];
    }

    var interpSummary = computeVoxelSummaryStatistics(interpF32, bundle.spatial_frame);
    var tInterp = (1.0 - alpha) * stLow.simulation_time + alpha * stHigh.simulation_time;

    return {
      state_index: Number(stateIndexOrPos.toFixed(4)),
      parent_state_indices: [idxLow, idxHigh],
      interpolation_alpha: Number(alpha.toFixed(4)),
      simulation_time: Number(tInterp.toFixed(3)),
      time_units: stLow.time_units,
      state_derivation: "interpolated_for_display",
      criterial_status: "unverified_exploratory_display",
      is_simulated_state: false,
      run_receipt: null,
      voxel_count: expectedVoxels,
      sha256_f32_le: "unverified_display_interpolation",
      min_value: interpSummary.min_value,
      max_value: interpSummary.max_value,
      mean_value: interpSummary.mean_value,
      total_integrated_inventory_mol: interpSummary.total_integrated_inventory_mol,
      center_of_mass_m: interpSummary.center_of_mass_m,
      analytical_bookkeeping_ledger: null,
      display_warning:
        "INTERPOLATED FOR DISPLAY ONLY: Linear visual blend between states " +
        idxLow +
        " and " +
        idxHigh +
        " (alpha=" +
        alpha.toFixed(2) +
        "); not a simulated or receipt-backed state.",
      float32Values: interpF32
    };
  }

  function computeDualTimeComparison(bundle, indexA, indexB) {
    var stateA = selectTimeState(bundle, indexA, { allowDisplayInterpolation: false });
    var stateB = selectTimeState(bundle, indexB, { allowDisplayInterpolation: false });
    var count = stateA.voxel_count;
    var diffF32 = new Float32Array(count);
    var sumSq = 0.0;
    var maxAbs = 0.0;
    var minDiff = Infinity;
    var maxDiff = -Infinity;

    for (var i = 0; i < count; i++) {
      var d = stateB.float32Values[i] - stateA.float32Values[i];
      diffF32[i] = d;
      sumSq += d * d;
      var absD = Math.abs(d);
      if (absD > maxAbs) maxAbs = absD;
      if (d < minDiff) minDiff = d;
      if (d > maxDiff) maxDiff = d;
    }

    var comShift = [
      Number((stateB.center_of_mass_m[0] - stateA.center_of_mass_m[0]).toFixed(6)),
      Number((stateB.center_of_mass_m[1] - stateA.center_of_mass_m[1]).toFixed(6)),
      Number((stateB.center_of_mass_m[2] - stateA.center_of_mass_m[2]).toFixed(6))
    ];

    return {
      index_a: stateA.state_index,
      index_b: stateB.state_index,
      time_a: stateA.simulation_time,
      time_b: stateB.simulation_time,
      delta_time_s: stateB.simulation_time - stateA.simulation_time,
      time_units: stateA.time_units,
      sha256_a: stateA.sha256_f32_le,
      sha256_b: stateB.sha256_f32_le,
      l2_norm: Math.sqrt(sumSq),
      linf_norm: maxAbs,
      min_diff: minDiff,
      max_diff: maxDiff,
      center_of_mass_shift_m: comShift,
      diffFloat32Values: diffF32
    };
  }

  function extractVoxelTimeSeries(bundle, i, j, k) {
    var dims = bundle.spatial_frame.dimensions_voxels;
    var nx = dims[0];
    var ny = dims[1];
    var nz = dims[2];
    if (
      !Number.isInteger(i) ||
      !Number.isInteger(j) ||
      !Number.isInteger(k) ||
      i < 0 ||
      i >= nx ||
      j < 0 ||
      j >= ny ||
      k < 0 ||
      k >= nz
    ) {
      throw new Error(
        "SpacetimeContractError: Voxel coordinates (" + i + ", " + j + ", " + k + ") out of bounds"
      );
    }
    var flatIdx = voxelIndex(i, j, k, nx, ny);
    var verifiedSeries = [];
    for (var s = 0; s < bundle.states.length; s++) {
      var st = bundle.states[s];
      verifiedSeries.push({
        state_index: st.state_index,
        simulation_time: st.simulation_time,
        time_units: st.time_units,
        value: st.values[flatIdx],
        state_derivation: st.state_derivation,
        criterial_status: st.criterial_status,
        sha256_f32_le: st.sha256_f32_le
      });
    }

    var prunedBranches = [];
    for (var b = 0; b < bundle.criterial_pruned_branches.length; b++) {
      var br = bundle.criterial_pruned_branches[b];
      var pts = [];
      for (var p = 0; p < br.time_points.length; p++) {
        var tp = br.time_points[p];
        var baseVal = bundle.states[p].values[flatIdx];
        var driftMult =
          1.0 + (typeof tp.relative_analytical_drift === "number" ? tp.relative_analytical_drift : 0.0);
        var prunedVal = baseVal * driftMult;
        if (i === 8 && j === 8 && k === 5) {
          prunedVal = tp.probe_voxel_8_8_5_value;
        } else if (i === 8 && j === 8 && k === 11) {
          prunedVal = tp.probe_voxel_8_8_11_value;
        }
        pts.push({
          state_index: tp.state_index,
          simulation_time: tp.simulation_time,
          value: prunedVal,
          relative_analytical_drift: tp.relative_analytical_drift,
          criterial_status: "criterial_pruned"
        });
      }
      prunedBranches.push({
        branch_id: br.branch_id,
        label: br.label,
        criterial_status: br.criterial_status,
        prune_reason: br.prune_reason,
        max_relative_analytical_drift: br.max_relative_analytical_drift,
        series: pts
      });
    }

    return {
      voxel: [i, j, k],
      flat_index: flatIdx,
      physical_coords_m: [i + 0.5, j + 0.5, k + 0.5],
      field_units: bundle.field_units,
      time_units: bundle.temporal_axis.time_units,
      verified_series: verifiedSeries,
      criterial_pruned_series: prunedBranches
    };
  }

  function extractOrthogonalSlice(float32Values, dims, plane, sliceIndex) {
    var nx = dims[0];
    var ny = dims[1];
    var nz = dims[2];
    if (plane === "XY") {
      if (sliceIndex < 0 || sliceIndex >= nz) {
        throw new Error("SpacetimeContractError: XY slice k index out of bounds");
      }
      var gridXY = new Float32Array(nx * ny);
      for (var j = 0; j < ny; j++) {
        for (var i = 0; i < nx; i++) {
          gridXY[i + nx * j] = float32Values[voxelIndex(i, j, sliceIndex, nx, ny)];
        }
      }
      return { plane: "XY", width: nx, height: ny, fixed_index: sliceIndex, axis_labels: ["x (i)", "y (j)"], data: gridXY };
    }
    if (plane === "XZ") {
      if (sliceIndex < 0 || sliceIndex >= ny) {
        throw new Error("SpacetimeContractError: XZ slice j index out of bounds");
      }
      var gridXZ = new Float32Array(nx * nz);
      for (var k = 0; k < nz; k++) {
        for (var x = 0; x < nx; x++) {
          gridXZ[x + nx * k] = float32Values[voxelIndex(x, sliceIndex, k, nx, ny)];
        }
      }
      return { plane: "XZ", width: nx, height: nz, fixed_index: sliceIndex, axis_labels: ["x (i)", "+k (z up)"], data: gridXZ };
    }
    if (plane === "YZ") {
      if (sliceIndex < 0 || sliceIndex >= nx) {
        throw new Error("SpacetimeContractError: YZ slice i index out of bounds");
      }
      var gridYZ = new Float32Array(ny * nz);
      for (var kz = 0; kz < nz; kz++) {
        for (var y = 0; y < ny; y++) {
          gridYZ[y + ny * kz] = float32Values[voxelIndex(sliceIndex, y, kz, nx, ny)];
        }
      }
      return { plane: "YZ", width: ny, height: nz, fixed_index: sliceIndex, axis_labels: ["y (j)", "+k (z up)"], data: gridYZ };
    }
    throw new Error("SpacetimeContractError: Unsupported orthogonal plane '" + plane + "'");
  }

  // 4D spatiotemporal projection: (x, y, z, w = t_norm) with +k (z) as vertical up-axis
  function project4DSpatiotemporalPoint(x, y, z, tNorm, params) {
    var cfg = params || {};
    var angleXT = typeof cfg.angleXT === "number" ? cfg.angleXT : 0.32;
    var angleZT = typeof cfg.angleZT === "number" ? cfg.angleZT : 0.22;
    var yaw = typeof cfg.yaw === "number" ? cfg.yaw : -0.62;
    var pitch = typeof cfg.pitch === "number" ? cfg.pitch : 0.38;

    // Normalize spatial coordinates from [0, 16] m to [-1, 1] and time w = tNorm from [0, 1] to [-1, 1]
    var xn = (x - 8.0) / 8.0;
    var yn = (y - 8.0) / 8.0;
    var zn = (z - 8.0) / 8.0; // +k vertical up axis
    var wn = (tNorm - 0.5) * 2.0; // 4th dimension w = continuous time evolution t

    // 4D rotation in the X-t hyperplane
    var cosXT = Math.cos(angleXT);
    var sinXT = Math.sin(angleXT);
    var x1 = xn * cosXT - wn * sinXT;
    var w1 = xn * sinXT + wn * cosXT;

    // 4D rotation in the Z-t (+k vs t) hyperplane
    var cosZT = Math.cos(angleZT);
    var sinZT = Math.sin(angleZT);
    var z2 = zn * cosZT - w1 * sinZT;
    var w2 = zn * sinZT + w1 * cosZT;

    // 4D perspective stereographic distance factor along w=t
    var distance4D = 3.2;
    var scale4D = distance4D / (distance4D - w2 * 0.55);

    var x3 = x1 * scale4D;
    var y3 = yn * scale4D;
    var z3 = z2 * scale4D; // +k remains vertical up in 3D space

    // 3D turntable rotation around +k vertical up-axis (yaw) and elevation (pitch)
    var cosYaw = Math.cos(yaw);
    var sinYaw = Math.sin(yaw);
    var xr = x3 * cosYaw - y3 * sinYaw;
    var yr = x3 * sinYaw + y3 * cosYaw;

    var cosPitch = Math.cos(pitch);
    var sinPitch = Math.sin(pitch);
    var screenX = xr;
    var screenY = z3 * cosPitch - yr * sinPitch; // +k projects upward on screen
    var depth = yr * cosPitch + z3 * sinPitch;

    return {
      screenX: screenX,
      screenY: screenY,
      depth: depth,
      wRotated: w2,
      scale4D: scale4D,
      fourthDimensionSymbol: "t"
    };
  }

  return {
    ALLOWED_EVIDENCE_CLASSES: ALLOWED_EVIDENCE_CLASSES,
    ALLOWED_STATE_DERIVATIONS: ALLOWED_STATE_DERIVATIONS,
    sha256Float32LE: sha256Float32LE,
    voxelIndex: voxelIndex,
    computeCenterOfMass: computeCenterOfMass,
    computeVoxelSummaryStatistics: computeVoxelSummaryStatistics,
    buildEmbeddedSyntheticBundle: buildEmbeddedSyntheticBundle,
    verifyBundleIntegrity: verifyBundleIntegrity,
    selectTimeState: selectTimeState,
    computeDualTimeComparison: computeDualTimeComparison,
    extractVoxelTimeSeries: extractVoxelTimeSeries,
    extractOrthogonalSlice: extractOrthogonalSlice,
    project4DSpatiotemporalPoint: project4DSpatiotemporalPoint
  };
});
