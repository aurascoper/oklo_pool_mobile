#!/usr/bin/env python3
"""Stage 0 validator and negative-control test runner for spacetime_geochemistry_v0.

Validates data/spacetime/synthetic_tracer_pulse_4d_v0.json against
data/spacetime/schema_4d_v0.json and executes 9 negative-control rejection checks.
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
        "source_commit",
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

    # Temporal axis validation
    temporal = bundle["temporal_axis"]
    temporal_req = set(schema["properties"]["temporal_axis"]["required"])
    _check_exact_keys(temporal, temporal_req, "temporal_axis")

    if temporal.get("fourth_dimension_symbol") != "t":
        raise SpacetimeValidationError("temporal_axis.fourth_dimension_symbol must equal 't'")
    time_units = temporal.get("time_units")
    if not isinstance(time_units, str) or not time_units.strip():
        raise SpacetimeValidationError("temporal_axis.time_units must be a non-empty string")
    if temporal.get("strict_monotonic_ordering") is not True:
        raise SpacetimeValidationError("temporal_axis.strict_monotonic_ordering must be True")
    if temporal.get("missing_frame_policy") != "fail_closed_no_invented_states":
        raise SpacetimeValidationError(
            "temporal_axis.missing_frame_policy must equal 'fail_closed_no_invented_states'"
        )
    interp_policy = temporal.get("interpolation_policy")
    if interp_policy not in {"none_discrete_states_only", "explicit_display_only_when_toggled"}:
        raise SpacetimeValidationError(f"Invalid temporal_axis.interpolation_policy: {interp_policy!r}")

    # States validation
    states = bundle.get("states")
    if not isinstance(states, list) or len(states) < 2:
        raise SpacetimeValidationError("states must be a list with at least 2 time states")
    if len(states) != temporal.get("state_count"):
        raise SpacetimeValidationError("len(states) does not match temporal_axis.state_count")

    state_req = set(schema["properties"]["states"]["items"]["required"])
    prev_time: float | None = None
    seen_hashes: set[str] = set()

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
        if prev_time is not None and t_float <= prev_time:
            raise SpacetimeValidationError(
                f"Non-monotonic simulation_time at states[{idx}]: {t_float} <= {prev_time}"
            )
        prev_time = t_float

        if st.get("time_units") != time_units:
            raise SpacetimeValidationError(f"states[{idx}].time_units mismatch with temporal_axis.time_units")

        derivation = st.get("state_derivation")
        if derivation not in ALLOWED_STATE_DERIVATIONS:
            raise SpacetimeValidationError(f"states[{idx}] invalid state_derivation: {derivation!r}")
        if interp_policy == "none_discrete_states_only" and derivation == "interpolated_for_display":
            raise SpacetimeValidationError(
                f"states[{idx}] cannot have state_derivation='interpolated_for_display' under none_discrete_states_only"
            )
        if derivation == "interpolated_for_display" and st.get("run_receipt") is not None:
            raise SpacetimeValidationError(
                f"states[{idx}] interpolated_for_display state must have run_receipt=None"
            )

        if st.get("criterial_status") != "verified_accepted":
            raise SpacetimeValidationError(
                f"states[{idx}] in primary states[] must have criterial_status='verified_accepted'"
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

        actual_sha = compute_f32_le_sha256(vals)
        if actual_sha != st.get("sha256_f32_le"):
            raise SpacetimeValidationError(
                f"states[{idx}].sha256_f32_le mismatch: expected {st.get('sha256_f32_le')}, computed {actual_sha}"
            )
        seen_hashes.add(actual_sha)

    if len(seen_hashes) != len(states):
        raise SpacetimeValidationError("Duplicate sha256_f32_le across supposedly distinct time states")

    # v2 compatibility block check
    v2_comp = bundle["v2_compatibility"]
    v2_ep = v2_comp.get("epistemic_record", {})
    if v2_ep.get("evidence_class") != bundle["evidence_class"]:
        raise SpacetimeValidationError("v2_compatibility.epistemic_record.evidence_class mismatch")
    if v2_ep.get("coordinate_system") != spatial["coordinate_system"]:
        raise SpacetimeValidationError("v2_compatibility.epistemic_record.coordinate_system mismatch")

    # Criterial pruned branches check
    pruned = bundle.get("criterial_pruned_branches")
    if not isinstance(pruned, list) or len(pruned) < 1:
        raise SpacetimeValidationError("criterial_pruned_branches must contain at least 1 control branch")
    for b_idx, br in enumerate(pruned):
        if br.get("criterial_status") != "criterial_pruned":
            raise SpacetimeValidationError(
                f"criterial_pruned_branches[{b_idx}] must have criterial_status='criterial_pruned'"
            )
        if not isinstance(br.get("prune_reason"), str) or not br["prune_reason"].strip():
            raise SpacetimeValidationError(f"criterial_pruned_branches[{b_idx}] requires non-empty prune_reason")


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

    # 2. Empty or missing time_units / field_units
    def mut_empty_units(b: dict[str, Any]) -> None:
        b["temporal_axis"]["time_units"] = "   "

    expect_rejection("2. Reject empty/whitespace temporal_axis.time_units", mut_empty_units)

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
