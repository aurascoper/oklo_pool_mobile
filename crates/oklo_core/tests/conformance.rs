//! M0 checks against the six revision 0 MRC files (CONFORMANCE_FIXTURE_SPEC.md 2.1).
//!
//! Policy (README.md, "M0 calibration and conformance policy"): these tests
//! measure discrepancies. They never search over parameters, and a failure is
//! reported as a failure, not tuned away.
//!
//! Scope: only one case exists on disk (phase 0.15, active, 32^3, written by
//! export_oklo_mrc.py with b = 0.17). Rust is a third implementation, not the
//! CF-2 reference generator, so a pass here means "agrees with the revision 0
//! Python export at one case", not "CF-2 passed".
//!
//! Fixture dir: $OKLO_FIXTURE_DIR, default the Antigravity scratch copy.
//! Run with `cargo test -- --nocapture` to see the measured differences.

use oklo_core::ffi::*;
use oklo_core::{compute_field, indicators, Field, SimConfig};
use std::path::PathBuf;

/// Relative to $HOME when OKLO_FIXTURE_DIR is unset.
const DEFAULT_FIXTURE_DIR: &str = ".gemini/antigravity/scratch/oklo_gui_scratch";
const N: u32 = 32;
const VOXELS: usize = 32 * 32 * 32;

fn fixture_dir() -> PathBuf {
    std::env::var_os("OKLO_FIXTURE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let home = std::env::var_os("HOME").expect("set OKLO_FIXTURE_DIR or HOME");
            PathBuf::from(home).join(DEFAULT_FIXTURE_DIR)
        })
}

struct Mrc {
    data: Vec<f32>,
    block: Vec<u8>,
}

fn read_mrc(field: Field) -> Mrc {
    let path = fixture_dir().join(format!("oklo_{}.mrc", field.name()));
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|e| panic!("missing fixture {}: {e}", path.display()));
    assert_eq!(bytes.len(), 1024 + 4 * VOXELS, "{}: file length", path.display());
    let word = |off: usize| i32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
    assert_eq!([word(0), word(4), word(8)], [32, 32, 32], "{}: NX NY NZ", path.display());
    assert_eq!(word(12), 2, "{}: MODE", path.display());
    assert_eq!(word(92), 0, "{}: NSYMBT", path.display());
    let block = bytes[1024..].to_vec();
    let data = block
        .chunks_exact(4)
        .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
        .collect();
    Mrc { data, block }
}

/// The configuration export_oklo_mrc.py used for the files on disk. b = 0.17
/// because the spec records that this script uses 0.17, not because 0.17
/// matches better. cycle_period_s is the manifest's 3.0 h; no check uses it.
fn python_export_config(pulse_boundary: f64) -> SimConfig {
    SimConfig {
        nx: N,
        ny: N,
        nz: N,
        phase: 0.15,
        null_scenario: false,
        pulse_boundary,
        cycle_period_s: 3.0 * 3600.0,
    }
}

fn to_bytes(v: &[f32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

/// Max |diff|, its voxel index, and the count of voxels outside the CF-2
/// tolerance `1e-6 * max(1, |ref|)`.
fn compare(ours: &[f32], reference: &[f32]) -> (f64, usize, usize) {
    let mut worst = (0.0f64, 0usize);
    let mut violations = 0;
    for (idx, (&a, &r)) in ours.iter().zip(reference).enumerate() {
        let diff = (a as f64 - r as f64).abs();
        if diff > worst.0 {
            worst = (diff, idx);
        }
        if diff > 1e-6 * (r as f64).abs().max(1.0) {
            violations += 1;
        }
    }
    (worst.0, worst.1, violations)
}

fn ijk(idx: usize) -> (u32, u32, u32) {
    let n = N as usize;
    ((idx % n) as u32, ((idx / n) % n) as u32, (idx / (n * n)) as u32)
}

/// (a) Every continuous field within the CF-2 tolerance of the file at b = 0.17;
/// compartments identical except voxels within 1e-5 of a threshold, which are listed.
#[test]
fn a_matches_python_export_at_recorded_boundary() {
    let cfg = python_export_config(0.17);
    let mut failures = Vec::new();
    for field in Field::ALL {
        let mrc = read_mrc(field);
        let ours = compute_field(&cfg, field);
        let identical = to_bytes(&ours) == mrc.block;
        if field == Field::Compartments {
            let mut near_threshold = Vec::new();
            let mut mismatched = 0;
            for (idx, (&a, &r)) in ours.iter().zip(&mrc.data).enumerate() {
                if a != r {
                    let (i, j, k) = ijk(idx);
                    let ind = indicators(&cfg, i, j, k);
                    let desic = oklo_core::desiccation(cfg.phase);
                    let near = (ind.core - 0.45).abs() < 1e-5
                        || (ind.fracture - 0.35).abs() < 1e-5
                        || (ind.halo - 0.35).abs() < 1e-5
                        || (desic - 0.4).abs() < 1e-5;
                    if near {
                        near_threshold.push((i, j, k, a, r));
                    } else {
                        mismatched += 1;
                    }
                }
            }
            eprintln!(
                "(a) compartments: byte_identical={identical} mismatched={mismatched} near_threshold_listed={near_threshold:?}"
            );
            if mismatched > 0 {
                failures.push(format!("compartments: {mismatched} label mismatches away from thresholds"));
            }
        } else {
            let (max_diff, at, violations) = compare(&ours, &mrc.data);
            eprintln!(
                "(a) {}: byte_identical={identical} max_abs_diff={max_diff:e} at voxel {:?} violations={violations}",
                field.name(),
                ijk(at)
            );
            if violations > 0 {
                failures.push(format!("{}: {violations} voxels outside tolerance", field.name()));
            }
        }
    }
    assert!(failures.is_empty(), "(a) failed: {failures:?}");
}

/// (b) Sensitivity diagnostic, not calibration and not an endorsement of
/// either value: the GUI's b = 0.1667 must make temperature differ from the
/// b = 0.17 file by more than 1 C (R0-1 predicts about 25.48 C at phase 0.15).
/// This shows check (a) can detect the R0-1 difference.
#[test]
fn b_sensitivity_to_gui_boundary_is_detectable() {
    let mrc = read_mrc(Field::Temperature);
    let ours = compute_field(&python_export_config(0.1667), Field::Temperature);
    let (max_diff, at, violations) = compare(&ours, &mrc.data);
    eprintln!(
        "(b) sensitivity diagnostic, temperature at b=0.1667 vs file: max_abs_diff={max_diff:.4} C at voxel {:?}, violations={violations}",
        ijk(at)
    );
    assert!(max_diff > 1.0, "(b) expected the R0-1 difference to be visible, got {max_diff}");
}

/// (c) Statistics and zone counts equal the values recorded in spec 2.1.
/// The recorded values are rounded to 6 decimals, hence the 1e-6 tolerance.
#[test]
fn c_matches_recorded_snapshot() {
    let cfg = python_export_config(0.17);
    let recorded: [(Field, [f64; 4]); 6] = [
        (Field::Temperature, [26.849241, 216.861908, 84.661757, 27.992193]),
        (Field::Hydration, [0.020000, 0.881707, 0.757855, 0.125662]),
        (Field::H2Concentration, [0.001111, 3.652889, 1.064380, 0.705570]),
        (Field::U235Ratio, [0.004214, 0.007249, 0.006734, 0.000593]),
        (Field::PolymerYield, [0.004336, 14.159290, 5.301710, 2.871762]),
        (Field::Compartments, [0.0, 4.0, 1.509155, 0.964539]),
    ];
    let mut failures = Vec::new();
    for (field, rec) in recorded {
        let v = compute_field(&cfg, field);
        let n = v.len() as f64;
        let min = v.iter().copied().fold(f32::INFINITY, f32::min) as f64;
        let max = v.iter().copied().fold(f32::NEG_INFINITY, f32::max) as f64;
        let mean = v.iter().map(|&x| x as f64).sum::<f64>() / n;
        let sd = (v.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / n).sqrt();
        let got = [min, max, mean, sd];
        eprintln!("(c) {}: min/max/mean/sd = {got:?} recorded {rec:?}", field.name());
        for (name, (g, r)) in ["min", "max", "mean", "sd"].iter().zip(got.iter().zip(rec)) {
            if (g - r).abs() > 1e-6 {
                failures.push(format!("{} {name}: got {g}, recorded {r}", field.name()));
            }
        }
    }
    let zones = compute_field(&cfg, Field::Compartments);
    let mut counts = [0usize; 5];
    for z in &zones {
        counts[*z as usize] += 1;
    }
    eprintln!("(c) zone counts {counts:?} recorded [7560, 3356, 20656, 0, 1196]");
    if counts != [7560, 3356, 20656, 0, 1196] {
        failures.push(format!("zone counts {counts:?}"));
    }
    assert!(failures.is_empty(), "(c) failed: {failures:?}");
}

/// (d) The C ABI round trip gives the same arrays as the Rust API and rejects bad input.
#[test]
fn d_abi_round_trip() {
    assert_eq!(oklo_core_abi_version(), 1);
    let good = OkloSimConfig {
        nx: N,
        ny: N,
        nz: N,
        null_scenario: 0,
        phase: 0.15,
        pulse_boundary: 0.17,
        cycle_period_s: 10800.0,
    };
    unsafe {
        assert!(oklo_sim_create(std::ptr::null()).is_null());
        for bad in [
            OkloSimConfig { pulse_boundary: 0.0, ..good },
            OkloSimConfig { pulse_boundary: f64::NAN, ..good },
            OkloSimConfig { phase: 1.5, ..good },
            OkloSimConfig { nx: 0, ..good },
            OkloSimConfig { nx: 1024, ny: 1024, ..good },
            OkloSimConfig { null_scenario: 2, ..good },
            OkloSimConfig { cycle_period_s: 0.0, ..good },
        ] {
            assert!(oklo_sim_create(&bad).is_null(), "accepted {bad:?}");
        }

        let sim = oklo_sim_create(&good);
        assert!(!sim.is_null());
        assert_eq!(oklo_sim_phase(sim), 0.15);
        let cfg = python_export_config(0.17);
        for field in Field::ALL {
            let mut len = 0usize;
            let ptr = oklo_sim_field(sim, field as u32, &mut len);
            assert!(!ptr.is_null());
            assert_eq!(len, VOXELS);
            let via_abi = std::slice::from_raw_parts(ptr, len);
            assert_eq!(via_abi, compute_field(&cfg, field).as_slice(), "{}", field.name());
        }
        assert!(oklo_sim_field(sim, 6, std::ptr::null_mut()).is_null());
        assert!(oklo_sim_field(std::ptr::null(), 0, std::ptr::null_mut()).is_null());
        assert_eq!(oklo_sim_step(std::ptr::null_mut(), 1.0), OKLO_ERR_NULL);
        assert_eq!(oklo_sim_step(sim, f64::INFINITY), OKLO_ERR_INVALID);
        assert!(oklo_sim_phase(std::ptr::null()).is_nan());
        oklo_sim_destroy(sim);
        oklo_sim_destroy(std::ptr::null_mut());
    }
}

/// Step behaviour depends on `advance_phase`, which is Hunter's decision.
#[test]
#[ignore = "needs advance_phase (lib.rs TODO for Hunter)"]
fn e_step_keeps_phase_in_range() {
    let mut sim = oklo_core::Sim::new(python_export_config(0.17)).unwrap();
    for _ in 0..1000 {
        sim.step(60.0).expect("step");
        let p = sim.config().phase;
        assert!((0.0..=1.0).contains(&p), "phase {p}");
    }
}
