# A synthetic uranium reduction-transport column: pre-registration v1

Status: frozen on 2026-10-18 at the commit that adds this file. Specification only. No implementation is approved; model code needs Hunter's separate go-ahead after this freeze. It authorizes no parameter fitting and makes no claim about actual Oklo conditions or history.

This is a synthetic reaction-transport toy. It is not a reconstruction of uranium mineralization at Oklo or in the Franceville Basin, and it is not built on, and shares no code or evidence with, the rna_biofilm_toy spatial-protection model. That model's per-step transfer coefficient of 0.15 is a dimensionless lattice number, not a physical diffusion coefficient, and is not used here.

## 1. Geological motivation, bounded

The sources read for this document motivate a simplified mechanism; they do not validate any rate or parameter below.

- Gauthier-Lafaye, Holliger and Blanc 1996 (Geochimica et Cosmochimica Acta 60, page 4833): the uranium ore deposits of the Franceville basin are related to old petroleum traps, and mineralization occurred when oxidized uranium-bearing fluids met reducing conditions in hydrocarbon accumulations; in the common ore, uraninite sits in and around nodules of organic matter filling the secondary porosity of the sandstones.
- Gauthier-Lafaye 2002 (C. R. Physique 3, page 841): most of the uranium is uraninite closely associated with migrated hydrocarbons filling the sandstone's secondary porosity; mineralization occurred during diagenesis, at about 3000 m depth, when oxidised uranium-bearing fluids met reduced fluids associated with hydrocarbons in tectonic traps; the uranium-bearing fluids were hot (100 to 170 C) saline waters.

Named by Hunter's review and not read here, so not cited for any content: Lecomte et al. (multistage Franceville mineralization with oil cracking, fluid mixing and hydrofracturing) and Nagy et al. (organic matter reducing U(VI) to U(IV) at Oklo; cited within Gauthier-Lafaye et al. 1996).

What the toy leaves out, stated so it is not mistaken for coverage: temperature (the real fluids were 100 to 170 C), salinity and aqueous speciation, pH variation, hydrocarbon migration and cracking, fluid mixing, hydrofracturing, multiple stages, U(IV) solubility, radiolysis, heat, two-phase flow and any biology.

## 2. Model, version 1

One-dimensional column 0 ≤ x ≤ L_col of water-saturated sandstone with constant porosity θ, bulk density ρ_b and pore-water velocity v. State variables:

- C(x, t): dissolved U(VI), mol per m³ of pore water.
- S(x, t): uranium in an immobile U(IV)-bearing solid, mol per kg of solid.

Equations:

R ∂C/∂t = D_eff ∂²C/∂x² - v ∂C/∂x - k_red χ_org(x) C

ρ_b ∂S/∂t = θ k_red χ_org(x) C

- R = 1 + ρ_b K_d / θ, equilibrium linear sorption of U(VI) only, with K_d in m³/kg.
- χ_org(x) = 1 inside the stationary organic zone x_a ≤ x ≤ x_b, else 0.
- k_red: first-order rate, 1/s, of an effective conversion of dissolved U(VI) into the immobile solid.
- Units: each term of the first equation is mol m⁻³ s⁻¹ per unit pore volume; each term of the second is mol m⁻³ s⁻¹ per unit bulk volume, which is why θ multiplies the reaction there.

Declared modelling choices:

- Reduction and immobilization are one effective step. Version 1 has no dissolved U(IV) state and no separate precipitation rate; uranium leaves the dissolved pool only into S. A later version that resolves U(IV) in solution needs its own amendment.
- K_d applies to U(VI) only. The solid S does not sorb, desorb, dissolve or move.
- The organic zone is stationary and not consumed.

Conditions:

- Initial: C(x, 0) = 0 and S(x, 0) = 0.
- Inlet x = 0: Dirichlet, C = C0 for t > 0.
- Outlet x = L_col: zero diffusive gradient, ∂C/∂x = 0, so uranium leaves by advection only.

## 3. Discretization, declared

- Cell-centred finite volumes, N cells of width Δx = L_col / N.
- Advective face values by central averaging of the two neighbouring cells; at the inlet face the advective value is C0; at the outlet face it is the last cell's value (consistent with ∂C/∂x = 0).
- Diffusive fluxes by central differences; at the inlet, D_eff (C_1 - C0) / (Δx / 2).
- Cell Péclet number v Δx / D_eff ≤ 2 for every run. N is therefore max(64, ceil(Pe_s / 2)), a rule fixed here, not a tuned choice.
- Time integration by Crank-Nicolson, with Δt = 0.5 R Δx / v (a Courant number of 0.5 on the retarded velocity), so Δt ∝ Δx.
- S is advanced with the same Crank-Nicolson weighting of θ k_red χ_org C as the C equation, so that the ledger closes exactly in arithmetic.

Expected convergence order, derived independently before any run: central differences in space have truncation error O(Δx²) for smooth solutions, and Crank-Nicolson has O(Δt²); with Δt ∝ Δx the combined error is O(Δx²), an expected order of 2. The inlet half-cell difference is locally first order, which typically does not lower the global order for this cell-centred scheme (the Darcy v1 run showed second order with the same boundary treatment); if it does here, the benchmarks will show it and the result stands.

## 4. Parameters

Every value is either from a source read for this document or a declared assumption with a sweep. Nothing below is an Oklo measurement.

| Quantity | Value or sweep | Tier |
| --- | --- | --- |
| Column length L_col | 16 m | assumption, chosen to match the fixture's vertical extent for comparability only |
| Organic zone x_a to x_b | 6 m to 10 m (L_org = 4 m) | assumption |
| Inlet concentration C0 | 1 mol/m³ | normalization; the model is linear in C, so C and S scale with C0 |
| Porosity θ | 0.15 | assumption: the fixture's unsourced sandstone flow porosity (Darcy A2, φ_z) |
| Grain density | 2650 kg/m³, so ρ_b = (1 - θ) 2650 = 2252.5 kg/m³ | assumption, quartz-dominated sandstone, unsourced here |
| Pore-water velocity v | 1e-10, 1e-9, 1e-8 m/s | assumption. Context only: the Darcy v1 base case gives a domain-mean pore velocity near 3.8e-10 m/s at its hypothetical 1 m head difference |
| D_eff | 1e-11, 1e-10, 1e-9 m²/s | assumption, unsourced; no read source gives a uranyl diffusion or dispersion value |
| k_red | 0, 1e-11, 1e-10, 1e-9 1/s | assumption, unsourced; no read source gives a rate |
| K_d of U(VI) | 0; 0.063; 6.3; 630 m³/kg (0; 63; 6,300; 630,000 mL/g) | sensitivity bounds from EPA 402-R-99-004B, Table 5.17 (page 5.75), at an assumed pH of 7: the minimum, the geometric mean of minimum and maximum, and the maximum, plus 0 as a no-sorption baseline |

On K_d: the EPA report states (front matter) that generic or default partition coefficients can produce significant errors and that site-specific values are absolutely essential, and (page 5.75) that its maximum curve is biased high, possibly by an order of magnitude or more, partly because some measurements may include precipitation. These values therefore bound a sensitivity sweep for a synthetic column; they are not Oklo values, and pH 7 is itself an assumption.

The resulting retardation factors are R = 1, 947.05, 94,606 and 9,460,501 (to the digits shown, from R = 1 + 2252.5 K_d / 0.15).

The sweep is full factorial: 3 velocities × 3 D_eff × 4 k_red × 4 K_d = 144 runs. All are reported; none is selected as representative.

Run length: T_end = 5 R L_col / v, five retarded advective travel times. Outputs are written at 0.1, 0.25, 0.5, 1, 2 and 5 travel times. These are model times; at large R they exceed the age of the Earth, which says something about the assumptions, not about Oklo.

## 5. Dimensionless groups

- Solute Péclet number, Pe_s = v L / D_eff, reported with L = L_col = 16 m, and the cell Péclet number v Δx / D_eff. This is distinct from the thermal Péclet number of the Darcy work (Amendment A1), which uses the water's heat capacity and the rock's thermal conductivity; the two are not interchangeable.
- Damköhler number of the organic zone, Da = k_red L_org / v. Retardation slows transit through the zone by R but the reaction acts only on dissolved U(VI), so R cancels.

## 6. Checks, with pass criteria declared now

These verify the implementation of the equations above. None validates the model against nature.

- **B1, Ogata-Banks benchmark (a separate problem, not the production column).** Ogata and Banks 1961 (USGS Professional Paper 411-A, equation 12, page A-3, with α = x / (2√(Dt)) and ε = ux / (4D) defined on page A-2) give the solution for a semi-infinite medium, zero initial concentration and a constant concentration C0 at x = 0: C / C0 = ½ [erfc((x - vt) / (2√(Dt))) + exp(vx / D) erfc((x + vt) / (2√(Dt)))]. B1 solves exactly that problem: R = 1, k_red = 0, v = 1e-9 m/s, D = 1e-9 m²/s, on a long column L_B1 = 64 m, compared at t* = 1.6e10 s (front at 16 m, spread √(2Dt*) ≈ 5.66 m, so front plus six spreads is about 50 m, inside the column, declared so the far boundary does not affect the comparison window). Comparison window 0 to 32 m. Grids N = 256, 512 and 1024. The analytic value is evaluated stably through the scaled complementary error function to avoid overflow of exp(vx / D). Pass: the observed L2 order p = log2(E_N / E_2N) is in [1.8, 2.2] for both grid pairs. A step input at the inlet is a non-smooth start, and Crank-Nicolson is known to damp such starts weakly; if that lowers the order, B1 fails and stays failed.
- **B2, finite-domain steady state (the same boundary-value problem as the production column).** Uniform reaction (χ_org = 1 everywhere), v = 1e-9 m/s, D = 1e-9 m²/s, k = 1e-10 1/s, L = 16 m, Dirichlet C0 at x = 0 and zero gradient at x = L. Exact solution: C(x) = C0 [r2 e^(r2 L) e^(r1 x) - r1 e^(r1 L) e^(r2 x)] / [r2 e^(r2 L) - r1 e^(r1 L)], with r1,2 = (v ± √(v² + 4Dk)) / (2D). Solved as the discrete steady system (Δt → ∞), which tests the spatial discretization alone. Grids N = 64, 128 and 256. Pass: both observed L2 orders in [1.8, 2.2].
- **Ledger closure, every run.** Per unit cross-section: Σ θ R C_i Δx + Σ ρ_b S_i Δx - (cumulative inflow - cumulative outflow) = 0, where inflow at x = 0 is θ (v C0 - D_eff (C_1 - C0) / (Δx / 2)) and outflow at x = L_col is θ v C_N, both integrated with the scheme's Crank-Nicolson weighting. Pass: the absolute closure error is at most 1e-10 times the cumulative inflow at every output time after the first step.
- **Control C1, k_red = 0.** Expected: S is exactly 0 everywhere at all times, and the ledger closes with dissolved and sorbed uranium alone. Pass: max |S| = 0 exactly, and ledger closure as above.
- **Control C2, negative tests that must be detected.** Two test-only corrupted variants, never used for results: (a) the sign of the reaction term in the C equation flipped, so uranium is created in both pools; (b) the sign of the S source flipped. Detection rule, declared now: variant (a) is detected when the ledger closure error exceeds 1e-10 times the cumulative inflow; variant (b) when S is negative anywhere. C2 passes only if both variants are detected in a run with k_red = 1e-10 1/s; a corrupted variant that goes undetected is a failure of the checks.

## 7. Outputs

For each of the 144 runs: C(x) and S(x) at the six output times, the outlet breakthrough C(L_col, t), the ledger time series, Pe_s, the cell Péclet number, Da, N, Δt, and the total immobilized uranium. A JSON receipt marks every check as run or specified-only, with its status, and states that no CI ran. Every output carries the statement that this is a synthetic column, not a reconstruction of Oklo or the Franceville Basin.

## 8. Implementation named, not authorized

Rust, inside oklo_core, as a module with zero dependencies and f64 throughout, with B1, B2, the ledger and the controls as tests and the sweep as an example. None of it may be written until Hunter approves implementation separately.

## 9. Sources

Read for this document, copies hashed in receipts/uranium_prereg_v1_freeze.txt:

- Gauthier-Lafaye, F., Holliger, P., and Blanc, P.-L. 1996. Geochimica et Cosmochimica Acta 60(23), 4831 to 4852 (page 4833).
- Gauthier-Lafaye, F. 2002. C. R. Physique 3, 839 to 849 (page 841).
- U.S. EPA 1999. Understanding Variation in Partition Coefficient, Kd, Values. Volume II. EPA 402-R-99-004B (front matter; section 5.11.6.2 and Table 5.17, page 5.75).
- Ogata, A., and Banks, R.B. 1961. A solution of the differential equation of longitudinal dispersion in porous media. USGS Professional Paper 411-A (pages A-2 and A-3).

Named by the review, not read, not used for any content: Lecomte et al.; Nagy et al.
