# Darcy pre-registration v1, Amendment A3: the sign of the conservation residual

Status: frozen on 2026-10-18 at the commit that adds this file, before any Darcy solver code exists and before any run. It amends Darcy pre-registration v1 (docs/specs/DARCY_FLOW_PREREG_v1.md, sha256 e0a13e103f090ac6adacdc249a6d75ed99f56e14577d726490b24aa0a7f2e1ca). Amendment A1 (sha256 39a7ae88b7861f065e02488076e3f52f4a84709b744a433383f469e5df1c55db) and Amendment A2 (sha256 1d31ad5f334a07e096f71091b46cdde09298a4589a7ba73e415bd2d39803e1c3) are unchanged. A3 corrects residual bookkeeping only. It changes no physical problem, parameter, case, check, tolerance, stopping rule or output, and it makes no claim about actual Oklo conditions. Hunter's implementation approval of 2026-10-18 stands, conditional on this amendment being frozen first.

## A3.1 The error

v1 section 4 defines the cell residual as net outward flux minus source and writes the scheme as A h = b, with A symmetric positive definite and b holding the bottom boundary terms T_b Δh plus s_c Δx³. For every cell, (A h)_c is the sum over its faces of T (h_c - h_neighbour), with T_b h_c for each Dirichlet face, so (A h - b)_c = net outward flux of cell c - s_c Δx³. The physical residual is therefore A h - b. v1 equated it with b - A h, which is its negative. Hunter's review of 2026-10-18 found this before any code was written.

## A3.2 Definitions

- Physical conservation residual, signed: r_phys = A h - b = (net outward flux of each cell) - s_c Δx³, in m³/s.
- Solver residual: r_solv = b - A h = -r_phys. It is what conjugate gradient updates internally.

## A3.3 What the sign does not change

- The CG stopping rule stays exactly as pre-registered: ||b - A h||_2 ≤ 1e-12 ||b||_2, at most 10 N³ iterations. A norm does not depend on the sign.
- ε = ||r_phys||_∞ / Q_ref has the same value as it would with r_solv.
- The C4 bound ε ≤ 1e-12 ||b||_2 / Q_ref stands, because ||r_phys||_∞ = ||r_solv||_∞ ≤ ||r_solv||_2.
- D1's thresholds, the zero-flow control and every other check are unchanged.

## A3.4 D1, made consistent

The residual of D1 is r_phys, computed twice by independent code: once from the matrix as A h - b, and once by summing separately coded outward face fluxes and subtracting the source. The two signed vectors must agree to max |r_phys,face - r_phys,matrix| / Q_ref ≤ 1e-12, as pre-registered. Reported norms ||r||_∞ and ||r||_2 are those of r_phys.

## A3.5 Superseded v1 text, quoted exactly, and replacements

v1 line 83 contains: "and r = b - A h exactly: the linear-solver residual vector is the cell mass-imbalance vector, in m³/s." It becomes: "and r_phys = A h - b exactly: the physical residual is the negative of the linear-solver residual b - A h, and either has the norm of the cell mass-imbalance vector, in m³/s."

v1 line 90 begins: "Reconciling tolerances (C4). Because r = b - A h, the CG stopping rule bounds the D1 mass-balance error". It becomes: "Reconciling tolerances (C4). Because r_phys = -(b - A h), the CG stopping rule bounds the D1 mass-balance error". The rest of that paragraph stands.

v1 line 96 (D1) contains: "The residual is computed twice, once as b - A h from the matrix and once by summing separately coded face fluxes". It becomes: "The residual r_phys is computed twice, once as A h - b from the matrix and once by summing separately coded outward face fluxes minus the source". The rest of D1 stands.

Nothing else in v1, A1 or A2 changes.
