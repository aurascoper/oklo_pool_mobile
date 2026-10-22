# Reference Audit & Provenance Registry: `TECDOC-643 Neutron Walk` and Companion Materials (`NEUTRON_WALK_REFERENCE`)

- **Document ID:** `NEUTRON_WALK_REFERENCE`
- **Parent Specification:** [`docs/specs/NEUTRON_TRANSPORT_EXPLORER_v0.md`](../specs/NEUTRON_TRANSPORT_EXPLORER_v0.md) (`neutron_transport_explorer_v0`, Stage 0)
- **Target Branch:** `neutron-transport-explorer/v0-spec` (branched from `main` at `942e3f8d02a3b73cfb26f099dbb36c4a908f6764`)
- **Disposition:** Reference audit and provenance contract only (`PROPOSED`; implementation, solver execution, and physical validation not authorized by this note)
- **Epistemic Status:** External design reference, transcribed unverified OpenMC playback/tally/depletion datasets (`pre-2026-10-19 materials`), and quarantined operator-algebra test fixture

---

## 1. Cryptographic Registry of Audited Reference Artifacts

None of the external reference artifacts listed below are committed into the `oklo_pool_mobile` repository. Their exact byte counts, SHA-256 digests, origins, licensing constraints, and assigned scientific roles are recorded below so that any future Stage 1 to Stage 4 work can verify provenance fail-closed.

| Artifact ID | Local Path Inspected | Size (Bytes) | SHA-256 Digest | Assigned Role | Repository Commit Policy |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `REF-NW-HTML-01` | `/Users/aurascoper/Downloads/TECDOC-643 Neutron Walk (Copy).html` (identical copy at `.user_uploaded/media_1792634875959_f2c39e13.html`) | `937,606` (`831` lines) | `dc15eb934fe6cd32a084dc13d05ee17d8f5ad988f7ea1bd2598bdcd9a0065b78` | **Role A:** Interactive 3D + 2D slice scientific-visualization design reference and embedded JSON data source | **Do not commit** (`937 KB` file with external CDN dependencies and pre-Oct 19 unverified OpenMC numbers) |
| `REF-NW-JSON-REAL` | Embedded `<script type="application/json" id="realdata">` inside `REF-NW-HTML-01` (line 327) | `857,722` | `df0c0e776c3cd69b1931d46e4668cc11e783ff6576b9fb3887140537cdf05f59` | **Role A:** Embedded OpenMC 0.15.3 (`ENDF/B-VIII.0`, `pre-2026-10-19 materials`) recorded tracks (`500` histories), $7 \times 6$ power maps (`2M` histories), and 5-point blade sweep (`800k` histories/pt) | **Unverified until Stage 2/3** (source HDF5 files absent locally; requires rerun with post-Oct 19 materials before quoting) |
| `REF-NW-JSON-BURN` | Embedded `<script type="application/json" id="burndata">` inside `REF-NW-HTML-01` (line 326) | `1,066` | `b98b8c4b165801516d712db7d993a8babd78ea2c88c8620d1a0dca38e78a3b29` | **Role A:** Embedded Oct 21 laptop demo 10 MW depletion series (`12` timestamps, `0` to `350 d`, 1 material/element, reduced chain, `294 K`) | **Demo only (`not paper data`)**; disagrees with reported full cluster crossing (`340.26 d` vs. `~298 d`) |
| `REF-NW-IMG-XE3D` | `.user_uploaded/media_1792634977055_2a26966f.jpg` | `82,521` | `1e538db7477e8e8accfb044bf8dd1e8541b75a84b156af8ec2a578490ac3181c` | **Role B:** Spatial-field design reference (ORNL 3D $^{135}\text{I}$, $^{135}\text{Xe}$, $^{149}\text{Pm}$, $^{149}\text{Sm}$ normalized concentration `0.0` to `1.0` at `0 hr` and `10 hr`, WB2 startup context) | **Do not commit** (third-party licensed image; not data from the 10 MW MTR model) |
| `REF-NW-IMG-CORE` | `.user_uploaded/media_1792634876099_d3a3bf19.jpg` | `166,672` | `0ac0826e54465813e9d741bb91cc2fb4716cc8b83886217ff084eeeaa36d8170` | **Role B:** 3D volumetric reactor cutaway visual reference | **Do not commit** (third-party visual reference only) |
| `REF-NW-IMG-ASNR` | `.user_uploaded/media_1792634977032_7f7fdb01.jpg` | `126,089` | `fccfab10371deabc73937d9e8286bfd4417415c14039f4c19e6fc29d4d853326` | **Role C:** Open-pool research reactor / ASNR neutron laboratory presentation context reference | **Do not commit** (third-party photograph; not evidence for MTR parameters) |
| `REF-NW-IMG-VAL` | `.user_uploaded/media_1792634977029_aa5729aa.jpg` | `42,730` | `a60bf1c925ce930484cce07cd48a6ecdca50f900e998c37c7777560f4705a4bc` | **Role D:** Surrogate parity & uncertainty plot (`"Predicted vs True Cd with Uncertainty (MC-Dropout)"`, `Ideal (y=x)`, `95% CI`) paired with pSeven validation-types article | **Do not commit** (methodology reference only; surrogate holdout testing is not physical reactor validation) |
| `REF-NW-JL-FIX` | `/Users/aurascoper/Downloads/tecdoc643_reference_fixture.jl` | `5,951` (`125` lines) | `813c9e9fb33e47d9ceead8ea8f08bf07122583cf4292aab651d4f919a572a849` | **Quarantined Julia Fixture:** Transcribed table and generic operator-algebra test hook (`TECDOC643ReferenceFixture`) | Audited read-only under local `julia 1.13.1` (see Section 4 below) |

---

## 2. Dependency, Provenance, and Local Availability Audit of `REF-NW-HTML-01`

### 2.1 Reactor Geometry Provenance vs. Numerical Prediction Validity

The reactor geometry encoded in `REF-NW-HTML-01` represents the generic 10 MW Light-Water Pool-Type Materials Testing Reactor (MTR) benchmark documented in **IAEA-TECDOC-643** (*Research Reactor Core Conversion Guidebook*, Vol. 2: Analysis, Appendix G) and related Argonne National Laboratory (ANL) MTR conversion studies:
- **Active Core & Reflector Layout:** A $6 \times 7$ lattice (`Columns A to F` along $x$ with $77\text{ mm}$ pitch; `Rows 1 to 7` along $y$ with $81\text{ mm}$ pitch) comprising a $5 \times 6$ active core (`Rows 2 to 6`) flanked on the $+y$ (`Row 1`) and $-y$ (`Row 7`) faces by $12$ graphite reflector elements (`GR`).
- **Fuel & Control Elements:** `23` Standard Fuel Elements (`SFE`, 23 plates per element, active fuel meat height $60\text{ cm}$, meat thickness $0.51\text{ mm}$) and `5` Control Fuel Elements (`CFE` at `C2`, `E3`, `B4`, `E5`, `C6`, 17 plates per element flanked by twin $\text{B}_4\text{C}$ absorber blades of thickness $3.1\text{ mm}$ and width $66.3\text{ mm}$).
- **Flux Traps:** Two water-filled irradiation flux traps (`FT`) at central position `D4` and corner position `A6` (modeled with a $2.82\text{ cm}$ radius water cylinder inside an aluminum block).
- **Water Pool Reflector Envelope:** Model domain extends $38.5\text{ cm}$ laterally beyond the $46.2 \times 56.7\text{ cm}$ lattice footprint ($123.2 \times 133.7\text{ cm}$ horizontal box) and $\pm 90\text{ cm}$ vertically ($180\text{ cm}$ total height, including $14\text{ cm}$ homogenized end boxes centered at $z = \pm 38\text{ cm}$).

**Provenance Boundary:** IAEA-TECDOC-643 establishes the provenance of this **geometric configuration**, not the accuracy or validation status of the specific OpenMC or JavaScript toy-model numerical results embedded in `REF-NW-HTML-01`.

### 2.2 External Network Dependencies (Non-Zero-Network Status)

Inspection of `REF-NW-HTML-01` lines 4 to 6 and line 328 confirms that the page is **not** self-contained for offline or zero-third-party-network deployment:

1. **Google Fonts Preconnect & Stylesheet (Lines 4 to 6):**
   - `<link rel="preconnect" href="https://fonts.googleapis.com">`
   - `<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>`
   - `<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Archivo:wght@500;700;800&family=Source+Sans+3:wght@400;600&family=IBM+Plex+Mono:wght@400;500&display=swap">`
2. **External Three.js CDN Script (Line 328):**
   - `<script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>`
   - When opened in an offline or network-restricted environment where `window.THREE` fails to load, the 3D core view (`#gl`) and 3D human-scale view (`#sc`) fail to initialize, displaying `"The 3-D library could not load. The slice view still works."` (line 421) and falling back to the 2D Canvas slice mode (line 824).

### 2.3 Availability Audit of Upstream OpenMC Run Files on Local Workstation

A filesystem audit of `/Users/aurascoper` confirmed that **none** of the upstream OpenMC input, statepoint, track, tally, or nuclear-data files cited in `#realdata.*.meta` and the HTML debrief are present on this workstation:

| Upstream Artifact Cited in HTML Metadata / Debrief | Cited Path or Identifier in HTML | Local Workstation Status | Impact on Stage 0 / Stage 2 |
| :--- | :--- | :--- | :--- |
| Converged fission source files | `viz_out/source/insertion_<pct>/source.h5` | **ABSENT LOCALLY** | Cannot independently verify source spatial convergence prior to Stage 2 archive import |
| Raw OpenMC 0.15.3 particle track HDF5 files | `viz_out/replicas/insertion_<pct>/rep_00/tracks.h5` (replica 0, seed 101 of $10 \times 300$ replica runs) | **ABSENT LOCALLY** | Only the 500 post-processed, surface-crossing-stripped tracks inside `#realdata.tracks` are present |
| Reclassified absorption death tags | `viz_out/stats/deaths_<pct>.csv` | **ABSENT LOCALLY** | The $\sigma_f / \sigma_a$ reclassification of OpenMC `MT 18` events (`1,571` summary numbers) is unverified locally |
| Lattice $\kappa$-fission tally summaries & peaking script | `viz_out/flux/insertion_<pct>/flux_summary.json` and `flux_maps.py` ($40,000 \times 50 = 2.0\text{ M}$ active histories) | **ABSENT LOCALLY** | Only the rounded 4-decimal `rel` and `sigma` $7 \times 6$ matrices in `#realdata.power` are present |
| Control-blade withdrawal sweep output | `viz_out/whatif/whatif.json` ($20,000 \times 40 = 800\text{k}$ active histories/point) | **ABSENT LOCALLY** | Only the rounded 5-decimal `k` and `sigma` arrays and 1-decimal `worth_pcm` arrays in `#realdata.sweep` are present |
| Evaluated nuclear data HDF5 library | `~/nuclear-data/endfb-viii.0-hdf5` (`ENDF/B-VIII.0`) | **ABSENT LOCALLY** | Required for Stage 3 numerical reproduction |
| Upstream side branch & voxel-shift minimal reproducer | `feature/track-viz` (`22 commits`, never pushed) & OpenMC 3D voxel export half-voxel shift test script | **ABSENT LOCALLY** | Half-voxel vertical shift in OpenMC 3D voxel export remains an **unverified lead** until a standalone reproducer is built in Stage 3 |

---

## 3. Forensic Inventory of Embedded Models, Data Tables, and Discrepancies

### 3.1 Model Separation in `REF-NW-HTML-01`: Client-Side Toy Surrogate vs. Recorded OpenMC Playback

`REF-NW-HTML-01` implements two distinct operational modes under the `Neutrons` segmented control (`Toy physics` vs. `Real OpenMC`):

| Feature | Mode 1: `Toy physics` (`walk()`, Client-Side Surrogate) | Mode 2: `Real OpenMC` (`realHistory()`, Recorded Playback) |
| :--- | :--- | :--- |
| **Execution Mechanism** | Live in-browser xorshift32 PRNG (`rng(seed)`) executing up to `6,000` collisions per neutron with maximum sub-step `0.12 cm` across `region(x, y, z)` | Deterministic cyclic playback (`realIdx[key]++ % 250`) of pre-recorded trajectories embedded in `#realdata.tracks.states["100"]` and `["0"]` (**not a live OpenMC calculation**) |
| **Energy Representation** | Two discrete groups: `fast = true` (born in fuel with chopped-cosine axial distribution $\cos(\pi z / 80\text{ cm})$) and `fast = false` (thermalized with probability `pt` per non-absorbing collision) | Binary flag `fast = 1` if outgoing energy $E > 0.625\text{ eV}$, else `0`, recorded at birth, collision, and termination vertices (straight-flight surface crossings dropped) |
| **Cross-Section / Step Parameters** | Hand-tuned region table `P`:<br>- `fuel`: `fm=3.0 cm, pt=1/20, fa=0.012, tm=0.55 cm, ta=0.135`, with `FISS_T=0.792`, `FISS_F=0.35`, $\bar{\nu}=2.43$<br>- `core`/`pool` water: `fm=2.5, pt=1/17, fa=0, tm=0.6, ta=0.020`<br>- `endbox`: `fm=2.9, pt=1/21, fa=0.001, tm=0.7, ta=0.018`<br>- `graphite`: `fm=2.6, pt=1/100, fa=0, tm=2.6, ta=0.0012`<br>- `al`: `fm=9.0, pt=0, fa=0.002, tm=9.0, ta=0.012`<br>- `b4c`: `fm=1.35, pt=0, fa=0.55, tm=0.06, ta=1.0` | Recorded from OpenMC `0.15.3` continuous-energy transport with `ENDF/B-VIII.0` (`pre-2026-10-19 materials`), using microscopic $\sigma_f / \sigma_a$ sampling to classify terminal absorptions (`out = "fission" | "capture"`) |
| **Control-Blade Positions Supported** | Continuous withdrawal fraction `BLADE_F` $\in [0.0, 1.0]$ in `5%` slider steps (`z0 = -30 + 60 * BLADE_F` cm) | **Two discrete states only:** `"100"` (blades 100% inserted, `BLADE_F = 0`) and `"0"` (blades 0% inserted / fully withdrawn, `BLADE_F = 1`). Moving the slider snaps to `0%` or `100%` |
| **Recorded Sample Size & Outcomes** | Arbitrary live stream (starts with a batch of `120` histories on load; `Run toy sweep` uses `5,000` histories at `11` blade positions with seed `4242`) | - **State `"100"` (Blades In):** `250` histories (`102` fission = `40.8%`, `148` capture = `59.2%`, `0` leak; `19,371` total vertices, `2` to `597` vertices/track; terminations by region: `fuel: 134, pool: 59, b4c: 25, core: 15, endbox: 10, al: 6, graphite: 1`)<br>- **State `"0"` (Blades Out):** `250` histories (`111` fission = `44.4%` in the 250-track sample vs. `49.3%` in the full 2M-history run, `139` capture = `55.6%`, `0` leak; `18,376` total vertices, `2` to `765` vertices/track; terminations by region: `fuel: 158, pool: 48, core: 27, endbox: 10, al: 4, b4c: 3`) |
| **Fuel Plate Geometry Resolution** | `region(x, y, z)` homogenizes the fuel stack (`dy < 3.8925 cm` for `SFE`, `dy < 2.8315 cm` for `CFE`) during particle transport; individual meat/clad/water plates are resolved **only for 2D slice display** in `viewRegion(x, y, z)` (line 603) | Underlying OpenMC geometry modeled heterogeneous fuel plates, cladding, water channels, and $\text{B}_4\text{C}$ blades |

### 3.2 Audit of the Four Fresh-Core $k_{\mathrm{eff}}$ and Blade-Worth Datasets in `REF-NW-HTML-01`

A critical finding of the Stage 0 audit is that `REF-NW-HTML-01` cites **four different fresh-core $k_{\mathrm{eff}}$ datasets** across its UI tables, debrief cards, and `#realdata.sweep` JSON payload. These datasets must never be merged or treated as interchangeable:

| Dataset ID | Location in HTML | Stated Provenance & Nuclear Data | $k_{\mathrm{eff}}$ (Blades In, `0%` Withdrawn) | $k_{\mathrm{eff}}$ (Blades Out, `100%` Withdrawn) | Recomputed Two-Point Worth $\Delta\rho = \frac{1}{k_{\mathrm{in}}} - \frac{1}{k_{\mathrm{out}}}$ | Stored / Displayed Worth in HTML |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Dataset K1** | UI `Fresh core` table (line 208) & fission share (lines 231 to 237) | `"OpenMC (Oct 19, VII.1)"` (post-material-update `ENDF/B-VII.1` run) | `0.98106` (no $\sigma$ shown; fission share $k/2.43 = 40.37\%$, displayed as `40.4%`) | `1.19732` (no $\sigma$ shown; fission share $k/2.43 = 49.27\%$, displayed as `49.3%`) | `18,410.72 pcm` ($\rho_{\mathrm{in}} = -1,930.56\text{ pcm}$, $\rho_{\mathrm{out}} = +16,480.15\text{ pcm}$) | Not computed in UI |
| **Dataset K2** | UI `Fresh core` table (line 209) | `"Reference MCNP model"` | `0.98189` (no $\sigma$ shown) | `1.19872` (no $\sigma$ shown) | `18,422.14 pcm` ($\rho_{\mathrm{in}} = -1,844.40\text{ pcm}$, $\rho_{\mathrm{out}} = +16,577.74\text{ pcm}$) | Not computed in UI |
| **Dataset K3** | Debrief cards & math block (lines 313 to 321) | Oct 14 run, `2 M` active histories (`40,000 x 50`), `ENDF/B-VIII.0`, `pre-Oct 19 materials` | `0.98057 +/- 0.00069` ($\rho_{\mathrm{in}} = -1,981.50\text{ pcm}$, displayed as `-1,981 pcm`) | `1.19621 +/- 0.00063` ($\rho_{\mathrm{out}} = +16,402.64\text{ pcm}$, displayed as `+16,403 pcm`) | **`18,384.14 +/- 84.19 pcm`** | **`18,384 +/- 84 pcm`** (line 321) |
| **Dataset K4** | `#realdata.sweep` (`viz_out/whatif/whatif.json`, line 258 & line 327) | Oct 14 5-point sweep, `800 k` active histories/pt (`20,000 x 40`, model rebuilt per position), `ENDF/B-VIII.0`, `pre-Oct 19 materials` | `0.97943 +/- 0.00102` ($\rho_{\mathrm{in}} = -2,100.20\text{ pcm}$) | `1.19810 +/- 0.00081` ($\rho_{\mathrm{out}} = +16,534.51\text{ pcm}$) | **`18,634.71 +/- 120.37 pcm`** (from rounded 5-decimal $k$ endpoints) | **`18,635.3 +/- 120.0 pcm`** (`worth_pcm[0]`, `worth_sigma[0]`) |

**Unresolved Discrepancy Between Dataset K3 and Dataset K4 (`1.71 sigma` / `251.16 pcm`):**
The two-point blade worth from the 2M-history runs (`18,384.14 +/- 84.19 pcm`) and the endpoint worth from the 800k-history sweep (`18,635.3 +/- 120.0 pcm`) differ by `251.16 pcm` ($z = 251.16 / \sqrt{84.19^2 + 120.0^2} = 1.713\sigma$). Until the original input scripts for the fixed-endpoint run (`viz_out/flux/`) and the rebuilt-per-position sweep (`viz_out/whatif/whatif.json`) are audited in Stage 3, this difference must be retained explicitly as an unresolved statistical/modeling discrepancy.

### 3.3 Spatial Power Map Inventory (`#realdata.power`)

`#realdata.power` stores the $7 \times 6$ relative element power distribution (`rel[row][col]`, normalized so the mean over the `28` fuel elements equals `1.0000`) and $1\sigma$ Monte Carlo relative uncertainty (`sigma[row][col]`) from the Oct 14 `2.0 M` active-history `ENDF/B-VIII.0` (`pre-2026-10-19 materials`) runs:
- **Blades 100% Inserted (`state "100"`):**
  - Radial power peaking: **`1.5485 +/- 0.0083`** at position **`C4`** (`Row 4, Col C`, `SFE` directly adjacent to the central **`D4` Flux Trap**, where `rel[3][2] = 1.5485` and single-element tally `sigma[3][2] = 0.0086`, while 4,000-draw extreme-value `peaking_sigma = 0.0083`).
  - Control-element depression: the five `CFE` positions with inserted $\text{B}_4\text{C}$ blades drop to `0.4888 +/- 0.0039` (`C6`), `0.5587 +/- 0.0049` (`E5`), `0.5617 +/- 0.0044` (`C2`), `0.5731 +/- 0.0048` (`E3`), and `0.6607 +/- 0.0047` (`B4`).
  - Mean over the 28 non-null fuel cells: $\frac{1}{28}\sum \mathrm{rel}_{j,i} = 1.0000036$.
- **Blades 0% Inserted / Fully Withdrawn (`state "0"`):**
  - Radial power peaking: flattens to **`1.3965 +/- 0.0066`** at position **`C4`** (`rel[3][2] = 1.3965`, `sigma[3][2] = 0.0068`, `peaking_sigma = 0.0066`), with secondary peaks at `D3` (`1.3353 +/- 0.0071`), `D5` (`1.3110 +/- 0.0071`), and `E4` (`1.3075 +/- 0.0068`) around the `D4` flux trap.
  - Control-element recovery: the five `CFE` positions rise to `0.8264` (`C6`), `0.8632` (`C2`), `0.8739` (`E5`), `0.8744` (`E3`), and `0.9470` (`B4`).
  - Mean over the 28 non-null fuel cells: $\frac{1}{28}\sum \mathrm{rel}_{j,i} = 1.0000036$.
- **Declared Uncertainty Limitation:** As noted in `#realdata.power.meta.sigma`, each element's `sigma` is the individual element tally standard deviation divided by the sample mean; the uncertainty of the 28-element mean itself is not propagated into the ratio.

### 3.4 Depletion / Burnup Time Machine Inventory (`#burndata`)

`#burndata` contains 12 discrete depletion steps (`days = [0, 5, 10, 20, 40, 80, 120, 160, 200, 250, 300, 350]`) at `10 MW` thermal power with control blades fully withdrawn:
- **Initial to Final Inventories (`0 d` to `350 d` / `3,500 MWd`):**
  - `k_eff`: `1.19676 +/- 0.00175` (`0 d`) $\to$ `1.15243 +/- 0.00126` (`5 d`) $\to$ `1.01743 +/- 0.00130` (`300 d`) $\to$ `0.99578 +/- 0.00170` (`350 d`).
  - `U235`: `10,402.9928 g` (`0 d`) $\to$ `6,155.7275 g` (`350 d`, `40.83%` burned).
  - `U238`: `42,270.1669 g` (`0 d`) $\to$ `41,747.9376 g` (`350 d`).
  - `Pu239`: `0.0 g` (`0 d`) $\to$ `273.5623 g` (`350 d`).
  - `Pu240`: `0.0 g` (`0 d`) $\to$ `45.5021 g` (`350 d`).
  - `Pu241`: `0.0 g` (`0 d`) $\to$ `18.7633 g` (`350 d`).
  - `Xe135`: `0.0 g` (`0 d`) $\to$ `0.0577 g` (`5 d`) $\to$ `0.0578 g` (`10 d` peak) $\to$ `0.0405 g` (`350 d`).
  - `Sm149`: `0.0 g` (`0 d`) $\to$ `0.1401 g` (`5 d`) $\to$ `0.5346 g` (`40 d` peak) $\to$ `0.4521 g` (`350 d`).
- **Critical Discrepancy (`340.26 d` Laptop Demo vs. `~298 d` Full Cluster Model):**
  Line 276 of `REF-NW-HTML-01` states explicitly: *"Your laptop run from Oct 21: blades fully out at 10 MW, one fuel material per element, ENDF/B-VIII.0 with a reduced chain and 294 K data. It's a demo, not paper data. The cluster runs with the full model cross k = 1 near day 298."* Consequently, `#burndata` must be labeled as an illustrative reduced-chain demo and never cited as the benchmark depletion lifetime of the IAEA-TECDOC-643 core.

---

## 4. Red-Team Audit of the `RadioPharm.jl` (`MURR`) Bridge and Quarantined Julia Fixture (`REF-NW-JL-FIX`)

A proposed code bridge suggested importing `#realdata.sweep`, `#realdata.power`, and `#burndata` directly into `RadioPharm.jl` (`murr_operator_kinetics.jl`), and a quarantined reference fixture (`/Users/aurascoper/Downloads/tecdoc643_reference_fixture.jl`, SHA-256 `813c9e9fb33e47d9ceead8ea8f08bf07122583cf4292aab651d4f919a572a849`) was prepared to isolate the transcribed data.

Both the proposed bridge and `tecdoc643_reference_fixture.jl` were audited and executed read-only under the workstation's local Julia installation (`/opt/homebrew/bin/julia`, `julia version 1.13.1`) against `/Users/aurascoper/.gemini/antigravity/scratch/RadioPharm.jl/src/murr_operator_kinetics.jl` (which remains unmodified). Four red-team findings govern any future cross-repository use:

### 4.1 Red-Team Finding 1: Generic Operator Equations Bridge; Reactor-Specific Physics Does Not Transfer

The reactivity definitions implemented in both `TECDOC643ReferenceFixture` and `/Users/aurascoper/.gemini/antigravity/scratch/RadioPharm.jl/src/murr_operator_kinetics.jl`:

$$\rho(k_{\mathrm{eff}}) = \frac{k_{\mathrm{eff}} - 1}{k_{\mathrm{eff}}}, \qquad \mathrm{SDM}(k_{\mathrm{eff}}) = \frac{1 - k_{\mathrm{eff}}}{k_{\mathrm{eff}}}, \qquad \Delta\rho(k_1 \to k_2) = \rho(k_2) - \rho(k_1) = \frac{k_2 - k_1}{k_1\,k_2}$$

are universal reactor-physics identities. Executing `TECDOC643ReferenceFixture.audit_operator_functions` in local `julia 1.13.1` against `murr_operator_kinetics.jl` confirmed exact IEEE-754 Float64 agreement (`atol = 1e-12, rtol = 0`, `agrees_with_fixture = true`) at both test pairs:
- $(k_1, k_2) = (0.97943, 1.19810)$: $\rho(k_1) = -2,100.201137396238\text{ pcm}$, $\Delta\rho(k_1 \to k_2) = +18,634.714116279465\text{ pcm}$.
- $(k_1, k_2) = (1.19676, 1.15243)$: $\rho(k_1) = +16,441.057521976003\text{ pcm}$, $\Delta\rho(k_1 \to k_2) = -3,214.223788039883\text{ pcm}$.

However, **the reactor-specific outputs do not transfer between the two facilities**:

| Characteristic | IAEA-TECDOC-643 Benchmark (`REF-NW-HTML-01`) | University of Missouri Research Reactor (`MURR` / `RadioPharm.jl`) |
| :--- | :--- | :--- |
| **Core Topology** | Rectangular $6 \times 7$ Cartesian lattice ($77 \times 81\text{ mm}$ pitch) | Cylindrical annular core of 8 wedge-shaped fuel assemblies around a central island tube |
| **Fuel & Plate Geometry** | Flat MTR plates (`23` plates in `SFE`, `17` in `CFE`) | Curved concentric fuel plates (24 curved plates per wedge assembly) |
| **Control Mechanism** | 5 in-core `CFE` elements each carrying twin flat $\text{B}_4\text{C}$ absorber blades (`10` blades total) | 4 exterior annular Boral control blades operating in a gap between the pressure vessel and beryllium reflector + 1 stainless-steel regulating blade |
| **Reflector Materials** | Two rows of rectangular graphite blocks (`GR`, Rows 1 and 7) surrounded by open pool water | Concentric cylindrical beryllium primary reflector surrounded by canned graphite secondary reflector Elements |
| **Irradiation Flux Traps** | Two rectangular lattice water holes (`D4` and corner `A6`) | Single high-flux central island flux trap (`~3.0e14` to `6.0e14 n cm^-2 s^-1`) plus reflector irradiation positions |
| **Operating Schedule** | Continuous 350-day full-power depletion in `#burndata` | Weekly 6.5-day (`156 h`) UP / 0.5-day (`12 h`) DOWN refueling and isotope-harvesting cycle |

**Rule:** TECDOC-643 tables may be used to unit-test generic reactivity and uncertainty functions, but must **never** be used to calibrate MURR physical parameters or supply MURR-specific neutron-activation GLMM variance components ($b_{\mathrm{cycle},i}$, $b_{\mathrm{pos},j}$).

### 4.2 Red-Team Finding 2: Precision-Aware Recomputation vs. Hard-Coded Summary Claims & Isotopic Attribution

Recomputing derived quantities directly from the rounded 5-decimal $k_{\mathrm{eff}}$ values embedded in `#realdata.sweep` and `#burndata` exposes systematic rounding differences relative to the pre-rounded summary numbers stored in the JSON:

| Quantity | Formula Applied to Displayed Rounded Inputs | Recomputed IEEE-754 Float64 Value | Stored Value in HTML JSON (`#realdata` / `#burndata`) | Rounding Discrepancy (Recomputed minus Stored) |
| :--- | :--- | :--- | :--- | :--- |
| Sweep worth at `0%` withdrawn ($k_1 = 0.97943 \to k_5 = 1.19810$) | $10^5 \left(\frac{1}{0.97943} - \frac{1}{1.19810}\right)$ | **`18,634.714116 pcm`** | `18,635.3 pcm` | `-0.586 pcm` |
| Sweep worth $1\sigma$ at `0%` withdrawn ($\sigma_1 = 0.00102, \sigma_5 = 0.00081$) | $10^5 \sqrt{\left(\frac{\sigma_1}{k_1^2}\right)^2 + \left(\frac{\sigma_5}{k_5^2}\right)^2}$ | **`120.374925 pcm`** | `120.0 pcm` | `+0.375 pcm` |
| Sweep worth at `25%` withdrawn ($k_2 = 1.02804 \to k_5 = 1.19810$) | $10^5 \left(\frac{1}{1.02804} - \frac{1}{1.19810}\right)$ | **`13,806.992649 pcm`** (`+/- 122.574 pcm`) | `13,807.0 pcm` (`+/- 122.2 pcm`) | `-0.007 pcm` (`+0.374 pcm` in $\sigma$) |
| Sweep worth at `50%` withdrawn ($k_3 = 1.11336 \to k_5 = 1.19810$) | $10^5 \left(\frac{1}{1.11336} - \frac{1}{1.19810}\right)$ | **`6,352.720926 pcm`** (`+/- 109.277 pcm`) | `6,352.7 pcm` (`+/- 109.3 pcm`) | `+0.021 pcm` (`-0.023 pcm` in $\sigma$) |
| Sweep worth at `75%` withdrawn ($k_4 = 1.17049 \to k_5 = 1.19810$) | $10^5 \left(\frac{1}{1.17049} - \frac{1}{1.19810}\right)$ | **`1,968.818270 pcm`** (`+/- 99.935 pcm`) | `1,969.1 pcm` (`+/- 99.8 pcm`) | `-0.282 pcm` (`+0.135 pcm` in $\sigma$) |
| Burnup linear $k_{\mathrm{eff}} = 1$ crossing (`Day 300` $k = 1.01743$ to `Day 350` $k = 0.99578$) | $300 + 50 \left(\frac{1.01743 - 1.0}{1.01743 - 0.99578}\right)$ | **`340.254042 days`** | `340.2610853681755 days` | `-0.007044 days` |
| Burnup `Day 0` ($k = 1.19676$) to `Day 5` ($k = 1.15243$) net $\Delta\rho$ | $10^5 \left(\frac{1.15243 - 1}{1.15243} - \frac{1.19676 - 1}{1.19676}\right)$ | **`-3,214.223788 pcm`** | Not stored as a separate field (note: conversational summary wrote `-3,214.027 pcm`) | Exact Float64 value from displayed 5-decimal $k$ is `-3,214.224 pcm` |

**Isotopic Attribution Rule for Day 0 to Day 5 $\Delta\rho$ (`-3,214.224 pcm`):**
Between Day 0 and Day 5 (`50 MWd`), $\text{Xe-135}$ rises from `0.0 g` to `0.0577 g` and $\text{Sm-149}$ rises from `0.0 g` to `0.1401 g`, while $\text{U-235}$ simultaneously depletes by `-65.0169 g` (`10,402.9928 g` $\to$ `10,337.9759 g`), $\text{Pu-239}$ builds in by `+2.9668 g`, and other fission products accumulate. Computing $\Delta\rho(1.19676 \to 1.15243) = -3,214.224\text{ pcm}$ from the two total $k_{\mathrm{eff}}$ values quantifies **net core reactivity change**; it does not isolate how many pcm are caused by $\text{Xe-135}$ vs. $\text{Sm-149}$ vs. fuel burnup.

### 4.3 Red-Team Finding 3: Do Not Multiply MTR Full-Power Equilibrium $\text{Xe-135}$ by a MURR Shutdown Transient Multiplier

The proposed bridge expression:
```julia
xe135_post_shutdown_peak_g = xe135_eq_g[i] * xe_trans.xe135_peak_rel_eq
```
combines the 10 MW MTR full-power equilibrium $\text{Xe-135}$ inventory (`#burndata.core_g.Xe135`) with a constant post-shutdown peak multiplier (`11.82x`) computed from a separate 0D point-kinetics Bateman model parameterized for MURR's central flux-trap thermal flux ($\phi_0 = 3.0 \times 10^{14}\text{ n cm}^{-2}\text{ s}^{-1}$).

In coupled $\text{I-135} \to \text{Xe-135}$ decay dynamics following a step shutdown ($\phi \to 0$), the peak-to-equilibrium ratio depends strongly on the pre-shutdown neutron flux $\phi_0$ through the equilibrium $\text{I-135}$ to $\text{Xe-135}$ concentration ratio:

$$\frac{N_{\mathrm{I},\mathrm{eq}}}{N_{\mathrm{Xe},\mathrm{eq}}} = \frac{\gamma_{\mathrm{I}}}{\gamma_{\mathrm{I}} + \gamma_{\mathrm{Xe}}}\,\frac{\lambda_{\mathrm{Xe}} + \sigma_{a,\mathrm{Xe}}\,\phi_0}{\lambda_{\mathrm{I}}}$$

Because a 10 MW plate-type MTR core has a core-average thermal flux roughly an order of magnitude lower than $3.0 \times 10^{14}\text{ n cm}^{-2}\text{ s}^{-1}$, its post-shutdown $\text{Xe-135}$ peaking multiplier is substantially smaller than $11.82\times$ and varies across depletion states as the flux spectrum and fissile mix ($\text{U-235}$ vs. $\text{Pu-239}$ fission yields $\gamma_{\mathrm{I}}, \gamma_{\mathrm{Xe}}$) evolve. Full-power equilibrium inventories must therefore remain strictly separated from shutdown transient curves unless an actual MTR shutdown transient calculation is executed.

### 4.4 Red-Team Finding 4: Self-Correlation Uncertainty Bug at the `100%` Withdrawn Reference Point in `tecdoc643_reference_fixture.jl`

Executing `TECDOC643ReferenceFixture.blade_sweep()` in local `julia 1.13.1` produced the following five rows:

```text
Row 1 (0%):   k = 0.97943, worth = 18634.714116 pcm, worth_sigma = 120.374925 pcm (JSON: 120.0)
Row 2 (25%):  k = 1.02804, worth = 13806.992649 pcm, worth_sigma = 122.573603 pcm (JSON: 122.2)
Row 3 (50%):  k = 1.11336, worth =  6352.720926 pcm, worth_sigma = 109.277388 pcm (JSON: 109.3)
Row 4 (75%):  k = 1.17049, worth =  1968.818270 pcm, worth_sigma =  99.934755 pcm (JSON:  99.8)
Row 5 (100%): k = 1.19810, worth =     0.000000 pcm, worth_sigma =  79.802019 pcm (JSON:   0.0)
```

At Row 5 (`withdrawn_pct = 100.0`), `worth_to_fully_withdrawn_pcm` is `0.0 pcm` because state 5 is compared against itself (`delta_rho(k_out, k_out)`), yet `blade_sweep()` (line 72 of `tecdoc643_reference_fixture.jl`) calls `worth_sigma_pcm(SWEEP_K[5], SWEEP_SIGMA_K[5], k_out, sigma_out)` without checking whether $i$ is the reference state. Because `worth_sigma_pcm` assumes **independent** random variables ($\mathrm{Cov}(k_i, k_{\mathrm{out}}) = 0$), applying it to $(k_{\mathrm{out}}, k_{\mathrm{out}})$ computes $\sqrt{2}\,\sigma_{\mathrm{out}} / k_{\mathrm{out}}^2 \times 10^5 = 79.802019\text{ pcm}$ on an identically zero quantity $\rho(k_{\mathrm{out}}) - \rho(k_{\mathrm{out}}) \equiv 0$. Any Stage 2 ingestion or Julia benchmark module must set $\sigma(\Delta\rho_{\mathrm{ref} \to \mathrm{ref}}) = 0.0\text{ pcm}$ when comparing the reference state against itself.
