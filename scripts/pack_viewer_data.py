"""Pack the dump_fields outputs into viewer/data/fields.json as base64 of the raw
little-endian float32 bytes, so the viewer can hash exactly what oklo_core wrote.

    cargo run --example dump_fields -- viewer/data 0.17
    cargo run --example dump_fields -- viewer/data 0.1667
    python3 -I scripts/pack_viewer_data.py viewer/data
"""
import base64
import json
import os
import sys

FILES = [
    "temperature_b0.17", "hydration_b0.17", "h2_concentration_b0.17", "u235_ratio_b0.17",
    "polymer_yield_b0.17", "compartments_b0.17", "temperature_b0.1667",
]

data_dir = sys.argv[1]
packed = {}
for name in FILES:
    with open(os.path.join(data_dir, name + ".f32"), "rb") as f:
        raw = f.read()
    assert len(raw) == 4 * 32 ** 3, (name, len(raw))
    packed[name] = base64.b64encode(raw).decode("ascii")
with open(os.path.join(data_dir, "fields.json"), "w") as f:
    json.dump(packed, f)
print(os.path.join(data_dir, "fields.json"))
