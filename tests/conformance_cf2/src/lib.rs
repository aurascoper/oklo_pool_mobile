//! CF-2 Independent Conformance Suite for Oklo Revision 0.

pub mod matrix;
pub mod reference;
pub mod sha256;

use reference::{compute_indicators, ref_desiccation, RefConfig};

#[derive(Clone, Copy, Debug)]
pub struct ContinuousDiff {
    pub passed: bool,
    pub max_abs_diff: f64,
    pub max_rel_diff: f64,
    pub worst_index: usize,
    pub violation_count: usize,
}

/// Evaluates test data against reference data under the relative tolerance rule:
/// |test - ref| <= 1e-6 * max(1.0, |ref|)
pub fn compare_continuous(test: &[f32], reference: &[f32]) -> ContinuousDiff {
    assert_eq!(test.len(), reference.len(), "Array lengths must match");

    let mut max_abs_diff = 0.0f64;
    let mut max_rel_diff = 0.0f64;
    let mut worst_index = 0;
    let mut violation_count = 0;

    for (idx, (&t, &r)) in test.iter().zip(reference.iter()).enumerate() {
        let t_f = t as f64;
        let r_f = r as f64;
        let abs_diff = (t_f - r_f).abs();
        let tol = 1e-6 * 1.0f64.max(r_f.abs());
        let rel_diff = abs_diff / 1.0f64.max(r_f.abs());

        if abs_diff > max_abs_diff {
            max_abs_diff = abs_diff;
        }
        if rel_diff > max_rel_diff {
            max_rel_diff = rel_diff;
            worst_index = idx;
        }

        if abs_diff > tol {
            violation_count += 1;
        }
    }

    ContinuousDiff {
        passed: violation_count == 0,
        max_abs_diff,
        max_rel_diff,
        worst_index,
        violation_count,
    }
}

#[derive(Clone, Debug)]
pub struct CompartmentDiff {
    pub passed: bool,
    pub total_mismatches: usize,
    pub borderline_mismatches: usize,
    pub fatal_mismatches: usize,
    pub borderline_voxels: Vec<usize>,
}

/// Evaluates categorical compartment labels (0.0 to 4.0) against reference.
/// Exact float match is required. Any voxels differing within 1e-5 of any
/// continuous decision boundary are logged as borderline diagnostics.
pub fn compare_compartments(test: &[f32], reference: &[f32], cfg: &RefConfig) -> CompartmentDiff {
    assert_eq!(test.len(), reference.len(), "Array lengths must match");

    let desic = if cfg.null_scenario {
        0.0
    } else {
        ref_desiccation(cfg.phase)
    };

    let mut borderline_mismatches = 0;
    let mut fatal_mismatches = 0;
    let mut borderline_voxels = Vec::new();

    for k in 0..cfg.nz {
        for j in 0..cfg.ny {
            for i in 0..cfg.nx {
                let idx = k * cfg.nx * cfg.ny + j * cfg.nx + i;
                let t = test[idx];
                let r = reference[idx];

                let ind = compute_indicators(cfg.nx, cfg.ny, cfg.nz, i, j, k);
                let near_c = (ind.core - 0.45).abs() < 1e-5;
                let near_f = (ind.fracture - 0.35).abs() < 1e-5;
                let near_h = (ind.halo - 0.35).abs() < 1e-5;
                let near_d = (desic - 0.40).abs() < 1e-5;
                let is_near_boundary = near_c || near_f || near_h || near_d;

                if is_near_boundary {
                    borderline_voxels.push(idx);
                }

                if (t - r).abs() > 0.0 {
                    if is_near_boundary {
                        borderline_mismatches += 1;
                    } else {
                        fatal_mismatches += 1;
                    }
                }
            }
        }
    }

    let total_mismatches = borderline_mismatches + fatal_mismatches;
    CompartmentDiff {
        passed: fatal_mismatches == 0,
        total_mismatches,
        borderline_mismatches,
        fatal_mismatches,
        borderline_voxels,
    }
}
