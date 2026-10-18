# Darcy pre-registration v1, Amendment A2: the two roles of porosity

Status: frozen on 2026-10-17 at the commit that adds this file. Not approved for implementation. It amends Darcy pre-registration v1 (docs/specs/DARCY_FLOW_PREREG_v1.md, sha256 e0a13e103f090ac6adacdc249a6d75ed99f56e14577d726490b24aa0a7f2e1ca) and Amendment A1 (docs/specs/DARCY_FLOW_PREREG_v1_AMENDMENT_A1.md, sha256 39a7ae88b7861f065e02488076e3f52f4a84709b744a433383f469e5df1c55db), both unchanged. It changes no numerical value, equation, case, check, tolerance, pass criterion or output. Like v1 and A1, it authorizes no solver code, no parameter fitting and no claim about actual Oklo conditions.

## A2.1 Why

A1 used porosity in two different roles but called both "porosity". A1.2 bounds the domain-wide thermal conductivity λ_m with a porosity span of 0.10 to 0.25, and λ_m is in the denominator of the Péclet number, so that porosity affects the reported Pe range. A1.3 then said porosity enters only the pore velocity and does not enter Pe, which is true only of the zone porosities used for pore velocity. Hunter's review of 2026-10-17 found the inconsistency. A2 names the two roles as separate parameters.

## A2.2 Two parameters

- **Thermal-property porosity, φ_th.** The span 0.10 to 0.25 used only in A1.2 to bound λ_m through Robertson's equation (6). It enters Pe through λ_m, at the low, representative and high values of A1.2. It enters nothing else.
- **Flow porosity, φ_z, one value per zone.** Core 0.125 (the declared representative of the sourced 10 to 15 percent), sandstone 0.15, halo 0.06 and conduit 0.25 (the last three unsourced). It is used only for the pore velocity v = |q| / φ_z of v1 section 7. It enters neither the Darcy head nor the Darcy flux, and it does not appear in the Pe formula.

The φ of v1 section 2 ("Mean pore velocity v = q / φ, with φ the porosity. Porosity enters nothing else: not the head field and not the Darcy flux.") is the flow porosity φ_z. Read that way, the v1 sentence is correct and stays unedited.

## A2.3 Corrections to A1.3

A1 line 65 reads:

> Porosity enters only the pore velocity of v1 section 7. Pore velocities are reported, but never described as physically validated while those assumptions remain. None of this blocks the head or Darcy-flux calculation, and porosity does not enter Pe.

Two of its clauses are superseded:

- "Porosity enters only the pore velocity of v1 section 7." becomes "Flow porosity φ_z enters only the pore velocity of v1 section 7."
- "and porosity does not enter Pe." becomes "flow porosity φ_z does not enter Pe; thermal-property porosity φ_th enters Pe through λ_m (A1.2)."

The middle sentence stands. Nothing else in A1 changes. A1.2's declared input "Porosity: 0.10 to 0.25" is φ_th.

## A2.4 Coupling rule

A1 drew the φ_th span from the v1 flow porosities of the sandstone-family zones: core 0.10 to 0.15, sandstone 0.15, conduit 0.25. That shared origin is a one-time declaration, not a link. A future change to any φ_z does not change φ_th, and a change to φ_th does not change any φ_z, unless an amendment says so explicitly.

## A2.5 What stays the same

λ_m stays 1.12523, 2.73931 and 6.66867 W/(m K) (low, representative, high). The water states, the eleven configurations, checks D1 to D5, their tolerances and the outputs are unchanged. The limitations of A1 stand: no temperature or pressure correction to λ_m, one domain-wide sandstone λ_m, and three unsourced flow porosities.

## A2.6 Labels on outputs

Every reported Pe states that its range comes from λ_m and therefore includes the φ_th span of 0.10 to 0.25 as well as the quartz span of 0 to 100 percent. Every reported pore velocity names the φ_z it used and whether that value is sourced.
