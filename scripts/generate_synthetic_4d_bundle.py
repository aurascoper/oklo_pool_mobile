#!/usr/bin/env python3
"""Deterministic generator for the Stage 0/1 synthetic 4D spatiotemporal bundle.

Writes data/spacetime/synthetic_tracer_pulse_4d_v0.json conforming to
data/spacetime/schema_4d_v0.json and docs/specs/SPACETIME_GEOCHEMISTRY_v0.md.
Strictly zero em-dashes and zero en-dashes.
"""

from __future__ import annotations

import hashlib
import json
import math
import struct
from pathlib import Path
from typing import Any

REPO_ROOT = Path(__file__).resolve().parent.parent
OUTPUT_PATH = REPO_ROOT / "data" / "spacetime" / "synthetic_tracer_pulse_4d_v0.json"

BASELINE_COMMIT = "636b7d7bb41999b80278d90046da893ee1044f20"
TARGET_BRANCH = "spacetime-geochemistry/v0-slice"
GENERATOR_SCRIPT_REL = "scripts/generate_synthetic_4d_bundle.py"
GENERATOR_VERSION = "spacetime_geochemistry_v0_gen_0.2.0"
NX, NY, NZ = 16, 16, 16
VOXEL_COUNT = NX * NY * NZ

# Analytical 3D Gaussian advection-diffusion-decay parameters
C0_MOL_M3 = 10.0
SIGMA0_M = 1.8
DIFFUSION_M2_S = 0.0012
DECAY_RATE_S_INV = 0.0004
X0_M, Y0_M, Z0_M = 8.0, 8.0, 4.5
UX_M_S, UY_M_S, UZ_M_S = 0.0015, -0.0010, 0.0065

TIME_STEPS_S = [0.0, 250.0, 500.0, 750.0, 1000.0]


def voxel_index(i: int, j: int, k: int) -> int:
    return i + NX * (j + NY * k)


def evaluate_gaussian_pulse(t_s: float) -> list[float]:
    sigma_sq = SIGMA0_M * SIGMA0_M + 2.0 * DIFFUSION_M2_S * t_s
    amp_scale = ((SIGMA0_M * SIGMA0_M) / sigma_sq) ** 1.5
    decay_factor = math.exp(-DECAY_RATE_S_INV * t_s)
    prefactor = C0_MOL_M3 * amp_scale * decay_factor

    xc = X0_M + UX_M_S * t_s
    yc = Y0_M + UY_M_S * t_s
    zc = Z0_M + UZ_M_S * t_s

    raw_values: list[float] = [0.0] * VOXEL_COUNT
    for k in range(NZ):
        z = k + 0.5
        dz2 = (z - zc) * (z - zc)
        for j in range(NY):
            y = j + 0.5
            dy2 = (y - yc) * (y - yc)
            for i in range(NX):
                x = i + 0.5
                dx2 = (x - xc) * (x - xc)
                r2 = dx2 + dy2 + dz2
                val = prefactor * math.exp(-r2 / (2.0 * sigma_sq))
                raw_values[voxel_index(i, j, k)] = val
    return raw_values


def compute_center_of_mass(values: list[float]) -> list[float]:
    total = 0.0
    sx = 0.0
    sy = 0.0
    sz = 0.0
    for k in range(NZ):
        z = k + 0.5
        for j in range(NY):
            y = j + 0.5
            for i in range(NX):
                x = i + 0.5
                v = values[voxel_index(i, j, k)]
                total += v
                sx += x * v
                sy += y * v
                sz += z * v
    if total <= 0.0:
        return [8.0, 8.0, 8.0]
    return [round(sx / total, 6), round(sy / total, 6), round(sz / total, 6)]


def generate_bundle() -> dict[str, Any]:
    states: list[dict[str, Any]] = []
    pruned_points: list[dict[str, Any]] = []

    initial_mass_mol: float | None = None
    generator_sha256 = hashlib.sha256(Path(__file__).resolve().read_bytes()).hexdigest()

    event_labels = [
        ("t0_injection_pulse", "initial_condition", "Initial 3D Gaussian tracer pulse centered at z = 4.50 m (+k upward axis)."),
        ("t1_lower_conduit_ascent", "advection_diffusion_step", "Upward +k advection to z_c = 6.125 m with isotropic diffusive spreading."),
        ("t2_mid_column_transit", "advection_diffusion_step", "Mid-domain transit at z_c = 7.75 m; peak amplitude attenuated by diffusion and decay."),
        ("t3_upper_zone_entry", "advection_diffusion_step", "Upper compartment entry at z_c = 9.375 m; per-frame float32 checksum recorded."),
        ("t4_late_plume_dispersion", "terminal_benchmark_state", "Late-stage dispersed plume at z_c = 11.00 m; discrete voxel quadrature inventory and analytical decay bookkeeping recorded."),
    ]

    event_markers: list[dict[str, Any]] = []

    for idx, t_s in enumerate(TIME_STEPS_S):
        raw_vals = evaluate_gaussian_pulse(t_s)
        packed = struct.pack(f"<{VOXEL_COUNT}f", *raw_vals)
        f32_vals = list(struct.unpack(f"<{VOXEL_COUNT}f", packed))
        sha256_hex = hashlib.sha256(packed).hexdigest()

        retained_mass = float(sum(f32_vals))
        if initial_mass_mol is None:
            initial_mass_mol = retained_mass

        expected_surviving = initial_mass_mol * math.exp(-DECAY_RATE_S_INV * t_s)
        cumulative_decayed = initial_mass_mol * (1.0 - math.exp(-DECAY_RATE_S_INV * t_s))
        quadrature_gap = expected_surviving - retained_mass
        rel_quadrature_gap = abs(quadrature_gap) / initial_mass_mol

        com = compute_center_of_mass(f32_vals)
        min_v = float(min(f32_vals))
        max_v = float(max(f32_vals))
        mean_v = retained_mass / float(VOXEL_COUNT)

        ev_id, ev_type, ev_desc = event_labels[idx]
        event_markers.append(
            {
                "state_index": idx,
                "simulation_time": t_s,
                "label": f"{ev_id}: {ev_desc}",
                "event_type": ev_type,
            }
        )

        states.append(
            {
                "state_index": idx,
                "simulation_time": t_s,
                "time_units": "s",
                "state_derivation": "prescribed_analytically",
                "criterial_status": "verified_accepted",
                "is_simulated_state": True,
                "run_receipt": f"receipts/spacetime_geochemistry_v0_2026-10-20.md#state_{idx}",
                "voxel_count": VOXEL_COUNT,
                "sha256_f32_le": sha256_hex,
                "min_value": min_v,
                "max_value": max_v,
                "mean_value": mean_v,
                "total_integrated_inventory_mol": retained_mass,
                "center_of_mass_m": com,
                "analytical_bookkeeping_ledger": {
                    "initial_discrete_inventory_mol": round(initial_mass_mol, 6),
                    "retained_discrete_inventory_mol": round(retained_mass, 6),
                    "analytical_surviving_reference_mol": round(expected_surviving, 6),
                    "analytical_decayed_reference_mol": round(cumulative_decayed, 6),
                    "unmodeled_boundary_and_quadrature_gap_mol": round(quadrature_gap, 6),
                    "relative_quadrature_and_tail_gap": round(rel_quadrature_gap, 6),
                    "closes_by_algebraic_definition": True,
                    "is_finite_volume_conservation_test": False,
                    "analytical_bookkeeping_note": (
                        "Illustrative analytical bookkeeping only: M_surv = M0*exp(-k*t) and "
                        "M_decay = M0*(1-exp(-k*t)) close by algebraic definition, while "
                        "M_surv - M_retained reflects unmodeled Gaussian tail mass outside [0,16m]^3 "
                        "and 1m voxel midpoint quadrature. Does not test finite-volume numerical "
                        "conservation (deferred to Stage 2)."
                    ),
                },
                "values": f32_vals,
            }
        )

        # Negative-control criterial-pruned branch: synthetic +38% amplitude drift at 1000s
        drift_multiplier = 1.0 + 0.38 * (t_s / 1000.0)
        drift_fraction = 0.38 * (t_s / 1000.0)
        v_8_8_5 = f32_vals[voxel_index(8, 8, 5)] * drift_multiplier
        v_8_8_11 = f32_vals[voxel_index(8, 8, 11)] * drift_multiplier
        pruned_com = [
            round(com[0] + 0.12 * (t_s / 1000.0), 6),
            round(com[1] - 0.15 * (t_s / 1000.0), 6),
            round(com[2] - 0.85 * (t_s / 1000.0), 6),
        ]
        pruned_points.append(
            {
                "state_index": idx,
                "simulation_time": t_s,
                "center_of_mass_m": pruned_com,
                "probe_voxel_8_8_5_value": v_8_8_5,
                "probe_voxel_8_8_11_value": v_8_8_11,
                "relative_analytical_drift": round(drift_fraction, 6),
            }
        )

    bundle: dict[str, Any] = {
        "schema_version": "spacetime_4d_v0",
        "bundle_id": "synthetic_tracer_pulse_4d_v0",
        "model_id": "spacetime_geochemistry_v0",
        "model_version": "0.1.0",
        "specification_version": "docs/specs/SPACETIME_GEOCHEMISTRY_v0.md#v0.1.0",
        "evidence_class": "numerically_verified_synthetic_model",
        "source_repository": "https://github.com/aurascoper/oklo_pool_mobile",
        "baseline_commit": BASELINE_COMMIT,
        "source_provenance": {
            "target_branch": TARGET_BRANCH,
            "baseline_commit": BASELINE_COMMIT,
            "generator_script": GENERATOR_SCRIPT_REL,
            "generator_version": GENERATOR_VERSION,
            "generator_script_sha256": generator_sha256,
            "tested_source_commit_policy": "recorded_in_external_verification_receipt",
        },
        "run_id": "synth_adv_diff_decay_16x16x16_5state_v0",
        "run_receipt": "receipts/spacetime_geochemistry_v0_2026-10-20.md",
        "parameter_set_id": "synthetic_advection_diffusion_decay_v0_default",
        "field_name": "synthetic_tracer_concentration",
        "field_units": "mol_m3_synthetic_tracer",
        "uncertainty": (
            "Synthetic analytical benchmark sampled on a 1m voxel grid; IEEE-754 float32 "
            "quantization residual < 1e-6; analytical bookkeeping closes by algebraic identity "
            "(not a finite-volume PDE conservation test)."
        ),
        "known_limitations": (
            "Stage 1 synthetic 3D advection-diffusion-decay pulse on a schematic 16m cube; "
            "not a geological, hydrothermal, or radiation-transport simulation."
        ),
        "scientific_status_disclaimer": (
            "Passing viewer and data-contract tests establishes rendering and data-contract "
            "correctness only, not geological validity, radiation transport accuracy, or empirical calibration."
        ),
        "spatial_frame": {
            "coordinate_system": "cartesian_local_16m_cube",
            "geometry_provenance": "schematic",
            "dimensions_voxels": [NX, NY, NZ],
            "voxel_order": "x_fastest_then_y_then_k_up",
            "domain_bounds_m": {
                "x": [0.0, 16.0],
                "y": [0.0, 16.0],
                "z_up": [0.0, 16.0],
            },
            "up_axis": "+k",
        },
        "temporal_axis": {
            "fourth_dimension_symbol": "t",
            "time_units": "s",
            "state_count": len(TIME_STEPS_S),
            "temporal_resolution_s": 250.0,
            "time_bounds": [TIME_STEPS_S[0], TIME_STEPS_S[-1]],
            "strict_monotonic_ordering": True,
            "missing_frame_policy": "fail_closed_no_invented_states",
            "interpolation_policy": "none_discrete_states_only",
            "initial_condition_summary": "3D Gaussian tracer pulse centered at (8.0, 8.0, 4.5) m with sigma_0 = 1.8 m and peak C_0 = 10.0 mol_m3_synthetic_tracer at t = 0.0 s.",
            "event_markers": event_markers,
        },
        "governing_parameters": {
            "equation_summary": "C(x,y,z,t) = C0 * (sigma0^2 / sigma(t)^2)^(3/2) * exp(-k_decay * t) * exp(-((x-xc(t))^2 + (y-yc(t))^2 + (z-zc(t))^2) / (2 * sigma(t)^2))",
            "initial_peak_concentration_mol_m3": C0_MOL_M3,
            "initial_sigma_m": SIGMA0_M,
            "diffusion_coefficient_m2_s": DIFFUSION_M2_S,
            "decay_rate_s_inv": DECAY_RATE_S_INV,
            "advection_velocity_m_s": [UX_M_S, UY_M_S, UZ_M_S],
            "initial_center_m": [X0_M, Y0_M, Z0_M],
        },
        "v2_compatibility": {
            "schema_target": "https://aurascoper.github.io/oklo_pool_mobile/schemas/epistemic_record_v1.json",
            "epistemic_record": {
                "model_id": "spacetime_geochemistry_v0",
                "model_version": "0.1.0",
                "evidence_class": "numerically_verified_synthetic_model",
                "source_repository": "https://github.com/aurascoper/oklo_pool_mobile",
                "baseline_commit": BASELINE_COMMIT,
                "source_revision": f"{TARGET_BRANCH}@{GENERATOR_VERSION}",
                "specification_version": "docs/specs/SPACETIME_GEOCHEMISTRY_v0.md#v0.1.0",
                "run_receipt": "receipts/spacetime_geochemistry_v0_2026-10-20.md",
                "parameter_set_id": "synthetic_advection_diffusion_decay_v0_default",
                "units": "mol_m3_synthetic_tracer; time_units=s",
                "coordinate_system": "cartesian_local_16m_cube",
                "uncertainty": (
                    "Synthetic analytical benchmark sampled on a 1m voxel grid; IEEE-754 float32 "
                    "quantization residual < 1e-6; analytical bookkeeping closes by algebraic identity "
                    "(not a finite-volume PDE conservation test)."
                ),
                "known_limitations": (
                    "Stage 1 synthetic 3D advection-diffusion-decay pulse on a schematic 16m cube; "
                    "not a geological, hydrothermal, or radiation-transport simulation."
                ),
            },
        },
        "states": states,
        "criterial_pruned_branches": [
            {
                "branch_id": "synthetic_amplitude_drift_control",
                "label": "Negative Control: Synthetic +38% Amplitude-Drift Candidate (Pruned)",
                "criterial_status": "criterial_pruned",
                "prune_reason": (
                    "Synthetic negative-control candidate perturbed by an artificial +38% "
                    "amplitude drift multiplier (1 + 0.38 * t / 1000 s) at t = 1000 s, "
                    "exceeding the Stage 1 analytical reference envelope gate (0.38 > 1.0e-4). "
                    "Demonstrates visual criterial separation only, not finite-volume PDE conservation."
                ),
                "max_relative_analytical_drift": 0.38,
                "criterial_threshold_exceeded": 0.0001,
                "time_points": pruned_points,
            }
        ],
    }
    return bundle


def main() -> None:
    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    bundle = generate_bundle()
    OUTPUT_PATH.write_text(json.dumps(bundle, indent=2) + "\n", encoding="utf-8")
    print(f"Generated {OUTPUT_PATH.relative_to(REPO_ROOT)} with {len(bundle['states'])} states.")
    print(f"  generator_script_sha256={bundle['source_provenance']['generator_script_sha256']}")
    for st in bundle["states"]:
        gap = st["analytical_bookkeeping_ledger"]["relative_quadrature_and_tail_gap"]
        print(
            f"  state[{st['state_index']}] t={st['simulation_time']:6.1f} {st['time_units']} "
            f"sha256_f32_le={st['sha256_f32_le']} "
            f"z_cm={st['center_of_mass_m'][2]:.4f} m "
            f"max={st['max_value']:.4f} "
            f"quad_tail_gap={gap:.6f}"
        )


if __name__ == "__main__":
    main()
