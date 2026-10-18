# Uranium column pre-registration v1, Amendment A1: time step, landing, non-negativity and a temporal-accuracy check

Status: frozen on 2026-10-18 at the commit that adds this file, before any solver code exists. It amends pre-registration v1 (docs/specs/URANIUM_REDOX_TRANSPORT_PREREG_v1.md, sha256 5370767fa8e64a607f7e169901f05d1ac1514ad699ce4721c2e39444591002e4), which stays unchanged. It changes no equation, parameter, sweep value, benchmark problem, tolerance of an existing check, or pass band. It changes the time-step rule, fixes how output times are reached, pins one floating-point evaluation, adds two checks and makes control C2 sound. Like v1, it authorizes no implementation, no parameter fitting and no claim about Oklo; implementation needs Hunter's separate go-ahead. The parameter manifest docs/specs/URANIUM_REDOX_TRANSPORT_PREREG_v1_PARAMETERS.json is frozen with it and adds no value.

## A1.1 Why

Hunter's reviewer found on 2026-10-18 that v1's step, Δt = 0.5 R Δx / v, gives a diffusion number d = D_eff Δt / (R Δx²) = 1/(2 Pe_cell) with no upper bound. In the sweep d reaches 20 (v = 1e-10, D_eff = 1e-9), and in B1 it doubles with each refinement (2, 4, 8). Crank-Nicolson is stable there, but its highest-frequency growth factor (1 - 2d)/(1 + 2d) approaches -1, and the column starts with a jump between the initial and inlet conditions, the case in which weakly damped oscillations are known to appear (Østerby, Five ways of reducing the Crank-Nicolson oscillations: equation 5, Table 1 and page 2). Checking that issue found three more: control C2(b) assumes that the correct scheme keeps S ≥ 0, which v1's step does not guarantee; v1 does not say how output times are reached; and the grid rule sits on Pe_cell = 2, where f64 rounding decides N. The analysis is in docs/research/numerical_method_review_v1.md, and the arithmetic is in receipts/uranium_a1_arithmetic_2026-10-18.txt, produced by scripts/uranium_a1_arithmetic.py.

## A1.2 Time step

v1 section 3 reads:

> - Time integration by Crank-Nicolson, with Δt = 0.5 R Δx / v (a Courant number of 0.5 on the retarded velocity), so Δt ∝ Δx.

It is superseded by: Time integration by Crank-Nicolson, with the step bounded by Δt_rule = min(0.5 R Δx / v, R / max_i |A_ii|).

Here A is the tridiagonal operator of v1's semi-discrete system R dC/dt = A C + b, built from v1's face rules without change, and its diagonal magnitudes are:

- interior cell: |A_ii| = 2 D_eff / Δx² + k_red χ_i;
- inlet cell: |A_11| = 3 D_eff / Δx² + v / (2 Δx) + k_red χ_1;
- outlet cell: |A_NN| = D_eff / Δx² + v / (2 Δx) + k_red χ_N.

χ_i is χ_org evaluated at cell centre i.

The derivation is in the numerical-method review. With Pe_cell ≤ 2 every off-diagonal entry of A is non-negative. Under the bound, the explicit matrix R I + (Δt/2) A has every diagonal entry at least R/2 and is entrywise non-negative. The implicit matrix R I - (Δt/2) A is a strictly diagonally dominant M-matrix. Therefore 0 ≤ C^n ≤ C0 implies 0 ≤ C^(n+1) ≤ C0, and S never decreases. For interior pure diffusion the bound is d ≤ 1/2, where every Crank-Nicolson growth factor is non-negative. The first term keeps v1's Courant limit, so Δt = O(Δx).

v1 section 3 also reads:

> with Δt ∝ Δx the combined error is O(Δx²), an expected order of 2.

"with Δt ∝ Δx" becomes "with Δt = O(Δx), as A1.2 keeps Δt ≤ 0.5 R Δx / v". The expected order stays 2.

In C2, the corrupted variants use the step computed from the uncorrupted operator, so each differs from the correct model only by its sign flip.

## A1.3 Landing on output times

Every run uses one constant step. In production, I = 0.05 τ with τ = R L_col / v, m = ceil(I / Δt_rule), Δt = I / m and T_end = 100 m steps. All six output times of v1 section 4 (0.1, 0.25, 0.5, 1, 2 and 5 travel times) are then whole numbers of steps: 2m, 5m, 10m, 20m, 40m and 100m. In B1, m = ceil(t* / Δt_rule) and Δt = t* / m. B2 is the steady discrete system and has no step.

## A1.4 Grid determinism

v1 section 3 reads:

> N is therefore max(64, ceil(Pe_s / 2)), a rule fixed here, not a tuned choice.

The rule stands. A1 adds the evaluation order: Pe_s is computed in IEEE 754 binary64 as (v × L_col) / D_eff, in that order, and ceil is applied to that value divided by 2. For the nine (v, D_eff) pairs, in the order v = 1e-10, 1e-9, 1e-8 and, within each, D_eff = 1e-11, 1e-10, 1e-9, this gives N = 81, 64, 64, 801, 80, 64, 8001, 800, 80. The implementation must reproduce these nine values. A mismatch is recorded as a failure of A1.4, and the run uses the N the rule produces.

## A1.5 New check N1, non-negativity and boundedness

N1 applies to every run of the uncorrupted model: all 144 production runs (which include C1's k_red = 0 runs), the three B1 grids, the T1 runs and the uncorrupted arms of C2. At every step and in every cell it requires:

- -1e-12 C0 ≤ C ≤ (1 + 1e-12) C0;
- S ≥ -1e-12 S_ref, where S_ref = θ C0 / ρ_b = 6.659267e-5 mol/kg.

The allowances are declared floating-point margins, fixed before any code. They are not tuned. Pass: no violation in any step of any of these runs. A violation is recorded with its run, step, cell and value, and N1 fails.

## A1.6 New check T1, temporal accuracy

T1 solves the B1 problem (v1 section 6: R = 1, k_red = 0, v = D = 1e-9, L_B1 = 64 m, t* = 1.6e10 s) at N = 256 with three steps: Δt_0 = t* / m_0, where m_0 = ceil(t* / Δt_rule), then Δt_0 / 2 and Δt_0 / 4. The A1 arithmetic gives m_0 = 800.

Let C_j be the solution at t* with step Δt_0 / 2^j. Let E_j = sqrt(Σ Δx (C_j - C_(j+1))²) over the cells whose centres lie in the comparison window 0 to 32 m. The differences cancel the spatial error, which all three runs share.

Pass: the observed temporal order p = log2(E_0 / E_1) is in [1.8, 2.2]. If the non-smooth start lowers the order, T1 fails and stays failed. B1's own refinement check and its pass band are unchanged.

## A1.7 Control C2 made sound

v1 section 6 reads:

> Detection rule, declared now: variant (a) is detected when the ledger closure error exceeds 1e-10 times the cumulative inflow; variant (b) when S is negative anywhere. C2 passes only if both variants are detected in a run with k_red = 1e-10 1/s; a corrupted variant that goes undetected is a failure of the checks.

Two clauses are superseded.

"variant (b) when S is negative anywhere" becomes "variant (b) when S < -1e-12 S_ref in any cell at any step".

"C2 passes only if both variants are detected in a run with k_red = 1e-10 1/s" becomes: "C2 is run in all 36 production configurations with k_red = 1e-10 1/s (3 v × 3 D_eff × 4 K_d). In each configuration the uncorrupted model and both variants are run. C2 passes only if, in every one of the 36 configurations, both variants are detected and the uncorrupted model triggers neither detection rule."

Variant (a)'s rule and the rest of C2 are unchanged.

## A1.8 What stays the same

Everything else in v1 is unchanged:
- the equations, conditions and face rules;
- the parameters, the 144-run sweep and T_end;
- B1's problem, grids, window and pass band, and B2's problem, grids and pass band;
- the ledger and its 1e-10 closure;
- C1;
- the outputs.

The JSON receipt of v1 section 7 also reports, for every run, m, Δt, whether the advective or the positivity term of Δt_rule was binding, and N1's status. It reports T1's three step counts, E_0, E_1 and p, A1.4's nine N values, and C2's result for each of the 36 configurations.
