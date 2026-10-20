# Field Ontology Amendment A1: Display-Ontology Clarification for the Revision-0 Polymer Field

Status: Active for v1 Cross-Platform Release (`PR #5`, `PR #6`, `PR #7`)
Scope: UI presentation, unit labels, and in-app/store provenance disclosures only
Authoritative Reference: `CONFORMANCE_FIXTURE_SPEC.md` (Section 2.1, Section 3, Section 4 finding `R0-8`, and Section 5 `CF-6`; SHA-256 `587745ede6c6997a298f8f0a4e01127823de1b7c606383cfa49b87db0cc60d0c`)
Frozen Fixture Impact: Zero changes to revision-0 equations, numerical values, C ABI symbols, Rust enum identifiers, or CF-2 SHA-256 reference digests

## 1. Authoritative Revision-0 Equation and Defect Addressed

In the frozen revision-0 Oklo GUI conformance fixture (`CONFORMANCE_FIXTURE_SPEC.md`, Section 3), field 4 (`polymer_yield` / `Field::PolymerYield` / `oklo_polymer_yield.mrc`) is defined on the $n_x \times n_y \times n_z = 32 \times 32 \times 32$ voxel grid ($i, j, k \in \{0, \dots, 31\}$) and normalized cycle phase $p \in [0, 1]$ by the closed-form algebraic expression:

$$P_{\text{poly}}(i, j, k, p) = \begin{cases} 0, & \text{null scenario} \\[4pt] 8.5 \left(0.7\, H(i, j, k) + 0.5\, F(i, j, k)\right) \left(1 + 0.5 \sin(2\pi p)\right), & \text{active scenario} \end{cases}$$

where:

- $c_x = c_y = (n_x - 1)/2 = 15.5$ and $c_{zc} = 0.35\, n_z = 11.2$,
- $x_N = (i - c_x)/(0.40\, n_x)$, $y_N = (j - c_y)/(0.40\, n_y)$, $z_N = (k - c_{zc})/(0.35\, n_z)$, and $r_c = \sqrt{x_N^2 + y_N^2 + z_N^2}$,
- $r_f = \sqrt{\left((i - c_x)/(0.12\, n_x)\right)^2 + \left((j - c_y)/(0.12\, n_y)\right)^2}$,
- $H(i, j, k) = \max\!\left(0,\, \exp\!\left(-\tfrac{1}{2} ((r_c - 1.1)/0.35)^2\right)\right)$ is the prescribed clay and phosphate halo indicator, and
- $F(i, j, k) = \exp\!\left(-\tfrac{1}{2} r_f^2\right)$ for $k \ge c_{zc}$ and $0$ for $k < c_{zc}$ is the prescribed fracture conduit indicator.

This identical equation is implemented across the production Rust engine (`crates/oklo_core/src/lib.rs`, lines 264 to 272), the independent Rust CF-2 reference (`tests/conformance_cf2/src/reference.rs`), and the independent Python CF-2 reference (`scripts/cf2_reference.py`). At the recorded revision-0 snapshot ($p = 0.15$, active scenario, `CONFORMANCE_FIXTURE_SPEC.md` Section 2.1), `oklo_polymer_yield.mrc` has $\min = 0.004336$, $\max = 14.159290$, $\text{mean} = 5.301710$, and population $\text{sd} = 2.871762$. At peak harmonic modulation ($p = 0.25$, where $1 + 0.5 \sin(2\pi p) = 1.5$), the active-scenario grid maximum reaches $15.12197$ where the halo shell $H$ and conduit $F$ overlap above $k \ge 11.2$.

The revision-0 browser prototype and fixture specification attached the concentration label `umol/L` (`\u00b5mol/L`) to this field. As recorded in `CONFORMANCE_FIXTURE_SPEC.md` Section 5 (`CF-6`: *"polymer_yield: umol/L for a hypothetical surrogate with no kinetic basis"*), assigning molar concentration units to $P_{\text{poly}}$ is an epistemic and modeling defect for four reasons:

1. **No defensible concentration basis in revision 0**: The fixture supplies an algebraic heuristic (`8.5 * (0.7*H + 0.5*F) * (1 + 0.5*sin(2*pi*p))`) with hand-prescribed coefficients (`8.5`, `0.7`, `0.5`), but no kinetic mass balance, equilibrium or thermodynamic relation, empirical calibration, or unit mapping that supports interpreting its values as $\mu\text{mol/L}$ (`umol/L`). The relevant synthesis, degradation, and transport processes are not modeled.
2. **Memoryless harmonic phase modulation decoupled from pulse and desiccation drivers**: The temporal factor $(1 + 0.5 \sin(2\pi p))$ is a prescribed single-frequency sinusoid of normalized phase $p \in [0, 1]$ ranging between $0.5$ and $1.5$. It does not depend on the pulse boundary $b$ (`0.1667` vs `0.1700`), the pulse factor $P(p)$, or the desiccation factor $D(p)$, and it exhibits no multi-cycle accumulation or history dependence.
3. **Prescribed geometric indicator weighting**: The spatial distribution $0.7\, H(i, j, k) + 0.5\, F(i, j, k)$ is a fixed linear combination of the analytical Gaussian halo indicator $H$ and conduit indicator $F$, rather than the solution of a reactive-transport or pore-network operator.
4. **Bookkeeping mass-balance closure (`R0-8`) and exclusion of post-hoc rationalizations**: As documented in `CONFORMANCE_FIXTURE_SPEC.md` Section 4 (`R0-8`), the revision-0 `auditMassBalance` routine defines free monomer as the residual closing a `100 umol/L` precursor pool (`100 - 1.15 * mean(polymer_yield)`), so carbon closure holds by construction rather than through a solved conservation law. Post-hoc appeals to entropic-spring mechanics, Flory-Huggins excluded volume, or Rouse/Zimm relaxation times do not supply a missing kinetic, equilibrium, or thermodynamic derivation and are excluded from project documentation and UI copy.

## 2. Option A Amendment (Enacted for v1 Release)

To preserve strict mathematical conformance with the frozen revision-0 fixture while preventing the v1 mobile applications from misrepresenting a heuristic scoring proxy as a physical concentration:

1. **UI Display Title**: Both the iOS app (`ios/OkloPool/Models/FieldCatalog.swift`) and the Android app (`android/app/src/main/kotlin/org/oklo/pool/ViewerModel.kt`) present field 4 in all selectors, headers, and legends as:
   - **Title**: `Polymer accumulation index`
   - **Units**: `arbitrary units`
2. **Required In-App Caveat**: Both applications display the following explicit caveat in the field description and in the Scope & Provenance view (`ProvenanceSheetView.swift` on iOS, `ModalBottomSheet` in `MainActivity.kt` on Android):
   > "Prescribed heuristic field. The revision-0 fixture labels this quantity in umol/L, but no kinetic or thermodynamic basis for those concentration units is established. Values are not measured or predicted polymer concentrations."
3. **Preservation of Frozen Revision-0 Identifiers and Hashes**:
   - The underlying formula $8.5\,(0.7\,H + 0.5\,F)\,(1 + 0.5\sin(2\pi p))$, voxel values (`[0, 15.12197]`), Rust enum variant (`Field::PolymerYield`), C ABI constant (`OKLO_FIELD_POLYMER_YIELD = 4`), reference `.mrc` filename (`oklo_polymer_yield.mrc`), and all 120 CF-2 reference volume SHA-256 hashes remain 100% unchanged.

## 3. Deferred Research Tracks (Out of Scope for v1)

Replacing the algebraic proxy with a mechanistic physical model is explicitly deferred to separate, preregistered research branches for future versions (`v2`/`v3+`):

- **Option B (Kinetic Mass Balance)**: Formulate and verify a coupled monomer/polymer reaction-diffusion-advection system over wet/dry cycles with explicit condensation ($k_{\text{cond}}$), hydrolysis ($k_{\text{hydro}}$), and radiolytic scission ($k_{\text{rad}}$) rate laws before re-introducing molar concentration units (`mol/m^3` or `umol/L`).
- **Option C (Thermodynamic Pore Partitioning)**: Only if pore-exclusion or conformational effects are explicitly modeled via a chemical potential $\mu_i = \mu_i^0 + k_B T \ln a_i + \Delta G_{\text{conf}}(R_g / r_{\text{pore}})$, introduce dimensionless partitioning coefficients alongside verifiable analytical benchmarks.
