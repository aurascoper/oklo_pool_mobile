# Specification: Neutron Transport Explorer (`neutron_transport_explorer_v0`)

- **Specification ID:** `neutron_transport_explorer_v0`
- **Requirement Number:** Unassigned (`PROPOSED` isolated scientific-visualization work package; no official R-number assigned pending project-owner sequencing)
- **Version:** `0.1.0`
- **Target Branch:** `neutron-transport-explorer/v0-spec` (branched directly from `main` at `942e3f8d02a3b73cfb26f099dbb36c4a908f6764`, following the R10 Stage 0/1 merge)
- **Companion Reference Audit:** [`docs/references/NEUTRON_WALK_REFERENCE.md`](../references/NEUTRON_WALK_REFERENCE.md)
- **Implementation Scope:** Stage 0 (Reference Audit and Provenance Contract) only; Stages 1 to 4 are **DEFERRED (Gate Locked)** and not authorized by this specification
- **Scientific Status:** Documentary specification and provenance audit only; **not** an authorized viewer implementation, live neutron-transport solver, or validated reactor benchmark

---

## 1. Core Principle, Scope Isolation, and Primary Acceptance Criterion

A visually convincing three-dimensional reactor animation or interactive particle-walk display is not numerical verification of a transport solver, and numerical verification of a solver or surrogate is not independent physical validation. Furthermore, **tuning a simplified client-side toy model to match OpenMC summary outputs is calibration to those outputs, not independent validation against them**.

### 1.1 Program Placement and Isolation Boundaries

1. **Post-R10 Baseline (`main` at `942e3f8d02a3b73cfb26f099dbb36c4a908f6764`):**
   This work package originates from `main` after the merge of Requirement R10 (`spacetime_geochemistry_v0`, PR #10) and is tracked on its own isolated branch (`neutron-transport-explorer/v0-spec`) and pull request.
2. **No Modification to R10 or Authorization of R10 Stage 2:**
   This specification does **not** alter the R10 Stage 0/1 local-verification qualification and does **not** authorize R10 Stage 2 (transient geochemical PDE solver). R10's synthetic Gaussian tracer (`SpacetimeFieldBundleV0`, `data/spacetime/schema_4d_v0.json`) is a continuum concentration field on a schematic $16\text{ m}$ cube; its schema must never be stretched to store branching Monte Carlo neutron histories or reactor lattice tallies.
3. **Unchanged Independent Review Gates for Open Pull Requests:**
   Pull requests [#7 (`release/v1-materials`)](https://github.com/aurascoper/oklo_pool_mobile/pull/7), [#8 (`release/on-render-gate`)](https://github.com/aurascoper/oklo_pool_mobile/pull/8), and [#9 (`planetary-explorer/v2-prototype`)](https://github.com/aurascoper/oklo_pool_mobile/pull/9) remain untouched under their separate review and merge gates.
4. **No Wholesale Import of Unverified Artifacts or Licensed Media:**
   The `937,606`-byte reference HTML (`TECDOC-643 Neutron Walk (Copy).html`) and the four third-party reference images are audited in [`docs/references/NEUTRON_WALK_REFERENCE.md`](../references/NEUTRON_WALK_REFERENCE.md) and are **not** copied or committed into the repository.

### 1.2 Primary Acceptance Criterion

> **A reviewer inspecting any view, particle trajectory, spatial tally, reactivity curve, or burnup time series in `neutron_transport_explorer_v0` can immediately establish its physical model identity, coordinate convention, nuclear-data and material revision, uncertainty basis, and whether the displayed state is a live client-side toy surrogate, a recorded OpenMC trajectory playback, a precomputed OpenMC tally/depletion record, a visual display interpolation, or an independently validated benchmark.**

---

## 2. Five-Stage Work Package Roadmap and Promotion Gates

| Stage | Deliverable | Promotion Gate | Current Status |
| :--- | :--- | :--- | :--- |
| **Stage 0: Reference Audit & Provenance Contract** | Specification (`docs/specs/NEUTRON_TRANSPORT_EXPLORER_v0.md`) and forensic reference audit (`docs/references/NEUTRON_WALK_REFERENCE.md`) recording the HTML's SHA-256 (`dc15eb93...`), external network dependencies, licensing boundaries, embedded JSON data inventory, local availability of upstream OpenMC files, precision discrepancies, and `RadioPharm.jl` (`MURR`) red-team boundaries | Every referenced figure and dataset has a documented origin and declared evidence status; no unverified or pre-Oct 19 material number is presented as a current validated benchmark | **ACTIVE (Documentary Scope Only)** |
| **Stage 1: Viewer-Only Offline Prototype** | Self-contained, zero-network viewer (`viewer/neutron_transport_explorer.html`, `viewer/js/neutron_transport.js`) and small versioned fixture (`data/neutron/manifest_v0.json`, `data/neutron/fixtures/`) reproducing rotatable 3D reactor geometry, orthogonal XY/XZ/YZ slices, individual particle tracks, spatial-field overlays, control-blade withdrawal interaction, and human-scale comparison | Reproducible offline rendering (`file://` compatible, zero external CDNs or Google Fonts) with persistent on-render badges separating toy surrogate, recorded OpenMC playback, and display interpolation, without claiming physical validation | **DEFERRED (Gate Locked)** |
| **Stage 2: OpenMC Data Ingestion & Integrity** | Dedicated neutron-transport data contract (`NeutronTransportBundleV0`) and ingestion/validation scripts (`scripts/import_openmc_tracks.py`, `scripts/validate_neutron_bundle.py`) for recorded particle histories, mesh/lattice tallies, control-blade sweeps, and depletion series with explicit units, coordinate conventions, OpenMC version, nuclear-data revision, and SHA-256 digests | Data imported from original OpenMC statepoints and HDF5 track outputs matches the declared coordinate system, full-precision source values, precision-aware derived quantities, and cryptographic checksums | **DEFERRED (Gate Locked)** |
| **Stage 3: Numerical Verification & Reference-Model Audit** | Regeneration of key OpenMC numerical outputs from locked input decks, including a standalone minimal reproducer test for the claimed half-voxel vertical shift in OpenMC 3D voxel export, verification of the `MT 18` ($\sigma_f / \sigma_a$) absorption classifier, and reconciliation of $k_{\mathrm{eff}}$, blade-worth, fission/capture share, power-peaking, and burnup-crossing discrepancies | Data pipeline and numerical results are deterministically reproducible from source inputs; all statistical and methodological discrepancies are either resolved or explicitly retained with quantified bounds | **DEFERRED (Gate Locked)** |
| **Stage 4: Independent Validation & Uncertainty Quantification** | Re-execution with locked post-Oct 19 material and nuclear-data revisions and formal comparison of specified quantities of interest against independent benchmark references (IAEA-TECDOC-643 / ANL reference MCNP and transport benchmarks) not used to tune the client-side toy model | Acceptance criteria verify numerical accuracy, independent benchmark agreement, separated uncertainty budgets (Monte Carlo sampling, nuclear-data/material sensitivity, surrogate calibration error, and applicability limits), and documented domain boundaries | **DEFERRED (Gate Locked)** |

---

## 3. Separation of Source Materials into Four Distinct Roles

As documented with cryptographic digests in [`docs/references/NEUTRON_WALK_REFERENCE.md`](../references/NEUTRON_WALK_REFERENCE.md), the motivating source materials serve four separate roles and must never be conflated:

### 3.1 Role A: `TECDOC-643 Neutron Walk (Copy).html` -- Interactive Design & Data-Structure Reference
- **Artifact Identity:** `937,606` bytes (`832` lines / `831` newlines), SHA-256 `dc15eb934fe6cd32a084dc13d05ee17d8f5ad988f7ea1bd2598bdcd9a0065b78`, public reference URL `https://claude.ai/artifact/KcgJyGkRmxzpEFSc8Y12md`.
- **Design Inventory:** Combines a sticky rotatable 3D reactor core (`<canvas id="gl">`), orthogonal 2D cross-sectional slices (`XY`, `XZ`, `YZ` on `<canvas id="sl">` with display-only fuel-plate resolution and thermal-flux accumulator overlay), single-neutron history step-by-step narration (`Follow one neutron` with optional 3D camera ride-along), control-blade bank withdrawal slider (`0%` to `100%`), outcome and regional capture tallies, generation-by-generation chain-reaction population chart, $7 \times 6$ lattice relative power map, 5-point blade-worth curve, 350-day 10 MW burnup panel, and a 3D human-scale comparison (`180 cm` model height and `60 cm` active fuel height next to a `175.3 cm` person).
- **Mandatory Caveats:**
  1. **"Real OpenMC" is Recorded Playback, Not a Live Solver:** Selecting `Real OpenMC` replays 500 embedded particle trajectories (`250` for blades 100% inserted and `250` for blades 0% inserted, extracted from replica 0, seed 101 of an upstream OpenMC 0.15.3 run) in deterministic order and snaps the blade slider to `0%` or `100%`.
  2. **Pre-October 19 Material Definitions:** Both the `#realdata` JSON payload (`857,722` bytes, SHA-256 `df0c0e776c3cd69b1931d46e4668cc11e783ff6576b9fb3887140537cdf05f59`) and the HTML debrief explicitly state that the embedded OpenMC tracks, power maps, and blade-worth sweep use `pre-2026-10-19 materials` (`ENDF/B-VIII.0`) and must be rerun before being quoted.
  3. **Laptop Demo Depletion Run:** The `#burndata` JSON payload (`1,066` bytes, SHA-256 `b98b8c4b165801516d712db7d993a8babd78ea2c88c8620d1a0dca38e78a3b29`) is explicitly labeled as an October 21 laptop demo (`one fuel material per element, ENDF/B-VIII.0 with a reduced chain and 294 K data`) that crosses $k_{\mathrm{eff}} = 1$ near day `340.26`, whereas full-model cluster runs are reported to cross $k_{\mathrm{eff}} = 1$ near day `298`.
  4. **External Network Dependencies:** The HTML loads Google Fonts (`https://fonts.googleapis.com`, `https://fonts.gstatic.com`) and Three.js r128 from `https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js`. It is therefore **not** an offline zero-network implementation as-is.

### 3.2 Role B: Xenon-135 & Fission-Product Spatial Visualizations -- Spatial-Field Design Reference
- **Artifact Identities:**
  - ORNL four-isotope 3D spatial concentration comparison (`135I`, `135Xe`, `149Pm`, `149Sm` at `0 hr` and `10 hr`, normalized concentration `0.0` to `1.0`; `media_1792634977055_2a26966f.jpg`, `82,521` bytes, SHA-256 `1e538db7477e8e8accfb044bf8dd1e8541b75a84b156af8ec2a578490ac3181c`).
  - 3D reactor core cutaway volumetric rendering (`media_1792634876099_d3a3bf19.jpg`, `166,672` bytes, SHA-256 `0ac0826e54465813e9d741bb91cc2fb4716cc8b83886217ff084eeeaa36d8170`).
- **Mandatory Caveats:**
  1. These images are visual presentation references only (derived from an ORNL WB2 startup simulation context), not data to import into the 10 MW MTR model.
  2. Licensed third-party images must not be redistributed in the repository without explicit permission and attribution.
  3. **0D Time Series vs. 3D Spatial Field:** The `Xe135` array inside `#burndata` (`0.0` to `0.0578 g`) is an aggregate core-wide mass time series at 12 discrete depletion timestamps. It is **not** a verified 3D $\text{Xe-135}$ concentration field $N_{\mathrm{Xe}}(\mathbf{r}, t)$. A spatial fission-product field must come from a spatially resolved coupled transport-depletion calculation; it must never be reconstructed from a reference image or inferred by multiplying a 0D burnup curve by a heuristic spatial shape.

### 3.3 Role C: ASNR Neutron Laboratory Photograph -- Scientific Context & Presentation Reference
- **Artifact Identity:** Open-pool facility / neutron laboratory photograph (`media_1792634977032_7f7fdb01.jpg`, `126,089` bytes, SHA-256 `fccfab10371deabc73937d9e8286bfd4417415c14039f4c19e6fc29d4d853326`).
- **Mandatory Caveat:** Retained strictly as visual context for pool-type reactor geometry and instrumentation scale, not as empirical validation of the MTR model's parameters or predictions.

### 3.4 Role D: Validation-Type Article & Surrogate Parity Plot -- Methodology Reference
- **Artifact Identity:** Surrogate parity plot `"Predicted vs True Cd with Uncertainty (MC-Dropout)"` (`media_1792634977029_aa5729aa.jpg`, `42,730` bytes, SHA-256 `a60bf1c925ce930484cce07cd48a6ecdca50f900e998c37c7777560f4705a4bc`) and accompanying pSeven validation-methodology discussion.
- **Mandatory Caveat:** Internal train/test/validation splits evaluate how well a surrogate approximates the high-fidelity solver to which it was fitted. They do **not** validate the physical reactor model against independent physical evidence. All verification, validation, and uncertainty quantification (VVUQ) gates in this work package follow the National Academies (*Assessing the Reliability of Complex Models*) and NIST computational-modeling reliability principles.

---

## 4. Stage 2 Data Contract Specification (`NeutronTransportBundleV0`)

When Stage 2 is authorized, all neutron-transport visualization bundles (`data/neutron/manifest_v0.json` and fixture payloads) must be validated by `scripts/validate_neutron_bundle.py` before being loaded by `viewer/js/neutron_transport.js`.

### 4.1 Explicit Separation from R10 (`SpacetimeFieldBundleV0`)

`NeutronTransportBundleV0` shares **engineering infrastructure conventions** with R10 (`spacetime_geochemistry_v0`) while maintaining a completely separate physical schema:

| Engineering Convention | Shared Rule Across R10 and `neutron_transport_explorer_v0` | Domain-Specific Difference in `neutron_transport_explorer_v0` |
| :--- | :--- | :--- |
| **Data Integrity & Checksums** | Fail-closed validation of array dimensions, IEEE-754 finiteness, and SHA-256 digests (`sha256_f32_le` / `sha256_json`) | Covers branching variable-length particle track polylines (`[x_cm, y_cm, z_cm, energy_eV_or_group, weight, event_code]`), $7 \times 6$ lattice tallies, and 1D sweep/depletion tables rather than uniform $16^3$ geochemical voxel cubes |
| **Derived-Metadata Recomputation** | Validator and browser runtime independently recompute summary statistics from raw numeric arrays and reject mismatched metadata | Independently recomputes $\rho = (k - 1)/k$, $\Delta\rho = (k_2 - k_1)/(k_1 k_2)$, first-order $\sigma_{\mathrm{worth}}$ (with exact $0.0\text{ pcm}$ variance when comparing a reference state against itself), lattice power normalization (`mean(rel) == 1.0` over the 28 fuel elements), radial peaking factor $\max(\mathrm{rel})$, and linear $k_{\mathrm{eff}} = 1$ crossing day |
| **Evidence & Derivation Classification** | Every state and overlay carries machine-readable `evidence_class` and `state_derivation` tags | Distinguishes `toy_surrogate_live`, `recorded_openmc_history_playback`, `precomputed_openmc_tally_or_depletion`, `interpolated_for_display`, and `independently_validated_benchmark` |
| **Source & Receipt Provenance** | Separates `baseline_commit`, generator/importer script SHA-256, `tested_source_commit`, and `receipt_head` | Additionally requires `openmc_version`, `nuclear_data_library`, `nuclear_data_hdf5_sha256`, `material_revision_id`, `active_batches`, `inactive_batches`, `particles_per_batch`, and `replica_seed` |
| **Zero-Network Offline Operation** | 100% offline execution (`file://` compatible, zero runtime sockets, zero external CDNs or web fonts) | Replaces external `cdnjs` Three.js and Google Fonts in the reference HTML with local bundled assets and native system font stacks |

### 4.2 Coordinate System and Lattice Geometry Contract (`geometry_frame`)

Every `NeutronTransportBundleV0` bundle must declare its coordinate system and lattice mapping explicitly so that 3D scenes, 2D slices, and imported OpenMC tallies cannot silently shift or flip axes:
- `coordinate_system`: `"iaea_tecdoc643_core_cm"`
- `length_units`: Strictly `"cm"`
- `origin_definition`: Core center $(0, 0, 0)$, with $z = 0\text{ cm}$ at the active fuel midplane ($z \in [-30.0, +30.0]\text{ cm}$ active fuel meat; $z \in [-31.0, +31.0]\text{ cm}$ clad extension; $z \in [-45.0, -31.0] \cup [+31.0, +45.0]\text{ cm}$ end boxes; $z \in [-90.0, +90.0]\text{ cm}$ full water-pool model domain).
- `horizontal_axes`:
  - $+x$ points from Column `A` to Column `F` across the 6 columns (`NX = 6`, pitch $\Delta x = 7.7\text{ cm}$, active lattice half-width $H_x = 23.1\text{ cm}$, full pool half-width $23.1 + 38.5 = 61.6\text{ cm}$).
  - $+y$ points toward Row `1` (`NY = 7`, pitch $\Delta y = 8.1\text{ cm}$, active lattice half-height $H_y = 28.35\text{ cm}$, full pool half-height $28.35 + 38.5 = 66.85\text{ cm}$). Row index $j \in \{0, \dots, 6\}$ corresponds to Row $j+1$ centered at $y_j = H_y - \Delta y \cdot (j + 0.5)$.
- `vertical_axis`: $+z$ points vertically upward along the fuel plates and control-blade withdrawal direction.
- `display_frame_mapping`: Any 3D WebGL renderer that uses $y$-up screen coordinates (such as Three.js `V(x, y, z) = [x, z, -y]`) must declare that permutation as a **view-only transform** while keeping all stored coordinates, slice plane sliders (`XY` at $z$, `XZ` at $y$, `YZ` at $x$), and hit-testing in canonical model coordinates $(x, y, z)\text{ [cm]}$.
- `lattice_map_7x6`:
  - Row 1 (`+y` edge): `["GR", "GR", "GR", "GR", "GR", "GR"]` (6 graphite reflector elements)
  - Row 2: `["SFE", "SFE", "CFE", "SFE", "SFE", "SFE"]` (`C2` = control fuel element)
  - Row 3: `["SFE", "SFE", "SFE", "SFE", "CFE", "SFE"]` (`E3` = control fuel element)
  - Row 4 (center): `["SFE", "CFE", "SFE", "FT", "SFE", "SFE"]` (`B4` = control fuel element, `D4` = central water-filled flux trap)
  - Row 5: `["SFE", "SFE", "SFE", "SFE", "CFE", "SFE"]` (`E5` = control fuel element)
  - Row 6: `["FT", "SFE", "CFE", "SFE", "SFE", "SFE"]` (`A6` = corner flux trap, `C6` = control fuel element)
  - Row 7 (`-y` edge): `["GR", "GR", "GR", "GR", "GR", "GR"]` (6 graphite reflector elements)
  - Element counts: `23 SFE` (23 fuel plates each) + `5 CFE` (17 fuel plates each + twin $\text{B}_4\text{C}$ absorber blades) = `28` fuel elements (`614` total fuel plates), `2 FT` water flux traps, and `12 GR` graphite reflector blocks (`42` total lattice positions).

### 4.3 Particle Track and Absorption Classification Contract (`tracks`)

1. **Full vs. Decimated Track Provenance:**
   OpenMC native track exports (`tracks.h5`) record every surface crossing along straight flights as well as collision points, together with particle position $(x, y, z)$, direction $(u_x, u_y, u_z)$, continuous energy $E\text{ [eV]}$, time $t\text{ [s]}$, weight $w$, cell ID, and material ID. By contrast, the embedded `#realdata.tracks` in `TECDOC-643 Neutron Walk (Copy).html` drops straight-flight surface crossings and quantizes energy to a binary group flag (`fast = 1` if $E > 0.625\text{ eV}$, else `0`).
   `NeutronTransportBundleV0` must explicitly record `surface_crossings_retained: boolean` and `energy_representation: "continuous_ev" | "two_group_0p625ev_threshold"`.
2. **Honest Absorption Classification (`MT 18` vs. $\sigma_f / \sigma_a$):**
   As noted in `#realdata.tracks.meta`, OpenMC's raw particle-track event flag can reflect secondary neutron creation rather than true physical fission vs. radiative capture. Any imported track dataset that classifies terminal absorption events into `fission` vs. `capture` must declare `termination_classifier`:
   - `"openmc_raw_event_flag"`
   - `"microscopic_sigma_f_over_sigma_a_sampled"` (with the exact seed, nuclide cross-section table, and verification test showing that the resulting fission fraction reproduces the run's $k_{\mathrm{eff}} / \bar{\nu}$ within declared statistical tolerance).
3. **Blade-State Playback Locking:**
   When displaying recorded OpenMC trajectories that exist only for discrete control-blade states (e.g., `100%` inserted and `0%` inserted), the UI must lock or snap the blade control to the available discrete states and forbid presenting recorded trajectories at unsimulated intermediate blade positions.

### 4.4 Interpolation and Surrogate-Aging Disclosure Policy

1. **Depletion Time-Series Interpolation (`blerp`):**
   Stored depletion states in `#burndata` exist only at 12 discrete days (`[0, 5, 10, 20, 40, 80, 120, 160, 200, 250, 300, 350]`). Whenever a user selects a non-tabulated intermediate day $d$, the viewer must stamp the state with `state_derivation: "interpolated_for_display"`, `run_receipt: null`, and display an explicit interpolation badge.
2. **Surrogate Reactivity-Aging Coupling (`applyAge`):**
   In the reference HTML, checking `"Age the toy core to this day"` scales the client-side toy model's thermal fission probability via:
   $$P_{\mathrm{fiss,thermal}}(d) = 0.792 \times \frac{k_{\mathrm{burnup}}(d)}{k_{\mathrm{burnup}}(0)}$$
   while leaving spatial cross sections and fission-product distributions uniform across the core. In Stage 1+, this mode must be labeled `"toy_surrogate_reactivity_scaled_by_0d_burnup"` so a user never mistakes it for a 3D spatially depleted OpenMC transport calculation.

---

## 5. Stage 3 Numerical Verification and Discrepancy Audit Requirements

Before Stage 3 can pass its promotion gate, the following five numerical verification tasks and unresolved discrepancies documented in [`docs/references/NEUTRON_WALK_REFERENCE.md`](../references/NEUTRON_WALK_REFERENCE.md) must be formally tested:

1. **Dedicated Minimal Reproducer for the Claimed OpenMC 3D Voxel Half-Voxel Shift:**
   - **Claim:** Step 5 of the HTML debrief claims that OpenMC's 3D voxel export (`openmc.VoxelPlot` / HDF5 voxel export) is shifted vertically by `+0.5` voxel (`half a voxel up`), and states that a small test case was written locally on `feature/track-viz` but never pushed or reported upstream.
   - **Status:** **UNVERIFIED LEAD.** The HTML alone contains neither the voxel export file nor the minimal reproducer script.
   - **Stage 3 Gate Requirement:** Construct a self-contained minimal OpenMC geometry (e.g., an analytically bounded slab or box at $z \in [-30.0, +30.0]\text{ cm}$ discretized on an explicit $N_z$ voxel grid), export the 3D voxel HDF5/VTK file, and compare voxel index centers $z_k = z_{\min} + (k + 0.5)\,\Delta z$ against the analytical cell boundaries. Document whether the offset originates in OpenMC's C++ voxel rasterizer, the Python post-processing script, or VTK/Three.js cell-corner vs. cell-center coordinate interpretation.
2. **Reconciliation of Four Fresh-Core $k_{\mathrm{eff}}$ Datasets:**
   Reconcile the exact input differences (material compositions, `ENDF/B-VIII.0` vs. `ENDF/B-VII.1`, batch/history counts, and blade model) among:
   - UI Table `"OpenMC (Oct 19, VII.1)"`: $k_{\mathrm{in}} = 0.98106$, $k_{\mathrm{out}} = 1.19732$ (implied worth $18,410.7\text{ pcm}$)
   - UI Table `"Reference MCNP model"`: $k_{\mathrm{in}} = 0.98189$, $k_{\mathrm{out}} = 1.19872$ (implied worth $18,422.1\text{ pcm}$)
   - Debrief 2M-history run (`Oct 14, ENDF/B-VIII.0, pre-Oct 19 materials`): $k_{\mathrm{in}} = 0.98057 \pm 0.00069$, $k_{\mathrm{out}} = 1.19621 \pm 0.00063$ (two-point worth $18,384.14 \pm 84.19\text{ pcm}$)
   - Sweep endpoints (`Oct 14, 800k histories/point, model rebuilt per position`): $k_{\mathrm{in}} = 0.97943 \pm 0.00102$, $k_{\mathrm{out}} = 1.19810 \pm 0.00081$ (recomputed endpoint worth $18,634.71 \pm 120.37\text{ pcm}$; stored `worth_pcm = 18,635.3 +/- 120.0 pcm`).
3. **Explanation of the $1.71\sigma$ (`251.16 pcm`) Two-Point vs. Sweep Blade-Worth Discrepancy:**
   Establish whether the difference between the 2M-history two-point worth (`18,384.14 +/- 84.19 pcm`) and the 800k-history sweep worth (`18,635.3 +/- 120.0 pcm`, difference `251.16 pcm`, combined $\sigma = 146.59\text{ pcm}$, $z = 1.71\sigma$) is purely Monte Carlo sampling variance between independent seeds/batch counts or reflects a geometry/material difference when `"model rebuilt per position"` was executed for `whatif.json`.
4. **Full-Precision Source Ingestion vs. Rounded Display Values:**
   Eliminate precision drift between rounded 5-decimal $k_{\mathrm{eff}}$ values and stored derived fields (`18,634.714 pcm` vs. `18,635.3 pcm`; `120.375 pcm` vs. `120.0 pcm`; linear crossing `Day 340.254` vs. `Day 340.2611`) by ingesting unrounded IEEE-754 Float64 values directly from OpenMC statepoints and computing all derived quantities deterministically in code.
5. **Reconciliation of the `Day 340.26` vs. `Day 298` Burnup Crossing Discrepancy:**
   Document and quantify the physical modeling approximations (1 fuel material per element vs. spatially resolved depletion zones, 294 K vs. operating temperature cross sections, and reduced depletion chain vs. full fission-product chain) that shift the $k_{\mathrm{eff}} = 1$ crossing from `~298 days` in the full cluster model to `340.26 days` in the laptop demo.

---

## 6. Cross-Repository Boundary with `RadioPharm.jl` (`MURR`)

Any bridge between the IAEA-TECDOC-643 benchmark data in `TECDOC-643 Neutron Walk (Copy).html` and the `RadioPharm.jl` (`murr_operator_kinetics.jl`) stack must enforce four mandatory red-team invariants (detailed in [`docs/references/NEUTRON_WALK_REFERENCE.md`](../references/NEUTRON_WALK_REFERENCE.md) Section 4):

1. **Generic Operator Equations Bridge; Reactor-Specific Physics Does Not Transfer:**
   The algebraic definitions $\rho = (k_{\mathrm{eff}} - 1)/k_{\mathrm{eff}}$, $\mathrm{SDM} = (1 - k_{\mathrm{eff}})/k_{\mathrm{eff}}$, and $\Delta\rho(k_1 \to k_2) = (k_2 - k_1)/(k_1 k_2)$ are universal. However, the IAEA-TECDOC-643 benchmark (a 10 MW open-pool rectangular $6 \times 7$ plate-type MTR core with graphite reflector rows and two lattice flux traps at `D4` and `A6`) is a **materially different physical system** from the University of Missouri Research Reactor (MURR, a 10 MW pressurized annular core of 8 wedge-shaped HEU/LEU assemblies surrounding a single central island flux trap with beryllium and graphite reflectors and annular Boral control blades). TECDOC-643 tables may be used in a quarantined test fixture (such as `/Users/aurascoper/Downloads/tecdoc643_reference_fixture.jl`) to unit-test reactivity algebra, but **must never** be imported as MURR calibration parameters, MURR blade-worth tables, or MURR neutron-activation GLMM variance components.
2. **Precision-Aware Recomputation and Honest Isotopic Attribution:**
   Ingestion functions must compute derived reactivity and uncertainty values from raw inputs rather than hard-coding asserted strings (`18,634.714 pcm` and `120.375 pcm` from rounded $k$ values vs. `18,635.3 pcm` and `120.0 pcm`; linear crossing `Day 340.254` vs. `Day 340.2611`). Furthermore, the net reactivity change between Day 0 ($k = 1.19676$) and Day 5 ($k = 1.15243$), $\Delta\rho = -3,214.224\text{ pcm}$, represents the **total core reactivity change** across all nuclides ($\text{U-235}$ depletion, $\text{Pu-239}$ buildup, $\text{Xe-135}$, $\text{Sm-149}$, and other fission products) and must not be attributed solely to $\text{Xe-135} + \text{Sm-149}$ without an explicit perturbation or reaction-rate worth decomposition.
3. **Prohibition on Combining MTR Equilibrium $\text{Xe-135}$ with a MURR Shutdown Transient Multiplier:**
   A pipeline must **never** multiply the MTR full-power equilibrium `Xe135` gram inventory (`#burndata.core_g.Xe135`) by MURR's 12-hour post-shutdown peak multiplier (`xe_trans.xe135_peak_rel_eq = 11.82`). That multiplier is derived for MURR's high thermal flux ($\phi_0 = 3.0 \times 10^{14}\text{ n cm}^{-2}\text{ s}^{-1}$); applying it across burnup states of a lower-flux rectangular MTR core fabricates an unphysical hybrid transient.
4. **Zero Variance at the Self-Compared Reference State ($i = N$):**
   When propagating $1\sigma$ uncertainty for integral blade worth relative to a reference state $k_{\mathrm{ref}}$ (such as `100%` withdrawn), the independent-endpoint formula $\sigma(\Delta\rho) = 10^5 \sqrt{(\sigma_i / k_i^2)^2 + (\sigma_{\mathrm{ref}} / k_{\mathrm{ref}}^2)^2}$ applies only when state $i$ and the reference state are independent Monte Carlo estimates ($i \neq \mathrm{ref}$). At the reference state itself ($i = \mathrm{ref}$, where $\Delta\rho(k_{\mathrm{ref}}, k_{\mathrm{ref}}) \equiv 0$ identically), perfect self-correlation ($\mathrm{Cov}(k_{\mathrm{ref}}, k_{\mathrm{ref}}) = \sigma_{\mathrm{ref}}^2$) requires $\sigma(\Delta\rho_{\mathrm{ref} \to \mathrm{ref}}) = 0.0\text{ pcm}$ (correcting the `79.802 pcm` self-comparison artifact found in `TECDOC643ReferenceFixture.blade_sweep()` row 5).

---

## 7. Proposed Repository Layout for Future Authorized Stages

The initial commit on `neutron-transport-explorer/v0-spec` is strictly documentary and creates only the first two files below. No viewer code, fixtures, or verification receipts are created until Stage 1+ is explicitly authorized:

```text
docs/
  specs/
    NEUTRON_TRANSPORT_EXPLORER_v0.md        # [Created in Stage 0] Work package spec & stage gates
  references/
    NEUTRON_WALK_REFERENCE.md               # [Created in Stage 0] Forensic audit of HTML, media & Julia fixture
data/
  neutron/
    manifest_v0.json                        # [Deferred: Stage 1/2] Bundle manifest & SHA-256 registry
    fixtures/                               # [Deferred: Stage 1/2] Small traceable track & tally fixtures
scripts/
  import_openmc_tracks.py                   # [Deferred: Stage 2] OpenMC statepoint/tracks.h5 importer
  validate_neutron_bundle.py                # [Deferred: Stage 2] Fail-closed contract & precision validator
viewer/
  neutron_transport_explorer.html           # [Deferred: Stage 1] Self-contained offline 3D + slice viewer
  js/
    neutron_transport.js                    # [Deferred: Stage 1] Renderer, slice engine & integrity verifier
tests/
  test_neutron_transport_v0.js              # [Deferred: Stage 1/2] Automated DOM, checksum & algebra tests
receipts/
  neutron_transport_v0_verification.json    # [Deferred: Stage 1+] Machine-readable verification receipt
  neutron_transport_v0_verification.md      # [Deferred: Stage 1+] Human-readable verification receipt
```

Full raw OpenMC HDF5 statepoint files, full `tracks.h5` archives, and multi-gigabyte `ENDF/B-VIII.0` nuclear-data libraries must **not** be committed into Git; future stages will commit only compact, cryptographically hashed fixtures alongside deterministic regeneration scripts and external archive manifests.

---

## 8. Coupled Geochemical-Neutronic Domain Boundary (UO2-Clay-Fracture Framework & Evidence Gates)

Any future proposal to couple neutron transport (`neutron_transport_explorer_v0`) with geological porous/fracture flow and reactive transport (`spacetime_geochemistry_v0` / Oklo natural reactor scenarios) must treat the **UO2-Clay-Fracture Conceptual Framework** as a separately gated, multi-physics specification rather than an implicit extension of Stage 0:

1. **Physical Scenario and State Variables (Bounded Parameter Ranges Required):**
   - **Uraninite ($\text{UO}_2$) Core:** Core temperature $T_{\mathrm{core}} \in [300, 2000]\text{ K}$ ($\text{UO}_2$ density $\approx 10.97\text{ g/cm}^3$, melting point $\approx 3340\text{ K}$, thermal conductivity $k_{\mathrm{th}} \approx 2\text{ to }8\text{ W/(m}\cdot\text{K)}$ decreasing with $T$, heat capacity $c_p \approx 270\text{ J/(kg}\cdot\text{K)}$ at $300\text{ K}$), $^{235}\text{U}$ enrichment $\eta \in [0.0072, 0.05]$, neutron flux $\Phi\text{ [n cm}^{-2}\text{ s}^{-1}]$ (up to $10^{11}\text{ to }10^{12}\text{ n cm}^{-2}\text{ s}^{-1}$ in natural low-power Oklo-type zones vs. $10^{13}\text{ to }10^{14}\text{ n cm}^{-2}\text{ s}^{-1}$ in engineered MTR cores), and Bateman decay/fission-product inventories ($N_i(t)$).
   - **Argillaceous Clay Host (e.g., Bentonite / Opalinus / Boom Clay):** Porosity $\phi \in [0.2, 0.5]$, permeability $k \sim 10^{-22}\text{ to }10^{-17}\text{ m}^2$ (hydraulic conductivity $k_h \approx 10^{-13}\text{ to }10^{-12}\text{ m/s}$), effective diffusivity $D_{\mathrm{eff}} \sim 10^{-11}\text{ to }10^{-10}\text{ m}^2\text{/s}$, thermal conductivity $1\text{ to }3\text{ W/(m}\cdot\text{K)}$, and linear/surface-complexation retardation $R = 1 + \rho_b K_d / \phi$.
   - **Vertical Fracture Conduit:** Aperture $a$ ($\text{mm}$ to $\text{cm}$ scale, Poiseuille flux $q_f = -(a^3 / 12\mu)\nabla P$ or Forchheimer non-Darcy flow at higher Reynolds numbers), water saturation $S \in [0, 1]$ (controlling both hydraulic transport and thermal neutron moderation), and fracture-to-clay diffusive/advective exchange.
2. **Dimensionless Regime Criteria:**
   Every coupled benchmark must declare its operating regime via five dimensionless groups:
   - **Reynolds number** $\mathrm{Re} = \rho U L / \mu$ ($\mathrm{Re} \ll 1$ laminar Darcy in clay; $\mathrm{Re} \sim 10^0\text{ to }10^2$ in mm-scale fractures).
   - **Peclet number** $\mathrm{Pe} = U L / D$ ($\mathrm{Pe} \gg 1$ advection-dominated in fractures; $\mathrm{Pe} \ll 1$ diffusion-dominated in clay).
   - **Damkohler number** $\mathrm{Da} = k_{\mathrm{rxn}} L / U$ ($\mathrm{Da} \gg 1$ local sorption equilibrium vs. $\mathrm{Da} \ll 1$ reaction-limited).
   - **Biot number** $\mathrm{Bi} = h L / k_{\mathrm{th}}$ and **Rayleigh number** $\mathrm{Ra} = g \beta \Delta T L^3 / (\nu \alpha)$ (onset of buoyancy-driven thermal convection).
3. **Candidate Solver Separation Matrix:**
   Neutron transport codes (`OpenMC`, `Serpent`) do not solve multiphase reactive geochemistry, and reactive-transport/CFD codes (`PFLOTRAN`, `OpenFOAM`, `FEniCS`, `MOOSE`, `COMSOL`) require explicit operator-splitting or monolithic coupling interfaces with verified mass and energy balance closure before coupling is claimed.
4. **Strict Evidence Gate for Radiolytic Polymerization and Polymer-Mediated Attenuation:**
   Consistent with Oklo v1 Gate 3 Option A (where Field 4 `Polymer accumulation index` is disclosed in `arbitrary units` with no established kinetic or thermodynamic basis) and R6A (`surface_matrix_protection_v0`), **radiolytic polymerization yield ($Y_{\mathrm{poly}}$), synthesis rate ($r_p$), and polymer-mediated radiation attenuation must remain quarantined out of production solvers** unless three empirical gates are satisfied:
   - **Gate E1 (Chemical Identification & Yield):** Laboratory irradiation of representative clay + organic precursors at target dose rates and temperatures demonstrates measurable polymer formation with quantified $G$-values (molecules per $100\text{ eV}$).
   - **Gate E2 (Kinetic & Thermodynamic Parameterization):** Rate constants, monomer availability, and thermal stability bounds are independently measured rather than assumed.
   - **Gate E3 (Measured Attenuation Cross Sections):** Gamma and neutron attenuation coefficients of the resulting polymer phase are experimentally shown to differ materially from the surrounding porewater and clay matrix.
