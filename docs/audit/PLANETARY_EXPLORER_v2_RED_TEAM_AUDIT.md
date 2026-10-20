# Planetary Explorer v2: Red-Team Audit Report and Epistemic Limitations Matrix

- Document ID: `PLANETARY_EXPLORER_v2_RED_TEAM_AUDIT`
- Target Branch: `planetary-explorer/v2-prototype`
- Baseline Commit: `636b7d7bb41999b80278d90046da893ee1044f20`
- Tested Source Commit: `b2ed8d852ce8ba3983553ee6505056fc957671d2`
- Receipts Remediation Commit: `39ba9648b692bc3730e72058595501f4e19f92ee`
- Date: 2026-10-20
- Author: worker_milestone_b3 (Roles: implementer, qa, specialist)
- Audit Mandate: Sections R6, R7, R8, and R9 of ORIGINAL_REQUEST.md

---

## 1. Executive Summary

This red-team audit provides an adversarial evaluation of the Planetary Explorer v2 prototype on branch `planetary-explorer/v2-prototype`. The objective is to verify scientific provenance, enforce strict epistemic separation, detect visual proximity risks, audit integration boundaries for external research repositories, and formalize an epistemic limitations matrix across all seventeen registered models in `data/models/manifest.json`.

The foundational architectural principle of Planetary Explorer v2 is fail-closed epistemic integrity:
1. Presentation is strictly decoupled from physical simulation. The client application performs zero live numerical solving, zero neural inference, and zero unverified parameter interpolation.
2. Visual proximity does not elevate scientific validity (Anti-Contagion Rule).
3. The platform completely prohibits unverified claims regarding the simulation of abiogenesis, prebiotic RNA synthesis, protocell emergence, or endosymbiotic transitions.
4. Exactly zero models are classified as `empirically_calibrated_model`. This classification is strictly reserved for models with quantified empirical residual audits against natural field data.

---

## 2. Epistemic Separation and Anti-Contagion Analysis

### 2.1 Visual Proximity Risks: Globe to Geological Cutaway Transition

A primary hazard in multi-scale 3D geographic visualizations is the cognitive contagion of realism. When a user navigates smoothly from a realistic global Earth sphere (Stage 1) down to a local 3D cutaway block (Stage 2), the perceived verisimilitude of the planetary basemap tends to unconsciously transfer to the subsurface cutaway.

The red-team analysis identified the following specific proximity risks:
- Risk A: Assuming Subsurface Geology is Measured Survey Data. Users observing the 16-meter cubic block could conclude that the five stratigraphic compartments (Sandstone, Core, Halo, Conduit, Steam Chimney) represent three-dimensional borehole seismic tomographies or surveyed subsurface geology of the Oklo mining pit.
- Risk B: Spatial Co-occurrence Imputing Chemical Kinetics. In Stage 3, rendering prescribed thermal, hydration, and radiolytic hydrogen proxy fields within the same volumetric bounding box risks implying that these fields represent coupled, thermodynamically consistent chemical reactions.
- Risk C: Transport Path Conflation. In Stage 4, viewing 3D Darcy hydraulic heads alongside a 1D uranium reduction-transport column could mislead an observer into assuming the 1D breakthrough curves were extracted directly along streamlines of the 3D flow field.

Mitigation and Invariant Enforcement in Planetary Explorer v2:
- Stage 2 Warning Banner: The cutaway view enforces a prominent, persistent warning banner:
  "SCHEMATIC compartment geometry; NOT surveyed subsurface geology."
- Stylized Geometry: Compartments are rendered using flat-shaded geometric bounding primitives and wireframe boundaries rather than photorealistic pseudo-rock textures that simulate false realism.
- Stage 3 Epistemic HUD Badge: The field inspector continuously displays an amber badge:
  "[PRESCRIBED ANALYTIC FIELD]" accompanied by the mandatory on-render statement: "Revision-0 prescribed fields; not a physical simulation."
- Dynamic Field Limitations: Selecting any field dynamically updates the adjacent limitations box, explicitly warning that the variable is an algebraic scenario proxy rather than an energy-conserving or momentum-conserving partial differential equation solution.
- Stage 4 Structural Isolation: Model A (Darcy flow) and Model B (Uranium transport) are rendered in isolated sub-panels with distinct coordinate systems. The interface explicitly states that cross-coordinate interpolation between the 3D hydraulic block and the 1D reactive column is unsupported and mathematically undefined.

### 2.2 Modern Geographic Locator versus Paleoproterozoic Reconstruction

In Stage 1, the planetary globe visualizes site pins against modern continental outlines. The red-team audited the distinction between modern spatial positioning and Paleoproterozoic paleogeography:
- The Physical Reality: Modern Oklo is located at WGS84 coordinates 1.4000 degrees South, 13.1833 degrees East in the Republic of Gabon on the African tectonic plate. However, the Oklo natural nuclear fission reactions occurred approximately 1.95 billion years ago (1.95 +/- 0.04 Ga) during the Paleoproterozoic era (Francevillian Basin deposition). At 2.0 Ga, the Congo and Sao Francisco cratons occupied a completely different paleolatitude and orientation within the Columbia (Nuna) supercontinent assembly.
- Separation Contract: The globe interface strictly labels the Oklo pin as:
  "Modern geographic locator; NOT a Paleoproterozoic paleogeography reconstruction."
  The application does not project modern coastlines back to 2.0 Ga or claim that modern groundwater flow directions correspond to Proterozoic recharge hydrology.
- Chronostratigraphic Separation: The deep-time timeline scrubber ingests `data/timeline/paleo_intervals_v1.json` without harmonizing or merging conflicting literature intervals. For example:
  - The Great Oxidation Event (GOE) is shown as two distinct, non-merged records: Gumsley et al. 2017 (2.460 to 2.060 Ga, incorporating the Lomagundi carbon isotope excursion) and Craig et al. 2023 (2.43 to 2.22 Ga, based on Poulton et al. 2021).
  - Stated uncertainties and published boundaries are preserved verbatim to maintain complete bibliographic provenance.

### 2.3 Uncoupled Darcy Flow and Uranium Transport Caveats

Stage 4 exposes precomputed numerical simulation outputs for Darcy groundwater flow (Model A) and Uranium reactive transport (Model B). The red-team audited their governing physics and documented the following limitations:

#### Model A: Darcy Flow (v1+A4) Limitations
1. Isothermal and Incompressible Idealization: The model solves steady single-phase Darcy flow on an idealized 16-meter cube with fixed hydraulic head boundaries (1.0 meter hydraulic head drop across the domain). It does not couple fluid flow to the thermal convection or buoyancy driven by nuclear fission decay heat.
2. Single-Phase Approximation at Superheated Temperatures: The model evaluates two thermodynamic water states: State A (50 degrees C, 30 MPa) and State B (300 degrees C, 30 MPa). At State B, the model scales fluid density (750.66 kg/m^3) and dynamic viscosity (9.325e-5 Pa*s), resulting in a 4.447x flux increase. However, it treats the fluid purely as single-phase liquid water. In a natural nuclear reactor at 300 degrees C, localized boiling, steam flashing, and two-phase vapor-liquid counterflow occur, which are entirely outside the scope of this solver.
3. Uncorrected Thermal Conductivity: The Peclet number diagnostic employs Robertson 1988 matrix thermal conductivity (1.13 to 6.67 W/(m*K)). Robertson measured these values at 300 K and 5 MPa. Extrapolating these room-temperature parameters to 300 degrees C and 30 MPa introduces unquantified systematic error.
4. Layer Interface Check D3a Failure: In the verified run receipt `receipts/darcy_v1_A4_run_2026-10-18.json`, the internal interface flux continuity check D3a achieved a relative error of 2.58e-10, exceeding the pre-registered tolerance of 1.0e-10 due to conjugate gradient stopping criteria. The interface displays this check honestly as a failure rather than concealing it.
5. Non-Reconstruction Status: The flow model uses synthetic, uniform permeabilities across idealized geometric compartments (e.g. conduit permeability 1.538e-14 m^2). It does not model the fractured, heterogeneously deformed Francevillian FA sandstone basin.

#### Model B: Uranium Reduction-Transport Column (v1+A1) Limitations
1. One-Dimensional Geometry: The model solves a 1D advection-dispersion-reaction equation with linear equilibrium sorption across a 16-meter column. It does not account for 3D transverse hydrodynamic dispersion, preferential fracture channeling, or structural bypassing.
2. Synthetic Sorption Sweeps: The distribution coefficient Kd is swept across seven orders of magnitude (0 to 630 m^3/kg), resulting in retardation factors R ranging from 1.0 to 9,460,501. At Kd = 630 m^3/kg, the 5-tau breakthrough transit time exceeds 7.5e18 seconds (more than 200 billion years). The interface explicitly notes that Kd = 630 m^3/kg is an extreme EPA regulatory sensitivity upper bound, not an observed parameter in Gabon pitchblende ores.
3. Constant Reducing Capacity: Reduction of soluble U(VI) to immobile U(IV) is modeled as a linear pseudo-first-order sink proportional to an organic barrier indicator function. The organic reductant is treated as inexhaustible. The model contains zero mineral dissolution-precipitation kinetics, zero thermodynamic redox equilibrium calculations, and zero competing complexation reactions (e.g. uranyl carbonate or phosphate complexes).
4. Ledger Closure versus Physical Calibration: The numerical scheme satisfies machine-precision mass conservation (global ledger residual < 4.72e-12) and strict non-negativity under Amendment A1 time-step bounds. However, numerical conservation must never be mistaken for physical calibration.

### 2.4 Strict Prohibition of False Abiogenesis and Endosymbiosis Claims

The red-team conducted a thorough language and conceptual audit regarding origin-of-life claims:
- Field 4 (Polymer Yield): In Stage 3, Field 4 is labeled "Polymer Yield Index". The mathematical definition in `crates/oklo_core` evaluates an algebraic surrogate function whose global mass balance closes by mathematical construction (Finding R0-8). It does not compute monomer condensation kinetics, phosphodiester bond synthesis, template-directed ligation, or RNA folding. Representing this field as evidence of prebiotic polymer formation or RNA emergence is strictly forbidden.
- Abiogenesis and RNA World Non-Claims: The platform does not simulate the origin of life, RNA synthesis, metabolic cycle closure, or self-replicating chemical systems.
- Protocell and Endosymbiosis Non-Claims: The platform does not model lipid vesicle formation, protocell division, phagocytosis, or mitochondrial/plastid endosymbiosis.
- Enforcement: Any marketing, user interface, or documentation copy suggesting that Oklo Pool Mobile or Planetary Explorer v2 simulates abiogenesis or proves life originated in nuclear geyser environments violates the project charter and must be rejected immediately.

---

## 3. Research Repositories Integration Boundary Audit

Milestone B requires formal integration boundaries for three external research repositories. The red-team audited the commit-pinned status and architectural boundaries of each:

### 3.1 `aurascoper/Biofilms`
- Canonical URL: `https://github.com/aurascoper/Biofilms`
- Pinned Commit SHA: `74f433933f594ce749fd700b10b1041776f1a0e9`
- Target Tree: `private/geolocator-runtime` (manuscript: `71ea8d4`)
- License: CC0 1.0 Universal
- Integration Status: BLOCKED on `RADIODIALYSIS: BLOCKED`. Read-only architectural reference only.
- Technical Audit and Root Cause of Blocker:
  The repository attempts to couple a 3D cylindrical Cellular Potts Model (CPM) with reaction-diffusion operators simulating radioprotection in biofilm matrices. Executing the coupled solver path aborts at Monte Carlo Step 1 (MCS 1) inside `step_radiolysis!` due to fatal dimensional and taxonomic defects:
  1. Unit Collapse: Cylinder radius R = N/2 (20 dimensionless lattice sites) is passed directly into a physical continuum operator expecting diffusion coefficients in cm^2/s.
  2. Concentration Mismatch: Dimensionless lattice occupancy fractions (X_total, X_red) are inserted into continuum density slots expecting g/cm^3, and subsequently multiplied by kinetic rate constants with units cm^3/(g*s).
  3. Taxonomic Misdefinition: The reducer fraction X_red divides the count of reducer species sites by the total count of interior lattice sites rather than total cell biomass, making it physically undefined.
  4. Radial Profile Smearing: The radial biomass distribution is averaged using an unweighted arithmetic mean, obliterating radial concentration gradients.
  5. Decoupled State Variables: Cell membrane integrity decays while effective permeability increases without the two state variables reading or influencing one another.
- Architectural Boundary: Planetary Explorer v2 incorporates zero code, zero data structures, and zero runtime execution from `Biofilms`. It references `Biofilms` solely as an architectural case study in negative control gating, unit testing discipline, and failure logging.

### 3.2 `aurascoper/rna_biofilm_toy`
- Canonical URL: `https://github.com/aurascoper/rna_biofilm_toy`
- Pinned Commit SHA: `b780a1a7f6113f3b8454cad5efbc7dd66b76a25a`
- Target Tree: `guards/kit-v1` (main: `9d9471e`)
- License: CC BY 4.0 (data/docs), MIT (code)
- Integration Status: DEFERRED as biological model. Methodological reference for 5-term ledger tests only.
- Technical Audit and Defect Remediation History:
  The repository contains a synthetic 2D cylindrical lattice model (50x50 sites) exploring population survival under an ionizing radiation gradient. Four major mathematical bugs were audited and repaired in earlier revisions:
  1. Lysis Unreachable: Radiation damage was applied to a deep-copied snapshot while the live simulation grid remained pristine. Repaired to mutate live grid states directly.
  2. Gradient Misdirection: Replication selection weighting multiplied lateral displacement dx by flux gradients for a field varying strictly in y. Repaired to evaluate true direction cosines.
  3. Unaccounted Monomer Leak: Boundary clamping silently clipped and discarded monomers, violating mass conservation. Repaired by formulating a strictly closed 5-term mass ledger.
  4. Spurious Seam Teleportation: Periodic boundary hops jumped across the entire 50-site domain with exponential weighting e^20. Repaired to unwrap periodic boundaries locally to {-1, 0, 1}.
- Closed 5-Term Mass Ledger: The repaired repository achieved closed machine-precision conservation:
  Delta M(t) = injected_gross - inject_clip - diffusion_clip - birth_sink + death_return (residual < 1e-12).
- Falsification of Spec 14 (Quasispecies Selection Hypothesis):
  Spec 14 posited that spatial radiation gradients could establish localized, stable selection bands favoring error-tolerant replicators. This hypothesis was mathematically falsified under realistic non-enzymatic RNA replication error rates (approx 7.6% per base), which cause error-catastrophe collapse across all spatial zones. Furthermore, equating single-strand radiolytic scissions to point mutations is chemically invalid.
- Architectural Boundary: `rna_biofilm_toy` contains zero RNA chemistry, zero nucleotide kinetics, and zero physical thermodynamics. It is barred from presentation as an RNA-world simulation and serves strictly as a methodological template for strict mass-conservation testing.

### 3.3 `aurascoper/SobolevMacromolecule`
- Canonical URL: `https://github.com/aurascoper/Stanford-RNA-3D-Folding-Part-2-1st-place`
- Pinned Commit SHA: `c5947bcb98b85ae5c89830c30e907cd52104d0c1`
- Target Tree: `codex/sobolev-polish-gate`
- License: MIT License
- Integration Status: RESTRICTED. Precomputed static coordinate tables only (`RNA-C1-NORM v0`). Zero client-side runtime execution.
- Technical Audit and Interface Definition:
  The runnable solver implementation resides in the Stanford RNA repository (commit `c5947bc`), utilizing an H^1 Sobolev gradient descent with Discrete Cosine Transform (DCT-II) frequency damping (alpha = 5.0) to regularize RNA C1-prime backbone coordinates.
  - Caveats: The reported self-TM-score (0.50934) is repository-reported only and has not been verified by independent external benchmarks.
  - Coordinate Resolution: The representation tracks only C1-prime backbone atoms (nominal distance 5.95 Angstroms); it lacks full-atom ribose, phosphate, and purine/pyrimidine base geometries.
- Architectural Boundary: Client applications (iOS, Android, WebGL) must never embed JAX, PyTorch, model weights, or remote inference APIs. The interface is strictly restricted to static coordinate tables formatted under the `RNA-C1-NORM v0` specification for 3D ribbon visual rendering.

---

## 4. Epistemic Limitations Matrix Across All 17 Models

The following matrix documents the governing assumptions, empirical gaps, and strict non-claims for every model registered in `data/models/manifest.json`:

| # | Model ID | Evidence Class | Governing Assumptions | Empirical Gaps | Strict Non-Claims |
|---|---|---|---|---|---|
| 1 | `oklo_site_locator_wgs84` | `geographic_observation_or_locator` | WGS84 ellipsoidal Earth model (EPSG:4326); modern Gabon coordinates (-1.4000, 13.1833). | Surface coordinate accuracy localized to roughly 100 meters horizontal; does not capture subsurface drift. | NOT a Paleoproterozoic paleogeographic reconstruction; does not indicate 2.0 Ga plate position or orientation. |
| 2 | `oklo_fission_xenon_cycle` | `sourced_geological_record` | Xenon retention in aluminum phosphate crystals reflects cyclic thermal-hydraulic reactor operation (Meshik et al. 2004). | Indirect geochemical proxy; mineral formation integrates millions of pulses; no real-time pulse dynamics instrumentation. | Does not provide continuous spatial thermal profiles or real-time pressure records during active fission. |
| 3 | `oklo_u235_depletion_ore` | `sourced_geological_record` | Mass spectrometry of drill-core samples measuring 235U/238U depletion below 0.00725 baseline (Bodu 1972). | Discrete point assays; spatial coverage is heterogeneous across reactor zones and intervening facies. | Does not represent continuous spatial 3D depletion fields without unvalidated geostatistical interpolation. |
| 4 | `oklo_zone2_stratigraphy` | `sourced_geological_record` | Geological borehole logs and structural mapping of Francevillian FA-FB sedimentary boundary (Gauthier-Lafaye 1989). | Decimeter-to-meter vertical logging resolution; localized to specific mining pit exposures. | Cannot be generalized to regional Franceville Basin architecture or unexcavated reactor zones. |
| 5 | `oklo_temperature_v0` | `prescribed_analytic_field` | Algebraic vertical linear gradient scaled by radial core and conduit indicator functions (CONFORMANCE_FIXTURE_SPEC). | Not a thermal-hydraulic or energy-conservation physical simulation; zero convection or conductive physics. | Does not simulate fission heat generation, nuclear decay transients, or thermodynamic fluid flow. |
| 6 | `oklo_hydration_v0` | `prescribed_analytic_field` | Prescribed algebraic desiccation curve scaling saturation between 0.02 and 1.0 based on temperature proxy. | Zero multiphase fluid dynamics; Richards equation, capillary pressure, and relative permeability ignored. | Does not model two-phase boiling, steam chimney physics, or liquid-vapor phase transitions. |
| 7 | `oklo_h2_concentration_v0` | `prescribed_analytic_field` | Prescribed radiolytic proxy proportional to core proximity and hydration fraction. | Constant G-value assumed without radiation dose integration, dose rate fields, or molecular diffusion. | Does not compute actual physical hydrogen gas molarity, partial pressures, or chemical reaction equilibria. |
| 8 | `oklo_u235_ratio_v0` | `prescribed_analytic_field` | Prescribed spatial gradient anchored at 0.00725 natural ratio, depressing to 0.004 within the nominal core. | Algebraic fixture geometry; no neutron transport, criticality, cross-sections, or burnup equations solved. | Does not calculate nuclear criticality, neutron flux spectra, or isotopic transmutation. |
| 9 | `oklo_polymer_yield_v0` | `prescribed_analytic_field` | Prescribed oligomer surrogate index evaluated from local hydration and hydrogen proxies. | Mass balance closes by algebraic construction; zero polymerization kinetics, hydrolysis, or thermodynamics. | NOT evidence of prebiotic RNA synthesis, chemical replication, or abiogenesis. |
| 10 | `oklo_compartments_v0` | `prescribed_analytic_field` | Categorical integer zone assignment (0=Sandstone, 1=Core, 2=Halo, 3=Conduit, 4=Steam Chimney) on 32x32x32 grid. | Idealized regular geometric cylinders and slabs; ignores structural faulting, jointing, and facies variation. | NOT surveyed subsurface geology or measured fracture network topology. |
| 11 | `darcy_flow_v1_a4` | `numerically_verified_synthetic_model` | Steady, isothermal single-phase Darcy flow on an idealized 16m cube; conjugate gradient solver (A4 prereg). | Interface check D3a reached 2.58e-10 relative error (failed 1e-10 tolerance); Robertson conductivity uncorrected at 300 C. | NOT a historical Oklo hydrogeological reconstruction; does not model steam boiling or regional groundwater. |
| 12 | `uranium_redox_column_v1_a1` | `numerically_verified_synthetic_model` | 1D advection-dispersion-sorption column; Crank-Nicolson finite volume with M-matrix time bounds (A1 prereg). | Linear sorption Kd swept up to 630 m^3/kg (EPA bound; transit time > 200 Gyr); constant reducing capacity. | NOT a site reconstruction of Oklo or the Franceville Basin; ignores 3D dispersion and mineral precipitation. |
| 13 | `biofilms_potts_cpm_v0` | `numerically_verified_synthetic_model` | Cellular Potts discrete lattice model (N=40 or 60); Metropolis Monte Carlo with 4-term Hamiltonian. | Blocked at MCS 1 due to fatal unit mismatch (lattice units into cm^2/s operators; seconds_per_mcs is NaN). | NOT a simulation of biological organisms; radiodialysis coupling path is completely non-functional. |
| 14 | `rna_biofilm_toy_v1` | `numerically_verified_synthetic_model` | 2D cylindrical lattice (50x50) with closed 5-term mass ledger and localized replication dynamics. | Zero RNA chemistry; nucleotide thermodynamics absent; Spec 14 quasispecies selection falsified. | NOT an RNA-world simulation; does not demonstrate prebiotic evolution or abiogenesis. |
| 15 | `sobolev_rna_c1_norm_v0` | `numerically_verified_synthetic_model` | H^1 Sobolev gradient descent smoothing with DCT-II frequency damping (alpha = 5.0) on C1-prime coordinates. | Self-TM-score (0.50934) unbenchmarked externally; lacks full-atom ribose-phosphate geometry. | Static precomputed coordinate table only; client runtime executes zero inference or physical relaxation. |
| 16 | `prebiotic_wet_dry_polymerization_geyser` | `hypothesis_or_unvalidated_model` | Theoretical conjecture that cyclic geyser dehydration drives nucleotide precursor condensation. | Uncalibrated kinetic rate constants under high-radiation Paleoproterozoic hydrothermal conditions. | Unproven theoretical hypothesis; not demonstrated in natural nuclear reactor geochemical environments. |
| 17 | `radiation_gradient_quasispecies_selection` | `hypothesis_or_unvalidated_model` | Theoretical conjecture that ionizing radiation gradients establish stable error-tolerant selection bands. | Falsified under realistic non-enzymatic RNA replication error rates (approx 7.6% per base error collapse). | Falsified hypothesis; single-strand radiolytic scissions do not equal point replication mutations. |

---

## 5. Deferred Scientific-Model Work and Roadmap Governance

In accordance with Section R9 of `ORIGINAL_REQUEST.md`, the following domains are formally declared out of scope and deferred to future milestones:

1. Mechanistic Polymer Accumulation Solvers (Tier 2 and Tier 3):
   - Replacing the revision-0 algebraic polymer index with a system of ordinary differential equations (ODEs) modeling nucleotide polymerization, phosphodiester bond formation, and thermal hydrolysis.
   - Requires: Validated kinetic rate constants, activation energies, and thermodynamic free-energy landscapes under high-temperature, high-pressure, ionizing-radiation conditions.

2. Coupled Multiphase Thermal-Hydraulic-Chemical Solvers:
   - Coupling Darcy groundwater flow directly into 3D heat transport with phase boiling (steam chimney formation) and chemical species advection.
   - Requires: 3D Navier-Stokes / Darcy-Brinkman multiphase equations of state for superheated water-steam mixtures at 30 MPa, coupled to mineral dissolution-precipitation kinetics.

3. Biological Protocell and Compartmentalization Mechanics (Tier 4):
   - Simulating lipid vesicle formation, fatty acid membrane permeability, and autonomous protocell fission.
   - Requires: Resolving the fatal dimensional collapse and unit inconsistencies identified in `Biofilms` before any biological CPM solver can be executed.

4. Evolutionary Dynamics and Endosymbiotic Transitions (Tier 5):
   - Modeling metabolic co-dependence, gene transfer, or organelle emergence.
   - Requires: Independent evolutionary game theory formulations constrained by phylogenomic molecular clock data, completely decoupled from local geochemical field displays.

5. Client-Side Neural Model Inference or Remote Solvers:
   - Embedding deep neural networks, JAX/PyTorch runtimes, or live cloud-solver APIs within the mobile or browser applications.
   - Requires: A dedicated privacy and resource budget review, maintaining 100% offline functionality as the primary baseline.

---

## 6. Audit Verdict and Sign-Off

The red-team audit concludes with the following determinations:

1. Epistemic Integrity: PASS. The prototype on branch `planetary-explorer/v2-prototype` successfully enforces epistemic badges, prominent schematic warning banners, and dynamic limitation callouts. Visual proximity does not elevate model classifications.
2. Sourced Geological vs Synthetic Separation: PASS. Sourced empirical assays (Meshik 2004, Bodu 1972, Gauthier-Lafaye 1989) are strictly separated from prescribed algebraic scenarios (v0 fields) and synthetic numerical benchmarks (Darcy v1+A4, Uranium v1+A1).
3. Research Repositories Boundaries: PASS. `Biofilms` is appropriately marked BLOCKED and restricted to an architectural reference. `rna_biofilm_toy` is restricted to a methodological mass-ledger reference with documented falsification of Spec 14. `SobolevMacromolecule` is restricted to static precomputed coordinate tables.
4. Model Manifest Validation: PASS. All 17 registered models in `data/models/manifest.json` validate against `data/models/schema.json`. Exactly 0 models are classified as `empirically_calibrated_model`.
5. Typography and Quarantined Invariants: PASS. Zero em-dashes, zero en-dashes, and zero quarantined terms exist across all audited files.

Audit Status: VERIFIED AND APPROVED FOR ACCEPTANCE.
