//! Darcy pre-registration v1 with A1 to A3: contract tests and checks D1 to D5.
//! Tolerances are the pre-registered constants in oklo_core::darcy; a failure
//! here is reported as a failure, never tuned away. Run with --release.
//! Hypothetical numerical experiment, not Oklo hydrogeology.

use oklo_core::darcy::*;
use oklo_core::mrc;

fn close(a: f64, b: f64, rel: f64) -> bool {
    (a - b).abs() <= rel * b.abs()
}

// ---------------------------------------------------------------- contract

#[test]
fn contract_permeability_digitization_reproduces_v1_table() {
    let (a, b) = calibration();
    eprintln!("calibration x = {a:.4} + {b:.5} log10(k_cm2)");
    assert!((a - 1652.11).abs() < 0.01 && (b - 88.771).abs() < 0.001);
    for (an, lo, rep, hi) in [
        (SANDSTONE_ANALOGUE, 8.54e-18, 1.27e-15, 1.90e-13),
        (SHALE_ANALOGUE, 6.64e-21, 7.65e-19, 8.82e-17),
        (FRACTURED_ANALOGUE, 1.29e-17, 1.54e-14, 1.83e-11),
    ] {
        let r = an.k_range();
        eprintln!("{}: low {:.6e} rep {:.6e} high {:.6e} m^2", an.class, r.low, r.rep, r.high);
        assert!(close(r.low, lo, 5e-3) && close(r.rep, rep, 5e-3) && close(r.high, hi, 5e-3), "{}", an.class);
        assert!(close(r.rep, (r.low * r.high).sqrt(), 1e-12));
    }
}

#[test]
fn contract_lambda_m_reproduces_a1_from_thermal_porosity_span() {
    let r = lambda_range();
    eprintln!("lambda_m low {:.5} rep {:.5} high {:.5} W/(m K) from phi_th {}..{} and quartz {:?}", r.low, r.rep, r.high, PHI_TH.lo, PHI_TH.hi, QUARTZ_SPAN_PCT);
    assert!((r.low - 1.12523).abs() < 5e-6);
    assert!((r.rep - 2.73931).abs() < 5e-6);
    assert!((r.high - 6.66867).abs() < 5e-6);
    assert!(close(r.rep, (r.low * r.high).sqrt(), 1e-15));
}

#[test]
fn contract_water_states_reproduce_v1_and_a1() {
    assert!(close(STATE_A.rho_g_over_mu(), 1.775126e7, 1e-6), "{}", STATE_A.rho_g_over_mu());
    assert!(close(STATE_B.rho_g_over_mu(), 7.893901e7, 1e-6), "{}", STATE_B.rho_g_over_mu());
    let ratio = STATE_B.rho_g_over_mu() / STATE_A.rho_g_over_mu();
    eprintln!("B/A flux factor {ratio:.6}");
    assert!((ratio - 4.44695).abs() < 1e-5);
    assert!(close(STATE_A.rho_cp(), 4.120844e6, 1e-6) && close(STATE_B.rho_cp(), 3.813582e6, 1e-6));
}

#[test]
fn contract_zone_mask_comes_from_the_fixture_null_scenario() {
    let mask = zone_mask_fixture();
    assert_eq!(mask.len(), N * N * N);
    let counts: Vec<usize> = ZONES.iter().map(|z| mask.iter().filter(|m| *m == z).count()).collect();
    eprintln!("zone counts sandstone/core/halo/conduit: {counts:?}");
    assert!(counts.iter().all(|c| *c > 0));
    let fine = refine_mask(&mask, N);
    assert_eq!(fine.len(), 8 * N * N * N);
    assert_eq!(fine[idx(2 * N, 7, 9, 31)], mask[idx(N, 3, 4, 15)]);
}

#[test]
fn contract_two_porosity_roles_stay_separate() {
    // phi_z: per zone, used only for pore velocity; only the core is sourced.
    for z in ZONES {
        let p = flow_porosity(z);
        assert_eq!(p.sourced, z == Zone::Core, "{z:?}");
    }
    assert_eq!(flow_porosity(Zone::Core).value, 0.125);
    // phi_th: a span used only for lambda_m.
    assert_eq!((PHI_TH.lo, PHI_TH.hi), (0.10, 0.25));
}

#[test]
fn contract_eleven_configurations() {
    let c = configurations();
    let names: Vec<&str> = c.iter().map(|x| x.name.as_str()).collect();
    eprintln!("configurations: {names:?}");
    assert_eq!(c.len(), 11);
}

#[test]
fn encode_array_writes_any_array_with_labels() {
    let data: Vec<f32> = (0..60).map(|v| v as f32 * 0.5 - 3.0).collect();
    let file = mrc::encode_array([3, 4, 5], 0.25, &data, &["first label", CAVEAT]);
    assert_eq!(file.len(), 1024 + 60 * 4);
    let i32_at = |o: usize| i32::from_le_bytes(file[o..o + 4].try_into().unwrap());
    let f32_at = |o: usize| f32::from_le_bytes(file[o..o + 4].try_into().unwrap());
    assert_eq!([i32_at(0), i32_at(4), i32_at(8), i32_at(12)], [3, 4, 5, 2]);
    assert_eq!([i32_at(28), i32_at(32), i32_at(36)], [3, 4, 5]);
    assert_eq!([f32_at(40), f32_at(44), f32_at(48)], [0.75, 1.0, 1.25]);
    assert_eq!([i32_at(64), i32_at(68), i32_at(72), i32_at(88), i32_at(92), i32_at(108)], [1, 2, 3, 1, 0, 20140]);
    assert_eq!(i32_at(220), 2);
    assert_eq!(String::from_utf8_lossy(&file[224..304]).trim_end(), "first label");
    assert_eq!(String::from_utf8_lossy(&file[304..384]).trim_end(), CAVEAT);
    assert_eq!(&file[208..212], b"MAP ");
    assert_eq!([f32_at(76), f32_at(80)], [-3.0, 26.5]);
    let back: Vec<f32> = file[1024..].chunks_exact(4).map(|b| f32::from_le_bytes(b.try_into().unwrap())).collect();
    assert_eq!(back, data);
}

// ---------------------------------------------------------------- D1 to D5

fn base_problem() -> (Vec<Zone>, Config, Problem) {
    let mask = zone_mask_fixture();
    let cfg = configurations().into_iter().next().unwrap();
    let p = Problem::from_config(N, &mask, &cfg, &STATE_A, DH);
    (mask, cfg, p)
}

#[test]
fn d1_conservation_base_case() {
    let (_, _, p) = base_problem();
    let s = cg(&p);
    let d = d1(&p, &s);
    eprintln!(
        "D1 base: converged {} iterations {} restarts {} rel_residual {:.3e}; Q_in {:.6e} Q_out {:.6e} m^3/s; eps {:.3e}; global {:.3e}; paths {:.3e}; ||b||/Q_ref {:.3}",
        s.converged, s.iterations, s.restarts, s.rel_residual, d.q_in, d.q_out, d.eps, d.global, d.paths, d.b_norm_over_q_ref
    );
    assert!(s.converged, "CG did not converge within the pre-registered cap");
    assert!(d.pass(), "D1 failed: {d:?}");
}

#[test]
fn d1_zero_flow_control() {
    let (mask, cfg, _) = base_problem();
    let (ok, s) = d1_zero_flow(&mask, &cfg);
    eprintln!("D1 zero flow: h and r_phys identically zero: {ok}, iterations {}", s.iterations);
    assert!(ok);
}

#[test]
fn d2_manufactured_solution_converges_at_second_order() {
    let levels: Vec<D2Level> = [16, 32, 64].into_iter().map(d2_level).collect();
    for l in &levels {
        eprintln!("D2 N={}: E2 {:.6e} Einf {:.6e} iterations {} converged {}", l.n, l.e2, l.einf, l.solve_iterations, l.converged);
        assert!(l.converged);
    }
    let p1 = (levels[0].e2 / levels[1].e2).log2();
    let p2 = (levels[1].e2 / levels[2].e2).log2();
    let q1 = (levels[0].einf / levels[1].einf).log2();
    let q2 = (levels[1].einf / levels[2].einf).log2();
    eprintln!("D2 observed L2 orders {p1:.4}, {p2:.4}; Einf orders (reported, no criterion) {q1:.4}, {q2:.4}");
    let (lo, hi) = D2_ORDER_RANGE;
    assert!((lo..=hi).contains(&p1) && (lo..=hi).contains(&p2), "D2 order outside [{lo}, {hi}]");
}

#[test]
fn d3a_layers_in_series() {
    let d = d3(true);
    eprintln!("D3a series: Q rel direct {:.3e}, heads direct {:.3e} dh, Q rel CG {:.3e} (converged {})", d.q_rel_direct, d.head_err_direct, d.q_rel_cg, d.cg_converged);
    assert!(d.pass());
}

#[test]
fn d3b_layers_in_parallel() {
    let d = d3(false);
    eprintln!("D3b parallel: Q rel direct {:.3e}, heads direct {:.3e} dh, Q rel CG {:.3e} (converged {})", d.q_rel_direct, d.head_err_direct, d.q_rel_cg, d.cg_converged);
    assert!(d.pass());
}

#[test]
fn d4_mirror_symmetry_base_case() {
    let (_, _, p) = base_problem();
    let s = cg(&p);
    let d = d4(&p, &s);
    eprintln!("D4 base: head x {:.3e} y {:.3e} (over dh); flux x {:.3e} y {:.3e} (over Q_ref)", d.head_x, d.head_y, d.flux_x, d.flux_y);
    assert!(d.pass());
}

#[test]
fn d5_equal_permeability_control() {
    let mask = zone_mask_fixture();
    let d = d5(&mask);
    let with_conduit = d.layers.iter().filter(|(_, a)| *a > 0.0).count();
    eprintln!("D5: max |flux share - area share| {:.3e} over {} layers ({} contain conduit), CG converged {}", d.max_dev, d.layers.len(), with_conduit, d.converged);
    assert_eq!(d.layers.len(), N);
    assert!(d.pass());
}
