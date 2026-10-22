# Cross-Repository Specification: Interface Falsification Bridge & Epistemic Contract (v0.1.0)

- **Contract ID:** `INTERFACE_FALSIFICATION_BRIDGE_v0`
- **Version:** `v0.1.0`
- **Canonical Repository:** `aurascoper/oklo_pool_mobile` (`docs/specs/INTERFACE_FALSIFICATION_BRIDGE_v0.md`)
- **Machine Contract:** `data/contracts/interface_falsification_contract_v0.json`
- **Validator:** `scripts/validate_interface_falsification_contract.py`
- **Baseline Commit (`oklo_pool_mobile` `main`):** `942e3f8d02a3b73cfb26f099dbb36c4a908f6764`
- **Linked Geochemistry Baseline (`RadioPharmacology.jl` `main`):** `c11241b59bcb2ebca5317883240020a848c22773`
- **Audited Read-Only Reference (`Biofilms`):** `origin/master` at `c766e7be31144f707155416b788bf7d1ba473ab9` (local worktree `74f433933f594ce749fd700b10b1041776f1a0e9`)
- **Audited Read-Only Reference (`rna_biofilm_toy`):** `b780a1a7f6113f3b8454cad5efbc7dd66b76a25a`

---

## 1. Purpose & Non-Conflation Principle

This specification establishes the formal mathematical, physical, and epistemic bridge between the theoretical apparatus of *Performance, Interaction, and Correspondence: A Coevolutionary Aesthetic and Teleological Perspective* (`core-paper-v2-core6.pdf`, Kinder, October 2026) and the computational models across `RadioPharmacology.jl`, `oklo_pool_mobile`, `Biofilms`, and `rna_biofilm_toy`.

**Core Epistemic Rule:** Sharing an interface-partition and falsification contract across repositories does **not** imply that abiotic geochemical systems, porous media, or spatial lattice toys possess perceptual interfaces, neural criterial causation, aesthetic agency, or an empirically validated origin-of-life pathway. Mathematical operators adapted from `core-paper-v2-core6.pdf` are used strictly to define **coarse-graining partitions, fiber-sufficiency error bounds, and preregistered falsification tests**.

---

## 2. Six Epistemic Evidence Classifications

Every model, parameter, equation, and claim governed by this contract must carry one of six machine-readable `evidence_classification` tags:

1. `empirically_supported_parameter`: Directly measured physical or nuclear constant with a peer-reviewed measurement uncertainty and matching experimental domain.
2. `source_reported_value`: Value transcribed from a cited publication, nuclear data evaluation, or benchmark specification (e.g., DDEP May 2025, DDEP 2010, IAEA/ENSDF 2017, Renne et al. 2011, Hansma 2022) whose applicability to a specific geological or chemical scenario requires separate validation.
3. `phenomenological_surrogate`: Dimensionally consistent constitutive or empirical formula (e.g., effective $K_d$ reduction under oxalate complexation, or Bragg-Kleeman alpha stopping range) that summarizes a trend without modeling the underlying molecular reaction mechanism.
4. `uncalibrated_candidate_kinetic_law`: Mathematically well-posed rate or transport equation (e.g., competing-hazard vs. sequential-hydrolysis polymerization kinetics, or Bell-Evans force-assisted rate law) whose parameters have not yet been calibrated against experimental measurements.
5. `illustrative_simulation`: Deterministic or stochastic synthetic calculation (e.g., Revision-0 Oklo prescribed analytic fields, the 2D shielding lattice in `rna_biofilm_toy`, or stochastic Beta-split vesicle fragmentation) that demonstrates mathematical or algorithmic behavior rather than physical reality.
6. `untested_hypothesis`: Conceptual, evolutionary, or geological proposal (e.g., the Mica Hypothesis for abiogenesis, or deep-subsurface mineral ancestry claims) requiring experimental or field falsification.

---

## 3. Four-Layer Mathematical Bridge

### Layer 1: Objective Physical Environment $R$
At spatial control volume (or voxel) $v$ and time $t$, the local physical environment is represented by the state vector:
$$r_v(t) = \left[ T,\ a_w,\ \dot{D},\ \mathrm{pH},\ \mathbf{u},\ \mathbf{c},\ \phi_m \right]_{v,t} \in R$$
where:
- $T$: absolute temperature ($\mathrm{K}$)
- $a_w$: dimensionless water activity ($0 \le a_w \le 1$)
- $\dot{D}$: absorbed ionizing radiation dose rate ($\mathrm{Gy/s} = \mathrm{J/(kg\cdot s)}$), qualified by an explicit dose-basis tag (`pore_water_absorbed_dose_rate` or `bulk_medium_absorbed_dose_rate`)
- $\mathrm{pH}$: aqueous proton activity index (dimensionless)
- $\mathbf{u}$: pore-fluid velocity vector ($\mathrm{m/s}$)
- $\mathbf{c}$: vector of dissolved species concentrations ($\mathrm{mol/m^3}$ or $\mathrm{mol/L}$)
- $\phi_m$: mineral/scaffold descriptor tuple including dry solid grain density $\rho_s$ ($\mathrm{g/cm^3}$), porosity $\varepsilon$ (dimensionless), surface lattice periodicity $a_{\mathrm{lattice}}$ ($\mathrm{m}$), and fluid interlayer gap $h_{\mathrm{gap}}$ ($\mathrm{m}$)

### Layer 2: Coarse-Graining Interface Map $f_\theta: R \to P$ and Fibers $\mathcal{F}_p$
Adapting Hoffman's many-to-one projection $f: R \to P$ solely as a **formal structural analogy** for state space reduction, let $f_\theta: R \to P$ map continuous physical microstates $r \in R$ into a finite or lower-dimensional measurable state class $p = f_\theta(r) \in P$ (such as wet/dry regime, pore-accessibility class, redox window, or mineral alteration state).

The **fiber** over coarse-grained state $p \in P$ is the equivalence class of physical states mapped to $p$:
$$\mathcal{F}_p = f_\theta^{-1}(p) = \{ r \in R \mid f_\theta(r) = p \}$$

#### Measure-Space Prerequisite for the Probability-of-Symmetric-Difference (PSD) Metric
In `core-paper-v2-core6.pdf` (Section 4), Hoffman's (2006) PSD metric is defined on measurable events $A, A' \in \mathcal{B}$ in an explicit probability/measure space $(\Omega, \mathcal{B}, \mu)$:
$$d_{\mathrm{PSD}}(A, A') = \mu(A \triangle A') = \mu((A \setminus A') \cup (A' \setminus A)) = \| \mathbf{1}_A - \mathbf{1}_{A'} \|_{L^2(\mu)}^2$$
with $\sqrt{d_{\mathrm{PSD}}}$ providing an isometric embedding of indicator functions modulo $\mu$-null sets into the Hilbert space $L^2(\mu)$.

**Contract Requirement:** No simulation module may invoke $d_{\mathrm{PSD}}$ or claim an $L^2(\mu)$ Hilbert-space embedding on voxel arrays unless it explicitly declares:
1. The sample space $\Omega$ (e.g., spatial domain $V \subset \mathbb{R}^3$ or ensemble of trajectories),
2. The operational threshold predicate defining the event sets $A = \{\omega \in \Omega \mid q(\omega) \ge q_{\mathrm{thresh}}\} \in \mathcal{B}$, and
3. The normalized measure $\mu$ ($\mu(\Omega) = 1$, e.g., pore-volume-weighted measure $d\mu(x) = \varepsilon(x)\,dV / \int_\Omega \varepsilon\,dV$).

### Layer 3: Physical Outcome Map $g_\psi: P \times M \times S \to Y$ and $t_1 \to t_2 \to t_3$ Protocol
Instead of the paper's internal aesthetic evaluation $e: P \to H$, physical models define a measurable outcome function:
$$y_v(t) = g_\psi\big(p_v(t),\ m_v(t),\ s_v(t)\big) \in Y$$
where $m_v(t) \in M$ is the molecular/nuclide state and $s_v(t) \in S$ is the scaffold/mineral state. Allowed observables $y \in Y$ must carry explicit physical units (e.g., surviving nuclide fraction, retardation factor $R$, retained mass fraction, chain-length distribution, or washout fraction).

Time-ordered experimental designs follow the 3-step protocol:
- $t_1$ (**Pre-Perturbation State Specification**): Declare initial molecular configuration $m(t_1)$, scaffold state $s(t_1)$, and environmental state $r(t_1)$.
- $t_2$ (**Controlled Perturbation / Cycle Application**): Apply a defined radiation pulse, wet-dry cycle, fluid velocity change, or mechanical compression schedule over $\Delta t = t_3 - t_2$.
- $t_3$ (**Observable Readout & Mass Ledger Accounting**): Measure $y(t_3) \in Y$ and verify independent mass/inventory closure across intact, degraded, sorbed, and eluted compartments.

---

## 4. Six Preregistered Falsification Gates (`F1` through `F6`)

Following the fail-closed architecture of `Biofilms/coupling/biofilm_openmc/feedback_gate.py`, every falsification gate requires an explicit `effect_threshold` ($\delta > 0$ or $\varepsilon_{\mathrm{fiber}} > 0$) declared **before** evaluation. If `effect_threshold` is `null` (`None`), the validator must return `NOT_EVALUATED`.

| Gate ID | Name | Primary Observable & Units | Required Control Arm(s) | Mathematical Rejection / Falsification Rule |
| --- | --- | --- | --- | --- |
| **F1** | Interface / Fiber Sufficiency | Outcome divergence $\Delta_{\mathrm{fiber}}(p) = \sup_{r_1, r_2 \in \mathcal{F}_p} \|g_{\mathrm{full}}(r_1, m, s) - g_{\mathrm{full}}(r_2, m, s)\|_Y$ on held-out states | Full-resolution un-collapsed state model $g_{\mathrm{full}}(r, m, s)$ | **Falsified (`FALSIFIED_PARTITION_TOO_COARSE`)** if $\Delta_{\mathrm{fiber}}(p) + u_{95} > \varepsilon_{\mathrm{fiber}}$ for any coarse state $p \in P$. |
| **F2** | Pulse Timing vs. Total Energy / Exposure | Net product yield or survival fraction difference $\Delta Y_{\mathrm{schedule}} = Y(\text{pulsed } t_1 \to t_2 \to t_3) - Y(\text{continuous matched exposure})$ | Continuous or time-averaged control matched on integrated absorbed dose $\int \dot{D}\,dt$, thermal exposure $\int T\,dt$, and boundary inputs | **Falsified (`FALSIFIED_TIMING_NO_EFFECT`)** if $q_{0.01}(s \cdot \Delta Y_{\mathrm{schedule}}) \le \delta_{\mathrm{timing}}$, where $s \in \{+1, -1\}$ is the predeclared sign. |
| **F3** | Scaffold-Specific Effect | Specific retention or oligomer yield on biotite $Y_{\mathrm{biotite}}$ vs. controls ($\mathrm{mol/m^2}$ normalized by BET surface area) | (1) Inert surface control (quartz / borosilicate) and (2) alternative phyllosilicate control (muscovite / montmorillonite) under identical water chemistry (Song et al. 2024 vs. Simonis et al. 2026) | **Falsified (`FALSIFIED_NON_SPECIFIC_SURFACE`)** if surface-area-normalized effect $q_{0.01}(Y_{\mathrm{biotite}} - \max(Y_{\mathrm{inert}}, Y_{\mathrm{alt}})) \le \delta_{\mathrm{scaffold}}$. |
| **F4** | Selective Retention vs. Passive Accumulation | Retained solute/polymer fraction $f_{\mathrm{ret}}$ relative to conservative non-reactive tracer $f_{\mathrm{tracer}}$ under matched Peclet number | Conservative tracer ($\mathrm{Br}^-$ or $^3\mathrm{H}_2\mathrm{O}$, $K_d = 0$) under identical flow rate $\mathbf{u}$, evaporation rate, and pore volume | **Falsified (`FALSIFIED_PASSIVE_CONCENTRATION`)** if selective retardation ratio $R_{\mathrm{obs}} / R_{\mathrm{tracer}} - 1 \le \delta_{\mathrm{retention}}$ after accounting for evaporative volume loss. |
| **F5** | Heritable Selection vs. Physical Sorting | Inter-cycle shift in variant frequency $\Delta p_{\mathrm{gen}}$ and reciprocal environmental covariance $C_{12}$ across $N_{\mathrm{transfers}} \ge 3$ serial dilution/transfer cycles | Zero-replication physical-sorting control ($k_{\mathrm{copy}} = 0$, pure differential adsorption/hydrolysis) | **Falsified (`FALSIFIED_NON_HERITABLE_SORTING`)** if copying fidelity / template replication rate $k_{\mathrm{copy}} = 0$ or if serial transfer frequency shift fails to exceed the non-replicating physical-sorting control by $\delta_{\mathrm{heritable}}$. |
| **F6** | Independent Mass & Decay-Chain Accounting | Relative mass/inventory closure residual $\varepsilon_{\mathrm{ledger}} = \frac{|M_{\mathrm{in}} - (M_{\mathrm{intact}} + M_{\mathrm{degraded}} + M_{\mathrm{sorbed}} + M_{\mathrm{effluent}})|}{M_{\mathrm{in}}}$ | Independent compartment measurement (none of the compartments defined algebraically as $M_{\mathrm{in}} - \sum_{\text{others}}$) | **Falsified / Blocked (`BLOCKED_ALGEBRAIC_TAUTOLOGY` or `FALSIFIED_MASS_IMBALANCE`)** if `closes_by_algebraic_definition == true` or if independently tallied $\varepsilon_{\mathrm{ledger}} > \tau_{\mathrm{mass}}$. |

---

## 5. Three Required Scientific Corrections & Mathematical Formulations

### 5.1 Nuclear-Data Conventions: $^{40}\mathrm{K}$ Decay Branch vs. Evaluated $1460.822\mathrm{ keV}$ Photon Intensity
In $^{40}\mathrm{K}$ branched decay:
1. The total decay constant is $\lambda_{\mathrm{total}} = \lambda_{\beta^-} + \lambda_{\mathrm{EC},\gamma} + \lambda_{\mathrm{EC,0}}$, where $\lambda_{\mathrm{EC},\gamma}$ is the transition rate to the $1460.822\mathrm{ keV}$ $2^+$ excited state of $^{40}\mathrm{Ar}^*$.
2. The excited $^{40}\mathrm{Ar}^*$ level de-excites via E2 gamma-ray emission (probability $I_\gamma$ per $^{40}\mathrm{K}$ decay) or internal conversion plus internal pair creation (total coefficient $\alpha_T = 10.28(15) \times 10^{-5} = 1.028 \times 10^{-4}$, Mougeot May 2025 DDEP evaluation), so:
   $$P_{\mathrm{EC}^*} = \frac{\lambda_{\mathrm{EC},\gamma}}{\lambda_{\mathrm{total}}} = I_\gamma (1 + \alpha_T)$$
3. Evaluated nuclear-data compilations report distinct values for $I_\gamma$ across evaluation editions:
   - **DDEP May 2025 (`K40_GAMMA_DDEP_2025`, Mougeot 2025, BIPM/LNHB):** $I_\gamma = 10.34 \pm 0.07\%$ ($0.1034 \pm 0.0007$), $P_{\mathrm{EC}^*} = 10.34 \pm 0.07\%$ ($0.10341063$ from $I_\gamma(1+\alpha_T)$, rounding to $10.34\%$), $\alpha_T = 10.28(15) \times 10^{-5}$ (including internal pair creation).
   - **DDEP 2010 Historical (`K40_GAMMA_DDEP_2010`, Chiste & Be 2010, *Monographie BIPM-5* Vol. 5):** $I_\gamma = 10.55 \pm 0.11\%$ ($0.1055 \pm 0.0011$).
   - **IAEA / ENSDF 2017 (`K40_GAMMA_IAEA_ENSDF`, Chen 2017, *Nuclear Data Sheets* 140:1-364):** $I_\gamma = 10.66 \pm 0.13\%$ ($0.1066 \pm 0.0013$).
4. **Rules (`RT-01` & `RT-02`):**
   - Models must store $P_{\mathrm{EC}^*}$ (`branch_ec_gamma_frac`) and $I_\gamma$ (`photon_emission_prob` in `K40GammaEmissionConvention`) as separate, provenance-bearing fields and never mislabel the May 2025 DDEP value ($0.1034$) as a 2004 evaluation or force $P_{\mathrm{EC}^*}$ and $I_\gamma$ to differ after display rounding.
   - When comparing a geochronology fixture's point branch ratio $P_{\mathrm{EC}^*}^{\mathrm{fixture}}$ against a photon convention's $P_{\mathrm{EC}^*}^{\mathrm{convention}} = I_\gamma(1+\alpha_T)$ without a full fixture covariance matrix, the ratio $|P_{\mathrm{EC}^*}^{\mathrm{fixture}} - P_{\mathrm{EC}^*}^{\mathrm{convention}}| / \sigma(I_\gamma)$ is a **one-sided convention-uncertainty-normalized engineering acceptance rule** (default threshold `2.0`), not a calibrated two-sample statistical significance test.

### 5.2 Reaction Kinetics: Competing Precursor Hazards vs. Sequential Product Hydrolysis
The force-clamped Boltzmann expression $P_{\mathrm{syn}} = \min(1, \exp(-(\Delta G^\ddagger - F\Delta x^\ddagger)/(k_B T)))$ (which yields $P_{\mathrm{syn}} = 1.0$ for all $F \ge \Delta G^\ddagger / \Delta x^\ddagger = 0.625\mathrm{ nN}$) is **rejected** as a calibrated reaction probability. Candidate kinetic models must distinguish two distinct reaction topologies:

1. **Topology A - Competing First-Order Precursor Hazards (`competing_precursor_hazards`):**
   An activated monomer precursor $M^*$ undergoes either phosphodiester bond synthesis (rate $k_{\mathrm{syn}}$) or competing precursor hydrolysis/deactivation (rate $k_{\mathrm{deact}}$) into an unreactive state:
   $$P_{\mathrm{syn}}(\Delta t) = \frac{k_{\mathrm{syn}}}{k_{\mathrm{syn}} + k_{\mathrm{deact}}} \left[ 1 - \exp\!\big(-(k_{\mathrm{syn}} + k_{\mathrm{deact}})\Delta t\big) \right]$$
2. **Topology B - Sequential Synthesis Followed by Product Hydrolysis (`sequential_product_hydrolysis`):**
   Precursor $M$ polymerizes into product $P_{\mathrm{chain}}$ at rate $k_{\mathrm{syn}}$, and the already-synthesized product $P_{\mathrm{chain}}$ is degraded by hydrolysis at rate $k_{\mathrm{hyd}}$ ($M \xrightarrow{k_{\mathrm{syn}}} P_{\mathrm{chain}} \xrightarrow{k_{\mathrm{hyd}}} D_{\mathrm{deg}}$). The intact product fraction at exposure time $\Delta t$ is given by the two-state Bateman solution:
   $$f_{\mathrm{product}}(\Delta t) = k_{\mathrm{syn}} \, \frac{\exp(-k_{\mathrm{syn}}\Delta t) - \exp(-k_{\mathrm{hyd}}\Delta t)}{k_{\mathrm{hyd}} - k_{\mathrm{syn}}}, \qquad (\text{with } f_{\mathrm{product}}(\Delta t) = k_{\mathrm{syn}}\Delta t \exp(-k_{\mathrm{syn}}\Delta t) \text{ when } k_{\mathrm{syn}} = k_{\mathrm{hyd}})$$
   with precursor fraction $f_{\mathrm{precursor}}(\Delta t) = \exp(-k_{\mathrm{syn}}\Delta t)$ and degraded product fraction $f_{\mathrm{degraded}}(\Delta t) = 1 - f_{\mathrm{precursor}}(\Delta t) - f_{\mathrm{product}}(\Delta t)$.

Any Bell-Evans mechanochemical rate modulation $k_{\mathrm{syn}}(\mathbf{F}, T) = k_0(T)\exp((\mathbf{F}\cdot\Delta\mathbf{x}^\ddagger)/(k_B T))$ requires vector projection $\mathbf{F}\cdot\Delta\mathbf{x}^\ddagger$ onto the reaction coordinate and carries `evidence_classification = "uncalibrated_candidate_kinetic_law"`.

### 5.3 Radiolysis Energy-Deposition & Dose-Basis Conventions
To prevent dimensional inconsistency and double-counting of water energy-partition factors, every radiolysis calculation must declare one of two mutually exclusive `dose_basis` conventions:

1. **`pore_water_absorbed_dose_rate`:**
   - Input $\dot{D}_w$ ($\mathrm{Gy/s} = \mathrm{J/(kg_w\cdot s)}$) is already the ionizing dose rate absorbed by the pore water.
   - Deposited energy rate in pore water per unit bulk porous volume:
     $$q_{\mathrm{abs},w} = \varepsilon \, \rho_w \, \dot{D}_w \quad [\mathrm{W/m^3_{bulk}}]$$
   - Pore-water radiolytic production rate:
     $$\left.\frac{\partial [\mathrm{H}_2]_w}{\partial t}\right|_{\mathrm{rad}} = \rho_w \, \dot{D}_w \, G(\mathrm{H}_2) \quad [\mathrm{mol/(m^3_w\cdot s)}]$$
   - **Invariant:** A separate water energy-partition factor $f_w$ must **not** be applied under `pore_water_absorbed_dose_rate` (`water_energy_partition_factor_applied` must be `false`).
2. **`bulk_medium_absorbed_dose_rate`:**
   - Input $\dot{D}_{\mathrm{bulk}}$ ($\mathrm{Gy/s} = \mathrm{J/(kg_{bulk}\cdot s)}$) is the dose rate averaged over the bulk fluid-plus-mineral porous medium of bulk density $\rho_{\mathrm{bulk}} = (1 - \varepsilon)\rho_s + \varepsilon\rho_w$.
   - Requires an independently derived and provenance-documented mass-stopping-power / energy-absorption partition fraction $f_w \in (0, 1)$:
     $$q_{\mathrm{abs},w} = \rho_{\mathrm{bulk}} \, f_w \, \dot{D}_{\mathrm{bulk}} \quad [\mathrm{W/m^3_{bulk}}]$$

Unit conversion factor: $1\text{ molecule}/100\text{ eV} = \frac{1}{100 \times 1.602176634 \times 10^{-19}\text{ J} \times 6.02214076 \times 10^{23}\text{ mol}^{-1}} = 1.036426965 \times 10^{-7}\text{ mol/J}$.

---

## 6. Separation of Physical Research Tracks & Repository Reality Audit

### 6.1 Track A (Mica Hypothesis & Prebiotic Chemistry) vs. Track B (Oklo & Subsurface Radiolytic Ecology)

| Dimension | Track A: Mica Hypothesis (Prebiotic Chemistry) | Track B: Oklo Natural Reactors & Subsurface Radiolysis | Ecological Analogue: *Ca. Desulforudis audaxviator* |
| --- | --- | --- | --- |
| **Chronological Window** | Hadean / Eoarchean ($\ge 3.8\text{ to }3.4\text{ Ga}$) | Paleoproterozoic ($\sim 1.95\text{ Ga}$, Franceville Basin, Gabon; postdates life's origin by $\sim 1.5\text{ Ga}$) | Modern deep continental aquifers ($2.0\text{ to }2.8\text{ km}$ depth, South Africa MP104 & Western Siberia BYF) |
| **Primary Scientific Question** | Whether potassium-rich micaceous interlayers ($a_{\mathrm{lattice}} = 0.5\mathrm{ nm}$ vs. fluid gap $h_{\mathrm{gap}} \in [1, 100]\mathrm{ nm}$), divalent $\mathrm{Mg}^{2+}$ bridging, and wet-dry / mechanical cycling promote nucleotide oligomerization (Hansma 2010, 2022; Song et al. 2024; Simonis et al. 2026) | How localized fission heat, water radiolysis ($q_{\mathrm{abs},w}$), and phyllosilicate retardation ($R$) govern radionuclide migration ($^{135}\mathrm{Cs}, {}^{137}\mathrm{Cs}, {}^{90}\mathrm{Sr}$) and actinide decay ($^{239}\mathrm{Pu} \to {}^{235}\mathrm{U} \to {}^{207}\mathrm{Pb}$) (Draganic et al. 1983) | How energy-limited subsurface microbes sustain chemolithoautotrophic ($\mathrm{H}_2 + \mathrm{SO}_4^{2-}$) vs. organotrophic/mixotrophic growth ($28.5\mathrm{ h}$ lab doubling time in strain BYF, Karnachuk et al. 2019) |
| **Prohibited Conflation** | Do not equate $0.5\mathrm{ nm}$ crystal periodicity with interlayer fluid thickness; do not treat $\mathrm{K}^+$ as a universal nucleic-acid adhesive where $\mathrm{Mg}^{2+}$ is required | Do not treat Oklo ($\sim 1.95\mathrm{ Ga}$) as the site of initial abiogenesis or claim that Oklo fission occurred inside nanoscale biotite sheets | Do not treat conserved bacterial housekeeping/stress genes (`kdpABC`, `recA`, `mutSL`, `cooS`, `acsB`, `gvp`) as genetic proof of a biotite abiogenesis lineage |

### 6.2 Audited Repository Capabilities vs. Unimplemented Claims

| Repository & Verified Commit | Actual Implemented Capabilities | Claims Audited as NOT Present in Repository Source |
| --- | --- | --- |
| `aurascoper/RadioPharmacology.jl` (`c11241b59bcb2ebca5317883240020a848c22773`) | $^{40}\mathrm{K}$ decay fixtures (`K40_STEIGER_JAEGER_1977`, `K40_RENNE_2011`), separated $1460.822\mathrm{ keV}$ gamma conventions (`K40_GAMMA_DDEP_2025`, `K40_GAMMA_DDEP_2010`, `K40_GAMMA_IAEA_ENSDF`), porous-media retardation surrogate $R$, reduced 3-state/4-state and expanded 8-state Bateman fission/actinide fate, relativistic radiohalo recoil, conditional Currie HPGe MDA | Does not model RNA polymerization, mechanochemical bond synthesis, or forward Monte Carlo detector transport |
| `aurascoper/oklo_pool_mobile` (`942e3f8d02a3b73cfb26f099dbb36c4a908f6764`) | Revision-0 6-field prescribed analytic fixture (`oklo_core`), 1D Darcy flow and synthetic U(VI) reduction-transport column, R10 4D synthetic tracer viewer, canonical `INTERFACE_FALSIFICATION_BRIDGE_v0` contract | Revision-0 fields are prescribed analytic functions, not coupled physical or prebiotic simulations |
| `aurascoper/Biofilms` (`origin/master` `c766e7be31144f707155416b788bf7d1ba473ab9`) | Overdamped Langevin / Cellular Potts biofilm models, OpenMC radiation coupling, read-only `observer.py`, and fail-closed `feedback_gate.py` (`EffectPolicy` requiring predeclared `effect_threshold`) | Does not contain `biotite_properties.toml`, `sub_lattice.json`, or a calibrated RNA-lipid biotite pinching pipeline |
| `aurascoper/rna_biofilm_toy` (`b780a1a7f6113f3b8454cad5efbc7dd66b76a25a`) | Standalone $50 \times 50$ square-grid cylinder lattice (`engine.py`) with assumed shielding rule, monomer pool, and preregistered run-pair contrasts (`PREDICTION.md`) | Explicitly contains **no RNA, no polymerization, no chemistry**, no $0.5\mathrm{ nm}$ grid spacing (`dx = 0.5e-9`), and no hexagonal biotite lattice |
