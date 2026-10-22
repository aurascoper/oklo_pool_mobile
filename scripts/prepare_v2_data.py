#!/usr/bin/env python3
"""
Assemble precomputed summaries for Planetary Explorer v2.
Reads:
  - receipts/darcy_v1_A4_run_2026-10-18.json
  - receipts/uranium_v1_A1_run_2026-10-18.json
  - out/uranium_v1_A1/*.json
  - data/timeline/paleo_intervals_v1.json
  - data/models/manifest.json
Writes:
  - viewer/data/darcy_summary.json
  - viewer/data/uranium_summary.json
  - viewer/data/paleo_intervals_v1.json
  - viewer/data/models_manifest.json
Ensures zero em-dashes and zero en-dashes across all outputs.
"""

import json
import os
import sys

def sanitize_text(text):
    if isinstance(text, str):
        return text.replace('\u2014', ' - ').replace('\u2013', ' - ')
    elif isinstance(text, list):
        return [sanitize_text(x) for x in text]
    elif isinstance(text, dict):
        return {k: sanitize_text(v) for k, v in text.items()}
    return text

def main():
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    viewer_data = os.path.join(root, 'viewer', 'data')
    os.makedirs(viewer_data, exist_ok=True)

    # 1. Darcy summary
    darcy_path = os.path.join(root, 'receipts', 'darcy_v1_A4_run_2026-10-18.json')
    with open(darcy_path, 'r', encoding='utf-8') as f:
        darcy_data = json.load(f)
    darcy_data = sanitize_text(darcy_data)
    darcy_out = os.path.join(viewer_data, 'darcy_summary.json')
    with open(darcy_out, 'w', encoding='utf-8') as f:
        json.dump(darcy_data, f, indent=2)
    print(f'Wrote {darcy_out} ({os.path.getsize(darcy_out)} bytes)')

    # 2. Uranium summary
    uranium_path = os.path.join(root, 'receipts', 'uranium_v1_A1_run_2026-10-18.json')
    with open(uranium_path, 'r', encoding='utf-8') as f:
        u_meta = json.load(f)

    runs = []
    sample_cell_centres = None
    for r in u_meta.get('runs', []):
        run_file = os.path.join(root, 'out', 'uranium_v1_A1', r['file'])
        if os.path.exists(run_file):
            with open(run_file, 'r', encoding='utf-8') as rf:
                rdata = json.load(rf)
            if sample_cell_centres is None and 'cell_centre' in rdata:
                sample_cell_centres = rdata.get('cell_centre')
            # breakthrough subsampling (every 4th point: 26 points)
            bt_raw = rdata.get('breakthrough_outlet_C', [])
            bt = [[round(pt[0], 1), float(f'{pt[1]:.4g}')] for pt in bt_raw[::4]]
            if bt_raw and bt[-1][0] != bt_raw[-1][0]:
                bt.append([round(bt_raw[-1][0], 1), float(f'{bt_raw[-1][1]:.4g}')])

            # snapshots subsampling (every 4th spatial point: 21 points)
            snaps = []
            for s in rdata.get('snapshots', []):
                snaps.append({
                    'tau': s['tau'],
                    't': s['t'],
                    'C': [float(f'{x:.4g}') for x in s['C'][::4]],
                    'S': [float(f'{x:.4g}') for x in s['S'][::4]]
                })
            runs.append({
                'label': r['label'],
                'v': r['v'],
                'D_eff': r['D_eff'],
                'k_red': r['k_red'],
                'K_d': r['K_d'],
                'R': r['R'],
                'Pe_cell': r['Pe_cell'],
                'Da': r['Da'],
                'tau_s': r['tau_s'],
                'dt_s': r['dt_s'],
                'steps': r['steps'],
                'ledger': r['ledger'],
                'breakthrough': bt,
                'snapshots': snaps
            })

    dx = 16.0 / 80.0
    x_coords = [round((i + 0.5) * dx, 3) for i in range(81)][::4]
    u_summary = {
        'name': u_meta.get('name'),
        'caveat': u_meta.get('caveat'),
        'statement': u_meta.get('statement'),
        'checks': u_meta.get('checks'),
        'x_m': x_coords,
        'reactive_barrier_m': [6.0, 10.0],
        'runs': runs
    }
    u_summary = sanitize_text(u_summary)
    u_out = os.path.join(viewer_data, 'uranium_summary.json')
    with open(u_out, 'w', encoding='utf-8') as f:
        json.dump(u_summary, f, separators=(',', ':'))
    print(f'Wrote {u_out} ({os.path.getsize(u_out)} bytes, {len(runs)} runs)')

    # 3. Timeline intervals
    timeline_path = os.path.join(root, 'data', 'timeline', 'paleo_intervals_v1.json')
    with open(timeline_path, 'r', encoding='utf-8') as f:
        timeline_data = json.load(f)
    timeline_data = sanitize_text(timeline_data)
    timeline_out = os.path.join(viewer_data, 'paleo_intervals_v1.json')
    with open(timeline_out, 'w', encoding='utf-8') as f:
        json.dump(timeline_data, f, indent=2)
    print(f'Wrote {timeline_out} ({os.path.getsize(timeline_out)} bytes)')

    # 4. Models manifest
    manifest_path = os.path.join(root, 'data', 'models', 'manifest.json')
    with open(manifest_path, 'r', encoding='utf-8') as f:
        manifest_data = json.load(f)
    manifest_data = sanitize_text(manifest_data)
    manifest_out = os.path.join(viewer_data, 'models_manifest.json')
    with open(manifest_out, 'w', encoding='utf-8') as f:
        json.dump(manifest_data, f, indent=2)
    print(f'Wrote {manifest_out} ({os.path.getsize(manifest_out)} bytes)')

    # Verify zero em-dash / en-dash
    for p in [darcy_out, u_out, timeline_out, manifest_out]:
        with open(p, 'rb') as f:
            c = f.read()
        if b'\xe2\x80\x94' in c or b'\xe2\x80\x93' in c:
            print(f'ERROR: em-dash or en-dash detected in {p}')
            sys.exit(1)
    print('All data files verified clean of em-dash and en-dash.')

if __name__ == '__main__':
    main()
