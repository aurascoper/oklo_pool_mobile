# Exploratory Research Work Package: Surface-Associated Fungal Matrix (`surface_matrix_protection_v0`)

- **Document ID:** `SURFACE_MATRIX_PROTECTION_v0`
- **Requirement Reference:** Requirement R6A of Planetary Explorer v2 Plan (`docs/specs/PLANETARY_EXPLORER_ARCHITECTURE_v2.md` Section 5.2)
- **Status:** Proposed; research-only; no production-model changes authorized
- **Disposition:** Research candidate only. Passing this work package does not promote it to a calibrated model or authorize production coupling.
- **Evidence Class:** `hypothesis_or_unvalidated_model`

---

## 1. Objective and Architectural Scope

Use the photographed wood-growing mushroom and its sharply demarcated black surface growth as a candidate biological observation for developing and testing a general spatial-matrix research workflow. Investigate whether the observation is consistent with the reported *Pleurotus cystidiosus* / *Antromycopsis macrocarpa* association, and whether characterized material provides a defensible basis for a separately specified surface-matrix transport model.

### 1.1 Scope Placement
This work package is folded into **Milestone B** immediately after **Requirement R6 (Research repository integration contracts)** and before **Requirement R7 (Epistemic separation contract)**. It is a cross-cutting research work package about surface-associated material, reaction-transport verification, and evidence quality. It is **not** a sixth scientific tier alongside the five planetary-exploration tiers, and it is **not** a change to the current `aurascoper/Biofilms` solver.

### 1.2 Explicit Non-Claims
This work package is exploratory. It does **not**:
1. Establish the specimen's taxonomic identity.
2. Identify its pigment as melanin.
3. Demonstrate reactive-oxygen-species (ROS) quenching.
4. Establish ionizing or ultraviolet radiation protection.
5. Support radiation-associated energy transduction (radiosynthesis or radiotrophy).

---

## 2. Part A: Evidence and Identification Registry

### 2.1 Photograph Provenance and Metadata Record

All known and unknown metadata for the motivating observation are recorded without interpolation:

| Field | Value | Provenance Status |
|---|---|---|
| `observation_id` | `obs_lima_morus_mushroom_2026_10_20` | User-supplied field photograph |
| `image_sha256` | `b7741a3560ab4fc1a93cf212df06ca436e0d4a49bf354a68e284ac26bd58086c` | Exact SHA-256 digest of original JPEG (`194,251` bytes) |
| `reported_location` | Lima, Peru | User report (coastal urban/peri-urban environment) |
| `geographic_coordinates` | `unknown` | No GPS EXIF or surveyed coordinates recorded |
| `observation_date` | `unknown` | Provided 2026-10-20; original capture timestamp unknown |
| `reported_substrate` | Living or standing *Morus* sp. (mulberry tree) bark | User report; visually consistent with fissured hardwood bark |
| `voucher_specimen_id` | `unknown` | No herbarium voucher, spore print, or axenic culture recorded |
| `additional_photographs` | `unknown` | Single oblique upper-pileus view only; underside hymenophore not photographed |
| `field_notes` | `unknown` | No microclimate humidity, temperature, or substrate decay logs recorded |

### 2.2 Four-Layer Structural Decomposition and Competing Hypotheses

Per Requirement R6A.A.2, the observation is registered as a hypothesis and separated into four distinct observational layers:

| Layer | Visible Features in Photograph | Candidate Hypothesis | Alternative Explanations Not Excluded by Gross Morphology | Required Diagnostic Evidence |
|---|---|---|---|---|
| **1. Visible Fruiting Bodies** | Two overlapping, pale cream-to-white, fleshy, laterally attached (pleurotoid/flabelliform) basidiocarps on fissured bark with a clean white outer margin. | Basidiomata of a wood-decay agaric in the *Pleurotus cystidiosus* species complex (subgenus *Coremiopleurotus*). | Other pleurotoid agarics (*Hohenbuehelia*, *Crepidotus*, white *Lentinus* / *Panus*) or non-gilled polypores (*Trametes*, *Osteina*) because the hymenophore is uninspected. | Underside macro photograph verifying decurrent lamellae (gills), basidiospore print color, and microscopic check for generative hyphae with clamp connections. |
| **2. Black Surface Growth** | A dense, matte-to-velvety, finely granular jet-black mat covering the proximal/upper pileus and inter-cap cleft, terminating at a sharp boundary before the white margin. | Direct teleomorph-associated production of anamorphic synnemata/coremia (*Antromycopsis macrocarpa* state) on the pileus surface. | 1. Secondary colonization by a dematiaceous hyphomycete or mycoparasite (*Stachybotrys*, *Cladosporium*, *Alternaria*, *Aspergillus niger* group).<br>2. Coalesced dark spore deposit from an overhanging fruiting body.<br>3. Localized bacterial or fungal necrosis of senescent cap tissue. | Hand-lens ($5\times\text{ to }20\times$) cross-section of the black-white margin to determine whether discrete white synnematal stipes arise directly from the pileus context vs. superficial foreign mycelium. |
| **3. Potential Reproductive Structures** | Packed dark surface granules across the black mat; individual stalks are below the resolution limit of the photograph. | Coremial heads bearing arthric chains of cylindrical, dark-walled arthroconidia (*Antromycopsis macrocarpa*; Stalpers, Seifert, and Samson 1991). | Foreign conidiophores (phialidic, sympodial, or dictyosporous conidia) or sterile dark microsclerotia/crustose hyphae. | Brightfield microscopy ($400\times\text{ to }1000\times$ in $3\%$ KOH and $\text{H}_2\text{O}$) testing for arthric disarticulation of cylindrical arthroconidia ($17\text{ to }25 \times 5\text{ to }7.5\ \mu\text{m}$ in Neotropical *P. cystidiosus* per *Acta Botanica Mexicana* 2024) and absence of foreign conidiogenous cells. |
| **4. Suspected Liquid or Extracellular Matrix** | Cohesive velvety-to-crustose dark surface layer across the proximal depression. | Coalesced apical coremioliquid / extracellular mucilaginous matrix embedding arthroconidia. | Dry, non-mucilaginous spore crust or surface moisture film without secreted polymeric matrix. | KOH/water mount test for mucilaginous sheath dispersion and chemical characterization of isolated matrix material. |

### 2.3 Research Evidence Registry (Claims, Literature, Uncertainty, and Tests)

| Claim ID | Claim Statement | Supporting Source | Evidence Class | Uncertainty and Discrepancies | Outstanding Test |
|---|---|---|---|---|---|
| `CLM-R6A-01` | Species in the *Pleurotus cystidiosus* complex form a coremial anamorph (*Antromycopsis*) with dark apical conidial masses on woody substrates (including Moraceae hosts such as *Broussonetia papyrifera* and *Morus*) and directly on basidiomata. | Stalpers, Seifert, and Samson (1991) *Can. J. Bot.* 69:6-15; Zervakis, Moncalvo, and Vilgalys (2004) *Mycologia* 96:274-288. | `sourced_geological_record` (peer-reviewed literature record) | Zervakis et al. (2004) demonstrated that the *P. cystidiosus* complex comprises multiple distinct phylogenetic lineages separate from Australasian *P. australis*. | Nuclear ribosomal ITS1-5.8S-ITS2 barcoding of the Lima specimen. |
| `CLM-R6A-02` | Neotropical *Pleurotus cystidiosus* specimens confirmed by ITS sequencing produce white-stiped, black-headed coremia with arthrospores measuring $17\text{ to }25 \times 5\text{ to }7.5\ \mu\text{m}$. | *Acta Botanica Mexicana* (2024) Neotropical *P. cystidiosus* taxonomic study. | `sourced_geological_record` (peer-reviewed literature record) | An unverified conversational estimate of $6\text{ to }12 \times 3\text{ to }6\ \mu\text{m}$ conflicts with the $17\text{ to }25 \times 5\text{ to }7.5\ \mu\text{m}$ measurements reported for Neotropical *P. cystidiosus* in 2024. | Calibrated ocular micrometer measurement ($N \ge 30$ arthroconidia in $3\%$ KOH). |
| `CLM-R6A-03` | Dark mucous arthrospore exudate in a *P. cystidiosus* variety contains pigment consistent with melanin characterization. | Selvakumar et al. (2008) pigment characterization study. | `sourced_geological_record` (peer-reviewed literature record) | Characterizes pigment in one cultured strain; does not verify pigment identity in the uncollected Lima specimen or establish a ROS-quenching rate constant. | EPR free-radical signature, FTIR spectroscopy, and alkaline $\text{H}_2\text{O}_2$ bleaching assay of Lima specimen material. |
| `CLM-R6A-04` | The Lima *Morus* mushroom's black pileus layer is an *Antromycopsis* coremial matrix that quenches extracellular reactive species and shields underlying tissue. | None (unverified field observation plus mechanistic hypothesis). | `hypothesis_or_unvalidated_model` | Taxon unconfirmed; pigment uncharacterized; reactive species undefined; quenching rate constant $k_M$ unmeasured. | Full Gate D empirical protocol (Section 5 below). |

---

## 3. Part B: Independent Model Specification (`surface_matrix_protection_v0`)

### 3.1 Epistemic and Chemical Separation Rules
1. **General Surface-Matrix vs. Melanin-Specific Mechanism:** The state variable $M_{\mathrm{surf}}(\mathbf{x}, t)$ represents a generic **surface-associated extracellular matrix field**. It must **not** be labeled as melanin unless supported by Gate D.2 material characterization.
2. **Single Named Reactive Species:** The transported field $C_R(\mathbf{x}, t)$ represents **one explicitly identified reactive chemical species** $R$ (e.g. aqueous $\text{H}_2\text{O}_2$), never an undifferentiated "total ROS" composite.
3. **No Universal Dose-Rate-to-ROS Source:** A radiation dose rate $\dot{D}(\mathbf{x}, t)$ must not be treated as a universal ROS source without an explicit radiation-chemistry contract specifying medium density $\rho(\mathbf{x})$, primary radiolytic yield $G_R$ ($\text{mol}\cdot\text{J}^{-1}$) for species $R$, and valid linear dose-rate bounds. By default, `surface_matrix_protection_v0` uses a declared boundary influx or volumetric source $S_R(\mathbf{x}, t)$ with the matrix-associated sink $k_M$ initially disabled ($k_M = 0$) or explicitly tagged `"synthetic_uncalibrated"`.

### 3.2 State Variables, Units, and Parameter Provenance

Every configuration must declare either **Basis A (Physical SI)** or **Basis B (Synthetic Dimensionless)** and reject mixed-unit inputs:

| Symbol | Description | Basis A (Physical SI Units) | Basis B (Synthetic Dimensionless) | Parameter Provenance in `v0` |
|---|---|---|---|---|
| $\mathbf{x} = (x, y, z)$ | Spatial coordinates on domain $\Omega = [0, L_x] \times [0, L_y] \times [0, L_z]$ | $\text{m}$ (`cartesian_local_meters`) | $\tilde{\mathbf{x}} \in [0, 1]^d$ (`dimensionless_unit_domain`) | `declared_geometry` |
| $t$ | Time coordinate | $\text{s}$ | $\tilde{t} \ge 0$ | `declared_time_grid` |
| $M_{\mathrm{surf}}(\mathbf{x}, t)$ | Surface-associated extracellular matrix field | $\text{kg}\cdot\text{m}^{-3}$ | Dimensionless index in $[0, 1]$ | `state_variable` |
| $B_{\mathrm{surf}}(\mathbf{x}, t)$ | Matrix-producing surface source support | $\text{kg}\cdot\text{m}^{-3}$ | Indicator $\chi_{\mathrm{surf}}(\mathbf{x}) \in \{0, 1\}$ | `prescribed_support` |
| $C_R(\mathbf{x}, t)$ | Concentration of named reactive species $R$ | $\text{mol}\cdot\text{m}^{-3}$ | Dimensionless $C_R / C_{R,\mathrm{ref}}$ | `state_variable` |
| $D_{\mathrm{mat}}$ | Matrix spreading coefficient ($D_{\mathrm{mat}} \ge 0$) | $\text{m}^2\cdot\text{s}^{-1}$ | Dimensionless $\tilde{D}_{\mathrm{mat}} \ge 0$ | `synthetic_uncalibrated` |
| $k_{\mathrm{sec}}$ | Matrix production rate constant ($k_{\mathrm{sec}} \ge 0$) | $\text{s}^{-1}$ | Dimensionless $\tilde{k}_{\mathrm{sec}} \ge 0$ | `synthetic_uncalibrated` |
| $k_{\mathrm{deg}}$ | Matrix loss/washout rate constant ($k_{\mathrm{deg}} \ge 0$) | $\text{s}^{-1}$ | Dimensionless $\tilde{k}_{\mathrm{deg}} \ge 0$ | `synthetic_uncalibrated` |
| $D_R(\mathbf{x})$ | Reactive species diffusivity ($D_R > 0$) | $\text{m}^2\cdot\text{s}^{-1}$ | Dimensionless $\tilde{D}_R > 0$ | `synthetic_uncalibrated` |
| $\mathbf{u}(\mathbf{x}, t)$ | Advective pore-fluid velocity | $\text{m}\cdot\text{s}^{-1}$ | Dimensionless $\tilde{\mathbf{u}}$ | `synthetic_uncalibrated` |
| $k_0$ | Background non-matrix decay rate ($k_0 \ge 0$) | $\text{s}^{-1}$ | Dimensionless $\tilde{k}_0 \ge 0$ | `synthetic_uncalibrated` |
| $k_M$ | Hypothesized matrix-associated sink constant ($k_M \ge 0$) | $\text{m}^3\cdot\text{kg}^{-1}\cdot\text{s}^{-1}$ | Dimensionless $\tilde{k}_M \ge 0$ | Initially $0$ (`disabled`) or `synthetic_uncalibrated` |

### 3.3 Governing Equations, Boundary Conditions, and Numerical Method

1. **Surface-Matrix Balance Equation:**
   $$\frac{\partial M_{\mathrm{surf}}(\mathbf{x}, t)}{\partial t} = \nabla \cdot \bigl(D_{\mathrm{mat}} \nabla M_{\mathrm{surf}}\bigr) + S_M(\mathbf{x}, t) - k_{\mathrm{deg}}\,M_{\mathrm{surf}}(\mathbf{x}, t)$$
   with homogeneous Neumann boundary condition $-\mathbf{n} \cdot (D_{\mathrm{mat}} \nabla M_{\mathrm{surf}}) = 0$ on $\partial \Omega$, initial condition $M_{\mathrm{surf}}(\mathbf{x}, 0) = M_{\mathrm{surf},0}(\mathbf{x}) \ge 0$, and selectable source arm:
   - **Arm A0 (Constitutive Developmental Secretion):** $S_M(\mathbf{x}, t) = k_{\mathrm{sec}}\,B_{\mathrm{surf}}(\mathbf{x}, t)$.
   - **Arm A1 (Stress-Modulated Secretion Counterfactual):** $S_M(\mathbf{x}, t) = k_{\mathrm{sec}}\,B_{\mathrm{surf}}(\mathbf{x}, t)\,\frac{\sigma(\mathbf{x}, t)}{\sigma(\mathbf{x}, t) + K_\sigma}$, where $\sigma \ge 0$ and $K_\sigma > 0$ share identical declared units.

2. **Reactive-Species Transport and Matrix-Sink Equation:**
   $$\frac{\partial C_R(\mathbf{x}, t)}{\partial t} = \nabla \cdot \bigl(D_R(\mathbf{x}) \nabla C_R\bigr) - \nabla \cdot \bigl(\mathbf{u}(\mathbf{x}, t)\,C_R\bigr) + S_R(\mathbf{x}, t) - \bigl(k_0 + k_M\,M_{\mathrm{surf}}(\mathbf{x}, t)\bigr)\,C_R(\mathbf{x}, t)$$
   with declared boundary fluxes $J_{R,\mathrm{bnd}}(\mathbf{x}, t) = \mathbf{n} \cdot \bigl(\mathbf{u}\,C_R - D_R \nabla C_R\bigr)$ on $\partial \Omega$ and initial condition $C_R(\mathbf{x}, 0) = C_{R,0}(\mathbf{x}) \ge 0$.

3. **Numerical Discretization and Diagnostics:**
   - Conservative cell-centered finite-volume spatial discretization with harmonic-mean face diffusivities, upwind or centered flux subject to cell Peclet constraints, and second-order Crank-Nicolson / IMEX time integration under positivity-preserving M-matrix step bounds.
   - **Matrix Inventory:** $\mathcal{I}_M(t) = \int_\Omega M_{\mathrm{surf}}(\mathbf{x}, t)\,dV$.
   - **Reactive Species Inventory:** $\mathcal{I}_R(t) = \int_\Omega C_R(\mathbf{x}, t)\,dV$.
   - **Sub-Matrix Exposure Proxy:** $E_{\mathrm{sub}}(t) = \int_{\Omega_{\mathrm{sub}}} C_R(\mathbf{x}, t)\,dV$.
   - **Conservation Residual:**
     $$\mathcal{R}_R^n = \frac{\left|\mathcal{I}_R^n - \mathcal{I}_R^0 - \Delta t \sum_{m=1}^n \bigl(\Phi_{\mathrm{src}}^m - \Phi_{\mathrm{sink},0}^m - \Phi_{\mathrm{sink},M}^m - \Phi_{\mathrm{bnd}}^m\bigr)\right|}{\max\bigl(\mathcal{I}_R^n,\,\Delta t \sum_{m=1}^n \Phi_{\mathrm{src}}^m + \Delta t \sum_{m=1}^n |\Phi_{\mathrm{bnd}}^m|,\,10^{-30}\bigr)}$$

---

## 4. Part C: Verification and Controls (C1 to C7)

Before considering any biological interpretation or labeling any output as `numerically_verified_synthetic_model`, all seven preregistered controls must be verified:

1. **C1 - Matrix-Free Baseline:** With $M_{\mathrm{surf}}(\mathbf{x}, 0) \equiv 0$ and $k_{\mathrm{sec}} = 0$, $M_{\mathrm{surf}}(\mathbf{x}, t) \equiv 0$ for all $t \ge 0$, and $C_R(\mathbf{x}, t)$ matches the analytical or reference matrix-free reaction-diffusion solution with decay $k_0 C_R$ within declared tolerance.
2. **C2 - Source-Free Limiting Case:** With $S_R(\mathbf{x}, t) \equiv 0$, zero boundary influx, and $C_R(\mathbf{x}, 0) \equiv 0$, $C_R(\mathbf{x}, t) \equiv 0$ identically across all cells and steps.
3. **C3 - Matrix Production and Loss Accounting:** For pure loss ($k_{\mathrm{sec}} = 0, D_{\mathrm{mat}} = 0, M_{\mathrm{surf},0} > 0$), $M_{\mathrm{surf}}(\mathbf{x}, t)$ converges at declared order to $M_{\mathrm{surf},0}(\mathbf{x})\exp(-k_{\mathrm{deg}} t)$; for driven production/loss ($k_{\mathrm{sec}} > 0, k_{\mathrm{deg}} > 0$), the discrete matrix mass ledger closes to $\mathcal{R}_M^n \le 10^{-12}$.
4. **C4 - Reaction-Transport Conservation and Numerical Convergence:** Discrete reactive-species inventory closes to $\mathcal{R}_R^n \le 10^{-10}$ across all steps, and grid/step refinement reproduces the formal spatial and temporal convergence orders ($\ge 1.95$ for second-order schemes).
5. **C5 - Sink-Strength Monotonicity under Linear Assumptions:** Under linear PDE and boundary-condition assumptions ($\mathbf{u}, D_R, S_R \ge 0$, fixed $M_{\mathrm{surf}}(\mathbf{x}) \ge 0$ independent of $C_R$, non-negative boundary influx), increasing $0 \le k_{M,1} < k_{M,2}$ yields pointwise $C_R(\mathbf{x}, t; k_{M,2}) \le C_R(\mathbf{x}, t; k_{M,1})$ and monotonically non-increasing sub-matrix exposure $E_{\mathrm{sub}}(t; k_M)$.
6. **C6 - Surface-Localized vs. Spatially Uniform Matrix at Equal Total Inventory:** Comparing an apical cap matrix layer $M_{\mathrm{surf}}^{\mathrm{apron}}(\mathbf{x})$ against a spatially uniform distribution $M_{\mathrm{surf}}^{\mathrm{unif}}(\mathbf{x}) \equiv M_{\mathrm{tot}} / |\Omega|$ at identical total inventory $\mathcal{I}_M^{\mathrm{apron}} = \mathcal{I}_M^{\mathrm{unif}} = M_{\mathrm{tot}}$ under apical stressor influx quantifies the geometric shielding ratio $E_{\mathrm{sub}}(\text{apron}) / E_{\mathrm{sub}}(\text{uniform})$ with closed mass ledgers in both cases.
7. **C7 - Rejection of Invalid Units, Unsupported Provenance, and Incompatible Coordinates:** Negative controls supplying mixed SI/dimensionless units, negative diffusivities or rate constants, undeclared `parameter_provenance`, or cross-coordinate coupling without an explicit adapter must fail closed with an explicit exception and non-zero exit status.

*Epistemic Invariant:* Numerically verified synthetic results must remain labeled as synthetic. Passing Controls C1 through C7 establishes properties of the numerical implementation, not the truth of the biological hypothesis.

---

## 5. Part D: Evidence Gate for Mechanistic Interpretation

Assigning biological meaning to the matrix-associated sink $k_M M_{\mathrm{surf}} C_R$ is **blocked** until all five empirical prerequisites are obtained:

1. **Taxonomic Evidence:** Voucher-backed morphological microscopy ($3\%$ KOH mount of coremia, arthroconidia, and hymenophore) and nuclear ribosomal ITS barcoding adequate for the intended claim.
2. **Material Characterization:** Chemical analysis of isolated matrix material (EPR, FTIR, UV-Vis, elemental analysis, and polysaccharide/protein characterization) adequate to establish composition.
3. **Defined Reactive Species Measurement:** Controlled measurement of the lifetime, concentration decay, or penetration of a single named reactive species $R$ across a characterized matrix layer.
4. **Experimental Controls:** Reproducible matrix-free, heat/chemically inactivated, and bleached controls with stated uncertainty bounds.
5. **Parameter-Identification Procedure:** A documented inverse-modeling protocol linking measured curves and uncertainties to $D_R$, $k_0$, and $k_M$.

A pigment assay alone does not establish a ROS-quenching rate constant. Evidence for oxidative protection does not establish radiation-associated energy transduction.

---

## 6. Part E: Integration Boundary

- This work package is used strictly as an independent research specification, test fixture, and provenance example.
- It does **not** modify `aurascoper/Biofilms`, `aurascoper/rna_biofilm_toy`, or `aurascoper/SobolevMacromolecule`.
- It does **not** bypass `Biofilms` calibration or coupling gates (`RADIODIALYSIS: BLOCKED`, `D-PITCH = awaiting_measurement`, `seconds_per_mcs = NaN`, `cpm.melanin_coupling = blocked`).
- It does **not** feed outputs into the Oklo revision-0 fields, Darcy flow, or 1D uranium column results.
