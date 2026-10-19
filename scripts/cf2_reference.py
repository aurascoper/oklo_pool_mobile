#!/usr/bin/env python3
"""
Independent CF-2 reference generator in pure Python 3.
Derived strictly from CONFORMANCE_FIXTURE_SPEC.md Section 3.
Evaluates 20 cases across 6 fields (120 volumes) on 32x32x32 grid.
Computes SHA-256 digests and statistics for cross-language verification.
"""

import argparse
import hashlib
import json
import math
import struct
import sys

NX = 32
NY = 32
NZ = 32
TOTAL_VOXELS = NX * NY * NZ  # 32768

FIELD_NAMES = [
    "temperature",
    "hydration",
    "h2_concentration",
    "u235_ratio",
    "polymer_yield",
    "compartments",
]

CF2_CASES = [
    {"index": 1,  "id": "CF2-01", "phase": 0.05, "null_scenario": False, "pulse_boundary": 0.1667},
    {"index": 2,  "id": "CF2-02", "phase": 0.05, "null_scenario": False, "pulse_boundary": 0.1700},
    {"index": 3,  "id": "CF2-03", "phase": 0.05, "null_scenario": True,  "pulse_boundary": 0.1667},
    {"index": 4,  "id": "CF2-04", "phase": 0.05, "null_scenario": True,  "pulse_boundary": 0.1700},
    {"index": 5,  "id": "CF2-05", "phase": 0.15, "null_scenario": False, "pulse_boundary": 0.1667},
    {"index": 6,  "id": "CF2-06", "phase": 0.15, "null_scenario": False, "pulse_boundary": 0.1700},
    {"index": 7,  "id": "CF2-07", "phase": 0.15, "null_scenario": True,  "pulse_boundary": 0.1667},
    {"index": 8,  "id": "CF2-08", "phase": 0.15, "null_scenario": True,  "pulse_boundary": 0.1700},
    {"index": 9,  "id": "CF2-09", "phase": 0.20, "null_scenario": False, "pulse_boundary": 0.1667},
    {"index": 10, "id": "CF2-10", "phase": 0.20, "null_scenario": False, "pulse_boundary": 0.1700},
    {"index": 11, "id": "CF2-11", "phase": 0.20, "null_scenario": True,  "pulse_boundary": 0.1667},
    {"index": 12, "id": "CF2-12", "phase": 0.20, "null_scenario": True,  "pulse_boundary": 0.1700},
    {"index": 13, "id": "CF2-13", "phase": 0.50, "null_scenario": False, "pulse_boundary": 0.1667},
    {"index": 14, "id": "CF2-14", "phase": 0.50, "null_scenario": False, "pulse_boundary": 0.1700},
    {"index": 15, "id": "CF2-15", "phase": 0.50, "null_scenario": True,  "pulse_boundary": 0.1667},
    {"index": 16, "id": "CF2-16", "phase": 0.50, "null_scenario": True,  "pulse_boundary": 0.1700},
    {"index": 17, "id": "CF2-17", "phase": 0.90, "null_scenario": False, "pulse_boundary": 0.1667},
    {"index": 18, "id": "CF2-18", "phase": 0.90, "null_scenario": False, "pulse_boundary": 0.1700},
    {"index": 19, "id": "CF2-19", "phase": 0.90, "null_scenario": True,  "pulse_boundary": 0.1667},
    {"index": 20, "id": "CF2-20", "phase": 0.90, "null_scenario": True,  "pulse_boundary": 0.1700},
]


def ref_pulse(phase: float, pulse_boundary: float) -> float:
    if phase < pulse_boundary:
        return math.sin((phase / pulse_boundary) * math.pi)
    else:
        return max(0.0, math.exp(-8.0 * (phase - pulse_boundary)))


def ref_desiccation(phase: float) -> float:
    if 0.10 <= phase <= 0.35:
        return math.sin(((phase - 0.10) / 0.25) * math.pi)
    else:
        return 0.0


def compute_indicators(i: int, j: int, k: int):
    cx = (NX - 1.0) / 2.0  # 15.5
    cy = (NY - 1.0) / 2.0  # 15.5
    czc = 0.35 * NZ        # 11.2

    x_n = (i - cx) / (0.40 * NX)  # (i - 15.5) / 12.8
    y_n = (j - cy) / (0.40 * NY)  # (j - 15.5) / 12.8
    z_n = (k - czc) / (0.35 * NZ) # (k - 11.2) / 11.2

    rc = math.sqrt(x_n * x_n + y_n * y_n + z_n * z_n)

    dx_frac = (i - cx) / (0.12 * NX)  # (i - 15.5) / 3.84
    dy_frac = (j - cy) / (0.12 * NY)  # (j - 15.5) / 3.84
    rf = math.sqrt(dx_frac * dx_frac + dy_frac * dy_frac)

    fracture = math.exp(-0.5 * rf * rf) if k >= czc else 0.0
    core = math.exp(-0.5 * ((rc / 0.6) ** 2))
    halo = max(0.0, math.exp(-0.5 * (((rc - 1.1) / 0.35) ** 2)))

    return core, fracture, halo


def generate_field(phase: float, null_scenario: bool, pulse_boundary: float, field_id: int):
    if null_scenario:
        pulse = 0.0
        desic = 0.0
    else:
        pulse = ref_pulse(phase, pulse_boundary)
        desic = ref_desiccation(phase)

    data = bytearray(TOTAL_VOXELS * 4)
    offset = 0

    for k in range(NZ):
        depth = k / float(NZ)
        for j in range(NY):
            for i in range(NX):
                c, f, h = compute_indicators(i, j, k)

                if field_id == 0:  # temperature
                    base_temp = 25.0 + 35.0 * depth
                    if null_scenario:
                        v = base_temp + 30.0 * c
                    else:
                        v = (base_temp
                             + 320.0 * pulse * c
                             + 180.0 * pulse * f
                             + 40.0 * h * (1.0 - 0.5 * pulse))
                elif field_id == 1:  # hydration
                    base_hyd = 0.85 - 0.10 * depth
                    if null_scenario:
                        v = base_hyd
                    else:
                        drying = desic * (0.80 * f + 0.65 * c)
                        raw = base_hyd - drying + 0.08 * h
                        v = min(1.0, max(0.02, raw))
                elif field_id == 2:  # h2_concentration
                    if null_scenario:
                        v = 0.001
                    else:
                        water_present = max(0.05, 1.0 - 0.7 * desic)
                        rad_zone = c + 0.3 * h
                        v = 5.0 * rad_zone * water_present + 1.8 * f * (1.0 - desic)
                elif field_id == 3:  # u235_ratio
                    v = 0.00725 - 0.00305 * c
                elif field_id == 4:  # polymer_yield
                    if null_scenario:
                        v = 0.0
                    else:
                        cat_surface = 0.7 * h + 0.5 * f
                        efficiency = 1.0 + 0.5 * math.sin(2.0 * math.pi * phase)
                        v = 8.5 * cat_surface * efficiency
                elif field_id == 5:  # compartments
                    if c > 0.45:
                        v = 1.0
                    elif f > 0.35:
                        v = 4.0 if desic > 0.4 else 3.0
                    elif h > 0.35:
                        v = 2.0
                    else:
                        v = 0.0
                else:
                    raise ValueError(f"Unknown field_id {field_id}")

                struct.pack_into("<f", data, offset, v)
                offset += 4

    return bytes(data)


def compute_statistics(raw_bytes: bytes):
    floats = struct.unpack(f"<{TOTAL_VOXELS}f", raw_bytes)
    n = len(floats)
    min_val = min(floats)
    max_val = max(floats)
    sum_val = sum(floats)
    mean_val = sum_val / n
    sum_sq = sum(x * x for x in floats)
    var = max(0.0, sum_sq / n - mean_val * mean_val)
    pop_sd = math.sqrt(var)
    rms = math.sqrt(sum_sq / n)
    return {
        "min": min_val,
        "max": max_val,
        "mean": mean_val,
        "pop_sd": pop_sd,
        "rms": rms,
    }


def compute_sha256(raw_bytes: bytes) -> str:
    return hashlib.sha256(raw_bytes).hexdigest()


def run_all_cases():
    results = []
    for c in CF2_CASES:
        case_info = {
            "case_index": c["index"],
            "case_id": c["id"],
            "phase": c["phase"],
            "null_scenario": c["null_scenario"],
            "pulse_boundary": c["pulse_boundary"],
            "fields": {},
        }
        for field_id, field_name in enumerate(FIELD_NAMES):
            buf = generate_field(c["phase"], c["null_scenario"], c["pulse_boundary"], field_id)
            digest = compute_sha256(buf)
            stats = compute_statistics(buf)
            case_info["fields"][field_name] = {
                "field_id": field_id,
                "sha256": digest,
                "stats": stats,
            }
        results.append(case_info)
    return results


def main():
    parser = argparse.ArgumentParser(description="CF-2 Reference Generator")
    parser.add_argument("--json", action="store_true", help="Output full JSON matrix")
    parser.add_argument("--verify", action="store_true", help="Verify Case 6 against spec snapshot")
    parser.add_argument("--hashes", action="store_true", help="Print all 120 SHA-256 hashes")
    args = parser.parse_args()

    results = run_all_cases()

    if args.verify or len(sys.argv) == 1:
        # Verify Case 6 against CONFORMANCE_FIXTURE_SPEC.md Sec 2.1
        c6 = results[5]
        expected_c6_hashes = {
            "temperature": "89d85fc802e4f4d638af529f66bf7f3eb73b14e704afaf31dc652b68c9cb8e47",
            "hydration": "eb75f8694ae51fb6d3b37e941f93e931ca4aa84250a02c7ec9c55b0330791356",
            "h2_concentration": "cb42d37ca160d80168b2fc9d0d8095c9f8d9303f7c1b8445e2166ae35e8cb290",
            "u235_ratio": "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
            "polymer_yield": "c84d5c5c2e3bee7ef8b50ad7092c8b124c69783aa99be8bf699bf6b08f78fe35",
            "compartments": "6271f65f4030e3a45b8a1b80817ae33aeae1b7596dfbbc07cd77c6c30c39dd5d",
        }
        all_ok = True
        for f_name, exp_hash in expected_c6_hashes.items():
            actual = c6["fields"][f_name]["sha256"]
            if actual != exp_hash:
                print(f"MISMATCH for {f_name}: expected {exp_hash}, got {actual}", file=sys.stderr)
                all_ok = False
            else:
                print(f"MATCH: Case 6 {f_name} -> {actual}")
        if all_ok:
            print("Python CF-2 reference generator: Case 6 verified against spec snapshot.")
        else:
            sys.exit(1)

    if args.hashes:
        for c in results:
            for f_name in FIELD_NAMES:
                h = c["fields"][f_name]["sha256"]
                scen = "null" if c["null_scenario"] else "active"
                b_str = "0.1667" if abs(c["pulse_boundary"] - 0.1667) < 1e-4 else "0.1700"
                p_str = f"{c['phase']:.2f}"
                idx_str = f"{c['case_index']:02d}"
                print(f"{h}  case{idx_str}_p{p_str}_{scen}_b{b_str}_{f_name}.bin")

    if args.json:
        print(json.dumps(results, indent=2))


if __name__ == "__main__":
    main()
