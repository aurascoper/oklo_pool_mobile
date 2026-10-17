//! The advance_phase contract (lib.rs) and the step path through Sim and the C ABI.
//! Stepping is navigation along the revision 0 cycle; it never changes the fields'
//! equations, only the phase they are evaluated at.

use oklo_core::ffi::*;
use oklo_core::{advance_phase, compute_field, ConfigError, Field, Sim, SimConfig};

const T: f64 = 10800.0;

fn cfg(phase: f64) -> SimConfig {
    SimConfig { nx: 32, ny: 32, nz: 32, phase, null_scenario: false, pulse_boundary: 0.17, cycle_period_s: T }
}

#[test]
fn zero_step_keeps_phase() {
    assert_eq!(advance_phase(0.15, 0.0, T), Ok(0.15));
    assert_eq!(advance_phase(0.15, -0.0, T), Ok(0.15));
}

#[test]
fn small_forward_step() {
    assert_eq!(advance_phase(0.15, 60.0, T), Ok(0.15 + 60.0 / T));
}

#[test]
fn full_period_from_zero_returns_zero() {
    assert_eq!(advance_phase(0.0, T, T), Ok(0.0));
}

#[test]
fn phase_one_is_accepted_and_canonicalised() {
    assert_eq!(advance_phase(1.0, 0.0, T), Ok(0.0));
}

#[test]
fn full_and_multiple_periods_return_to_start() {
    // 0.15 + 1 is not exactly representable, so the wrapped value can differ
    // from 0.15 in the last bits; 1e-12 bounds that rounding.
    let p = advance_phase(0.15, T, T).unwrap();
    assert!((p - 0.15).abs() < 1e-12, "{p}");
    let p = advance_phase(0.25, 7.5 * T, T).unwrap();
    assert!((p - 0.75).abs() < 1e-12, "{p}");
}

#[test]
fn step_across_the_cycle_end_wraps() {
    let p = advance_phase(0.95, 0.1 * T, T).unwrap();
    assert!((p - 0.05).abs() < 1e-12, "{p}");
}

#[test]
fn invalid_inputs_are_errors_not_panics() {
    for period in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert_eq!(advance_phase(0.1, 1.0, period), Err(ConfigError::PeriodNotPositive), "period {period}");
    }
    for dt in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(advance_phase(0.1, dt, T), Err(ConfigError::StepNotFinite), "dt {dt}");
    }
    assert_eq!(advance_phase(0.1, -1.0, T), Err(ConfigError::NegativeStep));
    for phase in [-0.1, 1.5, f64::NAN] {
        assert_eq!(advance_phase(phase, 1.0, T), Err(ConfigError::PhaseOutOfRange), "phase {phase}");
    }
    // dt / period overflows to infinity: reported, not propagated.
    assert_eq!(advance_phase(0.1, 1e308, 1e-300), Err(ConfigError::StepNotFinite));
}

#[test]
fn ten_thousand_valid_steps_stay_in_range() {
    // Deterministic pseudo-random steps from 0 to 3 periods (64-bit LCG, no dependency).
    let mut s: u64 = 0x5eed;
    let mut p = 0.0;
    for _ in 0..10_000 {
        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let dt = (s >> 11) as f64 / (1u64 << 53) as f64 * 3.0 * T;
        p = advance_phase(p, dt, T).unwrap();
        assert!((0.0..1.0).contains(&p), "phase {p}");
    }
}

#[test]
fn sim_step_recomputes_at_the_new_phase_and_errors_change_nothing() {
    let mut sim = Sim::new(cfg(0.15)).unwrap();
    sim.step(60.0).unwrap();
    let p = sim.config().phase;
    assert_eq!(p, 0.15 + 60.0 / T);
    for field in Field::ALL {
        assert_eq!(sim.field(field), compute_field(&cfg(p), field).as_slice(), "{}", field.name());
    }
    let before: Vec<f32> = sim.field(Field::Temperature).to_vec();
    assert_eq!(sim.step(-1.0), Err(ConfigError::NegativeStep));
    assert_eq!(sim.config().phase, p);
    assert_eq!(sim.field(Field::Temperature), before.as_slice());
}

#[test]
fn c_abi_step_returns_codes_and_never_panics() {
    let c = OkloSimConfig { nx: 32, ny: 32, nz: 32, null_scenario: 0, phase: 0.15, pulse_boundary: 0.17, cycle_period_s: T };
    unsafe {
        let sim = oklo_sim_create(&c);
        assert!(!sim.is_null());
        assert_eq!(oklo_sim_step(sim, 60.0), OKLO_OK);
        let p = oklo_sim_phase(sim);
        assert_eq!(p, 0.15 + 60.0 / T);
        for bad in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(oklo_sim_step(sim, bad), OKLO_ERR_INVALID, "dt {bad}");
            assert_eq!(oklo_sim_phase(sim), p, "phase changed after a rejected step");
        }
        assert_eq!(oklo_sim_step(std::ptr::null_mut(), 1.0), OKLO_ERR_NULL);
        assert_eq!(oklo_sim_step(sim, 3.0 * T), OKLO_OK);
        assert!((0.0..1.0).contains(&oklo_sim_phase(sim)));
        oklo_sim_destroy(sim);
    }
}
