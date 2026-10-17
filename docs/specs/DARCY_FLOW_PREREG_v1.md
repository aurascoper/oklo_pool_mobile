# Darcy flow on the Oklo fixture geometry: pre-registration v1

Status: frozen on 2026-10-17 at the commit that adds this file. Not approved for implementation. This document authorizes no solver code, no parameter fitting and no claim about actual Oklo conditions. It is versioned separately from the scientific-model revision 1 of the fixture, which will handle the pulse-boundary choice and the pulse discontinuity at p = b; nothing here touches the fixture.

Verification is not validation. Checks D1 to D5 below can show that a solver implements these equations correctly. Passing them would say nothing about whether the permeabilities, the head difference, the geometry or the scenario represent the Oklo reactors, past or present. Every output of an implementation must carry that statement.

This version supersedes DARCY_FLOW_PREREG_DRAFT.md (git history keeps it). It resolves the five review points of 2026-10-17 and Hunter's four corrections:

| Point | Where it is resolved |
| --- | --- |
| 1. Complete, sourced parameter table | Section 3 |
| 2. Exact numerical problem: boundary placement, Δh | Section 4 |
| 3. Well-defined conservation criterion, zero-flow case, solver residual reported apart from mass balance | Sections 5 and 6, D1 |
| 4. Explicit manufactured solution, source term, error norm, order, and interface scope | Section 6, D2 and D3 |
| 5. Exact D5: zone mask, faces summed | Section 6, D5 |
| C1. Intrinsic permeability k as the material property; K derived per water state | Sections 2 and 3 |
| C2. Manufactured source s_m and the residual sign convention | Sections 4 and 6, D2 |
| C3. D5 over all N layers including the top face; D4 denominators | Section 6, D4 and D5 |
| C4. Solver tolerance reconciled with benchmark tolerances | Sections 5 and 6, D1 and D3 |

## 1. Question

Revision 0 of the fixture has no flow: its conduit is a Gaussian indicator, hydration a prescribed shape, and the steam chimney a zone label. The question is the smallest one that adds real fluid mechanics: on the fixture's compartment geometry, with declared material properties, what steady groundwater flow does a declared head difference drive, and how much of it does the conduit carry? Groundwater at these conditions is a Newtonian fluid, so Darcy's law is the first model. Heat advection, boiling and non-Newtonian behaviour are out of scope.

## 2. Governing equations

- Darcy flux: q = -K ∇h, with h the hydraulic head in m and q in m/s.
- Conservation: ∇·q = s, with s a volumetric source in 1/s. Here s = 0 everywhere except in the manufactured-solution check D2, where s = s_m.
- Hydraulic conductivity: K = k ρ g / μ, in m/s.

The material property of each zone is the intrinsic permeability k, in m². K is derived from k separately for each water state in section 3; a tabulated K is never used as k. A source that reports K is recorded with the fluid conditions behind it, and it enters this document only as context unless the conversion assumptions are stated beside it.

Mean pore velocity v = q / φ, with φ the porosity. Porosity enters nothing else: not the head field and not the Darcy flux.

## 3. Parameters

### 3.1 Zones and material properties

Zones come from the fixture's compartment rule (CONFORMANCE_FIXTURE_SPEC.md section 3) evaluated at cell centres in the null scenario, where the desiccation term is 0: C > 0.45 gives the core; otherwise F > 0.35 gives the conduit; otherwise H > 0.35 gives the halo; otherwise sandstone. Zone 4 cannot occur in the null scenario.

No source read for this document gives a permeability for any of the four zones at Oklo. Every k below is therefore a generic material analogue, not an Oklo measurement, digitized from the Groundwater Project's Figure 32 (section 5.3 of Hydrogeologic Properties of Earth Materials and Principles of Groundwater Flow, 2020, after Freeze and Cherry 1979) on its intrinsic-permeability axis. The digitization is reproducible: the 14 decade ticks of the k axis fit x = 1652.11 + 88.771 log10(k in cm²) on the 1417 by 792 pixel image with a worst residual of 1.23 px (0.014 decade); bar ends are the outermost pixels of each bar. The receipt records the script and its output.

Representative-value rule, declared before any run: for a permeability range, the geometric mean of its two endpoints (the log-midpoint); for a porosity range, the arithmetic midpoint. Representative values are computed from the full-precision digitized endpoints in the receipt and shown here to three significant figures.

| Zone | Analogue class in Figure 32 | log10 k range (m²) | Representative k (m²) | Porosity φ | Porosity source and tier |
| --- | --- | --- | --- | --- | --- |
| Sandstone (0) | Sandstone | -17.07 to -12.72 | 1.27e-15 | 0.15 | Revision 0 registry value, described there as an engineering estimate with no citation: assumption, unsourced |
| Reactor core (1) | Sandstone, the same analogue as zone 0: assumption, since the core is mineralised, fractured sandstone and no core permeability was found | -17.07 to -12.72 | 1.27e-15 | 0.125 (range 0.10 to 0.15) | Open porosity of 10 to 15 percent required for criticality in fractured mineralised sandstone (Naudet 1991, cited in Gauthier-Lafaye et al. 1996, p. 4834): sourced, secondary citation |
| Clay and phosphate halo (2) | Shale, the closest lithified clay class | -20.18 to -16.05 | 7.65e-19 | 0.06 | Revision 0 registry value: assumption, unsourced |
| Fracture conduit (3) | Fractured igneous and metamorphic rocks, a generic fractured-rock class; the host lithology differs | -16.89 to -10.74 | 1.54e-14 | 0.25 | Revision 0 registry value: assumption, unsourced |

The unsourced porosities change only the reported pore velocity, never the head or the Darcy flux.

### 3.2 Water states

Two homogeneous water states, each a separate case, computed with the iapws package 1.5.5 (density from IAPWS-95, viscosity from the IAPWS 2008 viscosity formulation as implemented there), g = 9.80665 m/s²:

| State | T | P | ρ (kg/m³) | μ (Pa s) | ρg/μ (1/(m s)) | Phase reported by iapws |
| --- | --- | --- | --- | --- | --- | --- |
| A | 323.15 K (50 C) | 30 MPa | 1000.666795 | 5.528165e-4 | 1.775126e7 | compressible liquid |
| B | 573.15 K (300 C) | 30 MPa | 750.658667 | 9.325487e-5 | 7.893901e7 | compressible liquid |

ρ at state A equals the oklo_heartbeat water_300bar.csv row at 323.15 K to every printed digit. The heartbeat tables carry no viscosity, so μ is computed here, not read from them. For every zone, K = k ρg/μ at the state of the case. In this isothermal model the state multiplies every K by the same factor, so the head field and every flux share are identical in A and B; fluxes and pore velocities in B are about 4.447 times those in A (7.893901e7 / 1.775126e7). Both are reported.

Context, not used as parameters: Gauthier-Lafaye et al. 1996 (p. 4833) give reactor operating conditions of 120 to 150 C at 300 bar, citing Gauthier-Lafaye 1986. Toulhoat et al. 1996 (p. 8) report present-day hydraulic conductivities of 1e-8 to 1e-5 m/s for the hydraulic units at Okelobondo, a system they describe as strongly perturbed by mining, and effective values of 1e-8 to 1e-7 m/s at Bangombé. They state no fluid temperature. Converted with fresh water at an assumed 25 C and 1 atm (ρ = 997.05 kg/m³, μ = 8.900e-4 Pa s), these correspond to k of about 9.1e-16 to 9.1e-13 m², inside the upper part of the sandstone analogue range. Gauthier-Lafaye et al. 1996 also state, in their concluding section, that clay layers enclose the reactor cores and that the present permeability of the sandstone in the Oklo deposits is close to zero after diagenesis. Reactor-era permeabilities are not known from any source read here.

### 3.3 Cases, all reported, none chosen

- Base: every zone at its representative k.
- Conduit ratio sweep: conduit k = r × sandstone representative k, for r = 1, 10, 100, 1000; other zones at base. The base case corresponds to r of about 12.
- One-at-a-time endpoint cases: each analogue range at its low and then its high endpoint, the others at base. Core and sandstone share one analogue, so they move together. Six cases.

That is eleven configurations, each solved once and reported at water states A and B.

## 4. Numerical problem

- Domain [0, 16 m]³, N = 32 cells per side, Δx = 0.5 m, cell-centred finite volumes, x fastest, cell (i, j, k) centred at ((i + 1/2)Δx, (j + 1/2)Δx, (k + 1/2)Δx).
- Interior face conductance between neighbours i and j: T_ij = (2 K_i K_j / (K_i + K_j)) Δx, the harmonic mean over the two half-cells times face area Δx² over centre distance Δx.
- Boundary faces: Dirichlet values sit on the outer faces, not at cell centres, so the gradient spans exactly L = 16 m. Bottom face z = 0: h = Δh. Top face z = L: h = 0. The boundary conductance is T_b = 2 K_c Δx, half a cell from centre to face. This differs deliberately from rna_biofilm_toy spec 12, which places its Dirichlet values at boundary cell centres.
- Side faces x = 0, x = L, y = 0, y = L: zero flux.
- Δh = 1 m. The problem is linear in Δh, so every flux is also reported per unit mean gradient Δh / L = 1/16.
- Face flux and sign: F_ij = T_ij (h_i - h_j) is the volumetric flux from cell i to cell j in m³/s, so a face flux is positive along +x, +y or +z when it flows that way. A boundary face carries T_b (h_boundary - h_c) into the cell.
- Cell residual, the single convention used everywhere: r_c = (net outward flux of cell c) - s_c Δx³, where s_c is the source at the cell centre. With s = 0 this is the net outflow. In matrix form the scheme is A h = b, with A symmetric positive definite and b holding the boundary terms T_b Δh of the bottom layer plus s_c Δx³, and r = b - A h exactly: the linear-solver residual vector is the cell mass-imbalance vector, in m³/s.

## 5. Solvers

- Fixture runs and D2, D4, D5: conjugate gradient in float64, no preconditioner, initial guess the linear profile h = Δh (1 - z_c / L), stopping when ||b - A h||_2 ≤ 1e-12 ||b||_2, at most 10 N³ iterations. Not converging within the limit is a reported failure. Iteration count and final relative residual are reported for every solve.
- D3 additionally uses a direct solve: dense Cholesky factorization in float64 of the 512-unknown problems. It verifies the discretization independently of iterative convergence.

Reconciling tolerances (C4). Because r = b - A h, the CG stopping rule bounds the D1 mass-balance error: ||r||_∞ ≤ ||r||_2 ≤ 1e-12 ||b||_2, so ε (section 6, D1) ≤ 1e-12 ||b||_2 / Q_ref. For uniform K that ratio ||b||_2 / Q_ref is about 2. For heterogeneous K it is computed and reported for every run. D1's threshold of 1e-10 is declared now and is not adjusted whatever the ratio turns out to be. The D3 flux tests are a different quantity from the CG residual, so they are split: 1e-12 against the direct solve, which tests the method, and 1e-10 against CG, which tests that the declared stopping rule is adequate. D4 and D5 use 1e-10 for the same reason.

## 6. Checks, pass criteria declared now

No parameter, tolerance, case or check is changed after any result is seen. A failure is reported as a failure.

- **D1, conservation.** Let Q_in be the total flux entering through the bottom face and Q_out the total leaving through the top face, and Q_ref = |Q_in|. Report ||r||_∞ and ||r||_2 in m³/s. Pass when Q_ref > 0: ε = ||r||_∞ / Q_ref ≤ 1e-10, and |Q_in - Q_out| / Q_ref ≤ 1e-10. The residual is computed twice, once as b - A h from the matrix and once by summing separately coded face fluxes, and the two must agree to max |r_face - r_matrix| / Q_ref ≤ 1e-12. Zero-flow control: with Δh = 0 and s = 0, b = 0 and the initial guess is 0, so the solve must return h identically 0 and r identically 0. Relative quantities are not computed when Q_ref = 0.
- **D2, manufactured solution, uniform K.** h_m(x, y, z) = Δh (1 - z/L) + A sin(πz/L) cos(πx/L) cos(πy/L), with A = 0.1 m, Δh = 1 m and uniform K = 1e-6 m/s. It meets the Dirichlet values on the top and bottom faces exactly and has zero normal derivative on the four side faces. Because ∇²h_m = -(3π²A/L²) sin(πz/L) cos(πx/L) cos(πy/L), the equation ∇·q = s_m with q = -K∇h_m needs s_m = +(3π²KA/L²) sin(πz/L) cos(πx/L) cos(πy/L), evaluated at each cell centre and multiplied by Δx³ in the residual of section 4. Error e_c = h_c - h_m(cell centre). Norms: E_2 = (Σ e_c² Δx³ / L³)^(1/2) and E_∞ = max |e_c|. Grids N = 16, 32 and 64. Observed order p = log2(E_2(N) / E_2(2N)) for (16, 32) and (32, 64). Pass: both p in [1.9, 2.1]. E_∞ orders are reported without a criterion. Scope: smooth solution, uniform K only. D2 says nothing about accuracy at zone interfaces.
- **D3, interfaces, N = 8 (Δx = 2 m).** Layer conductivities K_k = 1e-5, 1e-7, 1e-6, 1e-8, 1e-5, 1e-7, 1e-6, 1e-8 m/s.
  - D3a, layers in series (K varies with z only): with Dirichlet values on the outer faces and harmonic face conductances, the discrete resistance equals the analytic Σ_k Δx / K_k exactly, so Q_exact = L² Δh / Σ_k (Δx / K_k).
  - D3b, layers in parallel (K varies with x only): each column carries q_i = K_i Δh / L and lateral faces carry nothing, so Q_exact = Σ_i K_i (Δh / L) Δx L.
  - Pass for each: |Q - Q_exact| / Q_exact ≤ 1e-12 with the direct solve and ≤ 1e-10 with CG; cell-centre heads within 1e-12 Δh of the exact piecewise-linear profile with the direct solve.
  - No convergence order is claimed for general three-dimensional zone interfaces, where harmonic averaging is not exact. On the fixture geometry a refinement study reports, without a pass threshold, the relative change in total flux between N = 32 and N = 64 with each voxel's zone copied to its eight children.
- **D4, symmetry.** The zone mask is exactly mirror-symmetric under i → N-1-i and j → N-1-j, because the indicators depend on (i - 15.5) and (j - 15.5), which are exact in binary. In the base case: head error max |h_c - h_mirror(c)| / Δh ≤ 1e-10; flux error max |F_f - F_mirror(f)| / Q_ref ≤ 1e-10, with the sign of a face flux flipped when its normal is reversed by the mirror. No pointwise relative error is used.
- **D5, a control that can fire.** All four zones get the same k (the sandstone representative) at state A. For every layer k = 0, ..., N-1 take the N² z-faces above it: the interior faces between layers k and k+1 for k ≤ N-2, and the top boundary faces for k = N-1. Each face counts for the zone of the cell (i, j, k) below it, read from the zone mask, never from a rendered image. Conduit flux share in layer k: Σ F over faces whose cell is conduit, divided by Σ F over all N² faces of that layer. Area share: the number of conduit cells in layer k divided by N². Pass: |flux share - area share| ≤ 1e-10 in every layer; in layers without conduit both are exactly 0. Boundary inflow and outflow are checked separately by D1.

## 7. Outputs

For each configuration and water state: the head field; the three cell-centred Darcy flux components (each the mean of the cell's two opposite face fluxes divided by Δx²); and the pore-velocity magnitude |q| / φ, as MRC files meeting the CF-3 header table. A JSON receipt records every D1 to D5 quantity, iteration counts, final residuals, ||b||_2 / Q_ref, Q_in, Q_out, the conduit's share of top-face flux overall and per layer, and the refinement-study change. A thermal Péclet number is reported only after a thermal diffusivity for water-saturated rock is taken from a cited source; until then it is not reported.

## 8. Implementation named, not authorized

Rust, inside oklo_core, so that tests, app and web viewer share one engine: a module crates/oklo_core/src/darcy.rs with zero dependencies, float64 throughout; D1 to D5 as Rust tests in crates/oklo_core/tests/darcy.rs; an example that writes the outputs of section 7. None of this may be written until Hunter approves implementation separately.

## 9. What this does not claim

Nothing here models boiling, steam, two-phase flow, the pulse, reactivity feedback, heat advection, transport of any solute, or non-Newtonian behaviour, and nothing couples to the prescribed revision 0 fields. The head difference, the boundary scenario (upward flow through a box with closed sides) and the zone permeabilities are hypothetical. The outputs describe a hypothetical groundwater-flow scenario on the fixture's geometry, not the hydrogeology of the Oklo reactors at any time.

## 10. Sources

Read for this document:

- Toulhoat, P., et al. 1996. Preliminary studies of groundwater flow and migration of uranium isotopes around the Oklo natural reactors (Gabon). Journal of Contaminant Hydrology 21, 3 to 17. Used: present-day hydraulic conductivity ranges (p. 8, sections 3.1 and 3.2), context only.
- Gauthier-Lafaye, F., Holliger, P., and Blanc, P.-L. 1996. Natural fission reactors in the Franceville basin, Gabon: a review of the conditions and results of a "critical event" in a geologic system. Geochimica et Cosmochimica Acta 60(23), 4831 to 4852. Used: core open porosity 10 to 15 percent (p. 4834, citing Naudet 1991); operating conditions (p. 4833), context; qualitative statements on enclosing clays and present sandstone permeability.
- Zetterström, L. 2000. Oklo: a review and critical evaluation of literature. SKB Technical Report TR-00-17. Used: qualitative only (hydrofracturing of the FA sandstone beneath the FB shale cap, p. 12).
- The Groundwater Project 2020. Hydrogeologic Properties of Earth Materials and Principles of Groundwater Flow, section 5.3, Figure 32, after Freeze and Cherry 1979. Used: every permeability range, digitized as described in section 3.1. The original Freeze and Cherry table was not read.
- iapws 1.5.5 (Python package): density and viscosity at states A and B.

Cited by the reviewer as methodological precedent, not read here and not used for any value: arXiv 1911.10774 (benchmark for flow in heterogeneous groundwater formations); E3S Web of Conferences 205, 02002 (2020) (COMSOL and DuMuX single-phase verification benchmark).

## 11. Open before implementation approval

A cited thermal diffusivity, if a Péclet number is wanted; whether the core should keep the sandstone analogue; sources for the three unsourced porosities, if pore velocities are to be interpreted; and Hunter's approval of implementation.
