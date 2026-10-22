# Oklo Prebiotic Nuclear-Geyser & Compartment Explorer v0

**Disposition:** quarantined synthetic visualization prototype. This export is for interface review, source audit, and synthetic fixture work only. It is not a physical reactor, heat-transfer, hydrology, radiolysis, depletion, polymerization, or biofilm simulation.

## Contents

- `index.html`: guarded entry point with a persistent scientific-status banner.
- `prototype.html`: UTF-8 text copy of the uploaded `index.html` prototype, preserved as a source artifact for audit.
- `artifact_manifest.json`: field-level epistemic classification, units, dependencies, and known limitations.

The wrapper does not remediate or silently rewrite the preserved prototype. Its warning remains visible outside the iframe.

## Triage findings

1. **Synthetic fields, not solver fields.** Temperature, hydration, H2 concentration, U-235 ratio and polymer-yield arrays are built from hand-authored spatial and phase functions. No neutron-transport/eigenvalue solver, heat PDE, multiphase-flow solver, radiolysis-dose calculation, Bateman depletion network, reactive-transport PDE or polymerization kinetic network is implemented by these formulas.
2. **Mass-balance badge is algebraic, not an independent test.** The ledger defines polymer from the mean of the polymer field, derives the bound pool from that value, derives monomer as the remainder, then sums those derived values. The apparent closure is therefore true by construction while the components remain within the chosen pool; it does not validate a reaction network.
3. **Water-continuity claim is not calculated.** The water continuity flag is assigned `true`. The displayed steam/vapor fraction is formed from one minus the arithmetic mean hydration field, without pore-volume weighting, fluid inventory, pressure, enthalpy or two-phase flow.
4. **Compartment palette defect.** The original colormap clamps field values to the interval [0, 1] before comparing categorical compartment values 0, 1, 2, 3 and 4. As a result, the nonzero labels collapse to the same mapped class in that path.
5. **Isotope chronology is inconsistent with a historical reactor model.** The U-235 field begins near a present-day natural-abundance ratio and subtracts a smooth spatial decrement. It does not reconstruct initial uranium composition circa two billion years ago, neutron-induced depletion or present-day remnant ore chemistry.
6. **Radiolytic H2 lacks a dimensionally closed source term.** The laboratory G-value cited in the UI is not converted to a local concentration through an absorbed-energy/dose field, water mass, geometry and reaction/scavenging network. The displayed formula is not a radiolysis calculation.
7. **Cycle and geological inputs are scenario choices.** The displayed three-hour cycle is associated with an interpretation of xenon isotope retention, not a directly measured historical geyser trace. Porosities and compartment shapes are assumptions unless a separately cited site-specific measurement is supplied.
8. **MRC export needs external conformance evidence.** The presence of a Mode 2 header builder is not by itself proof that exported files open correctly. Verify dimension and spacing units, origin and axis order with a format validator and a real consumer such as ChimeraX. Do not represent meters as angstroms or confuse grid counts with physical cell dimensions.
9. **Not offline as supplied.** The preserved prototype loads Tailwind CSS from an external URL. The exported source should not be described as zero-dependency or fully offline without replacing and retesting that dependency.
10. **UI terms are not physical evidence.** Labels such as “Measured,” “Verified,” “Volumetric Raycast,” and “Conserved” do not constitute a measurement, algorithmic conformance test, solver verification or validation receipt.

## Use policy

- Safe: inspect interaction design, field switching, cut-plane and slice UX, export workflow, provenance display, and known-bug reproduction.
- Not supported: claims of reactor criticality/power, physical temperature prediction, confirmed continuous liquid flow, calculated radiolytic H2, U-235 burnup, polymerization yield, ancient biofilm growth, or life originating at Oklo.
- Treat each field as `synthetic_illustrative` until its equation, units, input provenance, uncertainty and verification status are established.
- Keep this unit separate from the frozen Revision-0 conformance fixture `oklo_gui_scratch/index.html`, R10 `spacetime_geochemistry_v0`, and PR #11's Stage 0 neutron-reference audit.

## Minimum acceptance gates for any future remediation

1. A field-level schema declares unit, dimensional basis, coordinate order, physical time, generating equation, source/data hashes, uncertainty and epistemic tier.
2. Automated negative controls verify that status text cannot imply validation when a field is synthetic.
3. The categorical palette has tests for all labels 0 through 4.
4. Conservation tests are computed from independently evolved state variables and boundary fluxes, not by defining one compartment as the remaining total.
5. Radiolysis output is derived from a dimensionally explicit absorbed-energy rate and validated G-value model.
6. Historical uranium composition, operating history and modern ore analyses are separately represented.
7. MRC files are opened and checked by an independent validator/consumer with scale and axis assertions.
8. Any solver or coupling work receives its own specification, numerical verification, convergence testing, independent benchmark and authorization. An HTML export does not authorize a physical model.

## Export provenance

The file `prototype.html` is copied from the conversation upload `index.html`. Its UTF-8 content SHA-256 and byte count are recorded in `artifact_manifest.json`. The checksum identifies the exported text artifact; it is not a scientific-validation checksum.
