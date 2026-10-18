//! Uranium pre-registration v1 with Amendment A1: contract tests and the
//! pre-registered checks. Thresholds are the constants in oklo_core::uranium;
//! a failure here is reported as a failure, never tuned away. Run with
//! --release. A synthetic column, not Oklo or the Franceville Basin.

use oklo_core::uranium::*;

fn close(a: f64, b: f64, rel: f64) -> bool {
    (a - b).abs() <= rel * b.abs()
}

// ---------------------------------------------------------------- contract

#[test]
fn contract_constants_reproduce_v1_and_the_manifest() {
    assert_eq!(RHO_B, 2252.5);
    let r: Vec<f64> = KDS.iter().map(|&kd| retardation(kd)).collect();
    eprintln!("R by K_d: {r:?}");
    for (got, want) in r.iter().zip([1.0, 947.05, 94606.0, 9460501.0]) {
        assert!(close(*got, want, 1e-12), "{got} vs {want}");
    }
    assert!(close(S_REF, 6.659267e-5, 1e-6), "{S_REF}");
}

#[test]
fn contract_sweep_is_the_full_factorial_of_144_runs() {
    let s = sweep();
    assert_eq!(s.len(), 144);
    for (i, a) in s.iter().enumerate() {
        for b in &s[i + 1..] {
            assert_ne!(a, b);
        }
    }
    assert_eq!(s[0], RunConfig { v: 1e-10, d: 1e-11, k_red: 0.0, kd: 0.0 });
    assert_eq!(s[143], RunConfig { v: 1e-8, d: 1e-9, k_red: 1e-9, kd: 630.0 });
}

#[test]
fn contract_grid_rule_reproduces_the_nine_a1_values() {
    let want = [81, 64, 64, 801, 80, 64, 8001, 800, 80];
    let mut got = Vec::new();
    for v in VS {
        for d in DS {
            got.push(grid_n(v, d, L_COL));
        }
    }
    eprintln!("N by (v, D): {got:?}");
    assert_eq!(got, want);
    for cfg in sweep() {
        let col = cfg.column().expect("valid column");
        assert!(col.pe_cell() <= 2.0, "{}: Pe_cell {}", cfg.label(), col.pe_cell());
    }
}

#[test]
fn contract_organic_mask_marks_cell_centres_in_6_to_10_m() {
    let m = organic_mask(64, L_COL);
    let idx: Vec<usize> = (0..64).filter(|&i| m[i]).collect();
    assert_eq!(idx, (24..40).collect::<Vec<_>>());
    for n in [80, 81, 800, 801, 8001] {
        let dx = L_COL / n as f64;
        for (i, &c) in organic_mask(n, L_COL).iter().enumerate() {
            let x = (i as f64 + 0.5) * dx;
            assert_eq!(c, (6.0..=10.0).contains(&x));
        }
    }
}

#[test]
fn contract_step_obeys_both_a1_limits_and_lands_on_output_times() {
    for cfg in sweep() {
        let col = cfg.column().unwrap();
        let dx = col.dx();
        let adv = 0.5 * col.r * dx / col.v;
        let max_diag = (0..col.n)
            .map(|i| {
                let k = if col.chi[i] { col.k_red } else { 0.0 };
                match i {
                    0 => 3.0 * col.d / (dx * dx) + col.v / (2.0 * dx) + k,
                    i if i + 1 == col.n => col.d / (dx * dx) + col.v / (2.0 * dx) + k,
                    _ => 2.0 * col.d / (dx * dx) + k,
                }
            })
            .fold(0.0, f64::max);
        let pos = col.r / max_diag;
        assert_eq!(col.dt_rule(), adv.min(pos));
        let l = production_landing(&cfg, &col);
        assert!(l.dt <= adv * (1.0 + N1_REL) && l.dt <= pos * (1.0 + N1_REL), "{}", cfg.label());
        assert!(close(l.dt * l.m as f64, LANDING_FRACTION * cfg.tau(), 1e-12));
        assert!(close(100.0 * l.m as f64 * l.dt, 5.0 * cfg.tau(), 1e-12));
        for (k, t) in OUTPUT_INTERVALS.iter().zip(OUTPUT_TAUS) {
            assert!(close((*k * l.m) as f64 * l.dt, t * cfg.tau(), 1e-12));
        }
    }
}

#[test]
fn contract_step_counts_match_the_a1_arithmetic_record() {
    // receipts/uranium_a1_arithmetic_2026-10-18.txt, steps_A1 (= 100 m).
    let want = [(1e-10, 1e-11, 0.0, 900), (1e-10, 1e-11, 1e-9, 1300), (1e-10, 1e-10, 0.0, 4000), (1e-10, 1e-9, 0.0, 38600), (1e-9, 1e-11, 0.0, 8100), (1e-9, 1e-10, 0.0, 800), (1e-8, 1e-11, 0.0, 80100), (1e-8, 1e-10, 0.0, 8000), (1e-8, 1e-9, 0.0, 800)];
    for (v, d, k_red, steps) in want {
        for kd in KDS {
            let cfg = RunConfig { v, d, k_red, kd };
            let l = production_landing(&cfg, &cfg.column().unwrap());
            assert_eq!(100 * l.m, steps, "{}", cfg.label());
        }
    }
}

#[test]
fn contract_validation_rejects_invalid_columns() {
    let ok = |n: usize| organic_mask(n, L_COL);
    assert!(Column::new(L_COL, 64, 1e-9, 1e-9, 0.0, 1.0, ok(64)).is_ok());
    let cases = [
        (Column::new(0.0, 64, 1e-9, 1e-9, 0.0, 1.0, ok(64)), ColumnError::LengthNotPositive),
        (Column::new(L_COL, 1, 1e-9, 1e-9, 0.0, 1.0, ok(1)), ColumnError::TooFewCells),
        (Column::new(L_COL, 64, 0.0, 1e-9, 0.0, 1.0, ok(64)), ColumnError::VelocityNotPositive),
        (Column::new(L_COL, 64, -1e-9, 1e-9, 0.0, 1.0, ok(64)), ColumnError::VelocityNotPositive),
        (Column::new(L_COL, 64, f64::NAN, 1e-9, 0.0, 1.0, ok(64)), ColumnError::VelocityNotPositive),
        (Column::new(L_COL, 64, 1e-9, 0.0, 0.0, 1.0, ok(64)), ColumnError::DiffusionNotPositive),
        (Column::new(L_COL, 64, 1e-9, f64::INFINITY, 0.0, 1.0, ok(64)), ColumnError::DiffusionNotPositive),
        (Column::new(L_COL, 64, 1e-9, 1e-9, -1e-10, 1.0, ok(64)), ColumnError::RateNegative),
        (Column::new(L_COL, 64, 1e-9, 1e-9, 0.0, 0.5, ok(64)), ColumnError::RetardationBelowOne),
        (Column::new(L_COL, 64, 1e-9, 1e-9, 0.0, 1.0, ok(63)), ColumnError::MaskLength),
        (Column::new(L_COL, 64, 1e-8, 1e-11, 0.0, 1.0, ok(64)), ColumnError::CellPecletAboveTwo),
    ];
    for (got, want) in cases {
        assert_eq!(got.unwrap_err(), want);
    }
    assert_eq!(RunConfig { v: 1e-9, d: 1e-9, k_red: 0.0, kd: -1.0 }.column().unwrap_err(), ColumnError::RetardationBelowOne);
}

// ---------------------------------------------------------------- operator

/// The 144 production columns plus the benchmark columns of v1 section 6
/// (B1: 64 m, no reaction; B2: 16 m, reaction everywhere).
fn all_columns() -> Vec<(String, Column)> {
    let mut out: Vec<(String, Column)> = sweep().iter().map(|c| (c.label(), c.column().unwrap())).collect();
    for n in [256, 512, 1024] {
        out.push((format!("B1 N={n}"), Column::new(64.0, n, 1e-9, 1e-9, 0.0, 1.0, vec![false; n]).unwrap()));
    }
    for n in [64, 128, 256] {
        out.push((format!("B2 N={n}"), Column::new(16.0, n, 1e-9, 1e-9, 1e-10, 1.0, vec![true; n]).unwrap()));
    }
    out
}

/// Deterministic profile in [0, C0] for consistency checks (no dependencies).
fn test_profile(n: usize) -> Vec<f64> {
    let mut x: u64 = 0x9E3779B97F4A7C15;
    (0..n)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            C0 * (x >> 11) as f64 / (1u64 << 53) as f64
        })
        .collect()
}

#[test]
fn operator_from_face_rules_matches_the_a1_diagonals() {
    for (name, col) in all_columns() {
        let op = operator(&col);
        let (v, d, dx, n) = (col.v, col.d, col.dx(), col.n);
        for i in 0..n {
            let k = if col.chi[i] { col.k_red } else { 0.0 };
            let want = match i {
                0 => 3.0 * d / (dx * dx) + v / (2.0 * dx) + k,
                i if i + 1 == n => d / (dx * dx) + v / (2.0 * dx) + k,
                _ => 2.0 * d / (dx * dx) + k,
            };
            assert!(close(-op.a.di[i], want, 1e-14), "{name} cell {i}: {} vs {want}", -op.a.di[i]);
            if i > 0 {
                assert!(close(op.a.lo[i], (v / 2.0 + d / dx) / dx, 1e-14), "{name} lo {i}");
            }
            if i + 1 < n {
                let want_up = (d / dx - v / 2.0) / dx;
                assert!((op.a.up[i] - want_up).abs() <= 1e-14 * (d / (dx * dx)), "{name} up {i}");
            }
        }
        assert!(close(op.b0, (v / dx + 2.0 * d / (dx * dx)) * C0, 1e-14), "{name} b0");
    }
}

#[test]
fn operator_off_diagonals_are_non_negative_in_binary64() {
    for (name, col) in all_columns() {
        let op = operator(&col);
        let lo_min = op.a.lo[1..].iter().cloned().fold(f64::INFINITY, f64::min);
        let up_min = op.a.up[..col.n - 1].iter().cloned().fold(f64::INFINITY, f64::min);
        assert!(lo_min >= 0.0 && up_min >= 0.0, "{name}: lo {lo_min:e} up {up_min:e}");
    }
}

#[test]
fn crank_nicolson_matrices_have_the_a1_sign_structure_at_the_landed_step() {
    let mut worst_diag = f64::INFINITY;
    for cfg in sweep() {
        let col = cfg.column().unwrap();
        let dt = production_landing(&cfg, &col).dt;
        let (m, e) = cn_matrices(&col, &operator(&col), dt);
        for i in 0..col.n {
            worst_diag = worst_diag.min(e.di[i] / col.r);
            assert!(e.di[i] >= 0.5 * col.r * (1.0 - N1_REL), "{} E diag {i}: {}", cfg.label(), e.di[i] / col.r);
            assert!(e.lo[i] >= 0.0 && e.up[i] >= 0.0, "{} E off-diagonal {i}", cfg.label());
            assert!(m.lo[i] <= 0.0 && m.up[i] <= 0.0 && m.di[i] > 0.0, "{} M signs {i}", cfg.label());
            assert!(m.di[i] > -m.lo[i] - m.up[i], "{} M not strictly dominant in row {i}", cfg.label());
        }
    }
    eprintln!("min over all runs and cells of E_ii / R: {worst_diag:?}");
}

#[test]
fn uniform_inlet_profile_is_steady_without_reaction() {
    for (name, col) in all_columns().into_iter().filter(|(_, c)| c.k_red == 0.0) {
        let op = operator(&col);
        let r = op.apply(&vec![C0; col.n]);
        let worst = r.iter().fold(0.0f64, |a, x| a.max(x.abs()));
        assert!(worst <= 1e-12 * op.b0, "{name}: {worst:e}");
    }
}

#[test]
fn face_fluxes_are_consistent_with_the_operator_and_signed_correctly() {
    for (name, col) in all_columns() {
        let (n, dx) = (col.n, col.dx());
        let op = operator(&col);
        let c = test_profile(n);
        let f = face_fluxes(&col, &c);
        assert_eq!(f.len(), n + 1);
        let ac = op.apply(&c);
        for i in 0..n {
            let k = if col.chi[i] { col.k_red } else { 0.0 };
            let div = (f[i] - f[i + 1]) / dx - k * c[i];
            assert!((div - ac[i]).abs() <= 1e-12 * op.b0, "{name} cell {i}");
        }
        assert_eq!(f[n], col.v * c[n - 1]);
        let empty = face_fluxes(&col, &vec![0.0; n]);
        assert!(close(empty[0], (col.v + 2.0 * col.d / dx) * C0, 1e-15) && empty[0] > 0.0, "{name}");
        assert!(empty[1..].iter().all(|x| *x == 0.0));
        let full = face_fluxes(&col, &vec![C0; n]);
        assert!(full.iter().all(|x| close(*x, col.v * C0, 1e-12)), "{name}");
    }
}
