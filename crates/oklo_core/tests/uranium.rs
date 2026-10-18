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
