#!/usr/bin/env python3
"""Stage 0 validator and negative-control test runner for spacetime_geochemistry_v0.

Validates data/spacetime/synthetic_tracer_pulse_4d_v0.json against
data/spacetime/schema_4d_v0.json and executes 23 negative-control rejection checks.
Strictly zero em-dashes and zero en-dashes.
"""

from __future__ import annotations

import copy
import hashlib
import json
import math
import struct
import sys
from pathlib import Path
from typing import Any

REPO_ROOT = Path(__file__).resolve().parent.parent
SCHEMA_PATH = REPO_ROOT / "data" / "spacetime" / "schema_4d_v0.json"
BUNDLE_PATH = REPO_ROOT / "data" / "spacetime" / "synthetic_tracer_pulse_4d_v0.json"

ALLOWED_EVIDENCE_CLASSES = {
    "geographic_observation_or_locator",
    "sourced_geological_record",
    "prescribed_analytic_field",
    "numerically_verified_synthetic_model",
    "hypothesis_or_unvalidated_model",
    "empirically_calibrated_model",
}

ALLOWED_STATE_DERIVATIONS = {
    "computed",
    "measured",
    "prescribed_analytically",
    "interpolated_for_display",
}


class SpacetimeValidationError(ValueError):
    """Raised when a 4D spatiotemporal bundle violates Stage 0 contract rules."""


def compute_f32_le_sha256(values: list[float]) -> str:
    try:
        packed = struct.pack(f"<{len(values)}f", *values)
    except (struct.error, OverflowError, ValueError) as exc:
        raise SpacetimeValidationError(f"Failed to pack float32 buffer: {exc}") from exc
    return hashlib.sha256(packed).hexdigest()


def compute_voxel_summary_statistics(values: list[float], spatial_frame: dict[str, Any]) -> dict[str, Any]:
    dims = spatial_frame["dimensions_voxels"]
    nx, ny, nz = int(dims[0]), int(dims[1]), int(dims[2])
    bounds_m = spatial_frame["domain_bounds_m"]
    x_min, x_max = float(bounds_m["x"][0]), float(bounds_m["x"][1])
    y_min, y_max = float(bounds_m["y"][0]), float(bounds_m["y"][1])
    z_min, z_max = float(bounds_m["z_up"][0]), float(bounds_m["z_up"][1])

    try:
        packed = struct.pack(f"<{len(values)}f", *values)
        f32_values = struct.unpack(f"<{len(values)}f", packed)
    except (struct.error, OverflowError, ValueError) as exc:
        raise SpacetimeValidationError(f"Failed to pack float32 buffer for summary statistics: {exc}") from exc

    dx = (x_max - x_min) / float(nx)
    dy = (y_max - y_min) / float(ny)
    dz = (z_max - z_min) / float(nz)
    voxel_volume_m3 = dx * dy * dz

    min_v = float("inf")
    max_v = float("-inf")
    voxel_sum = 0.0
    sx = 0.0
    sy = 0.0
    sz = 0.0

    for k in range(nz):
        z = z_min + (float(k) + 0.5) * dz
        for j in range(ny):
            y = y_min + (float(j) + 0.5) * dy
            base_idx = nx * (j + ny * k)
            for i in range(nx):
                x = x_min + (float(i) + 0.5) * dx
                v = float(f32_values[base_idx + i])
                if v < min_v:
                    min_v = v
                if v > max_v:
                    max_v = v
                voxel_sum += v
                sx += x * v
                sy += y * v
                sz += z * v

    voxel_count = nx * ny * nz
    mean_v = voxel_sum / float(voxel_count)
    total_inventory_mol = voxel_sum * voxel_volume_m3
    if voxel_sum > 0.0:
        com = [round(sx / voxel_sum, 6), round(sy / voxel_sum, 6), round(sz / voxel_sum, 6)]
    else:
        com = [
            round(0.5 * (x_min + x_max), 6),
            round(0.5 * (y_min + y_max), 6),
            round(0.5 * (z_min + z_max), 6),
        ]

    return {
        "min_value": min_v,
        "max_value": max_v,
        "mean_value": mean_v,
        "voxel_sum": voxel_sum,
        "voxel_volume_m3": voxel_volume_m3,
        "total_integrated_inventory_mol": total_inventory_mol,
        "center_of_mass_m": com,
    }


def _check_exact_keys(obj: dict[str, Any], required_keys: set[str], context: str) -> None:
    if not isinstance(obj, dict):
        raise SpacetimeValidationError(f"{context} must be an object")
    missing = required_keys - set(obj.keys())
    if missing:
        raise SpacetimeValidationError(f"{context} missing required keys: {sorted(missing)}")
    extra = set(obj.keys()) - required_keys
    if extra:
        raise SpacetimeValidationError(f"{context} has unexpected additional keys: {sorted(extra)}")


def validate_bundle(bundle: dict[str, Any], schema: dict[str, Any]) -> None:
    top_required = set(schema["required"])
    _check_exact_keys(bundle, top_required, "root")

    if bundle.get("schema_version") != "spacetime_4d_v0":
        raise SpacetimeValidationError("schema_version must equal 'spacetime_4d_v0'")

    for str_field in (
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
        "scientific_status_disclaimer",
    ):
        val = bundle.get(str_field)
        if not isinstance(val, str) or not val.strip():
            raise SpacetimeValidationError(f"{str_field} must be a non-empty string")

    baseline_commit = bundle["baseline_commit"]
    if len(baseline_commit) < 7 or any(c not in "0123456789abcdef" for c in baseline_commit):
        raise SpacetimeValidationError("baseline_commit must be a lowercase hexadecimal git commit hash")

    # Source provenance validation (Finding 1: separate baseline from R10 generator revision)
    src_prov = bundle.get("source_provenance")
    src_prov_req = set(schema["properties"]["source_provenance"]["required"])
    _check_exact_keys(src_prov, src_prov_req, "source_provenance")
    for prov_field in (
        "target_branch",
        "baseline_commit",
        "generator_script",
        "generator_version",
        "generator_script_sha256",
        "tested_source_commit_policy",
    ):
        pval = src_prov.get(prov_field)
        if not isinstance(pval, str) or not pval.strip():
            raise SpacetimeValidationError(f"source_provenance.{prov_field} must be a non-empty string")

    if src_prov["baseline_commit"] != baseline_commit:
        raise SpacetimeValidationError("source_provenance.baseline_commit must match top-level baseline_commit")
    gen_ver = src_prov["generator_version"]
    if (
        gen_ver == baseline_commit
        or gen_ver.startswith(baseline_commit[:7])
        or (len(gen_ver) >= 7 and all(c in "0123456789abcdef" for c in gen_ver))
    ):
        raise SpacetimeValidationError(
            "source_provenance.generator_version must be a semantic generator identifier and must not conflate generator revision with baseline_commit or a raw commit hash"
        )
    gen_script_path = REPO_ROOT / src_prov["generator_script"]
    if not gen_script_path.is_file():
        raise SpacetimeValidationError(f"source_provenance.generator_script not found: {src_prov['generator_script']}")
    expected_gen_sha = hashlib.sha256(gen_script_path.read_bytes()).hexdigest()
    if src_prov["generator_script_sha256"] != expected_gen_sha:
        raise SpacetimeValidationError(
            f"source_provenance.generator_script_sha256 mismatch: expected {expected_gen_sha}, "
            f"got {src_prov['generator_script_sha256']}"
        )

    ev_class = bundle.get("evidence_class")
    if ev_class not in ALLOWED_EVIDENCE_CLASSES:
        raise SpacetimeValidationError(f"Invalid evidence_class: {ev_class!r}")
    if ev_class == "empirically_calibrated_model":
        raise SpacetimeValidationError(
            "Unauthorized evidence_class 'empirically_calibrated_model' in Stage 0/1 synthetic slice"
        )

    # Spatial frame validation
    spatial = bundle["spatial_frame"]
    spatial_req = set(schema["properties"]["spatial_frame"]["required"])
    _check_exact_keys(spatial, spatial_req, "spatial_frame")

    if not isinstance(spatial.get("coordinate_system"), str) or not spatial["coordinate_system"].strip():
        raise SpacetimeValidationError("spatial_frame.coordinate_system must be a non-empty string")
    if spatial.get("geometry_provenance") not in {"schematic", "surveyed_or_sourced"}:
        raise SpacetimeValidationError("spatial_frame.geometry_provenance must be 'schematic' or 'surveyed_or_sourced'")
    if spatial.get("up_axis") != "+k":
        raise SpacetimeValidationError("spatial_frame.up_axis must be '+k'")
    if spatial.get("voxel_order") != "x_fastest_then_y_then_k_up":
        raise SpacetimeValidationError("spatial_frame.voxel_order must be 'x_fastest_then_y_then_k_up'")

    dims = spatial.get("dimensions_voxels")
    if (
        not isinstance(dims, list)
        or len(dims) != 3
        or any(not isinstance(d, int) or isinstance(d, bool) or d <= 0 for d in dims)
    ):
        raise SpacetimeValidationError("spatial_frame.dimensions_voxels must be 3 positive integers")
    expected_voxels = dims[0] * dims[1] * dims[2]

    bounds_m = spatial.get("domain_bounds_m")
    if not isinstance(bounds_m, dict):
        raise SpacetimeValidationError("spatial_frame.domain_bounds_m must be an object")
    _check_exact_keys(bounds_m, {"x", "y", "z_up"}, "spatial_frame.domain_bounds_m")
    for axis_name in ("x", "y", "z_up"):
        ax_b = bounds_m.get(axis_name)
        if (
            not isinstance(ax_b, list)
            or len(ax_b) != 2
            or any(not isinstance(v, (int, float)) or isinstance(v, bool) or not math.isfinite(float(v)) for v in ax_b)
            or float(ax_b[0]) >= float(ax_b[1])
        ):
            raise SpacetimeValidationError(
                f"spatial_frame.domain_bounds_m.{axis_name} must be [min, max] with finite min < max"
            )

    # Governing parameters validation
    gov = bundle.get("governing_parameters")
    gov_req = set(schema["properties"]["governing_parameters"]["required"])
    _check_exact_keys(gov, gov_req, "governing_parameters")
    if not isinstance(gov.get("equation_summary"), str) or not gov["equation_summary"].strip():
        raise SpacetimeValidationError("governing_parameters.equation_summary must be a non-empty string")
    for gp_num in (
        "initial_peak_concentration_mol_m3",
        "initial_sigma_m",
        "diffusion_coefficient_m2_s",
        "decay_rate_s_inv",
    ):
        gval = gov.get(gp_num)
        if not isinstance(gval, (int, float)) or isinstance(gval, bool) or not math.isfinite(float(gval)):
            raise SpacetimeValidationError(f"governing_parameters.{gp_num} must be a finite number")
    if float(gov["decay_rate_s_inv"]) < 0.0:
        raise SpacetimeValidationError("governing_parameters.decay_rate_s_inv must be >= 0")
    k_decay = float(gov["decay_rate_s_inv"])
    for gp_vec in ("advection_velocity_m_s", "initial_center_m"):
        vec = gov.get(gp_vec)
        if (
            not isinstance(vec, list)
            or len(vec) != 3
            or any(not isinstance(x, (int, float)) or isinstance(x, bool) or not math.isfinite(float(x)) for x in vec)
        ):
            raise SpacetimeValidationError(f"governing_parameters.{gp_vec} must be a 3-number list")

    # Temporal axis validation (Finding 4: fixed-step seconds grid)
    temporal = bundle["temporal_axis"]
    temporal_req = set(schema["properties"]["temporal_axis"]["required"])
    _check_exact_keys(temporal, temporal_req, "temporal_axis")

    if temporal.get("fourth_dimension_symbol") != "t":
        raise SpacetimeValidationError("temporal_axis.fourth_dimension_symbol must equal 't'")
    time_units = temporal.get("time_units")
    if time_units != "s":
        raise SpacetimeValidationError("temporal_axis.time_units must equal 's' in spacetime_4d_v0")
    dt_res = temporal.get("temporal_resolution_s")
    if not isinstance(dt_res, (int, float)) or isinstance(dt_res, bool) or not math.isfinite(float(dt_res)) or float(dt_res) <= 0.0:
        raise SpacetimeValidationError("temporal_axis.temporal_resolution_s must be a finite positive number")
    dt_res_float = float(dt_res)

    if temporal.get("strict_monotonic_ordering") is not True:
        raise SpacetimeValidationError("temporal_axis.strict_monotonic_ordering must be True")
    if temporal.get("missing_frame_policy") != "fail_closed_no_invented_states":
        raise SpacetimeValidationError(
            "temporal_axis.missing_frame_policy must equal 'fail_closed_no_invented_states'"
        )
    interp_policy = temporal.get("interpolation_policy")
    if interp_policy not in {"none_discrete_states_only", "explicit_display_only_when_toggled"}:
        raise SpacetimeValidationError(f"Invalid temporal_axis.interpolation_policy: {interp_policy!r}")
    init_cond = temporal.get("initial_condition_summary")
    if not isinstance(init_cond, str) or not init_cond.strip():
        raise SpacetimeValidationError("temporal_axis.initial_condition_summary must be a non-empty string")

    # States validation
    states = bundle.get("states")
    if not isinstance(states, list) or len(states) < 2:
        raise SpacetimeValidationError("states must be a list with at least 2 time states")
    if len(states) != temporal.get("state_count"):
        raise SpacetimeValidationError("len(states) does not match temporal_axis.state_count")

    state_req = set(schema["properties"]["states"]["items"]["required"])
    ledger_req = set(
        schema["properties"]["states"]["items"]["properties"]["analytical_bookkeeping_ledger"]["required"]
    )
    prev_time: float | None = None
    seen_hashes: set[str] = set()
    initial_integrated_inventory = 0.0

    for idx, st in enumerate(states):
        _check_exact_keys(st, state_req, f"states[{idx}]")
        if st.get("state_index") != idx:
            raise SpacetimeValidationError(
                f"Missing or out-of-order state_index at states[{idx}]: got {st.get('state_index')}"
            )
        t_val = st.get("simulation_time")
        if not isinstance(t_val, (int, float)) or isinstance(t_val, bool) or not math.isfinite(float(t_val)):
            raise SpacetimeValidationError(f"states[{idx}].simulation_time must be a finite number")
        t_float = float(t_val)
        if prev_time is not None:
            if t_float <= prev_time:
                raise SpacetimeValidationError(
                    f"Non-monotonic simulation_time at states[{idx}]: {t_float} <= {prev_time}"
                )
            dt_step = t_float - prev_time
            if abs(dt_step - dt_res_float) > 1e-6:
                raise SpacetimeValidationError(
                    f"Inconsistent time step at states[{idx}]: dt={dt_step} != temporal_resolution_s={dt_res_float}"
                )
        prev_time = t_float

        if st.get("time_units") != "s":
            raise SpacetimeValidationError(f"states[{idx}].time_units must equal 's'")

        derivation = st.get("state_derivation")
        if derivation not in ALLOWED_STATE_DERIVATIONS:
            raise SpacetimeValidationError(f"states[{idx}] invalid state_derivation: {derivation!r}")
        if interp_policy == "none_discrete_states_only" and derivation == "interpolated_for_display":
            raise SpacetimeValidationError(
                f"states[{idx}] cannot have state_derivation='interpolated_for_display' under none_discrete_states_only"
            )
        if derivation == "interpolated_for_display":
            if st.get("run_receipt") is not None:
                raise SpacetimeValidationError(
                    f"states[{idx}] interpolated_for_display state must have run_receipt=None"
                )
            if st.get("is_simulated_state") is not False:
                raise SpacetimeValidationError(
                    f"states[{idx}] interpolated_for_display state must have is_simulated_state=False"
                )
        else:
            if st.get("is_simulated_state") is not True:
                raise SpacetimeValidationError(
                    f"states[{idx}] discrete state ({derivation}) must have is_simulated_state=True"
                )
            rec = st.get("run_receipt")
            if not isinstance(rec, str) or not rec.strip():
                raise SpacetimeValidationError(
                    f"states[{idx}] discrete state ({derivation}) must have a non-empty run_receipt string"
                )

        if st.get("criterial_status") != "verified_accepted":
            raise SpacetimeValidationError(
                f"states[{idx}] in primary states[] must have criterial_status='verified_accepted'"
            )

        # Analytical bookkeeping ledger validation (Finding 2)
        ledger = st.get("analytical_bookkeeping_ledger")
        _check_exact_keys(ledger, ledger_req, f"states[{idx}].analytical_bookkeeping_ledger")
        if ledger.get("closes_by_algebraic_definition") is not True:
            raise SpacetimeValidationError(
                f"states[{idx}].analytical_bookkeeping_ledger.closes_by_algebraic_definition must be True"
            )
        if ledger.get("is_finite_volume_conservation_test") is not False:
            raise SpacetimeValidationError(
                f"states[{idx}].analytical_bookkeeping_ledger.is_finite_volume_conservation_test must be False in Stage 1"
            )
        note = ledger.get("analytical_bookkeeping_note")
        if not isinstance(note, str) or not note.strip():
            raise SpacetimeValidationError(
                f"states[{idx}].analytical_bookkeeping_ledger.analytical_bookkeeping_note must be non-empty"
            )
        for num_field in (
            "initial_discrete_inventory_mol",
            "retained_discrete_inventory_mol",
            "analytical_surviving_reference_mol",
            "analytical_decayed_reference_mol",
            "unmodeled_boundary_and_quadrature_gap_mol",
            "relative_quadrature_and_tail_gap",
        ):
            nval = ledger.get(num_field)
            if not isinstance(nval, (int, float)) or isinstance(nval, bool) or not math.isfinite(float(nval)):
                raise SpacetimeValidationError(
                    f"states[{idx}].analytical_bookkeeping_ledger.{num_field} must be a finite number"
                )
        if float(ledger["relative_quadrature_and_tail_gap"]) < 0.0:
            raise SpacetimeValidationError(
                f"states[{idx}].analytical_bookkeeping_ledger.relative_quadrature_and_tail_gap must be >= 0"
            )

        if st.get("voxel_count") != expected_voxels:
            raise SpacetimeValidationError(
                f"states[{idx}].voxel_count={st.get('voxel_count')} != expected {expected_voxels}"
            )
        vals = st.get("values")
        if not isinstance(vals, list) or len(vals) != expected_voxels:
            raise SpacetimeValidationError(
                f"states[{idx}].values length={len(vals) if isinstance(vals, list) else 'non-list'} != {expected_voxels}"
            )
        for v_i, v in enumerate(vals):
            if not isinstance(v, (int, float)) or isinstance(v, bool) or not math.isfinite(float(v)):
                raise SpacetimeValidationError(
                    f"states[{idx}].values[{v_i}] is not a finite number: {v!r}"
                )

        # Independently recompute derived summary statistics from hashed voxel array and spatial_frame
        recomputed = compute_voxel_summary_statistics(vals, spatial)

        for sum_field in ("min_value", "max_value", "mean_value", "total_integrated_inventory_mol"):
            sval = st.get(sum_field)
            if not isinstance(sval, (int, float)) or isinstance(sval, bool) or not math.isfinite(float(sval)):
                raise SpacetimeValidationError(f"states[{idx}].{sum_field} must be a finite number")

        if abs(float(st["min_value"]) - recomputed["min_value"]) > 1e-6:
            raise SpacetimeValidationError(
                f"states[{idx}].min_value ({st['min_value']}) does not match recomputed voxel minimum ({recomputed['min_value']})"
            )
        if abs(float(st["max_value"]) - recomputed["max_value"]) > 1e-6:
            raise SpacetimeValidationError(
                f"states[{idx}].max_value ({st['max_value']}) does not match recomputed voxel maximum ({recomputed['max_value']})"
            )
        if abs(float(st["mean_value"]) - recomputed["mean_value"]) > 1e-6:
            raise SpacetimeValidationError(
                f"states[{idx}].mean_value ({st['mean_value']}) does not match recomputed voxel mean ({recomputed['mean_value']})"
            )
        if abs(float(st["total_integrated_inventory_mol"]) - recomputed["total_integrated_inventory_mol"]) > 1e-5:
            raise SpacetimeValidationError(
                f"states[{idx}].total_integrated_inventory_mol ({st['total_integrated_inventory_mol']}) "
                f"does not match recomputed integrated voxel inventory ({recomputed['total_integrated_inventory_mol']:.6f})"
            )

        com = st.get("center_of_mass_m")
        if (
            not isinstance(com, list)
            or len(com) != 3
            or any(not isinstance(c, (int, float)) or isinstance(c, bool) or not math.isfinite(float(c)) for c in com)
        ):
            raise SpacetimeValidationError(f"states[{idx}].center_of_mass_m must be 3 finite numbers")
        for axis_i, axis_label in enumerate(("x", "y", "z_up")):
            if abs(float(com[axis_i]) - recomputed["center_of_mass_m"][axis_i]) > 1e-5:
                raise SpacetimeValidationError(
                    f"states[{idx}].center_of_mass_m[{axis_i}] ({axis_label}={com[axis_i]}) "
                    f"does not match recomputed voxel center of mass ({recomputed['center_of_mass_m'][axis_i]:.6f})"
                )

        state_integrated_inventory = recomputed["total_integrated_inventory_mol"]
        if idx == 0:
            initial_integrated_inventory = state_integrated_inventory

        init_inv = float(ledger["initial_discrete_inventory_mol"])
        ret_inv = float(ledger["retained_discrete_inventory_mol"])
        surv_ref = float(ledger["analytical_surviving_reference_mol"])
        dec_ref = float(ledger["analytical_decayed_reference_mol"])
        gap_mol = float(ledger["unmodeled_boundary_and_quadrature_gap_mol"])
        rel_gap = float(ledger["relative_quadrature_and_tail_gap"])

        if init_inv <= 0.0 or abs(init_inv - initial_integrated_inventory) > 1e-5:
            raise SpacetimeValidationError(
                f"states[{idx}].analytical_bookkeeping_ledger.initial_discrete_inventory_mol ({init_inv}) "
                f"does not match states[0] discrete integrated inventory ({initial_integrated_inventory:.6f})"
            )
        if abs(ret_inv - state_integrated_inventory) > 1e-5:
            raise SpacetimeValidationError(
                f"states[{idx}].analytical_bookkeeping_ledger.retained_discrete_inventory_mol ({ret_inv}) "
                f"does not match discrete integrated inventory ({state_integrated_inventory:.6f})"
            )

        expected_surv_ref = initial_integrated_inventory * math.exp(-k_decay * t_float)
        expected_dec_ref = initial_integrated_inventory * (1.0 - math.exp(-k_decay * t_float))
        if abs(surv_ref - expected_surv_ref) > 1e-5:
            raise SpacetimeValidationError(
                f"states[{idx}].analytical_bookkeeping_ledger.analytical_surviving_reference_mol ({surv_ref}) "
                f"does not match M0 * exp(-k_decay * simulation_time) ({expected_surv_ref:.6f})"
            )
        if abs(dec_ref - expected_dec_ref) > 1e-5:
            raise SpacetimeValidationError(
                f"states[{idx}].analytical_bookkeeping_ledger.analytical_decayed_reference_mol ({dec_ref}) "
                f"does not match M0 * (1 - exp(-k_decay * simulation_time)) ({expected_dec_ref:.6f})"
            )

        if abs((surv_ref + dec_ref) - init_inv) > 1e-5:
            raise SpacetimeValidationError(
                f"states[{idx}].analytical_bookkeeping_ledger surviving + decayed ({surv_ref + dec_ref:.6f}) "
                f"does not close algebraically to initial inventory ({init_inv:.6f})"
            )
        expected_gap_mol = surv_ref - ret_inv
        if abs(gap_mol - expected_gap_mol) > 1e-5:
            raise SpacetimeValidationError(
                f"states[{idx}].analytical_bookkeeping_ledger.unmodeled_boundary_and_quadrature_gap_mol ({gap_mol}) "
                f"contradicts M_surv - M_retained ({expected_gap_mol:.6f})"
            )
        expected_rel_gap = abs(expected_gap_mol) / init_inv
        if abs(rel_gap - expected_rel_gap) > 1e-5:
            raise SpacetimeValidationError(
                f"states[{idx}].analytical_bookkeeping_ledger.relative_quadrature_and_tail_gap ({rel_gap}) "
                f"contradicts |M_surv - M_retained| / M0 ({expected_rel_gap:.6f})"
            )

        actual_sha = compute_f32_le_sha256(vals)
        if actual_sha != st.get("sha256_f32_le"):
            raise SpacetimeValidationError(
                f"states[{idx}].sha256_f32_le mismatch: expected {st.get('sha256_f32_le')}, computed {actual_sha}"
            )
        seen_hashes.add(actual_sha)

    if len(seen_hashes) != len(states):
        raise SpacetimeValidationError("Duplicate sha256_f32_le across supposedly distinct time states")

    t_bounds = temporal.get("time_bounds")
    if (
        not isinstance(t_bounds, list)
        or len(t_bounds) != 2
        or any(not isinstance(v, (int, float)) or isinstance(v, bool) or not math.isfinite(float(v)) for v in t_bounds)
        or float(t_bounds[0]) != float(states[0]["simulation_time"])
        or float(t_bounds[1]) != float(states[-1]["simulation_time"])
    ):
        raise SpacetimeValidationError(
            "temporal_axis.time_bounds must equal [states[0].simulation_time, states[-1].simulation_time]"
        )

    ev_markers = temporal.get("event_markers")
    if not isinstance(ev_markers, list) or len(ev_markers) != len(states):
        raise SpacetimeValidationError("temporal_axis.event_markers must be a list matching len(states)")
    em_req = set(schema["properties"]["temporal_axis"]["properties"]["event_markers"]["items"]["required"])
    for em_idx, em in enumerate(ev_markers):
        _check_exact_keys(em, em_req, f"temporal_axis.event_markers[{em_idx}]")
        s_i = em.get("state_index")
        if not isinstance(s_i, int) or isinstance(s_i, bool) or s_i != em_idx:
            raise SpacetimeValidationError(f"event_markers[{em_idx}].state_index mismatch: got {s_i}")
        if float(em.get("simulation_time", -1.0)) != float(states[s_i]["simulation_time"]):
            raise SpacetimeValidationError(
                f"event_markers[{em_idx}].simulation_time does not match states[{s_i}].simulation_time"
            )
        for em_str in ("label", "event_type"):
            eval_s = em.get(em_str)
            if not isinstance(eval_s, str) or not eval_s.strip():
                raise SpacetimeValidationError(f"event_markers[{em_idx}].{em_str} must be a non-empty string")

    # v2 compatibility block check
    v2_comp = bundle["v2_compatibility"]
    if not isinstance(v2_comp, dict) or v2_comp.get("schema_target") != "https://aurascoper.github.io/oklo_pool_mobile/schemas/epistemic_record_v1.json":
        raise SpacetimeValidationError("v2_compatibility.schema_target mismatch")
    v2_ep = v2_comp.get("epistemic_record", {})
    v2_ep_req = set(schema["properties"]["v2_compatibility"]["properties"]["epistemic_record"]["required"])
    _check_exact_keys(v2_ep, v2_ep_req, "v2_compatibility.epistemic_record")
    if v2_ep.get("evidence_class") != bundle["evidence_class"]:
        raise SpacetimeValidationError("v2_compatibility.epistemic_record.evidence_class mismatch")
    if v2_ep.get("coordinate_system") != spatial["coordinate_system"]:
        raise SpacetimeValidationError("v2_compatibility.epistemic_record.coordinate_system mismatch")
    if v2_ep.get("baseline_commit") != baseline_commit:
        raise SpacetimeValidationError("v2_compatibility.epistemic_record.baseline_commit mismatch")
    src_rev = v2_ep.get("source_revision")
    if not isinstance(src_rev, str) or not src_rev.strip() or src_rev == baseline_commit or src_rev.startswith(baseline_commit[:7]):
        raise SpacetimeValidationError(
            "v2_compatibility.epistemic_record.source_revision must be a non-empty string distinct from baseline_commit"
        )

    # Criterial pruned branches check
    pruned = bundle.get("criterial_pruned_branches")
    if not isinstance(pruned, list) or len(pruned) < 1:
        raise SpacetimeValidationError("criterial_pruned_branches must contain at least 1 control branch")
    branch_req = set(schema["properties"]["criterial_pruned_branches"]["items"]["required"])
    tp_req = set(
        schema["properties"]["criterial_pruned_branches"]["items"]["properties"]["time_points"]["items"]["required"]
    )
    for b_idx, br in enumerate(pruned):
        _check_exact_keys(br, branch_req, f"criterial_pruned_branches[{b_idx}]")
        if br.get("criterial_status") != "criterial_pruned":
            raise SpacetimeValidationError(
                f"criterial_pruned_branches[{b_idx}] must have criterial_status='criterial_pruned'"
            )
        for br_str in ("branch_id", "label", "prune_reason"):
            bval = br.get(br_str)
            if not isinstance(bval, str) or not bval.strip():
                raise SpacetimeValidationError(f"criterial_pruned_branches[{b_idx}] requires non-empty {br_str}")
        max_drift = br.get("max_relative_analytical_drift")
        thresh = br.get("criterial_threshold_exceeded")
        if (
            not isinstance(max_drift, (int, float))
            or isinstance(max_drift, bool)
            or not isinstance(thresh, (int, float))
            or isinstance(thresh, bool)
            or not math.isfinite(float(max_drift))
            or not math.isfinite(float(thresh))
            or float(thresh) <= 0.0
            or float(max_drift) <= float(thresh)
        ):
            raise SpacetimeValidationError(
                f"criterial_pruned_branches[{b_idx}] must have max_relative_analytical_drift > criterial_threshold_exceeded > 0"
            )
        tps = br.get("time_points")
        if not isinstance(tps, list) or len(tps) != len(states):
            raise SpacetimeValidationError(
                f"criterial_pruned_branches[{b_idx}].time_points length must match len(states)={len(states)}"
            )
        for p_idx, tp in enumerate(tps):
            _check_exact_keys(tp, tp_req, f"criterial_pruned_branches[{b_idx}].time_points[{p_idx}]")
            if tp.get("state_index") != p_idx:
                raise SpacetimeValidationError(
                    f"criterial_pruned_branches[{b_idx}].time_points[{p_idx}].state_index mismatch"
                )
            if float(tp.get("simulation_time", -1.0)) != float(states[p_idx]["simulation_time"]):
                raise SpacetimeValidationError(
                    f"criterial_pruned_branches[{b_idx}].time_points[{p_idx}].simulation_time mismatch"
                )
            tp_com = tp.get("center_of_mass_m")
            if (
                not isinstance(tp_com, list)
                or len(tp_com) != 3
                or any(not isinstance(c, (int, float)) or isinstance(c, bool) or not math.isfinite(float(c)) for c in tp_com)
            ):
                raise SpacetimeValidationError(
                    f"criterial_pruned_branches[{b_idx}].time_points[{p_idx}].center_of_mass_m must be 3 finite numbers"
                )
            for tp_num in ("probe_voxel_8_8_5_value", "probe_voxel_8_8_11_value", "relative_analytical_drift"):
                tnval = tp.get(tp_num)
                if not isinstance(tnval, (int, float)) or isinstance(tnval, bool) or not math.isfinite(float(tnval)):
                    raise SpacetimeValidationError(
                        f"criterial_pruned_branches[{b_idx}].time_points[{p_idx}].{tp_num} must be a finite number"
                    )


def run_negative_control_suite(base_bundle: dict[str, Any], schema: dict[str, Any]) -> list[tuple[str, bool, str]]:
    results: list[tuple[str, bool, str]] = []

    def expect_rejection(name: str, mutator: Any) -> None:
        mutated = copy.deepcopy(base_bundle)
        mutator(mutated)
        try:
            validate_bundle(mutated, schema)
        except SpacetimeValidationError as exc:
            results.append((name, True, str(exc)))
        else:
            results.append((name, False, "Validator failed to reject invalid bundle"))

    # 1. Non-monotonic simulation_time
    def mut_non_monotonic(b: dict[str, Any]) -> None:
        b["states"][2]["simulation_time"] = 200.0

    expect_rejection("1. Reject non-monotonic simulation_time (t_2 = 200.0 < t_1 = 250.0)", mut_non_monotonic)

    # 2. Non-'s' or empty time_units
    def mut_invalid_units(b: dict[str, Any]) -> None:
        b["temporal_axis"]["time_units"] = "minutes"

    expect_rejection("2. Reject non-'s' temporal_axis.time_units ('minutes' != 's')", mut_invalid_units)

    # 3. Invalid fourth_dimension_symbol
    def mut_wrong_4d_symbol(b: dict[str, Any]) -> None:
        b["temporal_axis"]["fourth_dimension_symbol"] = "w_arbitrary_distortion"

    expect_rejection("3. Reject invalid fourth_dimension_symbol != 't'", mut_wrong_4d_symbol)

    # 4. Voxel count / array length mismatch
    def mut_voxel_mismatch(b: dict[str, Any]) -> None:
        b["states"][1]["values"].pop()

    expect_rejection("4. Reject voxel array length mismatch (4095 != 4096)", mut_voxel_mismatch)

    # 5. Non-finite voxel value (NaN)
    def mut_nan_voxel(b: dict[str, Any]) -> None:
        b["states"][3]["values"][128] = float("nan")

    expect_rejection("5. Reject non-finite voxel value (NaN at voxel 128)", mut_nan_voxel)

    # 6. Corrupted sha256_f32_le digest
    def mut_bad_sha(b: dict[str, Any]) -> None:
        b["states"][0]["sha256_f32_le"] = "0" * 64

    expect_rejection("6. Reject corrupted sha256_f32_le frame digest", mut_bad_sha)

    # 7. Unauthorized interpolated_for_display state inside discrete bundle
    def mut_unlabeled_interp(b: dict[str, Any]) -> None:
        b["states"][2]["state_derivation"] = "interpolated_for_display"

    expect_rejection(
        "7. Reject state_derivation='interpolated_for_display' under none_discrete_states_only",
        mut_unlabeled_interp,
    )

    # 8. Unauthorized empirically_calibrated_model evidence class
    def mut_calibrated_claim(b: dict[str, Any]) -> None:
        b["evidence_class"] = "empirically_calibrated_model"
        b["v2_compatibility"]["epistemic_record"]["evidence_class"] = "empirically_calibrated_model"

    expect_rejection("8. Reject unauthorized evidence_class='empirically_calibrated_model'", mut_calibrated_claim)

    # 9. Missing intermediate frame (state_index gap)
    def mut_missing_frame_gap(b: dict[str, Any]) -> None:
        b["states"][3]["state_index"] = 4

    expect_rejection("9. Reject missing intermediate frame (state_index gap)", mut_missing_frame_gap)

    # 10. Discrete state missing run_receipt
    def mut_missing_receipt(b: dict[str, Any]) -> None:
        b["states"][1]["run_receipt"] = None

    expect_rejection("10. Reject discrete state with run_receipt=None", mut_missing_receipt)

    # 11. Mismatched temporal_axis.time_bounds
    def mut_bad_time_bounds(b: dict[str, Any]) -> None:
        b["temporal_axis"]["time_bounds"] = [0.0, 500.0]

    expect_rejection("11. Reject temporal_axis.time_bounds mismatch with states", mut_bad_time_bounds)

    # 12. Inverted spatial_frame.domain_bounds_m
    def mut_inverted_domain_bounds(b: dict[str, Any]) -> None:
        b["spatial_frame"]["domain_bounds_m"]["x"] = [16.0, 0.0]

    expect_rejection("12. Reject inverted spatial_frame.domain_bounds_m (x_min >= x_max)", mut_inverted_domain_bounds)

    # 13. Inconsistent time step interval vs temporal_resolution_s (Finding 4)
    def mut_non_uniform_step(b: dict[str, Any]) -> None:
        b["states"][2]["simulation_time"] = 600.0
        b["temporal_axis"]["event_markers"][2]["simulation_time"] = 600.0

    expect_rejection(
        "13. Reject inconsistent time step interval vs temporal_axis.temporal_resolution_s",
        mut_non_uniform_step,
    )

    # 14. False finite-volume conservation claim in Stage 1 ledger (Finding 2)
    def mut_false_fv_conservation_claim(b: dict[str, Any]) -> None:
        b["states"][0]["analytical_bookkeeping_ledger"]["is_finite_volume_conservation_test"] = True

    expect_rejection(
        "14. Reject false Stage 1 is_finite_volume_conservation_test=True claim",
        mut_false_fv_conservation_claim,
    )

    # 15. Conflated baseline_commit and generator revision (Finding 1)
    def mut_conflated_provenance(b: dict[str, Any]) -> None:
        b["source_provenance"]["generator_version"] = b["baseline_commit"][:7]

    expect_rejection(
        "15. Reject source_provenance conflating generator_version with baseline_commit or prefix",
        mut_conflated_provenance,
    )

    # 16. Fabricated zero quadrature/tail gap contradicting discrete voxel sum (Finding 2)
    def mut_fabricated_zero_gap(b: dict[str, Any]) -> None:
        b["states"][1]["analytical_bookkeeping_ledger"]["unmodeled_boundary_and_quadrature_gap_mol"] = 0.0
        b["states"][1]["analytical_bookkeeping_ledger"]["relative_quadrature_and_tail_gap"] = 0.0

    expect_rejection(
        "16. Reject fabricated 0.0 quadrature/tail gap contradicting discrete voxel sum",
        mut_fabricated_zero_gap,
    )

    # 17. Mismatched generator_script_sha256 vs scripts/generate_synthetic_4d_bundle.py on disk (Finding 1)
    def mut_bad_generator_sha(b: dict[str, Any]) -> None:
        b["source_provenance"]["generator_script_sha256"] = "a" * 64

    expect_rejection(
        "17. Reject source_provenance.generator_script_sha256 mismatch with generator script on disk",
        mut_bad_generator_sha,
    )

    # 18. Mutated min_value contradicting hashed voxel array (Derived metadata integrity)
    def mut_bad_min_value(b: dict[str, Any]) -> None:
        b["states"][1]["min_value"] = 0.25

    expect_rejection(
        "18. Reject mutated min_value contradicting hashed voxel buffer",
        mut_bad_min_value,
    )

    # 19. Mutated max_value contradicting hashed voxel array (Derived metadata integrity)
    def mut_bad_max_value(b: dict[str, Any]) -> None:
        b["states"][2]["max_value"] = 9.99

    expect_rejection(
        "19. Reject mutated max_value contradicting hashed voxel buffer",
        mut_bad_max_value,
    )

    # 20. Mutated mean_value contradicting hashed voxel array (Derived metadata integrity)
    def mut_bad_mean_value(b: dict[str, Any]) -> None:
        b["states"][3]["mean_value"] = 0.75

    expect_rejection(
        "20. Reject mutated mean_value contradicting hashed voxel buffer",
        mut_bad_mean_value,
    )

    # 21. Mutated total_integrated_inventory_mol contradicting hashed voxel array (Derived metadata integrity)
    def mut_bad_total_inventory(b: dict[str, Any]) -> None:
        b["states"][1]["total_integrated_inventory_mol"] = 500.0

    expect_rejection(
        "21. Reject mutated total_integrated_inventory_mol contradicting hashed voxel buffer",
        mut_bad_total_inventory,
    )

    # 22. Mutated center_of_mass_m contradicting hashed voxel array (Derived metadata integrity)
    def mut_bad_center_of_mass(b: dict[str, Any]) -> None:
        b["states"][4]["center_of_mass_m"] = [8.0, 8.0, 2.5]

    expect_rejection(
        "22. Reject mutated center_of_mass_m contradicting hashed voxel buffer",
        mut_bad_center_of_mass,
    )

    # 23. Mutated analytical surviving/decayed reference diverging from M0*exp(-k_decay*t) while algebraically closed
    def mut_bad_exponential_decay_ref(b: dict[str, Any]) -> None:
        led = b["states"][2]["analytical_bookkeeping_ledger"]
        m0 = float(led["initial_discrete_inventory_mol"])
        ret = float(led["retained_discrete_inventory_mol"])
        fake_surv = round(m0 * 0.95, 6)
        led["analytical_surviving_reference_mol"] = fake_surv
        led["analytical_decayed_reference_mol"] = round(m0 - fake_surv, 6)
        led["unmodeled_boundary_and_quadrature_gap_mol"] = round(fake_surv - ret, 6)
        led["relative_quadrature_and_tail_gap"] = round(abs(fake_surv - ret) / m0, 6)

    expect_rejection(
        "23. Reject analytical surviving/decayed reference diverging from M0*exp(-k_decay*t) despite algebraic closure",
        mut_bad_exponential_decay_ref,
    )

    return results


def main() -> int:
    schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))
    bundle = json.loads(BUNDLE_PATH.read_text(encoding="utf-8"))

    validate_bundle(bundle, schema)
    print(f"[PASS] Primary bundle validated: {BUNDLE_PATH.relative_to(REPO_ROOT)}")
    print(
        f"       bundle_id={bundle['bundle_id']} evidence_class={bundle['evidence_class']} "
        f"states={len(bundle['states'])} dimensions={bundle['spatial_frame']['dimensions_voxels']}"
    )

    neg_results = run_negative_control_suite(bundle, schema)
    failed = 0
    for name, passed, detail in neg_results:
        status = "PASS" if passed else "FAIL"
        print(f"[{status}] {name} -> {detail}")
        if not passed:
            failed += 1

    if failed > 0:
        print(f"Validation FAILED: {failed} negative control(s) did not reject invalid input.", file=sys.stderr)
        return 1

    print(
        f"Stage 0 contract validation complete: 1 primary bundle + {len(neg_results)}/{len(neg_results)} "
        "negative-control rejections passed."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
