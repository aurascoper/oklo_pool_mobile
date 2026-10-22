# Planetary Explorer v2: Architecture Specification and Epistemic Separation Contracts

- Document ID: `PLANETARY_EXPLORER_ARCHITECTURE_v2`
- Target Branch: `planetary-explorer/v2-prototype`
- Baseline Commit: `636b7d7bb41999b80278d90046da893ee1044f20`
- Authoritative Prompt Reference: `ORIGINAL_REQUEST.md` (Requirements R0, R4, R5, R6, R7, R8, R9)
- Milestone: Milestone B1 (Planetary Explorer v2 Architecture Specification)
- Status: Approved Architecture Specification

---

## 1. Executive Summary

The Planetary Explorer v2 platform is an interactive, browser-based multi-scale exploration environment designed to provide an educational and scientific journey from the planetary scale down to localized geological, hydrological, and geochemical compartments:
1. Stage 1 (Planetary Globe): Rotatable and zoomable Earth with sourced geographic locators and deep-time paleo-interval timelines.
2. Stage 2 (Geological Cutaway): Smooth camera transition from the site locator into a local 3D schematic compartment block.
3. Stage 3 (Revision-0 Field Inspection): Volumetric and orthogonal slice inspection of the six prescribed Oklo fields with persistent limitation disclosures.
4. Stage 4 (Separate Transport-Model Inspection): Independent inspection panels for precomputed numerical transport models (steady Darcy porous flow and 1D uranium reduction-transport reactive column).

A foundational imperative of this architecture is strict epistemic separation (Requirement R7):
- Prescribed analytic fields are never conflated with solved physical differential equations.
- Synthetic transport models solved on idealized geometries are never represented as calibrated historical subsurface reconstructions of the Oklo reactor zones or the Franceville Basin.
- Modern geographic locators (WGS84) are never conflated with Paleoproterozoic (2.0 Ga) paleogeography.
- External research repositories (`Biofilms`, `rna_biofilm_toy`, `SobolevMacromolecule`) are integrated strictly as read-only architectural, methodological, or precomputed visual references under explicit boundary contracts (Requirement R6).
- Coupled origins-of-life simulations, mechanistic polymer replacements, and biological dynamics are explicitly out of scope (Requirement R9).

---

## 2. Architectural Principles and System Boundaries

### 2.1 The Core Architectural Principles

1. **Decoupled Computation and Presentation**:
   The client-side browser runtime (`viewer/planetary_explorer.html` and modular assets) is strictly a visualization and exploration client. Numerical simulation runs, partial differential equation (PDE) solves, and finite-volume matrix inversions are computed offline and bundled as cryptographically verified run receipts. The client performs zero live PDE execution, zero neural network inference, and zero unverified parameter interpolation.

2. **Epistemic Invariance (Anti-Contagion Rule)**:
   Visual proximity in a 3D scene does not confer scientific validity. Visualizing a prescribed analytic field adjacent to a real geographic basemap or within a photorealistic terrain mesh does not promote that field to an empirical observation. Every rendered entity carries an immutable, machine-readable evidence class badge.

3. **Units and Coordinate System Discipline**:
   Every spatial coordinate and physical quantity must declare an explicit coordinate system and physical unit. Blending quantities across incompatible coordinate frames (e.g. mapping 1D column coordinates into 3D voxel grids without a defined transformation tensor) is forbidden and fails closed.

4. **Fail-Closed Verification**:
   If an asset lacks cryptographic hash verification, contains undefined units, or fails schema validation, the system refuses to render the asset in an elevated state and emits an explicit error or fallback notice.

---

## 3. The Five Interoperable Subsystems

The Planetary Explorer v2 platform is composed of five modular, interoperable subsystems:

```
+-----------------------------------------------------------------------------------+
|                            5. EXPLORATION INTERFACE                               |
|   (Stage State Machine, Camera Transitions, HUD Readouts, Timeline, Badges)       |
+-----------------------------------------------------------------------------------+
         |                                   |                              |
         v                                   v                              v
+------------------+             +----------------------+         +-----------------+
|  1. SPATIAL      |             | 2. 3D VISUALIZATION  |         | 3. SCIENTIFIC   |
|     ENGINE       |             |    ENGINE            |         |    MODEL ENGINE |
| (WGS84, Local    |<----------->| (WebGL2 Raymarcher,  |<------->| (Precomputed    |
|  Cartesian 16m^3,|             |  Canvas2D Fallback,  |         |  Run Loaders,   |
|  Coordinate Maps)|             |  Shaders, Colormaps) |         |  Field Buffers) |
+------------------+             +----------------------+         +-----------------+
         |                                   |                              |
         +-----------------------------------+------------------------------+
                                             |
                                             v
                         +---------------------------------------+
                         | 4. EVIDENCE AND PROVENANCE LAYER      |
                         | (Epistemic Classes, Manifest Schema,  |
                         |  Audit Hashes, Bibliographic Sources) |
                         +---------------------------------------+
```

---

### 3.1 Subsystem 1: Spatial Engine

The Spatial Engine manages all spatial frames, geodetic mappings, coordinate transformations, and depth stratifications across scales ranging from 12,742 kilometers (global Earth diameter) down to 0.5 meters (local voxel pitch).

#### A. Coordinate Frames and Definitions

1. **Global Geographic Frame (`FRAME_GEO_WGS84`)**:
   - Standard: WGS84 ellipsoidal coordinates (EPSG:4326).
   - Coordinates: `(latitude, longitude, elevation_m)` with latitude in `[-90.0, 90.0]` degrees, longitude in `[-180.0, 180.0]` degrees, elevation in meters above sea level.
   - Purpose: Positioning geographic site locators on the planetary sphere.

2. **Geocentric Spherical Direction Vector (`FRAME_SPHERICAL_UNIT`)**:
   - Direction vector on the unit sphere:
     $$\mathbf{u} = \begin{bmatrix} \cos(\phi) \cos(\lambda) \\ \sin(\phi) \\ -\cos(\phi) \sin(\lambda) \end{bmatrix}$$
     where $\phi$ is latitude in radians and $\lambda$ is longitude in radians.
   - Radius $R_E$: Scaled for WebGL rendering (default $R_E = 1.0$ unit).

3. **Local Site Cartesian Frame (`FRAME_LOCAL_CARTESIAN`)**:
   - Standard: Right-handed Euclidean Cartesian system `(x, y, z)` in meters.
   - Origin: Site reference benchmark at the surface of Oklo Reactor Zone 2.
   - Orientation: $+x$ eastward, $+y$ upward (elevation), $+z$ southward.

4. **Prescribed Compartment Fixture Space (`FRAME_FIXTURE_GRID`)**:
   - Standard: Dimensionless integer voxel indices `(i, j, k)` with $i, j, k \in \{0, 1, \dots, 31\}$.
   - Metric Domain: Nominal 16-meter cubic volume $[0, 16\text{ m}]^3$.
   - Voxel Discretization: $\Delta x = \Delta y = \Delta z = 0.5\text{ m}$ ($N = 32$, $N^3 = 32,768$ voxels).
   - Coordinate Mapping:
     $$x = (i + 0.5) \cdot 0.5\text{ m}, \quad y = (j + 0.5) \cdot 0.5\text{ m}, \quad z = (k + 0.5) \cdot 0.5\text{ m}$$

5. **One-Dimensional Transport Column Frame (`FRAME_TRANSPORT_1D`)**:
   - Standard: One-dimensional flow-path coordinate $x \in [0, 16\text{ m}]$.
   - Spatial Step: $\Delta x = 16.0 / (N_{col} - 1)$ meters, where $N_{col} \in \{64, 81, 128, 256, 512, 1024\}$.
   - Origin: Inflow boundary $x = 0\text{ m}$; Outflow boundary $x = 16\text{ m}$.
   - Stationary Reactive Barrier: Organic-rich reducing zone located between $x = 6.0\text{ m}$ and $x = 10.0\text{ m}$.

#### B. Depth Mappings and Layer Stratification

Within the local 3D compartment volume $[0, 16\text{ m}]^3$, the vertical index $k \in [0, 31]$ maps depth relative to the geological cutaway:
- Depth index $k = 0$ corresponds to the base of the block ($z = 0.25\text{ m}$).
- Depth index $k = 31$ corresponds to the top surface of the block ($z = 15.75\text{ m}$).
- Stratigraphic Compartment Assignment:
  - Base Sandstone Matrix (Zone 0): Default host rock across all indices unless overridden by core, halo, conduit, or chimney.
  - Reactor Core (Zone 1): Centered at $k_{core} = \text{round}(0.35 \times 32) = 11$ ($z \approx 5.5\text{ m}$), occupying radial radius $r_{core} = 4\text{ voxels}$ ($2.0\text{ m}$).
  - Hydrothermal Alteration Halo (Zone 2): Annular shell wrapping the core, extending to $r_{halo} = 7\text{ voxels}$ ($3.5\text{ m}$).
  - Fracture Conduit (Zone 3): Vertical permeable corridor extending from the core center upward ($k \ge 11$) with cross-sectional radius $r_{conduit} = 2\text{ voxels}$ ($1.0\text{ m}$). Evaluates to exactly zero below $k = 11$.
  - Steam Chimney (Zone 4): Degassed vadose conduit at upper boundary ($k \ge 24$) appearing conditionally during active heating phases.

#### C. Cross-Frame Coordinate Transformation Rules

- Rule 1: A conversion between `FRAME_GEO_WGS84` and `FRAME_LOCAL_CARTESIAN` requires an explicit origin anchor record with verified geodetic coordinates.
- Rule 2: Mapping between `FRAME_TRANSPORT_1D` and `FRAME_FIXTURE_GRID` is disallowed without an approved spatial projection contract. 1D transport results must render in isolated Cartesian line/profile viewports, never smeared arbitrarily across 3D voxels.
- Rule 3: Coordinate validation functions must enforce strict range checking. Inputs outside $[-90, 90]$ latitude or $[-180, 180]$ longitude throw immediate exceptions.

---

### 3.2 Subsystem 2: 3D Visualization Engine

The 3D Visualization Engine renders global, cutaway, volumetric, and slice graphics across all prototype stages.

#### A. Rendering Pipeline Architecture

1. **Primary Volume Raymarcher (WebGL2)**:
   - Target: Modern desktop and mobile browsers supporting WebGL2 (`WebGL2RenderingContext`).
   - Texture Representation: Single-channel 8-bit unsigned normalized 3D texture (`gl.R8`, format `gl.RED`, type `gl.UNSIGNED_BYTE`) with dimensions $32 \times 32 \times 32$.
   - Sampling Strategy: Front-to-back raymarching through the unit bounding cube $[-0.5, 0.5]^3$.
   - Ray Step Budget: Exactly 192 uniform ray steps along the view vector.
   - Compositing: Alpha-blended emission-absorption transfer function:
     $$C_{accum} = C_{accum} + (1.0 - A_{accum}) \cdot C_{sample} \cdot A_{sample}, \quad A_{accum} = A_{accum} + (1.0 - A_{accum}) \cdot A_{sample}$$
     with early ray termination when $A_{accum} \ge 0.98$.
   - Transfer Function Colormap: $256 \times 1$ pixel RGBA 2D texture (`gl.RGBA8`).

2. **Secondary Cutaway Mesh Renderer**:
   - Target: Visualizing conceptual compartment boundaries and stratigraphic slabs.
   - Representation: Indexed triangle meshes and wireframe bounding edges with depth testing enabled.
   - Shading: Ambient plus directional Phong diffuse shading to convey 3D form without misleading photorealism.

3. **Fallback 2D Canvas Slice Renderer**:
   - Target: Environments where WebGL2 initialization fails, context is lost, or software rendering is active.
   - Implementation: Dual HTML5 Canvas 2D contexts (`CanvasRenderingContext2D`):
     - Plan Slice: Horizontal $X-Y$ cross-section at constant depth index $k \in [0, 31]$.
     - Section Slice: Vertical $X-Z$ cross-section at constant lateral index $j \in [0, 31]$.
   - Image Generation: Direct manipulation of `ImageData` 32-bit pixel arrays with bilinear scaling to target canvas dimensions.
   - Robustness: Seamless fallback without unhandled JavaScript exceptions. When WebGL2 fails, the UI disables the 3D canvas tab, switches to the 2D slice tab, and displays a prominent informative banner: `"3D volumetric rendering requires WebGL2; displaying 2D orthogonal slice fallback."`

#### B. Camera State Machine and Stage Transition Controllers

The camera system operates as a finite state machine managing 5 distinct exploration stages:

```
[STATE_1_GLOBE] <==== Dive Animation ====> [STATE_2_CUTAWAY]
       |                                           |
       |                                           v
       +<=== Navigation Tabs ===> [STATE_3_FIELD_INSPECTOR]
       |                                           |
       |                                           v
       +<=== Navigation Tabs ===> [STATE_4_TRANSPORT_MODELS]
```

1. **`STATE_1_GLOBE` (Planetary Globe View)**:
   - Camera: Spherical orbit camera centered at planetary origin $(0, 0, 0)$.
   - Radius: Adjustable from $1.5 R_E$ to $6.0 R_E$.
   - Controls: Mouse drag / touch drag rotates longitude ($\theta$) and latitude ($\phi$); mouse wheel / pinch zooms radius.
   - Constraints: Pitch $\phi$ is clamped to $[-1.45, 1.45]$ radians ($[-83^\circ, +83^\circ]$) to eliminate gimbal lock at poles.

2. **`STATE_TRANSITION_DIVE` (Smooth Site Descent)**:
   - Trigger: User clicks on Oklo site marker pin.
   - Interpolation: Slerp (spherical linear interpolation) for camera orientation; smoothstep (cubic Hermite) for camera distance:
     $$s(t) = 3t^2 - 2t^3, \quad t \in [0, 1]$$
   - Duration Budget: Exactly 600 milliseconds.
   - Interruption Safety: If user clicks or drags during transition, the active animation loop is canceled, and the camera assumes the target cutaway state cleanly.

3. **`STATE_2_CUTAWAY` (Geological Cutaway View)**:
   - Camera: Orbital camera focused on the center of the local 16-meter compartment block $(8, 8, 8)$.
   - Distance: 28 meters from block center; elevation angle $30^\circ$, azimuth angle $45^\circ$.
   - Visual: Semi-transparent bounding cube displaying schematic strata (Sandstone, Core, Halo, Conduit).

4. **`STATE_3_FIELD_INSPECTOR` (Revision-0 Field Inspector)**:
   - Camera: Synchronized between 3D volume raymarcher and orthogonal 2D slice views.
   - Slice Plane Sliders: Depth slice index $k \in [0, 31]$ and Lateral slice index $j \in [0, 31]$.

5. **`STATE_4_TRANSPORT_MODELS` (Transport Model Inspector)**:
   - Camera: Dedicated dual-panel layout rendering 3D Darcy hydraulic head contours alongside 1D Uranium breakthrough curves.

#### C. Colormap Specifications

To avoid perceptual distortions and maintain colorblind accessibility:
- `temperature`: Colormap `inferno` (black to violet to orange to yellow). Opacity scales linearly from 0.0 at $T_{min}$ to 0.85 at $T_{max}$.
- `hydration`: Colormap `blues` (light cyan to dark navy). Opacity is inverted: highest opacity at low hydration ($0.02$) to accentuate desiccated reactor steam zones; near-transparent at saturated pore space ($1.0$).
- `h2_concentration`: Colormap `viridis` (purple to teal to yellow).
- `u235_ratio`: Colormap `cividis`, inverted (bright yellow at depleted ratio $0.004$, dark blue at natural baseline $0.00725$).
- `polymer_yield`: Colormap `magma` (black to purple to red to yellow).
- `compartments`: Discrete categorical palette:
  - Sandstone (Zone 0): Tan `#d8c79f`
  - Reactor Core (Zone 1): Deep Plum `#6b2d82`
  - Alteration Halo (Zone 2): Olive Sage `#8ea86c`
  - Fracture Conduit (Zone 3): Cobalt Blue `#3c7fc0`
  - Steam Chimney (Zone 4): Vapor Gray `#f0eee8`

---

### 3.3 Subsystem 3: Scientific Model Engine

The Scientific Model Engine manages data loading, field array allocation, phase cycling, and parameter sweep selection for all active scientific models.

#### A. Revision-0 Prescribed Fields Integration

- WebAssembly Module: Loaded from `data/oklo_web.wasm` (compiled from Rust crate `crates/oklo_web`).
- Exported ABI Interface:
  - `oklo_alloc(n: usize) -> *mut f32`: Allocates heap memory for $N^3 = 32,768$ float32 values ($131,072$ bytes).
  - `oklo_free(p: *mut f32, n: usize)`: Deallocates voxel buffer.
  - `oklo_compute(field_id: u32, phase: f64, null_scenario: u32, pulse_boundary: f64, out: *mut f32) -> i32`: Evaluates the requested field at normalized cycle phase $\theta \in [0.0, 1.0]$.
- Supported Fields:
  - Field 0: `temperature` (degrees C index)
  - Field 1: `hydration` (pore saturation fraction)
  - Field 2: `h2_concentration` (radiolytic proxy index)
  - Field 3: `u235_ratio` (atom ratio 235U/238U)
  - Field 4: `polymer_yield` (oligomer surrogate index)
  - Field 5: `compartments` (categorical zone ID 0 to 4)
- Cycle Transport: 60 discrete phase frames spanning a nominal 180-minute hydrothermal pulse cycle.
- Memory Layout: Row-major order with $x$ index fastest:
  $$\text{index}(i, j, k) = k \cdot 1024 + j \cdot 32 + i$$

#### B. Precomputed Darcy Groundwater Flow Integration

- Governing Equations: Isothermal, steady, single-phase Darcy flow:
  $$\mathbf{q} = -K \nabla h, \quad \nabla \cdot \mathbf{q} = 0$$
  where $K = \frac{k_{perm} \rho g}{\mu}$ is hydraulic conductivity in $\text{m}/\text{s}$, and $h$ is hydraulic head in $\text{m}$.
- Verified Run Receipt: `receipts/darcy_v1_A4_run_2026-10-18.json`.
- Configurations:
  1. Base Case: $k_{sandstone} = 1.275 \times 10^{-15}\text{ m}^2$, $k_{core} = 1.275 \times 10^{-15}\text{ m}^2$, $k_{halo} = 7.653 \times 10^{-19}\text{ m}^2$, $k_{conduit} = 1.538 \times 10^{-14}\text{ m}^2$. Top-face conduit flux share = $6.82\%$.
  2. Conduit Ratio Sweeps: $r = 1$ ($4.12\%$ share), $r = 10$ ($6.71\%$ share), $r = 100$ ($7.34\%$ share), $r = 1000$ ($7.41\%$ share).
  3. Sensitivity Cases: `sandstone_core_low`, `sandstone_core_high`, `halo_low`, `halo_high`, `conduit_low`.
- Thermodynamic Water States:
  - State A ($50^\circ\text{C}$, $323.15\text{ K}$, $30\text{ MPa}$): $\rho = 1000.67\text{ kg}/\text{m}^3$, $\mu = 5.528 \times 10^{-4}\text{ Pa}\cdot\text{s}$, total inflow $q_{in} = 1.449 \times 10^{-8}\text{ m}^3/\text{s}$.
  - State B ($300^\circ\text{C}$, $573.15\text{ K}$, $30\text{ MPa}$): $\rho = 750.66\text{ kg}/\text{m}^3$, $\mu = 9.325 \times 10^{-5}\text{ Pa}\cdot\text{s}$, flux factor $4.447\times$, total inflow $q_{in} = 6.444 \times 10^{-8}\text{ m}^3/\text{s}$.
- Verification Indicators and Honest Caveats:
  - Conservation check D1: Passed across all configurations ($\epsilon \le 10^{-10}$).
  - Layer interface check D3a: Recorded honestly as `FAIL` ($q_{rel,cg} = 2.58 \times 10^{-10}$ versus $10^{-10}$ tolerance under A4 CG stopping rule).
  - Péclet screening diagnostic: Evaluated using Robertson 1988 thermal conductivity $\lambda_m$ ($1.13$ to $6.67\text{ W}/(\text{m}\cdot\text{K})$). UI discloses that Robertson $\lambda_m$ was measured at $300\text{ K} / 5\text{ MPa}$ and has unquantified error when applied at $300^\circ\text{C}$.

#### C. Precomputed Uranium Reduction-Transport Column Integration

- Governing Equations: 1D advection-dispersion-reaction column with linear sorption:
  $$R \frac{\partial C}{\partial t} = D_{eff} \frac{\partial^2 C}{\partial x^2} - v \frac{\partial C}{\partial x} - k_{red} \chi_{org}(x) C$$
  $$\rho_b \frac{\partial S}{\partial t} = \theta k_{red} \chi_{org}(x) C$$
  where $R = 1 + \frac{\rho_b K_d}{\theta}$ is the retardation factor, $\theta = 0.15$ is porosity, and $\rho_b = 2252.5\text{ kg}/\text{m}^3$ is bulk density.
- Verified Run Receipt: `receipts/uranium_v1_A1_run_2026-10-18.json`.
- Parameter Sweep Grid: 144 full-factorial runs:
  - Darcy velocity $v \in \{10^{-10}, 10^{-9}, 10^{-8}\}\text{ m}/\text{s}$
  - Effective dispersion $D_{eff} \in \{10^{-11}, 10^{-10}, 10^{-9}\}\text{ m}^2/\text{s}$
  - Reduction rate $k_{red} \in \{0, 10^{-11}, 10^{-10}, 10^{-9}\}\text{ s}^{-1}$
  - Sorption coefficient $K_d \in \{0, 0.063, 6.3, 630\}\text{ m}^3/\text{kg}$ (yielding $R$ from $1.0$ up to $9,460,501$)
- Numerical Discretization: Finite volumes with central differences under cell Péclet constraint $Pe_{cell} \le 2$. Crank-Nicolson time integration with Amendment A1 step bound $dt \le \min(0.5 R \Delta x / v, R / \max |A_{ii}|)$ guaranteeing M-matrix non-negativity.
- Verification Indicators and Honest Caveats:
  - Global mass ledger closure: Passed across all 144 runs ($< 4.72 \times 10^{-12}$).
  - Non-negativity check N1: Passed across all 144 runs ($C_{min} \ge 0.0$, $S_{min} \ge 0.0$).
  - Ogata-Banks benchmark B1, Temporal convergence T1, and Reactive benchmark B2: Passed with spatial/temporal order $2.00$.
  - Extreme retardation caveat: At $K_d = 630\text{ m}^3/\text{kg}$, transit time $5\tau$ exceeds $7.5 \times 10^{18}\text{ seconds}$ ($> 200\text{ billion years}$). UI explicitly notes that this value is an EPA regulatory sensitivity upper bound rather than an observed Oklo parameter.

---

### 3.4 Subsystem 4: Evidence and Provenance Layer

The Evidence and Provenance Layer enforces epistemic classification, cryptographic provenance validation, and citation tracking across all explorer features.

#### A. The Six Machine-Readable Evidence Classes

Every data object, visual field, model panel, and timeline interval rendered by the system belongs to exactly one of the six mutually exclusive evidence classes:

1. **`geographic_observation_or_locator`**:
   - Empirical observations of present-day Earth geography (e.g. WGS84 coordinates, digital elevation models, satellite imagery).
   - Applied to: Modern Oklo site marker (Gabon, $1.4000^\circ\text{S}, 13.1833^\circ\text{E}$), comparative geothermal system locators.
   - Constraint: Must not be represented as a Paleoproterozoic paleogeography map.

2. **`sourced_geological_record`**:
   - Empirical measurements published in peer-reviewed scientific literature derived from rock, mineral, or isotopic assays.
   - Applied to: Measured natural $^{235}\text{U}/^{238}\text{U}$ depletion ratios ($0.00725$ natural baseline down to $\approx 0.004$ in core; Bodu et al. 1972), xenon isotopic retention cycles (Meshik et al. 2004), Francevillian stratigraphy (Gauthier-Lafaye et al. 1989).
   - Constraint: Sourced records describe historical nature; they do not elevate numerical models that approximate them.

3. **`prescribed_analytic_field`**:
   - Closed-form algebraic formulas or procedural geometric indicators constructed for educational scenario exploration.
   - Applied to: All six revision-0 fields (`temperature`, `hydration`, `h2_concentration`, `u235_ratio`, `polymer_yield`, `compartments`).
   - Constraint: Must carry persistent on-render disclosure: `"Revision-0 prescribed fields; not a physical simulation."`

4. **`numerically_verified_synthetic_model`**:
   - Numerical solutions of mathematical differential equations whose stability, convergence, and conservation laws are proven by automated tests, but which are not calibrated to real-world field data.
   - Applied to: 3D Darcy groundwater flow (`darcy_flow_v1_a4`), 1D uranium reduction-transport column (`uranium_redox_column_v1_a1`), `rna_biofilm_toy` 5-term mass ledger (`rna_biofilm_toy_v1`), `Biofilms` Cellular Potts solver (`biofilms_potts_cpm_v0`), `SobolevMacromolecule` $H^1$ coordinate relaxation (`sobolev_rna_c1_norm_v0`).
   - Constraint: Numerical correctness is not physical reality. Must never be termed an Oklo subsurface reconstruction.

5. **`hypothesis_or_unvalidated_model`**:
   - Conceptual frameworks, speculative reaction mechanisms, or unvalidated biophysical conjectures that lack experimental proof.
   - Applied to: Prebiotic wet-dry geyser polymerization hypotheses (Ebisuzaki and Maruyama 2017; Adam et al. 2018), radiation-gradient quasispecies selection (`rna_biofilm_toy` Spec 14).
   - Constraint: Must be clearly labeled as theoretical conjectures.

6. **`empirically_calibrated_model`**:
   - Models whose constitutive parameters and predictions have been calibrated against real-world measurements with formal residual and uncertainty audits.
   - Applied to: **NONE.** This class is strictly reserved. There are zero empirically calibrated models in this repository or the referenced research repositories.
   - Constraint: Prohibited from application to any current or prototype model.

#### B. Machine-Readable Metadata Schema

All models must validate against the formal schema `data/models/schema.json` and be indexed in `data/models/manifest.json`. Every record contains:
- `model_id` (string, unique)
- `model_version` (string, semantic version)
- `evidence_class` (enum string, one of the 6 allowed classes)
- `source_repository` (canonical URL)
- `source_commit` (hexadecimal git SHA)
- `specification_version` (governing specification document)
- `run_receipt` (string path/hash or null)
- `parameter_set_id` (string or null)
- `units` (explicit physical or dimensionless units)
- `coordinate_system` (explicit coordinate frame)
- `uncertainty` (explicit error description or null)
- `known_limitations` (detailed disclosure of boundaries and caveats)

#### C. Cryptographic Audit Validation Rules

1. Before rendering precomputed transport datasets, the client or verification test suite validates the SHA-256 digest of the data file against the frozen receipt digest.
2. If the computed hash diverges from the receipt record, the dataset is marked as modified or unverified and is barred from production display.

---

### 3.5 Subsystem 5: Exploration Interface

The Exploration Interface coordinates stage navigation, camera movements, HUD telemetry readouts, timeline scrubbing, and epistemic disclosure badges.

#### A. Stage-by-Stage Navigation Architecture

1. **Stage 1: Planetary Globe and Deep-Time Timeline**:
   - Interactive 3D Earth sphere with geographic markers.
   - Primary Marker: Oklo Natural Reactor Zone, Gabon ($1^\circ 24' 00''\text{S}, 13^\circ 11' 00''\text{E}$; Gauthier-Lafaye et al. 1996).
   - Comparative Analogs (strictly sourced):
     - Dallol Hydrothermal System, Ethiopia ($14^\circ 14' 30''\text{N}, 40^\circ 18' 00''\text{E}$; Cavalazzi et al. 2019; extreme brine prebiotic analog).
     - Yellowstone Norris Geyser Basin, USA ($44^\circ 43' 42''\text{N}, 110^\circ 42' 15''\text{W}$; Damer and Deamer 2015; terrestrial wet-dry cycling analog).
     - Lost City Hydrothermal Field, Mid-Atlantic Ridge ($30^\circ 07' 00''\text{N}, 42^\circ 07' 00''\text{W}$; Kelley et al. 2001; serpentinization alkaline hydrothermal analog).
   - Deep-Time Timeline Scrubber: Ingests `data/timeline/paleo_intervals_v1.json` directly without merging distinct literature definitions:
     - Gumsley et al. 2017 GOE: $2.460$ to $2.060\text{ Ga}$ (onset $2.460-2.426\text{ Ga}$, oscillating oxygen until $\ge 2.24\text{ Ga}$, Lomagundi carbon-isotope excursion termination at $2.060\text{ Ga}$).
     - Craig et al. 2023 GOE: $2.43$ to $2.22\text{ Ga}$ (attributed to Poulton et al. 2021).
     - Oklo Fission Event: $1.95 \pm 0.04\text{ Ga}$ dated marker with error whiskers.
     - Craig et al. 2023 Eukaryogenesis: Core interval $2.04$ to $1.79\text{ Ga}$; conservative range $2.2$ to $1.5\text{ Ga}$.
   - UI Invariant: The basemap declares: `"Modern geographic locator; NOT a Paleoproterozoic paleogeography reconstruction."`

2. **Stage 2: Geological Cutaway**:
   - Initiated via camera dive upon clicking the Oklo site pin.
   - Displays 3D schematic compartment block ($16\text{ m} \times 16\text{ m} \times 16\text{ m}$) with conceptual layer labels (Sandstone, Core, Halo, Conduit).
   - Prominent HUD Warning Banner: `"SCHEMATIC compartment geometry; NOT surveyed subsurface geology."`
   - Strict numerical constraint: Displays zero fake velocities, uncalibrated permeabilities, or synthetic temperatures.

3. **Stage 3: Revision-0 Field Inspection**:
   - Integrates the six prescribed fields with live WebGL2 volume and 2D canvas orthogonal slice controls.
   - Voxel Inspector HUD: Hovering over slice canvases displays exact voxel coordinates $(i, j, k)$, physical metric location $(x, y, z)$, and raw scalar value.
   - Persistent Boundary Banner: `"Revision-0 prescribed fields; not a physical simulation."`
   - Adjacent Field Limitation Callout: Updates dynamically when selecting fields.

4. **Stage 4: Transport Model Inspection**:
   - Model A (Darcy Groundwater Flow):
     - Displays 3D hydraulic head contours and conduit flux fractions.
     - Allows toggling between State A ($50^\circ\text{C}$) and State B ($300^\circ\text{C}$).
     - Honest D3a Failure Badge: Notes that the D3a layer interface check reached $2.58 \times 10^{-10}$ relative error versus $1.0 \times 10^{-10}$ tolerance.
     - Caveat: `"HYPOTHETICAL numerical experiment; Darcy v1+A1-A3. Not a description of Oklo hydrogeology."`
   - Model B (Uranium Reduction-Transport Column):
     - Displays interactive breakthrough curves $C(L_{col}, t)$ and 1D spatial profiles $C(x), S(x)$ across the 6 output $\tau$ times.
     - Parameter Selectors: Darcy velocity $v$, dispersion $D_{eff}$, reduction rate $k_{red}$, and sorption $K_d$.
     - Ledger Indicator: Displays machine-precision mass conservation closure ($< 4.72 \times 10^{-12}$).
     - Caveat: `"SYNTHETIC 1D column; uranium prereg v1+A1. Not a reconstruction of Oklo or the Franceville Basin."`

#### B. Epistemic Badges and UI Layout Controls

- Visual Badges: Every active viewport renders a colored epistemic badge:
  - `[GEOGRAPHIC OBSERVATION]` (Emerald Green `#059669`)
  - `[SOURCED GEOLOGICAL RECORD]` (Sapphire Blue `#2563eb`)
  - `[PRESCRIBED ANALYTIC FIELD]` (Amber `#d97706`)
  - `[NUMERICALLY VERIFIED SYNTHETIC MODEL]` (Indigo `#4f46e5`)
  - `[HYPOTHESIS / UNVALIDATED MODEL]` (Purple `#9333ea`)
  - `[EMPIRICALLY CALIBRATED MODEL]` (Reserved Crimson `#dc2626`)
- Tapping or clicking the badge opens an Epistemic Modal displaying the complete JSON metadata record from `data/models/manifest.json`.
- Responsive Layout: Dual-column instrument console on desktop viewports ($\ge 1024\text{ px}$); stacked collapsible single-column console on mobile viewports.
- Typography Invariant: Strictly zero em-dashes and zero en-dashes across all rendered labels, HUD text, and documentation.

---

## 4. Five-Tier Scientific Roadmap

The multi-tier scientific roadmap outlines the progression of scientific models, establishing clear boundaries between active implementations, methodological references, and deferred future solvers:

```
+----------------------------------------------------------------------------------------------------+
|                                    FIVE-TIER SCIENTIFIC ROADMAP                                    |
+-------------------+-------------------------------------------+------------------------------------+
| Tier 1            | Geochemical and Geophysical Foundation    | ACTIVE (Oklo v1 & v2 prototype)    |
|                   | (Radiation, thermal, hydrologic gradients)| (Prescribed fields & Darcy flow)   |
+-------------------+-------------------------------------------+------------------------------------+
| Tier 2            | Prebiotic Synthesis & Organic Geochemistry| DEFERRED                           |
|                   | (Formose, HCN, phosphorylation kinetics)  | (Requires validated reaction ODEs) |
+-------------------+-------------------------------------------+------------------------------------+
| Tier 3            | RNA-World Biophysics and Polymers         | DEFERRED                           |
|                   | (Template copying, ribozyme kinetics)     | (Requires mechanistic RNA model)   |
+-------------------+-------------------------------------------+------------------------------------+
| Tier 4            | Compartments and Selective Permeability   | DEFERRED                           |
|                   | (Fatty acid vesicles, division, transport)| (Requires calibrated physics CPM)  |
+-------------------+-------------------------------------------+------------------------------------+
| Tier 5            | Endosymbiotic Integration & Transitions   | DEFERRED                           |
|                   | (Metabolic coupling, organelle emergence) | (Long-term theoretical research)   |
+-------------------+-------------------------------------------+------------------------------------+
| Cross-Cutting     | Molecular Visualization Provider          | RESTRICTED PRECOMPUTED             |
| Asset Provider    | (SobolevMacromolecule / Stanford-RNA)     | (Static C1-prime coordinate tables)|
+-------------------+-------------------------------------------+------------------------------------+
```

### 4.1 Tier Specification Matrix

| Tier | Tier Name | Repository / Scope | Current Implementation Status | Governing Model Class | State Variables | Evidence Basis | Validation Needs | Promotion Gate to Advance Tier |
|---|---|---|---|---|---|---|---|---|
| **Tier 1** | Geochemical & Geophysical Foundation | `oklo_pool_mobile` | Active. Prescribed fields frozen; Darcy flow and Uranium transport verified. | Prescribed analytic fields; steady Darcy flow; 1D advection-dispersion-reaction. | $T, S_w, \text{H}_2\text{ index}, \text{U ratio}, \text{polymer index}, \text{zone}; h, \mathbf{q}; C(x,t), S(x,t)$. | Xenon retention (Meshik 2004); Zone 2 deposit mapping (Gauthier-Lafaye 1996); EPA Kd bounds. | In-situ hydraulic tests; reactive mineral precipitation kinetics; 3D fracture flow. | Strict empirical verification of coupled thermal-hydraulic-chemical state variables; site-calibrated permeabilities. |
| **Tier 2** | Prebiotic Synthesis & Organic Geochemistry | Future modular chemistry modules | Deferred roadmap concept. Not implemented. | Mass-action chemical reaction networks; stiff ODE kinetic solvers. | Precursor concentrations (HCN, formose intermediates), radical yields, pH, redox potential. | Laboratory synthesis literature (e.g. Sutherland, Patel, Powner); radiation chemistry G-values. | Mass conservation; thermodynamic free energy consistency; reaction rate calibration. | Demonstrable synthesis of RNA/peptide monomers under validated Tier 1 geochemical conditions. |
| **Tier 3** | RNA-World Biophysics & Polymers | `aurascoper/rna_biofilm_toy` (reference only) | Deferred. Synthetic 2D spatial toy with 5-term mass ledger. | Stochastic lattice population models; discrete template copying. | Replicator population density, purine ratio, template length, mutation error rate. | Theoretical quasispecies and spatial protection models (Szathmary, Takeuchi). | Stochastic convergence; experimental replication fidelities. | Integration with biophysical compartment boundaries; empirical error-threshold verification. |
| **Tier 4** | Protocell Compartmentalization | `aurascoper/Biofilms` (reference only) | Deferred. 3D Cellular Potts Model with nutrient diffusion. Coupled path blocked. | Glazier-Graner-Hogeweg (CPM) Hamiltonian mechanics; PDE diffusion. | Parcel spin labels, membrane surface area, volume, contact energy matrix, nutrient field. | Experimental fatty acid vesicle biophysics (e.g. Szostak, Deamer). | Mass-volume conservation; calibration of interfacial adhesion energies; SI dimensional consistency. | Stable protocell growth and division coupling with internal Tier 3 replicator metabolism. |
| **Tier 5** | Endosymbiotic Integration | Future evolutionary models | Deferred roadmap concept. Not implemented. | Multi-level evolutionary population dynamics; co-evolutionary game theory. | Host-symbiont gene transfer fractions, ATP/electron transfer fluxes, metabolic fitness. | Phylogenomic molecular-clock analyses (Craig 2023; Williams 2013). | Evolutionary stability proofs; genome reduction benchmarks. | Mechanistic demonstration of stable metabolic dependency and host-controlled organelle division. |

---

### 4.2 Role of `aurascoper/SobolevMacromolecule`

`aurascoper/SobolevMacromolecule` serves as a potential molecular structure and large-scene visualization provider:
- **Architectural Boundary**: Strictly separates solver-owned simulation state from renderer-owned camera, instancing, lighting, and frame delivery.
- **Client Constraints**: Mobile apps and browser runtimes must not embed PyTorch, JAX, CUDA, neural network model weights, or remote inference APIs.
- **Interface Contract (`RNA-C1-NORM v0`)**:
  - Consumes static normalized coordinate tables exported offline.
  - One record per residue at C1-prime coordinate resolution.
  - Coordinate units in Angstroms; right-handed Cartesian frame.
  - Renderer uses precomputed atom coordinate tables to instance visual ribbon and sphere representations without executing live structural relaxation algorithms.

---

## 5. Research Repositories Integration Registry (Requirement R6)

The integration registry defines the authoritative ground truth, commit-pinned references, licenses, failure modes, and boundaries for external research repositories:

| Repository Key | Canonical Git Repository | Pinned Commit SHA | Target Branch / Tree | License | Integration Status |
|---|---|---|---|---|---|
| `repo_biofilms_cpm` | `https://github.com/aurascoper/Biofilms` | `74f433933f594ce749fd700b10b1041776f1a0e9` | `private/geolocator-runtime` (manuscript: `71ea8d4`) | CC0 1.0 Universal | **BLOCKED** on `RADIODIALYSIS: BLOCKED`. Read-only architectural reference only. |
| `repo_rna_biofilm_toy` | `https://github.com/aurascoper/rna_biofilm_toy` | `b780a1a7f6113f3b8454cad5efbc7dd66b76a25a` | `guards/kit-v1` (main: `9d9471e`) | CC BY 4.0 (data/docs), MIT (code) | **DEFERRED** as biological model. Methodological reference for 5-term ledger tests only. |
| `repo_sobolev_rna` | `https://github.com/aurascoper/Stanford-RNA-3D-Folding-Part-2-1st-place` | `c5947bcb98b85ae5c89830c30e907cd52104d0c1` | `codex/sobolev-polish-gate` | MIT License | **RESTRICTED**. Static precomputed `RNA-C1-NORM v0` coordinate tables only. Zero runtime execution. |

### 5.1 Repository Audit Details

#### 1. `aurascoper/Biofilms`
- Paradigm: 3D Cylindrical lattice Cellular Potts Model ($N = 40$ or $N = 60$) with Metropolis Monte Carlo transitions governed by a 4-term acceptance Hamiltonian:
  $$\Delta H_{CPM} = \Delta H_{adhesion} + \Delta H_{volume} + \Delta H_{radiation} + \Delta H_{melanin}$$
- Field Coupling: Melanin production is coupled to cell occupancy and radiation; radiation is a static cylindrical Beer-Lambert decay initialized once at step 0; proposal motility is isotropic (motility bias parameter $D_s$ is unreferenced in the acceptance kernel).
- Root Cause Analysis of Blocker (`RADIODIALYSIS: BLOCKED`):
  Executing coupled simulation aborts at Monte Carlo Step 1 (MCS 1) inside `step_radiolysis!` due to severe dimensional collapse:
  1. Cylinder radius $R = N/2$ (20 lattice sites) is fed directly into a physical operator expecting $\text{cm}^2/\text{s}$.
  2. Dimensionless site-occupancy fractions $X_{total}$ and $X_{red}$ are placed into $\text{g}/\text{cm}^3$ slots and multiplied by kinetic rate constants in $\text{cm}^3/(\text{g}\cdot\text{s})$.
  3. Taxonomic defect: $X_{red}$ divides one species sites by all interior lattice sites, making it neither a biomass fraction nor a reducer fraction.
  4. Radial biomass profile is collapsed by an unweighted arithmetic `mean()`, smearing the spatial uptake sink uniformly across radius.
  5. Decoupled membrane state: membrane integrity $m$ decays while effective permeability $P_{eff}$ grows without either reading the state of the other.
- Boundary Decision: **BLOCKED.** Do not execute or couple Biofilms in Oklo v2. Use only its claims-ledger conventions and negative-control test patterns.

#### 2. `aurascoper/rna_biofilm_toy`
- Paradigm: 2D cylindrical lattice sandbox ($50 \times 50$, $x$ periodic, $y$ non-periodic) modeling spatial shielding against a linear radiation gradient.
- The 4 Repaired Defects:
  1. Lysis unreachable (damage applied to deep-copied snapshot while live grid remained untouched; repaired to decrement live grid directly).
  2. Gradient misdirection (replication weight multiplied $\Delta x$ by flux difference for a field varying purely in $y$; repaired to use true direction cosine $\hat{\mathbf{d}} \cdot \hat{\nabla} F$).
  3. Unaccounted monomer leak (hard clamp silently discarded monomers; repaired with exact closed 5-term mass ledger).
  4. Spurious seam teleportation ($\Delta x$ boundary hops unwrapped properly to $\{-1, 0, 1\}$ instead of jumping 49 sites with weight $e^{20}$).
- Mass Ledger Accounting: Exact closed 5-term mass balance holds to machine precision ($\approx 10^{-12}$):
  $$\Delta M(t) = \text{injected\_gross} - \text{inject\_clip} - \text{diffusion\_clip} - \text{birth\_sink} + \text{death\_return}$$
- Falsification of Spec 14 Quasispecies Selection:
  Spec 14 explored whether radiation gradients create stable selection bands in protocells. The core hypothesis was falsified under realistic copying fidelities (empirical non-enzymatic error $\approx 7.6\%$ per base collapses habitable bands; baseline error rate $\epsilon_0 = 0.00574$ sits below literature bounds; equating single-strand breaks to replication errors is physically unvalidated).
- Boundary Decision: **DEFERRED.** Contains zero RNA chemistry and zero prebiotic thermodynamics. Use strictly as a methodological template for unit-cell mass accounting and falsification registers.

#### 3. `aurascoper/SobolevMacromolecule`
- Working Tree Audit: Local path `/Users/aurascoper/Developer/SobolevMacromolecule` is a hollow scaffold. Runnable implementation exists in sister repository `/Users/aurascoper/Developer/Stanford-RNA-3D-Folding-Part-2-1st-place` at commit `c5947bc`.
- Formulation: Refines candidate RNA 3D backbone structures by smoothing gradient descent under an $H^1$ Sobolev metric on C1-prime coordinates via DCT-II frequency damping:
  $$\hat{g}_{smooth}(k) = \frac{\text{DCT}(g)}{1 + \alpha k^2}, \quad \alpha = 5.0$$
- Energy Functional: Total energy balances bond restraints ($d_0 = 5.95\text{ \AA}$), steric clash penalties ($< 3.0\text{ \AA}$), pairwise contact restraints, and radius of gyration compactness.
- Caveats: The reported TM-score of 0.50934 is repository-reported only and has not been verified by external benchmarks.
- Boundary Decision: **RESTRICTED.** Consume precomputed static coordinate tables adhering to `RNA-C1-NORM v0`. Prohibit JAX/Python dependencies and live solver execution in client applications.

### 5.2 Exploratory Research Work Package: Surface-Associated Fungal Matrix (Requirement R6A)

- **Specification Reference:** `docs/specs/SURFACE_MATRIX_PROTECTION_v0.md` (`surface_matrix_protection_v0`)
- **Status:** Proposed; research-only; no production-model changes authorized.
- **Disposition:** Research candidate only. Passing this work package does not promote it to a calibrated model or authorize production coupling.
- **Architectural Placement:** Cross-cutting exploratory research work package under Milestone B (R6 to R8), **not** a sixth scientific tier alongside the five planetary-exploration tiers and **not** a change to the current `aurascoper/Biofilms` solver.
- **Objective and Evidence Boundary:** Uses the photographed wood-growing mushroom on *Morus* in Lima, Peru (`obs_lima_morus_mushroom_2026_10_20`, SHA-256 `b7741a3560ab4fc1a93cf212df06ca436e0d4a49bf354a68e284ac26bd58086c`) and its sharply demarcated black surface growth to exercise the platform's hypothesis-to-model evidence architecture without promoting a visual observation into a demonstrated mechanism:
  1. **Evidence and Identification (R6A.A):** Preserves original photograph provenance and records unknown metadata (`exact_coordinates`, `observation_date`, `voucher_specimen_id`, `additional_views`) explicitly as `unknown`. Registers the *Pleurotus cystidiosus* complex / *Antromycopsis macrocarpa* identification as an unconfirmed hypothesis alongside competing explanations (secondary dematiaceous mold/mycoparasite, overhanging spore deposit, senescent necrosis) and required microscopy ($3\%$ KOH arthroconidial biometrics at $400\times\text{ to }1000\times$) and ITS barcoding.
  2. **Independent Model Specification (`surface_matrix_protection_v0`, R6A.B):** Specifies a generic surface-associated matrix field $M_{\mathrm{surf}}(\mathbf{x}, t)$ (never labeled as melanin without material characterization) and transport of one explicitly named reactive species $C_R(\mathbf{x}, t)$ with a hypothesized sink $k_M M_{\mathrm{surf}} C_R$ initially disabled ($k_M = 0$) or tagged `synthetic_uncalibrated`. Refuses to treat radiation dose rate as a universal ROS source without an explicit radiation-chemistry and unit contract.
  3. **Preregistered Verification and Controls (R6A.C):** Requires seven controls (C1 Matrix-free baseline, C2 Source-free limiting case, C3 Matrix production and loss accounting, C4 Reaction-transport conservation and convergence, C5 Sink-strength monotonicity under linear assumptions, C6 Surface-localized vs. spatially uniform matrix at equal total inventory, and C7 Rejection of invalid units, unsupported provenance, and incompatible coordinates).
  4. **Evidence Gate for Mechanistic Interpretation (R6A.D & R6A.E):** Blocks biological interpretation until taxonomic voucher/ITS confirmation, chemical matrix characterization, single-species lifetime/penetration measurement, matrix-free/inactivated controls, and parameter identification are completed. Keeps `Biofilms`, `rna_biofilm_toy`, `SobolevMacromolecule`, and Oklo transport models strictly isolated and unmodified.

---

## 6. Epistemic Separation Contract and Invariants (Requirement R7)

The Planetary Explorer platform enforces four non-negotiable architectural invariants:

### Invariant 1: No Promotion by Proximity (Anti-Contagion Rule)
Rendering a synthetic model or prescribed field within the same 3D viewport, coordinate frame, or visual scene as real geographic or geological data **DOES NOT** elevate its evidence class.
- Example: Visualizing the prescribed `polymer_yield` field inside a 3D terrain cutaway of Gabon does not make polymer accumulation an empirical finding. The field remains `prescribed_analytic_field`.

### Invariant 2: No Spatial-Chemical Causal Conflation
The spatial coincidence or proximity of two physical phenomena (e.g. radioactive fission products co-occurring in water-saturated sandstone) must never be asserted or implied to establish chemical causality or prebiotic synthesis.
- Enforcement: User interfaces and technical documentation must explicitly distinguish spatial coexistence from demonstrated chemical reaction networks.

### Invariant 3: Prohibition of False Abiogenesis and Endosymbiosis Claims
No model, visualization, or label in this platform may be described as a "simulation of the origin of life", "abiogenesis engine", "RNA-world evolution", "protocell emergence", or "endosymbiotic transition".
- Enforcement: Any label suggesting that current models prove or simulate these evolutionary transitions is a direct integrity violation that triggers an immediate gate veto.

### Invariant 4: Fail-Closed Unit and Coordinate Discipline
Missing physical parameters must remain explicitly `null`, `unknown`, or `unquantified`. They must never be silently zero-filled, interpolated, or coerced into physical units. Cross-coordinate operations without explicit adapters fail closed with an unhandled conversion error.

---

## 7. Verification and Acceptance Criteria (Requirement R8)

Prototype delivery on branch `planetary-explorer/v2-prototype` requires passing an automated verification suite:

1. **Coordinate Transformation Tests**:
   - Round-trip geographic to unit direction vector error $< 10^{-6}$.
   - Out-of-bounds latitude or longitude values trigger explicit exceptions.
2. **Timeline Parser Tests**:
   - Schema validation of `data/timeline/paleo_intervals_v1.json`.
   - Asserts non-merging: Every interval retains independent bibliographic provenance and stated uncertainty.
3. **Camera State Machine Tests**:
   - Verifies state transitions across Globe, Cutaway, Field Inspector, and Transport Models.
   - Validates that dive animation parameters remain finite and non-NaN.
4. **Field Slicing and Pixel Variance Tests**:
   - Asserts voxel array size is exactly $32 \times 32 \times 32 \times 4 = 131,072\text{ bytes}$.
   - Asserts non-zero pixel variance across both Plan and Section slices for all 6 fields.
   - Verifies that field switching alters raw float buffer values and not merely the colormap palette.
5. **Headless Browser Smoke Tests and Captured Screenshots**:
   - Headless Chromium verification capturing high-resolution PNG screenshots:
     - `receipts/screenshots/v2_stage1_globe.png`
     - `receipts/screenshots/v2_stage2_cutaway.png`
     - `receipts/screenshots/v2_stage3_field_temperature.png`
     - `receipts/screenshots/v2_stage3_field_compartments.png`
     - `receipts/screenshots/v2_stage4_darcy_inspection.png`
     - `receipts/screenshots/v2_stage4_uranium_inspection.png`
6. **WebGL2 Fallback Degradation Tests**:
   - Simulates WebGL2 context creation failure; verifies graceful fallback to 2D canvas slices with informative banner and zero uncaught exceptions.
7. **Performance Budgets**:
   - Cold page load to first useful canvas frame $< 250\text{ ms}$.
   - Stage transition animation latency $< 500\text{ ms}$.
   - Voxel raymarching frame latency $< 100\text{ ms}$.
   - Field switching latency $< 50\text{ ms}$.
8. **Typography Audit**:
   - Strictly zero em-dashes (U+2014) and zero en-dashes (U+2013) across all files.

---

## 8. Explicitly Out of Scope (Requirement R9)

The following items are forbidden from implementation in Milestone B:
- Replacing the revision-0 polymer accumulation index with a kinetic polymerization solver.
- Coupling Darcy fluid velocities into thermal advection or species transport solvers.
- Live, client-side PDE or numerical differential equation execution in the browser.
- Biological protocell, RNA replication, or endosymbiotic transition solver implementation.
- Ingestion of live external network APIs, cloud rendering pipelines, or remote molecular inference services.
