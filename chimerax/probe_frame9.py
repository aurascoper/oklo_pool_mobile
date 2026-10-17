# CF-4 ChimeraX probe for series frame 9 (p = 0.15), run inside ChimeraX after
# oklo_cycle.cxc. For every opened volume whose file ends in _f09.mrc it hashes
# the data matrix as little-endian float32 with X fastest, records step and
# origin on all three axes, and compares the hash with the data-block hash in
# CONFORMANCE_FIXTURE_SPEC.md section 2.1. Writes out/probes/chimerax_frame9.json.

import hashlib
import json
import os

import chimerax
from chimerax.map import Volume

SPEC_2_1 = {
    "temperature": "89d85fc802e4f4d638af529f66bf7f3eb73b14e704afaf31dc652b68c9cb8e47",
    "hydration": "eb75f8694ae51fb6d3b37e941f93e931ca4aa84250a02c7ec9c55b0330791356",
    "h2_concentration": "cb42d37ca160d80168b2fc9d0d8095c9f8d9303f7c1b8445e2166ae35e8cb290",
    "u235_ratio": "faff264263d13277cf6f4cdb4d60e20e59249e91278df035f9ca269f481de944",
    "polymer_yield": "c84d5c5c2e3bee7ef8b50ad7092c8b124c69783aa99be8bf699bf6b08f78fe35",
    "compartments": "6271f65f4030e3a45b8a1b80817ae33aeae1b7596dfbbc07cd77c6c30c39dd5d",
}

records = []
for v in session.models.list(type=Volume):
    path = getattr(v.data, "path", "") or ""
    base = os.path.basename(path)
    if not base.endswith("_f09.mrc"):
        continue
    field = base[: -len("_f09.mrc")]
    m = v.full_matrix()  # numpy array indexed [k, j, i], so C order is X fastest
    digest = hashlib.sha256(m.astype("<f4").tobytes(order="C")).hexdigest()
    records.append({
        "field": field,
        "shape_kji": list(m.shape),
        "step": [float(s) for s in v.data.step],
        "origin": [float(o) for o in v.data.origin],
        "sha256": digest,
        "spec_2_1_match": digest == SPEC_2_1.get(field),
        "step_ok": all(abs(s - 0.5) < 1e-6 for s in v.data.step),
        "origin_ok": all(abs(o) < 1e-6 for o in v.data.origin),
    })

records.sort(key=lambda r: r["field"])
out_dir = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "out", "probes")
os.makedirs(out_dir, exist_ok=True)
result = {
    "tool": "ChimeraX",
    "version": chimerax.app_dirs.version,
    "case": "series b0.17 frame 9 (phase 0.15, active)",
    "fields_found": len(records),
    "all_pass": len(records) == 6 and all(r["spec_2_1_match"] and r["step_ok"] and r["origin_ok"] for r in records),
    "records": records,
}
with open(os.path.join(out_dir, "chimerax_frame9.json"), "w") as f:
    json.dump(result, f, indent=2)
session.logger.info(f"probe_frame9: {len(records)} fields, all_pass={result['all_pass']}")
