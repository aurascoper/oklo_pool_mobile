# Darcy pre-registration v1, Amendment A4: a tighter solver rule for a separate verification rerun

Status: frozen on 2026-10-18 at the commit that adds this file, before any code change for it and before any rerun. It amends Darcy pre-registration v1 (sha256 e0a13e103f090ac6adacdc249a6d75ed99f56e14577d726490b24aa0a7f2e1ca); Amendments A1 (39a7ae88b7861f065e02488076e3f52f4a84709b744a433383f469e5df1c55db), A2 (1d31ad5f334a07e096f71091b46cdde09298a4589a7ba73e415bd2d39803e1c3) and A3 (54c0e72387cbf6583edd404231e1fb06455a9eea9768be0f33ee1fba7241dc75) are unchanged. It authorizes no parameter fitting and makes no claim about actual Oklo conditions.

## A4.1 Why

The v1 run (commit 075734a) recorded two failures: D3a's conjugate-gradient flux error, 2.58e-10 against 1e-10, while the direct solve of the same problem passed at 3.1e-13; and D1's global balance in configuration ratio_1, 1.68e-10 against 1e-10. Both are consistent with the pre-registered CG stopping rule, ||b - A h||_2 ≤ 1e-12 ||b||_2, being too loose to guarantee those acceptance tolerances. A4 asks one question: do both checks pass when the iterative solve alone is made more accurate, with everything else held fixed?

## A4.2 The only change

For the A4 rerun, the CG acceptance rule is ||b - A h||_2 ≤ 1e-13 ||b||_2 on the true residual, in place of 1e-12. Everything else is unchanged: the iteration cap of 10 N³, no preconditioner, the initial guess, acceptance on the true residual with restart on mismatch, every equation, parameter, configuration, check, acceptance threshold (including D1's 1e-10 and 1e-12 and D3's 1e-12 and 1e-10), output and label. The C4 bound becomes ε ≤ 1e-13 ||b||_2 / Q_ref.

The tenfold tightening is a proposed numerical setting, not a guarantee. It may be at or below what float64 CG can attain on some problems; reaching the iteration cap is then a recorded non-convergence.

## A4.3 What runs, and how it is recorded

- The complete pre-registered Darcy suite reruns under the A4 rule, not only the two failed checks: tests/darcy.rs (contract tests and D1 to D5) and examples/darcy_run.rs (all eleven configurations at both water states, the zero-flow control, D1 per configuration, D2 to D5, the refinement study and the Péclet diagnostic). The other suites (conformance, step) run alongside, unchanged.
- The rule is selected explicitly in code: the tolerance becomes a parameter, with v1's 1e-12 and A4's 1e-13 as named constants, chosen by the environment variable OKLO_DARCY_RULE ("v1" or "a4") for the tests and by an argument for the run. Every log and receipt states which rule it used. v1's results stay reproducible by selecting "v1".
- The v1 receipt (receipts/darcy_v1_run_2026-10-18.json) and test log (receipts/darcy_v1_tests_2026-10-18.txt) are preserved unchanged; their hashes are recorded in the A4 freeze receipt and rechecked after the rerun. A4 results go to separate files: receipts/darcy_v1_A4_run_2026-10-18.json and receipts/darcy_v1_A4_tests_2026-10-18.txt, and outputs to out/darcy_A4.
- Every outcome is recorded as it falls: new passes, failures that remain, new failures, and any non-convergence.

## A4.4 How the result is read, declared now

- The A4 rerun is a new verification experiment. It is not a retroactive validation of v1. v1's two failures remain the result of v1 under v1's rule, permanently.
- If D3a and D1 for ratio_1 both pass under A4 and nothing else changes status, the result is consistent with the v1 failures arising from iterative-solver accuracy alone.
- If either still fails, or a check that passed under v1 fails under A4, or a configuration does not converge, that outcome stands and stays visible.
- After the rerun, no further change to the solver (preconditioning, another tolerance, other settings) is made on the basis of these results without another explicit, frozen amendment.
