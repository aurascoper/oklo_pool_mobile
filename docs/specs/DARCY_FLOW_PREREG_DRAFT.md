# Darcy flow on the Oklo fixture geometry: pre-registration draft

Status: draft written on 2026-10-17 at Hunter's request, for review. It authorizes no code, no parameter choice and no claim. Hunter decides where it lives, for example as a numbered spec in rna_biofilm_toy, and whether it is approved. Until then nothing here is run.

## 1. Why

Revision 0 of the fixture has no flow. Its "conduit" is a Gaussian indicator, "hydration" is a prescribed saturation shape, and "steam chimney" is a zone label applied where the prescribed desiccation term exceeds 0.4. No velocity, pressure or head is computed anywhere in Hunter's repositories (search of 2026-10-17). The question this draft asks is the smallest one that puts real fluid mechanics into the picture: given the fixture's compartment geometry and declared permeabilities, what steady groundwater flow results from a declared head difference, and how much of it does the conduit carry? Groundwater at these conditions is a Newtonian fluid, so Darcy's law for a Newtonian fluid is the appropriate first model. Non-Newtonian behaviour belongs to a later biofilm or clay lane, if anywhere.

A second question, deferred to an amendment, is whether advection of heat along that flow would change the temperature pattern in ways the prescribed field cannot show. The Péclet number reported in section 5 decides whether that amendment is worth writing.

## 2. Model, version 0

Steady, single-phase, incompressible, isothermal Darcy flow:

- ∇·(K ∇h) = 0 in the 16 m box, with hydraulic head h in metres
- K = k ρ g / μ, the hydraulic conductivity in m/s, from the permeability k in m² and the water density ρ and viscosity μ
- Darcy flux q = −K ∇h; mean pore velocity v = q / φ, with φ the porosity

Geometry: the revision 0 grid, 32 × 32 × 32 voxels of 0.5 m. Each voxel takes one zone from the fixture's compartment rule (spec section 3), evaluated in the null scenario so that the zones do not depend on the desiccation term. Zones 3 and 4 are both treated as conduit. Water properties: ρ and μ at one declared temperature and pressure, taken from oklo_heartbeat's IAPWS-95 tables (water_tables.py; 300 bar baseline). Temperature dependence of μ is deferred to an amendment.

Boundary conditions for version 0: a declared head difference Δh between the bottom face (k = 0) and the top face (k = 31), upward flow, and no flow through the four side faces. This is a hypothetical scenario chosen to exercise the conduit, not a reconstruction of the Francevillian basin's hydrogeology.

Discretization: cell-centred finite volumes with harmonic-mean face conductances, the structure already written down in rna_biofilm_toy/specs/12_oklo_conduction_field.md (lines 29 to 35) for an electric potential. The matrix is symmetric positive definite, so conjugate gradient with a declared relative residual of 1e-10 is the solver. Candidate implementations, to be chosen at approval: Rust inside oklo_core, which keeps one engine for the app, the web viewer and the tests, or Julia reusing PRRT-spatial-CPM/src/engine/poisson_exact.jl.

## 3. Parameters, all declared before any run

| Zone | Porosity φ | Permeability k (m²) | Tier |
| --- | --- | --- | --- |
| Sandstone (0) | 0.15 | to be sourced | porosity: revision 0 registry assumption |
| Reactor core (1) | to be declared | to be sourced | no value on file |
| Clay and phosphate halo (2) | 0.06 | to be sourced | porosity: registry assumption |
| Fracture conduit (3 and 4) | 0.25 | to be sourced | porosity: registry assumption |

Permeabilities must come from a cited source before approval. Standard hydrogeology texts (for example Freeze and Cherry 1979) tabulate ranges spanning several orders of magnitude for sandstone, clay and fractured rock. Those ranges have not been checked here and are not quoted. Because the ranges are wide, version 0 also runs a declared sweep of the conduit-to-sandstone permeability ratio (1, 10, 100, 1000), and reports results for every point of the sweep rather than choosing one.

## 4. Checks, with pass criteria declared now

- **D1, mass conservation.** Every cell's net flux is at most 1e-10 times the largest face flux, and total inflow equals total outflow to 1e-10 relative.
- **D2, convergence on a manufactured solution.** With uniform K and a smooth manufactured head field, the head error falls at an observed order between 1.9 and 2.1 across grids of 16, 32 and 64 cells per side.
- **D3, layered analytic case.** A 1-D stack of layers in series reproduces the analytic flux, set by the harmonic mean of the layer conductivities, to 1e-8 relative.
- **D4, symmetry.** The fixture geometry is mirror-symmetric in x and y about the box centre, so the solution is too, to 1e-12 relative.
- **D5, a control that can fire.** With all four zones given the same permeability, every horizontal slice carries uniform vertical flux, so the conduit's share of flux in each slice must equal its share of that slice's area to 1e-8 relative. If it does not, the zone map, the face conductances or the solver is wrong.

None of these is a calibration. No parameter is adjusted to make a check pass. A failure is reported as a failure, under the same policy as the conformance work.

## 5. What is reported

The head field, the three Darcy flux components and the pore velocity magnitude as MRC files meeting the CF-3 header table; the conduit's share of vertical flux at each permeability ratio; and the thermal Péclet number Pe = v L / α at the 0.5 m voxel scale, with α the thermal diffusivity of the saturated rock taken from a cited source, to judge whether a heat-advection amendment matters.

## 6. What this does not claim

It does not model boiling, steam, two-phase flow, the pulse, reactivity feedback or any coupling to the prescribed revision 0 fields. It says nothing about the actual hydrogeology at Oklo beyond the declared assumptions. Prior heat and fluid-transfer modelling of Oklo exists (Royer et al. 1995, cited in oklo_heartbeat/manuscript_gate/OTD-RST-SIM-01_v0.1.md lines 34 to 37). It has not been read for this draft, and any comparison with it is future work.

## 7. Decisions for Hunter

Where the spec lives and its number; the boundary-condition scenario and Δh; the permeability sources; the temperature and pressure for ρ and μ; the core's porosity and permeability; and the implementation language.
