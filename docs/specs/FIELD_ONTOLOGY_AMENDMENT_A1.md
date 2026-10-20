# Field Ontology Amendment A1: Display-Ontology Clarification for the Revision-0 Polymer Field

Status: Active for v1 Cross-Platform Release (`PR #5`, `PR #6`, `PR #7`)
Scope: UI presentation, unit labels, and in-app/store provenance disclosures only
Frozen Fixture Impact: Zero changes to revision-0 equations, numerical values, C ABI symbols, Rust enum identifiers, or CF-2 SHA-256 reference digests

## 1. Defect Addressed: Concentration Units Without a Kinetic or Thermodynamic Rate Law

In the frozen revision-0 Oklo GUI conformance fixture (`CONFORMANCE_FIXTURE_SPEC.md`, Section 3.6), field 4 (`polymer_yield` / `oklo_polymer_yield.mrc`) is evaluated at each voxel $(x, y, z) \in [-1, 1]^3$ and phase $\varphi \in [0, 1)$ by an instantaneous closed-form algebraic expression:

$$P_{\text{poly}}(x, y, z, \varphi) = \begin{cases} 0, & \text{null scenario} \\ 15 \cdot \operatorname{clamp}_{[0,1]}\!\left(d(\varphi)\, e^{-r_{xy}^2 / 0.35^2} \left(0.6 + 0.4\, e^{-|\varphi - \varphi_b| / 0.1}\right) \left(1 + 0.35 \sin(4\pi(x+y+z))\right)\right), & \text{active scenario} \end{cases}$$

where $d(\varphi) = \frac{1}{2}(1 - \cos(2\pi(\varphi - \varphi_b)))$ is the desiccation driver and $r_{xy} = \sqrt{x^2 + y^2}$.

The revision-0 browser prototype and fixture specification attached the concentration label `umol/L` (`\u00b5mol/L`) to this field. Assigning molar concentration units to $P_{\text{poly}}$ is a modeling defect for four reasons:

1. **No conservation law or differential equation**: A chemical concentration in $\mu\text{mol/L}$ requires a mass-balance equation with transport, condensation synthesis, hydrolytic cleavage, and radiolytic degradation terms. No such differential equation, rate constant, or reaction order exists in revision 0.
2. **Instantaneous algebraic evaluation**: The field is evaluated as a memoryless function of instantaneous phase $\varphi$, returning immediately to zero at $\varphi = \varphi_b$ where $d(\varphi_b) = 0$ and showing no multi-cycle accumulation or hysteresis.
3. **Unphysical spatial ripple**: The factor $(1 + 0.35 \sin(4\pi(x+y+z)))$ is a geometric texture modulation introduced for visual contrast in 2D/3D slicing rather than a solution to a pore-network transport operator.
4. **Inappropriate post-hoc thermodynamic rationalizations**: Post-hoc appeals to entropic-spring mechanics, Flory-Huggins excluded volume, or Rouse/Zimm relaxation times do not supply the missing kinetic rate law and are excluded from project documentation and UI copy.

## 2. Option A Amendment (Enacted for v1 Release)

To preserve strict mathematical conformance with the frozen revision-0 fixture while preventing the v1 mobile applications from misrepresenting a heuristic scoring proxy as a physical concentration:

1. **UI Display Title**: Both the iOS app (`ios/OkloPool/Models/FieldCatalog.swift`) and the Android app (`android/app/src/main/kotlin/org/oklo/pool/ViewerModel.kt`) present field 4 in all selectors, headers, and legends as:
   - **Title**: `Polymer accumulation index`
   - **Units**: `arbitrary units`
2. **Required In-App Caveat**: Both applications display the following explicit caveat in the field description and in the Scope & Provenance view (`ProvenanceSheetView.swift` on iOS, `ModalBottomSheet` in `MainActivity.kt` on Android):
   > "Prescribed heuristic field. The revision-0 fixture labels this quantity in umol/L, but no kinetic or thermodynamic basis for those concentration units is established. Values are not measured or predicted polymer concentrations."
3. **Preservation of Frozen Revision-0 Identifiers and Hashes**:
   - The underlying formula, voxel values (`[0, 15.12197]`), Rust enum variant (`Field::PolymerYield`), C ABI constant (`OKLO_FIELD_POLYMER_YIELD = 4`), reference `.mrc` filename (`oklo_polymer_yield.mrc`), and all 120 CF-2 reference volume SHA-256 hashes remain 100% unchanged.

## 3. Deferred Research Tracks (Out of Scope for v1)

Replacing the algebraic proxy with a mechanistic physical model is explicitly deferred to separate, preregistered research branches for future versions (`v2`/`v3+`):

- **Option B (Kinetic Mass Balance)**: Formulate and verify a coupled monomer/polymer reaction-diffusion-advection system over wet/dry cycles with explicit condensation ($k_{\text{cond}}$), hydrolysis ($k_{\text{hydro}}$), and radiolytic scission ($k_{\text{rad}}$) rate laws before re-introducing molar concentration units (`mol/m^3` or `umol/L`).
- **Option C (Thermodynamic Pore Partitioning)**: Only if pore-exclusion or conformational effects are explicitly modeled via a chemical potential $\mu_i = \mu_i^0 + k_B T \ln a_i + \Delta G_{\text{conf}}(R_g / r_{\text{pore}})$, introduce dimensionless partitioning coefficients alongside verifiable analytical benchmarks.
