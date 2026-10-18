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

// ---------------------------------------------------------------- Crank-Nicolson

#[test]
fn thomas_solves_the_implicit_system() {
    for (name, col) in all_columns() {
        let op = operator(&col);
        let dt = col.dt_rule();
        let (m, _) = cn_matrices(&col, &op, dt);
        let x_true = test_profile(col.n);
        let rhs: Vec<f64> = (0..col.n)
            .map(|i| {
                let mut y = m.di[i] * x_true[i];
                if i > 0 {
                    y += m.lo[i] * x_true[i - 1];
                }
                if i + 1 < col.n {
                    y += m.up[i] * x_true[i + 1];
                }
                y
            })
            .collect();
        let mut x = rhs.clone();
        Thomas::new(&m).solve(&mut x);
        let err = x.iter().zip(&x_true).fold(0.0f64, |a, (p, q)| a.max((p - q).abs()));
        assert!(err <= 1e-13, "{name}: {err:e}");
    }
}

#[test]
fn one_step_satisfies_the_crank_nicolson_relation() {
    for (name, col) in all_columns() {
        let op = operator(&col);
        let dt = col.dt_rule();
        let integ = Integrator::new(&col, &op, dt);
        let c = test_profile(col.n);
        let mut c1 = vec![0.0; col.n];
        integ.step(&c, &mut c1);
        let mid: Vec<f64> = c.iter().zip(&c1).map(|(a, b)| 0.5 * (a + b)).collect();
        let rhs = op.apply(&mid);
        for i in 0..col.n {
            let lhs = col.r * (c1[i] - c[i]) / dt;
            assert!((lhs - rhs[i]).abs() <= 1e-11 * op.b0, "{name} cell {i}: {lhs:e} vs {:e}", rhs[i]);
        }
    }
}

#[test]
fn without_reaction_the_column_fills_to_c0_and_stays_in_bounds() {
    let col = Column::new(16.0, 64, 1e-9, 1e-9, 0.0, 1.0, vec![false; 64]).unwrap();
    let integ = Integrator::new(&col, &operator(&col), col.dt_rule());
    let (mut c, mut next) = (vec![0.0; 64], vec![0.0; 64]);
    let mut b = Bounds::default();
    // 200000 steps of 2e7 s: about 15 diffusion times L^2 / D and 250 travel times L / v.
    for step in 1..=200_000 {
        integ.step(&c, &mut next);
        std::mem::swap(&mut c, &mut next);
        b.observe_c(step, &c);
    }
    let worst = c.iter().fold(0.0f64, |a, x| a.max((x - C0).abs()));
    eprintln!("after 200000 steps max |C - C0| = {worst:e}; C in [{:e}, {:e}]", b.c_min, b.c_max);
    assert!(worst <= 1e-12, "{worst:e}");
    assert!(b.n1_pass(), "{:?}", b.first);
}

#[test]
fn bounds_flag_values_outside_n1() {
    let mut b = Bounds::default();
    b.observe_c(1, &[0.0, 0.5, 1.0, -1e-13, 1.0 + 1e-13]);
    b.observe_s(1, &[0.0, -1e-13 * S_REF]);
    assert!(b.n1_pass());
    b.observe_c(2, &[0.2, -2e-12]);
    b.observe_c(3, &[1.0 + 2e-12]);
    b.observe_s(4, &[-2e-12 * S_REF]);
    b.observe_c(5, &[f64::NAN]);
    assert_eq!(b.violations, 4);
    assert_eq!(b.first, Some(Violation { step: 2, cell: 1, what: "C below -1e-12 C0", value: -2e-12 }));
}

// ---------------------------------------------------------------- solid inventory and ledger

#[test]
fn solid_inventory_grows_only_in_the_zone_and_matches_the_reaction_sink() {
    for cfg in [RunConfig { v: 1e-9, d: 1e-10, k_red: 1e-9, kd: 0.0 }, RunConfig { v: 1e-10, d: 1e-9, k_red: 1e-10, kd: 630.0 }] {
        let col = cfg.column().unwrap();
        let r = run_production(&cfg).unwrap();
        let mut prev = vec![0.0; col.n];
        for snap in &r.snapshots {
            for i in 0..col.n {
                assert!(snap.s[i] >= prev[i], "{} S decreased at cell {i}", cfg.label());
                if !col.chi[i] {
                    assert_eq!(snap.s[i], 0.0, "{} S outside the zone", cfg.label());
                }
            }
            prev = snap.s.clone();
        }
        let last = r.ledger.last().unwrap();
        eprintln!("{}: immobilized {:e} mol/m2, inflow {:e}, outflow {:e}, closure {:e}", cfg.label(), last.immobilized, last.inflow, last.outflow, last.closure());
        assert!(last.immobilized > 0.0);
    }
}

#[test]
fn ledger_closes_on_representative_columns() {
    // One run per (v, D) pair at the largest k_red and K_d; every run is
    // checked by the ledger control test.
    for v in VS {
        for d in DS {
            let cfg = RunConfig { v, d, k_red: 1e-9, kd: 630.0 };
            let r = run_production(&cfg).unwrap();
            let (worst, pass) = r.ledger_check();
            eprintln!("{}: worst closure / inflow {worst:e}", cfg.label());
            assert!(pass, "{}: {worst:e}", cfg.label());
            assert_eq!(r.snapshots.len(), 6);
            assert_eq!(r.ledger.len(), 101);
            for (snap, tau) in r.snapshots.iter().zip(OUTPUT_TAUS) {
                assert!(close(snap.t, tau * cfg.tau(), 1e-12));
            }
        }
    }
}

#[test]
fn without_reaction_the_solid_stays_exactly_zero() {
    let cfg = RunConfig { v: 1e-9, d: 1e-9, k_red: 0.0, kd: 6.3 };
    let r = run_production(&cfg).unwrap();
    assert!(r.snapshots.iter().all(|s| s.s.iter().all(|x| *x == 0.0)));
    assert!(r.ledger.iter().all(|p| p.immobilized == 0.0));
}

// ---------------------------------------------------------------- B1 and T1

#[test]
fn erfc_matches_reference_values_on_both_branches() {
    // Reference values from CPython math.erfc / math.erf (C library).
    let refs = [(0.5, 0.4795001221869535), (1.0, 0.15729920705028516), (2.0, 0.0046777349810472645), (3.0, 2.2090496998585438e-05), (5.0, 1.537459794428035e-12)];
    for (x, want) in refs {
        assert!(close(erfc(x), want, 1e-14), "erfc({x}) = {:e} vs {want:e}", erfc(x));
        assert!(close(erfc(-x), 2.0 - want, 1e-15), "erfc(-{x})");
    }
    assert!(close(erfcx(5.0), 0.11070463773306864, 1e-14));
    assert!(close(erfcx(10.0), 0.05614099274382259, 1e-14));
    assert_eq!(erfc(0.0), 1.0);
    for x in [1.9, 1.99, 1.999999, 2.0, 2.000001, 2.01, 2.5] {
        let a = erfcx(x);
        let b = (x * x).exp() * erfc(x);
        assert!(close(a, b, 1e-13), "erfcx({x}) {a:e} vs exp(x^2) erfc(x) {b:e}");
    }
    // The two branches agree where they meet.
    let x = ERFC_SWITCH;
    let series = 1.0 - (1.0 - erfc(x - 1e-12));
    assert!(close(series, erfc(x), 1e-11), "{series:e} vs {:e}", erfc(x));
}

#[test]
fn ogata_banks_meets_its_boundary_conditions() {
    for t in [1e8, 1e9, B1_T] {
        assert!(close(ogata_banks(0.0, t, B1_V, B1_D), C0, 1e-14), "x = 0 at t = {t:e}");
        // At t*, x = 64 m is six spreads (2 sqrt(D t*) = 8 m) past the front, so
        // C / C0 is about erfc(6) / 2 + exp(-36) erfcx(10) / 2, near 2.4e-17.
        assert!(ogata_banks(B1_LENGTH, t, B1_V, B1_D) < 1e-15);
    }
    let mut prev = C0;
    for i in 1..=320 {
        let c = ogata_banks(0.1 * i as f64, B1_T, B1_V, B1_D);
        assert!(c <= prev && c >= 0.0, "not monotone at {i}");
        prev = c;
    }
    eprintln!("Ogata-Banks at the front x = vt = 16 m: {:.15}", ogata_banks(16.0, B1_T, B1_V, B1_D));
}

#[test]
fn b1_ogata_banks_spatial_order() {
    let r = b1();
    eprintln!("B1 steps {:?} dt {:?}", r.steps, r.dt);
    eprintln!("B1 L2 errors {:?} orders {:?} (pass band {:?}); N1 {}; worst ledger closure / inflow {:e}", r.errors, r.orders, ORDER_RANGE, r.n1_pass, r.ledger_worst);
    assert!(r.n1_pass, "N1 failed on B1");
    assert!(r.pass(), "B1 orders {:?} outside {:?}", r.orders, ORDER_RANGE);
}

#[test]
fn t1_temporal_order_on_b1() {
    let r = t1();
    eprintln!("T1 steps {:?} E0 {:e} E1 {:e} order {} (pass band {:?}); N1 {}", r.steps, r.e[0], r.e[1], r.order, ORDER_RANGE, r.n1_pass);
    assert_eq!(r.steps, [800, 1600, 3200]);
    assert!(r.n1_pass, "N1 failed on T1");
    assert!(r.pass(), "T1 order {} outside {:?}", r.order, ORDER_RANGE);
}

// ---------------------------------------------------------------- B2

#[test]
fn b2_exact_solution_satisfies_its_problem() {
    let (r1, r2) = b2_roots();
    // receipts/uranium_prereg_v1_freeze.txt
    assert!(close(r1, 1.0916079783099617, 1e-14) && close(r2, -0.09160797830996159, 1e-14), "{r1} {r2}");
    assert!(close(b2_exact(0.0), C0, 1e-14));
    // Analytic derivative of the exact solution: zero at the outlet.
    let l = B2_LENGTH;
    let den = r2 * (r2 * l).exp() - r1 * (r1 * l).exp();
    let dc = |x: f64| C0 * r1 * r2 * ((r2 * l).exp() * (r1 * x).exp() - (r1 * l).exp() * (r2 * x).exp()) / den;
    assert!(dc(l).abs() <= 1e-12 * dc(0.0).abs(), "outlet slope {:e} vs inlet {:e}", dc(l), dc(0.0));
    // It satisfies D C'' - v C' - k C = 0 (checked through the characteristic roots).
    for r in [r1, r2] {
        assert!((B2_D * r * r - B2_V * r - B2_K).abs() <= 1e-12 * B2_K);
    }
}

#[test]
fn b2_finite_domain_steady_state_order() {
    let r = b2();
    eprintln!("B2 L2 errors {:?} orders {:?} (pass band {:?}); max |A C + b| / b0 {:e}", r.errors, r.orders, ORDER_RANGE, r.residual);
    assert!(r.residual < 1e-12);
    assert!(r.pass(), "B2 orders {:?} outside {:?}", r.orders, ORDER_RANGE);
}
