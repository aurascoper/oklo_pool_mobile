# Darcy pre-registration v1, Amendment A1: thermal Péclet policy and porosity statements

Status: frozen on 2026-10-17 at the commit that adds this file. Not approved for implementation. It amends Darcy pre-registration v1 (docs/specs/DARCY_FLOW_PREREG_v1.md, sha256 e0a13e103f090ac6adacdc249a6d75ed99f56e14577d726490b24aa0a7f2e1ca), which stays frozen and unedited. A1 changes no v1 equation, parameter, case, check or tolerance. It only fills v1 section 7's open item, the thermal Péclet number, and records two porosity decisions. Like v1, it authorizes no solver code, no parameter fitting and no claim about actual Oklo conditions.

## A1.1 Thermal Péclet number

Definition, the porous-media form chosen by Hunter on 2026-10-17:

Pe = ρ_w c_p,w |q| L / λ_m

- q: the cell-centred Darcy flux of v1 section 7, magnitude in m/s.
- L = 0.5 m, labelled "voxel scale". Pe is linear in L; only the voxel scale is reported.
- ρ_w c_p,w: the volumetric heat capacity of the pore water at the water state of the case, from iapws 1.5.5 (IAPWS-95).
- λ_m: the effective thermal conductivity of the water-saturated rock, not of water alone (section A1.2).

This is the ratio of advective to conductive heat transport in the heat equation for flow through porous rock, where heat is carried by the Darcy flux at the water's heat capacity and conducted through the saturated medium. The diffusivity it implies is α* = λ_m / (ρ_w c_p,w). That is not the bulk diffusivity of the saturated rock, λ_m / (ρc)_m, which governs heat storage and is not used here; the two differ by the ratio of the bulk to the water volumetric heat capacity.

Reporting, for every configuration of v1 section 3.3 at both water states A and B: the maximum of Pe over all cells, the maximum over conduit cells, and the median over all cells, each at the low, representative and high λ_m of section A1.2.

Pe is a screening diagnostic. It is not a heat-advection simulation, it has no pass or fail threshold, and no value of it validates anything about Oklo.

## A1.2 The rock property

Source: Robertson, E.C. 1988. Thermal properties of rocks. U.S. Geological Survey Open-File Report 88-441. The relation is Robertson's equation (6), report page 10:

K = K_F + γ² [(K_S + p S) - K_F]

with γ = 1 - φ the solidity, p the quartz content in volume percent, and, for sandstone with water in the pores (figure 10, report page 22), K_F = 1.48 CU, K_S = 3.63 CU, S = 0.157 CU per percent quartz. 1 CU = 10^-3 cal/(cm s C) = 0.4184 W/(m K) (report page 5). The data behind figure 10 are measurements on water-saturated sandstones at 300 K and 5 MPa by the authors Robertson lists on page 22. Robertson states no numerical uncertainty; he gives the scatter of the plotted points about the isopleths as the measure of uncertainty (page 10). Equation (6) with S = 0.157 gives 19.33 CU at γ² = 1 and 100 percent quartz, against the tabulated intercept 19.35 CU; the equation, as Robertson instructs, is used.

Declared inputs, set before any Pe is computed:

- Porosity: 0.10 to 0.25, the span v1 uses for its sandstone-family zones (core 0.10 to 0.15 from Gauthier-Lafaye et al. 1996, sandstone 0.15 and conduit 0.25 unsourced).
- Quartz content: 0 to 100 percent. No source read gives the quartz content of the FA sandstone at Oklo. Gauthier-Lafaye et al. 1996 (p. 4834) note that criticality was easily reached where the sandstones had already lost some silica, which supports keeping the low-quartz end.
- One domain-wide λ_m for all cells, the water-saturated sandstone analogue. Per-zone conductivities, including clay for the halo, are deferred.

Resulting range and representative value, by the geometric-mean rule of v1:

| | Porosity | Quartz | λ_m (W/(m K)) |
| --- | --- | --- | --- |
| Low | 0.25 | 0 percent | 1.12523 |
| High | 0.10 | 100 percent | 6.66867 |
| Representative | | | 2.73931 |

Water heat capacity at the v1 states (iapws 1.5.5, 30 MPa):

| State | T | ρ_w (kg/m³) | c_p,w (J/(kg K)) | ρ_w c_p,w (J/(m³ K)) | α* at representative λ_m (m²/s) |
| --- | --- | --- | --- | --- | --- |
| A | 323.15 K | 1000.666795 | 4118.0976 | 4.120844e6 | 6.6474e-7 |
| B | 573.15 K | 750.658667 | 5080.3138 | 3.813582e6 | 7.1830e-7 |

State A's c_p,w equals the oklo_heartbeat water_300bar.csv row at 323.15 K to every printed digit.

Limitations, stated with every reported Pe:

- λ_m comes from measurements at 300 K and 5 MPa. It is applied without correction at 323 K and at 573 K and 30 MPa. Robertson (report page 37) finds the temperature effect small and of either sign for porous rocks with K below 5 CU at temperatures below 200 C, a marked negative effect for dense quartz-rich rocks, and calls extrapolation to higher temperature uncertain. State B lies outside that range, so its Pe carries an unquantified temperature error.
- The quartz and porosity spans are wide because neither is known for the Oklo sandstone; the low-to-high range of λ_m spans a factor of about 5.9, and Pe inherits it.
- λ_m is the sandstone analogue everywhere, including the halo, where clay would conduct differently.

Blocked rule: had no numeric water-saturated sandstone conductivity been readable, Pe would be reported as blocked pending a sourced value, with no textbook number substituted. That rule stays in force for any future change of λ_m.

## A1.3 Porosity statements

- Core porosity stays at 0.125, the declared representative (arithmetic midpoint) of the 10 to 15 percent open porosity sourced from Gauthier-Lafaye et al. 1996, p. 4834, citing Naudet 1991. It is a porosity, not a permeability measurement.
- The sandstone (0.15), halo (0.06) and conduit (0.25) porosities stay flagged as unsourced revision 0 registry values.
- Porosity enters only the pore velocity of v1 section 7. Pore velocities are reported, but never described as physically validated while those assumptions remain. None of this blocks the head or Darcy-flux calculation, and porosity does not enter Pe.

## A1.4 Sources

Read: Robertson 1988, USGS Open-File Report 88-441 (pages 5, 10, 22 and 37 as cited above; public domain); Gauthier-Lafaye, Holliger and Blanc 1996, Geochimica et Cosmochimica Acta 60(23), 4831 to 4852 (page 4834); iapws 1.5.5.

Named by the review, not read and not used for any value: Hanley, DeWitt and Roy 1978, The thermal diffusivity of eight well-characterized rocks for the temperature range 300 to 1000 K (measured at atmospheric pressure, as the review notes); Seipold and Raab 2000, A method to measure thermal conductivity and thermal diffusivity under pore and confining pressure (abstract only); ASTM D4612-16, which the review reports as withdrawn in 2025, historical method guidance only.

## A1.5 What remains open

Hunter's decision on implementing v1 together with A1. A temperature-dependent or zone-specific λ_m, if wanted later, needs its own amendment and sources.
