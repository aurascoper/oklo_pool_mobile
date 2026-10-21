# Specification: 4D Spatiotemporal Geochemistry Explorer (`spacetime_geochemistry_v0`)

- **Specification ID:** `spacetime_geochemistry_v0`
- **Requirement:** R10 (Stage 0 Data Contract and Stage 1 Synthetic Temporal Renderer Proof)
- **Version:** `0.1.0`
- **Target Branch:** `spacetime-geochemistry/v0-slice` (branched directly from `main` at `636b7d7bb41999b80278d90046da893ee1044f20`)
- **Implementation Scope:** Stage 0 (4D Data Contract) and Stage 1 (Synthetic Temporal Renderer Proof) only; Stages 2 to 4 deferred behind explicit promotion gates
- **Scientific Status:** Numerically verified synthetic spatiotemporal data contract and renderer proof; **not** a calibrated geological, hydrothermal, or radiation-transport simulation

---

## 1. Core Principle and Primary Acceptance Criterion

Four-dimensional visualization $(x, y, z, t)$ is not the same thing as a four-dimensional physical model. The scientific model, evidence registry, and spatiotemporal renderer must remain independently versioned, and the display must never invent unmeasured temperatures, fluid velocities, reaction rates, or radiation fields to make a scene look physically complete.

### Primary Acceptance Criterion

> **A reviewer can inspect a time-evolving 3D dataset and independently establish what each displayed state represents, where it came from, and whether it is synthetic, measured, computed, or interpolated.**

Passing the Stage 0 data-contract validator and Stage 1 viewer semantic test suite establishes **rendering and data-contract correctness only**, not geological validity, radiation transport accuracy, or empirical calibration.

---

## 2. Five-Stage Roadmap and Promotion Gates

| Stage | Deliverable | Promotion Gate | Current Status |
| :--- | :--- | :--- | :--- |
| **Stage 0: 4D Data Contract** | Schema (`data/spacetime/schema_4d_v0.json`) and validator (`scripts/validate_spacetime_4d.py`) for time-indexed 3D fields, fixed-step seconds time grid (`time_units: "s"`), coordinate frames, separated baseline vs. generator provenance, uncertainty, missing-frame policy, interpolation policy, and per-frame Float32LE SHA-256 checksums | Invalid, non-monotonic, or non-uniform time grids, non-`"s"` time units, mismatched field dimensions, non-finite values, invalid provenance, and corrupted per-frame checksums are rejected automatically | **ACTIVE (In Scope)** |
| **Stage 1: Temporal Renderer Proof** | Standalone offline 4D viewer (`viewer/spacetime_explorer_4d.html`, `viewer/js/spacetime_4d.js`) animating a deliberately synthetic 3D-plus-time field with timeline scrubber, playback controls, dual-time comparison $(t_A, t_B)$, orthogonal slice inspection, 4D trajectory projection ($+k$ up, $w=t$), and selected-voxel time-series plot | Automated tests (`tests/test_spacetime_4d_v0.js`) prove that time selection changes the underlying 3D `Float32Array` buffers (`sha256_f32_le` and voxel values), cross-check frame digests against an externally pinned manifest and receipt record, fail closed on missing frames, and label interpolated display states `"interpolated_for_display"` | **ACTIVE (In Scope)** |
| **Stage 2: Independent Physical Model** | Select and specify a small, defensible geological or reactive-transport transient PDE model | Governing equations, parameters, boundary conditions, true finite-volume mass/energy conservation closure, and numerical convergence are independently derived and verified | **DEFERRED (Gate Locked)** |
| **Stage 3: Radioactive-Geochemical Extension** | Introduce radioactive source terms and relevant chemical or thermal couplings where justified | Source terms, decay chains, reaction yields, units, and coupling assumptions pass independent validation | **DEFERRED (Gate Locked)** |
| **Stage 4: Integrated 4D Research Scene** | Render verified time-indexed outputs with provenance, limitations, and uncertainty | Scene passes data-integrity and scientific red-team review | **DEFERRED (Gate Locked)** |

---

## 3. Stage 0: 4D Spatiotemporal Data Contract (`SpacetimeFieldBundleV0`)

All Stage 0 and Stage 1 spatiotemporal datasets must conform to `data/spacetime/schema_4d_v0.json` and pass `scripts/validate_spacetime_4d.py` before being loaded by the renderer.

### 3.1 Spatial Coordinate Frame (`spatial_frame`)

Every bundle must explicitly declare its 3D spatial domain:
- `coordinate_system`: Machine-readable identifier (`"cartesian_local_16m_cube"`).
- `geometry_provenance`: Must be one of `["schematic", "surveyed_or_sourced"]`. For Stage 1, this is explicitly `"schematic"`.
- `dimensions_voxels`: 3-element integer array `[nx, ny, nz]`, fixed at `[16, 16, 16]` (`4096` voxels per state) for the Stage 1 proof.
- `voxel_order`: `"x_fastest_then_y_then_k_up"`, corresponding to flat linear index:
  $$\mathrm{idx}(i, j, k) = i + n_x \cdot (j + n_y \cdot k), \quad i \in [0, n_x - 1], \; j \in [0, n_y - 1], \; k \in [0, n_z - 1]$$
- `domain_bounds_m`: Physical domain bounds in meters: `{"x": [0.0, 16.0], "y": [0.0, 16.0], "z_up": [0.0, 16.0]}`.
- `up_axis`: Strictly `"+k"`, denoting that index $k$ (physical coordinate $z$) points vertically upward.

### 3.2 Temporal Axis, Fixed-Step Seconds Grid, and Strict Monotonic Ordering (`temporal_axis`)

Every bundle must explicitly define the fourth dimension $w = t$ on a uniform fixed-step grid in seconds:
- `fourth_dimension_symbol`: Must equal `"t"`. In accordance with dimensional integrity invariants, the 4th coordinate $w$ in this module represents continuous time evolution $t$ and must never be conflated with arbitrary 3D perspective distortion.
- `time_units`: Strictly `"s"` (seconds) in `spacetime_4d_v0`. Any non-`"s"`, missing, empty, or whitespace-only unit string is rejected so `temporal_resolution_s` and `simulation_time` share an unambiguous unit basis.
- `state_count`: Integer $M \ge 2$ (`5` in the Stage 1 synthetic reference bundle).
- `temporal_resolution_s`: Positive number giving the uniform fixed step $\Delta t$ in seconds between consecutive discrete states (`250.0` s).
- `time_bounds`: Two-element numeric array `[t_start, t_end]` (`[0.0, 1000.0]`).
- `strict_monotonic_ordering`: Across `states[0 .. M-1]`:
  1. `states[m].state_index` must equal $m$ ($0, 1, \dots, M-1$) with no gaps.
  2. `states[m].simulation_time` must strictly increase ($t_0 < t_1 < \dots < t_{M-1}$) AND match the declared fixed step `temporal_resolution_s` within $10^{-6}\text{ s}$:
     $$|(t_m - t_{m-1}) - \Delta t_{\mathrm{resolution}}| \le 10^{-6}\text{ s} \quad \forall m \in \{1, \dots, M-1\}$$
     Any duplicate ($t_m = t_{m-1}$), decreasing ($t_m < t_{m-1}$), or non-uniformly spaced timestamp is rejected automatically by both the Python validator and the JavaScript runtime verifier.
- `initial_condition_summary`: Explicit textual description of the state at $t = t_0$.
- `event_markers`: Array of discrete regime or diagnostic markers along the timeline, each recording `state_index`, `simulation_time`, `label`, and `event_type`.

### 3.3 Field Dimensions and Value Units

- `field_name`: Machine-readable quantity identifier (`"synthetic_tracer_concentration"`).
- `field_units`: Explicit unit string (`"mol_m3_synthetic_tracer"`).
- Within each entry of `states[]`:
  - `voxel_count` must equal $n_x \times n_y \times n_z = 4096$.
  - `values` must be a flat array of length `voxel_count` containing only finite IEEE-754 numbers (`NaN`, `+Infinity`, and `-Infinity` are rejected).
  - Derived summary statistics (`min_value`, `max_value`, `mean_value`, `total_integrated_inventory_mol`, and `center_of_mass_m`) are independently recomputed from the hashed `Float32LE` voxel array and `spatial_frame` cell centers/volume ($\Delta V = \Delta x \Delta y \Delta z$) by both `scripts/validate_spacetime_4d.py` and `viewer/js/spacetime_4d.js` (`verifyBundleIntegrity()` and `selectTimeState()`), rejecting any bundle whose stored summary metadata diverges from the voxel buffer.

### 3.4 Provenance Separation, Epistemic Evidence Class, and Per-Frame Checksums

Every bundle carries separated baseline vs. generator provenance metadata and per-frame cryptographic checksums:
- `baseline_commit`: The 40-character hexadecimal SHA-1 of the repository `main` baseline commit (`"636b7d7bb41999b80278d90046da893ee1044f20"`) from which `spacetime-geochemistry/v0-slice` was branched.
- `source_provenance`: Structured object defining the R10 generator and source revision separately from `baseline_commit` without embedding a self-referential final git commit hash inside generated content:
  - `target_branch`: `"spacetime-geochemistry/v0-slice"`
  - `baseline_commit`: `"636b7d7bb41999b80278d90046da893ee1044f20"`
  - `generator_script`: `"scripts/generate_synthetic_4d_bundle.py"`
  - `generator_version`: `"spacetime_geochemistry_v0_gen_0.2.0"`
  - `generator_script_sha256`: 64-character lowercase hex SHA-256 digest of `scripts/generate_synthetic_4d_bundle.py`, verified against the script on disk by `scripts/validate_spacetime_4d.py`.
  - `tested_source_commit_policy`: `"recorded_in_external_verification_receipt"` (the exact git commit under test is bound in `receipts/spacetime_geochemistry_v0_verification.json` and `receipts/spacetime_geochemistry_v0_2026-10-20.md`).
- `evidence_class`: Restricted to the six epistemic classes defined in Requirement R7:
  1. `"geographic_observation_or_locator"`
  2. `"sourced_geological_record"`
  3. `"prescribed_analytic_field"`
  4. `"numerically_verified_synthetic_model"`
  5. `"hypothesis_or_unvalidated_model"`
  6. `"empirically_calibrated_model"`
  In Stage 0 and Stage 1, any bundle claiming `"empirically_calibrated_model"` is rejected automatically by `scripts/validate_spacetime_4d.py` and `verifyBundleIntegrity()`.
- `sha256_f32_le` (Per-Frame Checksum vs. External Receipt Verification):
  - Every discrete state in `states[]` records the 64-character lowercase hexadecimal SHA-256 digest of its `values` array serialized as little-endian IEEE-754 32-bit floats (`Float32Array`, `16,384` bytes for `4,096` voxels).
  - At runtime in the browser (`verifyBundleIntegrity()` and `selectTimeState()`), the JavaScript engine performs **per-frame Float32LE checksum verification against the bundle metadata** (`state.sha256_f32_le`) and requires a non-empty `run_receipt` reference string for discrete states.
  - Independent verification against an externally pinned expected-digest manifest and the external receipt files (`receipts/spacetime_geochemistry_v0_verification.json` and `receipts/spacetime_geochemistry_v0_2026-10-20.md`) is executed in the Node.js verification test suite (`tests/test_spacetime_4d_v0.js`).

### 3.5 State Derivation Status, Missing-Frame Behavior, and Interpolation Policy

To ensure that a reviewer can always distinguish computed or prescribed states from visual interpolation:
1. **`state_derivation` Enumeration:**
   Every state object must carry `state_derivation` set to one of four mutually exclusive values:
   - `"prescribed_analytically"`
   - `"computed"`
   - `"measured"`
   - `"interpolated_for_display"`
2. **Missing-Frame Behavior (`missing_frame_policy: "fail_closed_no_invented_states"`):**
   - If a caller requests a state index outside $[0, M-1]$, or if a bundle has a gap in `state_index`, the selector throws an explicit error (`RangeError` / `SpacetimeContractError`).
   - The system never silently falls back to an adjacent timestamp or invents unverified intermediate states when discrete mode is active.
3. **Interpolation Policy (`interpolation_policy: "none_discrete_states_only"` by default):**
   - Stored states in `data/spacetime/synthetic_tracer_pulse_4d_v0.json` are strictly discrete verified states (`interpolation_policy: "none_discrete_states_only"`). Any stored state inside `states[]` marked `"interpolated_for_display"` under this policy is rejected by the validator.
   - In the viewer UI, discrete state selection is the default (`Allow Display Interpolation: OFF`).
   - When the reviewer explicitly enables the UI display-interpolation option (`allowDisplayInterpolation: true`) and selects an intermediate fractional position $\alpha \in (0, 1)$ between uniformly spaced states $m$ and $m+1$:
     - The viewer computes linear visual blend $(1 - \alpha) F(\mathbf{x}, t_m) + \alpha F(\mathbf{x}, t_{m+1})$ for display inspection only.
     - The returned state object is explicitly stamped with:
       - `state_derivation: "interpolated_for_display"`
       - `is_simulated_state: false`
       - `run_receipt: null`
       - `sha256_f32_le: "unverified_display_interpolation"`
     - The viewer displays a prominent amber warning banner stating that the displayed frame is a linear visual interpolation between two discrete states and has no simulation receipt.

### 3.6 Criterial Separation (`verified_accepted` vs `criterial_pruned`)

In compliance with the 4D visualization criterial separation invariant:
- Verified time states in `states[]` carry `criterial_status: "verified_accepted"` and are rendered in solid emerald green (`#10b981`).
- Negative-control trajectories that violate the Stage 1 analytical reference envelope are stored separately in `criterial_pruned_branches[]` with `criterial_status: "criterial_pruned"`, explicit `prune_reason`, `max_relative_analytical_drift`, and `criterial_threshold_exceeded`. They are rendered in dashed crimson red (`#ef4444`) on both the 4D spatiotemporal trajectory canvas and the selected-voxel time-series plot so reviewers can visually contrast accepted vs. pruned candidate trajectories without claiming finite-volume PDE conservation.

---

## 4. Stage 1 Synthetic 3D Advection-Diffusion-Decay Benchmark and Illustrative Analytical Bookkeeping

The Stage 1 reference dataset (`data/spacetime/synthetic_tracer_pulse_4d_v0.json`, generated deterministically by `scripts/generate_synthetic_4d_bundle.py`) samples an analytical 3D Gaussian tracer pulse undergoing upward advection along $+k$, isotropic diffusion, and first-order decay on $\Omega = [0, 16\text{ m}]^3$:

$$C(x, y, z, t) = C_0 \left(\frac{\sigma_0^2}{\sigma(t)^2}\right)^{3/2} \exp(-k_{\mathrm{decay}} t) \exp\left(-\frac{(x - x_c(t))^2 + (y - y_c(t))^2 + (z - z_c(t))^2}{2\sigma(t)^2}\right)$$

with:
- Voxel centers: $x_i = i + 0.5\text{ m}$, $y_j = j + 0.5\text{ m}$, $z_k = k + 0.5\text{ m}$ for $i, j, k \in \{0, 1, \dots, 15\}$ ($\Delta x = \Delta y = \Delta z = 1.0\text{ m}$).
- Spreading variance: $\sigma(t)^2 = \sigma_0^2 + 2 D t$, where $\sigma_0 = 1.8\text{ m}$ and $D = 0.0012\text{ m}^2\text{ s}^{-1}$.
- Advection trajectory:
  $$x_c(t) = 8.0 + 0.0015\,t, \quad y_c(t) = 8.0 - 0.0010\,t, \quad z_c(t) = 4.5 + 0.0065\,t$$
- Initial peak amplitude: $C_0 = 10.0\text{ mol m}^{-3}$.
- First-order decay constant: $k_{\mathrm{decay}} = 0.0004\text{ s}^{-1}$.
- Discrete evaluated states: $M = 5$ at uniform fixed step $\Delta t = 250.0\text{ s}$, $t \in \{0.0, 250.0, 500.0, 750.0, 1000.0\}\text{ s}$.
- **Illustrative Analytical Bookkeeping (`analytical_bookkeeping_ledger`):**
  Because Stage 1 samples a closed-form analytical Gaussian rather than solving a discretized finite-volume PDE, the analytical survival $M_{\mathrm{surv}}(t) = M_0 e^{-k_{\mathrm{decay}} t}$ and analytical decay $M_{\mathrm{decay}}(t) = M_0 (1 - e^{-k_{\mathrm{decay}} t})$ sum to $M_0$ by algebraic identity (`closes_by_algebraic_definition: true`, `is_finite_volume_conservation_test: false`). Each state records the discrete voxel sum $M_{\mathrm{retained}}(t) = \sum_{\mathbf{x} \in \Omega} C(\mathbf{x}, t)\,\Delta V$ alongside the unmodeled tail and midpoint-quadrature gap $M_{\mathrm{gap}}(t) = M_{\mathrm{surv}}(t) - M_{\mathrm{retained}}(t)$ and `relative_quadrature_and_tail_gap` $= |M_{\mathrm{gap}}(t)| / M_0$. True finite-volume flux reconstruction and discretized mass-balance testing are deferred to the Stage 2 physical solver gate.
- **Negative-Control Pruned Branch (`synthetic_amplitude_drift_control`):**
  Perturbs the analytical evaluation with an artificial amplitude drift multiplier $(1 + 0.38 \cdot (t / 1000.0))$, exceeding the Stage 1 analytical reference envelope gate (`max_relative_analytical_drift = 0.38 > 1.0e-4`) and triggering `criterial_status: "criterial_pruned"` for visual criterial-separation auditing.

---

## 5. Compatibility Contract with Planetary Explorer v2 (`planetary-explorer/v2-prototype`)

Branch `spacetime-geochemistry/v0-slice` branches directly from `main` (`636b7d7bb41999b80278d90046da893ee1044f20`) and does not depend on any unmerged files from PR #9 (`planetary-explorer/v2-prototype`). To ensure seamless future interoperability once both workstreams pass review, `SpacetimeFieldBundleV0` embeds a `v2_compatibility` block that maps onto Planetary Explorer v2's `EpistemicModelRecord`:

| Planetary Explorer v2 Field (`EpistemicModelRecord`) | `SpacetimeFieldBundleV0` Mapping (`data/spacetime/schema_4d_v0.json`) | Stage 1 Value |
| :--- | :--- | :--- |
| `model_id` | `model_id` & `v2_compatibility.epistemic_record.model_id` | `"spacetime_geochemistry_v0"` |
| `model_version` | `model_version` & `v2_compatibility.epistemic_record.model_version` | `"0.1.0"` |
| `evidence_class` | `evidence_class` (identical 6-value enum) | `"numerically_verified_synthetic_model"` |
| `source_repository` | `source_repository` | `"https://github.com/aurascoper/oklo_pool_mobile"` |
| `baseline_commit` | `baseline_commit` & `v2_compatibility.epistemic_record.baseline_commit` | `"636b7d7bb41999b80278d90046da893ee1044f20"` |
| `source_revision` | `source_provenance` & `v2_compatibility.epistemic_record.source_revision` | `"spacetime-geochemistry/v0-slice@spacetime_geochemistry_v0_gen_0.2.0"` |
| `specification_version` | `specification_version` | `"docs/specs/SPACETIME_GEOCHEMISTRY_v0.md#v0.1.0"` |
| `run_receipt` | `run_receipt` | `"receipts/spacetime_geochemistry_v0_2026-10-20.md"` |
| `parameter_set_id` | `parameter_set_id` | `"synthetic_advection_diffusion_decay_v0_default"` |
| `units` | `field_units` + `temporal_axis.time_units` | `"mol_m3_synthetic_tracer; time_units=s"` |
| `coordinate_system` | `spatial_frame.coordinate_system` | `"cartesian_local_16m_cube"` |
| `uncertainty` | `uncertainty` | `"Synthetic analytical benchmark sampled on a 1m voxel grid; IEEE-754 float32 quantization residual < 1e-6; analytical bookkeeping closes by algebraic identity (not a finite-volume PDE conservation test)."` |
| `known_limitations` | `known_limitations` | `"Stage 1 synthetic 3D advection-diffusion-decay pulse on a schematic 16m cube; not a geological, hydrothermal, or radiation-transport simulation."` |

Visual and typographic conventions also match Planetary Explorer v2:
- 100% offline execution (`file://` compatible, zero external CDNs, zero Google Fonts, native system font stacks).
- Identical dark-theme palette (`#0b0f14` primary background, `#151d27` card surface, `#4f46e5` synthetic model badge) and `+k` vertical up-axis convention.
- Strict compliance with `scripts/text_audit.sh` (zero em-dashes, zero en-dashes, zero quarantined terms).
