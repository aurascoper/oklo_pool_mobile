# TEST_READY: Dual-Track Test Suite and Verification Guide

- Branch: `planetary-explorer/v2-prototype`
- Target Milestone: Milestone B3 (Acceptance Verification, Red-Team Audit, and Dual-Track Test Suite Publication)
- Date: 2026-10-20
- Author: worker_milestone_b3
- Baseline Commit: `636b7d7bb41999b80278d90046da893ee1044f20`
- Tested Source Commit: `934ffb6c3e2be6b63c27aba5973012ec152c787b`
- Receipts Remediation Commit: `39ba9648b692bc3730e72058595501f4e19f92ee`
- Overall Status: READY / PASS

---

## 1. Overview

This document specifies the dual-track opaque-box and white-box test suites for the Planetary Explorer v2 prototype and its underlying Rust core (`oklo_core`). It provides test execution commands, a four-tier coverage breakdown, invariant verification checks, and a formal feature sign-off checklist.

---

## 2. Test Runner Invocation Commands

The test suite consists of three primary automated test runners plus a repository typography and invariant scan:

### 2.1 Node.js Automated Test Suite
Executes the headless JavaScript test suite covering geodetic coordinate transformations, camera constraints, paleo-interval chronostratigraphy, bit-identical Frame 9 field hashes, transport model loaders, epistemic metadata schema compliance, and typography.

```bash
node tests/test_planetary_explorer_v2.js
```
- Expected Result: 10/10 tests passed (exit code 0).

### 2.2 Rust Conformance Suite (`oklo_core`)
Executes the locked cargo conformance test suite in `oklo_core`, validating mathematical fixture invariance, ABI round-trips, phase stepping, temperature isovalues, and boundary parameter sensitivity.

```bash
cargo test -p oklo_core --locked --test conformance
```
- Expected Result: 7/7 tests passed (exit code 0).

### 2.3 Epistemic Model Manifest Validation
Executes the Python strict schema validator across all 17 registered models in `data/models/manifest.json`, enforcing schema compliance and the strict emptiness of the `empirically_calibrated_model` evidence class.

```bash
python3 scripts/validate_models.py
```
- Expected Result: 17/17 models validated, 0 empirically calibrated models (exit code 0).

### 2.4 Tracked Files Typography and Quarantined Invariants Scan
Verifies that strictly zero em-dashes (`\xe2\x80\x94`), zero en-dashes (`\xe2\x80\x93`), and zero quarantined terms exist anywhere across tracked repository files.

```bash
python3 -c '
import subprocess, sys
files = subprocess.check_output(["git", "ls-files"], text=True).splitlines()
q1 = ("radiotrophic" + " methanogens").encode()
q2 = ("free-will" + " octane").encode()
em_dash = "\u2014".encode("utf-8")
en_dash = "\u2013".encode("utf-8")
hits = []
for p in files:
    if p.endswith((".png", ".jpg", ".bin", ".dat", ".so", ".xcframework")):
        continue
    with open(p, "rb") as f:
        d = f.read()
    dl = d.lower()
    if q1 in dl or q2 in dl: hits.append((p, "quarantined term"))
    if em_dash in d: hits.append((p, "em-dash"))
    if en_dash in d: hits.append((p, "en-dash"))
if hits:
    print("FAILED:", hits); sys.exit(1)
print(f"SUCCESS: 0 hits across {len(files)} tracked files.")
'
```
- Expected Result: `SUCCESS: 0 hits across tracked files.` (exit code 0).

---

## 3. Test Tier Coverage Breakdown

The verification suite is structured across four comprehensive testing tiers:

### Tier 1: Feature Coverage
Validates the core functional capabilities across all four vertical stages:
- Stage 1 (Planetary Globe):
  - Rendering of 3D Earth sphere with WGS84 site markers.
  - Sourced Oklo Zone 2 coordinates (-1.4000 deg, 13.1833 deg; Gauthier-Lafaye 1996).
  - Comparative hydrothermal analog markers (Dallol, Yellowstone, Lost City).
  - Deep-time timeline navigation ingesting `data/timeline/paleo_intervals_v1.json`.
- Stage 2 (Geological Cutaway):
  - 16-meter cubic compartment block [0, 16m]^3 with 5 distinct zones (Sandstone, Core, Halo, Conduit, Steam Chimney).
  - Warning banners explicitly distinguishing schematic geometry from surveyed subsurface geology.
- Stage 3 (Revision-0 Prescribed Fields):
  - Multi-field 32x32x32 voxel inspection across all six fields (Temperature, Hydration, H2 Concentration, U235 Ratio, Polymer Yield, Compartments).
  - Bit-identical hash verification against frozen baseline Frame 9 (b=0.17).
  - Dynamic adjacent field limitation disclosures and amber epistemic badges.
- Stage 4 (Transport Models Inspection):
  - Model A: 3D Darcy groundwater flow across 11 permeability sweeps and 2 thermodynamic water states (50 deg C and 300 deg C).
  - Model B: 1D Uranium reduction-transport column across 144 factorial sweeps, reporting spatial concentration profiles and breakthrough curves.

### Tier 2: Boundary and Corner Cases
Validates boundary defenses, numeric clamping, and edge cases:
- Coordinate Limits:
  - Latitude strictly clamped to [-90.0, 90.0] degrees; out-of-range inputs throw immediate exceptions.
  - Longitude strictly clamped to [-180.0, 180.0] degrees; out-of-range inputs throw immediate exceptions.
- Camera Pitch Clamping:
  - Spherical camera pitch constrained to [-1.45, 1.45] radians (approx -83 to +83 degrees) to prevent polar gimbal lock and matrix singularity.
- Pulse Boundary Sensitivity:
  - Distinguishes pulse boundary parameter b = 0.17 (standard fixture) from b = 0.1667 (conduit boundary threshold), confirmed by Rust conformance test `b_sensitivity_to_gui_boundary_is_detectable`.
- Null Scenario Verification:
  - Evaluates null scenario toggle, verifying that null baseline fields suppress hydrothermal pulses and maintain background values.
- M-Matrix Time-Step Bounds:
  - Verifies that Uranium transport time-step selection satisfies dt <= min(0.5 R dx / v, R / max|A_ii|), preventing spurious oscillations and guaranteeing non-negativity across all 144 sweep runs.

### Tier 3: Combinations and Transitions
Validates state machine transitions, graphic fallbacks, and multi-subsystem interactions:
- Stage State Machine Transitions:
  - Tests smooth transitions: Stage 1 (Globe) -> Dive Animation (600ms smoothstep) -> Stage 2 (Cutaway) -> Stage 3 (Field Inspector) -> Stage 4 (Transport Models).
  - Asserts camera target, position, and up-vectors remain finite, non-NaN, and stable during and after transitions.
- WebGL2 Context Fallback Degradation:
  - Simulates WebGL2 context failure or unsupported environment.
  - Verifies graceful degradation to dual Canvas 2D orthogonal slice views (Plan X-Y and Vertical Section X-k).
  - Asserts zero unhandled JavaScript exceptions and displays informative fallback notification.
- Field Switching Data Invariance:
  - Asserts that switching between fields mutates raw 32x32x32 float32 array buffers rather than merely swapping color palettes.

### Tier 4: Real-World Scenarios
Validates user exploration journeys and scientific review workflows:
- Deep-Time Timeline Exploration:
  - Verifies non-merging of competing literature definitions: Gumsley et al. 2017 GOE (2.460 to 2.060 Ga) vs Craig et al. 2023 GOE (2.43 to 2.22 Ga).
  - Verifies distinct display of Oklo fission event (1.95 +/- 0.04 Ga) with stated uncertainty whiskers.
- Precomputed Transport Breakthrough Inspection:
  - Inspects breakthrough curves C(L_col, t) across extreme retardation factors (R from 1.0 up to 9,460,501).
  - Confirms honest disclosure of extreme Kd (630 m^3/kg) as an EPA regulatory sensitivity upper bound.
  - Confirms honest disclosure of Darcy layer interface continuity check D3a failure (relative error 2.58e-10 vs 1.0e-10 tolerance).
- Offline Independence Verification:
  - Complete vertical slice executes from local disk under `file://` protocol with zero external HTTP/HTTPS network calls.

---

## 4. Feature Checklist and Sign-Off

Mapping to Milestone B Feature Inventory (`PROJECT.md`):

| Feature ID | Feature Name | Test Tier | Automated Test Target | Status |
|---|---|---|---|---|
| F18 | Milestone B Branch Setup | Tier 1 | Clean branch `planetary-explorer/v2-prototype` from `636b7d7` | PASS |
| F19 | Architecture Specification | Tier 1 | `docs/specs/PLANETARY_EXPLORER_ARCHITECTURE_v2.md` | PASS |
| F20 | Scientific Tiers Roadmap | Tier 4 | Section 4 of Architecture Spec (Tiers 1-5 + Sobolev) | PASS |
| F21 | Prototype Shell and Navigation | Tier 3 | `viewer/planetary_explorer.html` + modular JS modules | PASS |
| F22 | Stage 1 Planetary Globe | Tier 1, 2 | `tests/test_planetary_explorer_v2.js` (Tests 1, 2, 3, 4) | PASS |
| F23 | Stage 2 Geological Cutaway | Tier 1, 3 | Schematic 16m cube + camera dive controller | PASS |
| F24 | Stage 3 Rev-0 Field Inspection | Tier 1, 2 | `tests/test_planetary_explorer_v2.js` (Tests 5, 6) | PASS |
| F25 | Stage 4 Transport Inspection | Tier 1, 4 | `tests/test_planetary_explorer_v2.js` (Tests 7, 8) | PASS |
| F26 | Research Repos Integration Registry | Tier 4 | `docs/specs/PLANETARY_EXPLORER_ARCHITECTURE_v2.md` §5 | PASS |
| F27 | Epistemic Separation Contract | Tier 2, 4 | `data/models/manifest.json` + `validate_models.py` | PASS |
| F28 | Milestone B Automated Checks | Tier 1-4 | Node (10/10), Cargo (7/7), Python (17/17) | PASS |
| F29 | Red-Team and Limitations Report | Tier 4 | `docs/audit/PLANETARY_EXPLORER_v2_RED_TEAM_AUDIT.md` | PASS |

---

## 5. Verification Sign-Off

- Lead Verifier: worker_milestone_b3
- Target Branch: `planetary-explorer/v2-prototype`
- Test Execution Date: 2026-10-20
- Automated Suites Pass Rate: 100% (Node: 10/10, Cargo: 7/7, Python: 17/17)
- Invariant Violations: 0 (Zero em-dashes, zero en-dashes, zero quarantined terms)
- Verification Verdict: SIGN-OFF COMPLETE / TEST READY.
