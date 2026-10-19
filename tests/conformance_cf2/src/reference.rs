//! Independent mathematical reference generator for the Oklo Revision 0 Six-Field Fixture.
//!
//! Derived strictly from CONFORMANCE_FIXTURE_SPEC.md Section 3 without any dependency
//! on or imports from oklo_core.

use std::f64::consts::PI;

pub const GRID_NX: usize = 32;
pub const GRID_NY: usize = 32;
pub const GRID_NZ: usize = 32;
pub const TOTAL_VOXELS: usize = GRID_NX * GRID_NY * GRID_NZ; // 32768

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefField {
    Temperature = 0,
    Hydration = 1,
    H2Concentration = 2,
    U235Ratio = 3,
    PolymerYield = 4,
    Compartments = 5,
}

impl RefField {
    pub const ALL: [RefField; 6] = [
        RefField::Temperature,
        RefField::Hydration,
        RefField::H2Concentration,
        RefField::U235Ratio,
        RefField::PolymerYield,
        RefField::Compartments,
    ];

    pub fn name(self) -> &'static str {
        match self {
            RefField::Temperature => "temperature",
            RefField::Hydration => "hydration",
            RefField::H2Concentration => "h2_concentration",
            RefField::U235Ratio => "u235_ratio",
            RefField::PolymerYield => "polymer_yield",
            RefField::Compartments => "compartments",
        }
    }

    pub fn id(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RefConfig {
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    pub phase: f64,
    pub null_scenario: bool,
    pub pulse_boundary: f64,
}

impl Default for RefConfig {
    fn default() -> Self {
        Self {
            nx: GRID_NX,
            ny: GRID_NY,
            nz: GRID_NZ,
            phase: 0.15,
            null_scenario: false,
            pulse_boundary: 0.17,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FieldStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub pop_sd: f64,
    pub rms: f64,
}

pub fn compute_stats(data: &[f32]) -> FieldStats {
    if data.is_empty() {
        return FieldStats {
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            pop_sd: 0.0,
            rms: 0.0,
        };
    }
    let n = data.len() as f64;
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut sum = 0.0;
    let mut sum_sq = 0.0;

    for &x in data {
        let v = x as f64;
        if v < min {
            min = v;
        }
        if v > max {
            max = v;
        }
        sum += v;
        sum_sq += v * v;
    }

    let mean = sum / n;
    let variance = (sum_sq / n - mean * mean).max(0.0);
    let pop_sd = variance.sqrt();
    let rms = (sum_sq / n).max(0.0).sqrt();

    FieldStats {
        min,
        max,
        mean,
        pop_sd,
        rms,
    }
}

/// Pulse term P(p, b) per CONFORMANCE_FIXTURE_SPEC.md Sec 3.
pub fn ref_pulse(phase: f64, pulse_boundary: f64) -> f64 {
    if phase < pulse_boundary {
        ((phase / pulse_boundary) * PI).sin()
    } else {
        (-8.0 * (phase - pulse_boundary)).exp().max(0.0)
    }
}

/// Desiccation term D(p) per CONFORMANCE_FIXTURE_SPEC.md Sec 3.
pub fn ref_desiccation(phase: f64) -> f64 {
    if (0.10..=0.35).contains(&phase) {
        (((phase - 0.10) / 0.25) * PI).sin()
    } else {
        0.0
    }
}

/// Spatial indicators at voxel (i, j, k) per CONFORMANCE_FIXTURE_SPEC.md Sec 3.
#[derive(Clone, Copy, Debug)]
pub struct SpatialIndicators {
    pub core: f64,
    pub fracture: f64,
    pub halo: f64,
}

pub fn compute_indicators(nx: usize, ny: usize, nz: usize, i: usize, j: usize, k: usize) -> SpatialIndicators {
    let nx_f = nx as f64;
    let ny_f = ny as f64;
    let nz_f = nz as f64;
    let i_f = i as f64;
    let j_f = j as f64;
    let k_f = k as f64;

    let cx = (nx_f - 1.0) / 2.0;
    let cy = (ny_f - 1.0) / 2.0;
    let czc = 0.35 * nz_f;

    let x_n = (i_f - cx) / (0.40 * nx_f);
    let y_n = (j_f - cy) / (0.40 * ny_f);
    let z_n = (k_f - czc) / (0.35 * nz_f);

    let rc = (x_n * x_n + y_n * y_n + z_n * z_n).sqrt();

    let dx_frac = (i_f - cx) / (0.12 * nx_f);
    let dy_frac = (j_f - cy) / (0.12 * ny_f);
    let rf = (dx_frac * dx_frac + dy_frac * dy_frac).sqrt();

    let fracture = if k_f >= czc {
        (-0.5 * rf * rf).exp()
    } else {
        0.0
    };

    let core = (-0.5 * (rc / 0.6).powi(2)).exp();
    let halo = (-0.5 * ((rc - 1.1) / 0.35).powi(2)).exp().max(0.0);

    SpatialIndicators {
        core,
        fracture,
        halo,
    }
}

/// Compute 3D field array independently from mathematical specification.
/// Returned vector has length nx * ny * nz, ordered X fastest: index = k*nx*ny + j*nx + i.
pub fn generate_reference_field(cfg: &RefConfig, field: RefField) -> Vec<f32> {
    let total = cfg.nx * cfg.ny * cfg.nz;
    let mut buffer = Vec::with_capacity(total);

    let (pulse, desic) = if cfg.null_scenario {
        (0.0, 0.0)
    } else {
        (ref_pulse(cfg.phase, cfg.pulse_boundary), ref_desiccation(cfg.phase))
    };

    let p = cfg.phase;
    let nz_f = cfg.nz as f64;

    for k in 0..cfg.nz {
        let depth = (k as f64) / nz_f;
        for j in 0..cfg.ny {
            for i in 0..cfg.nx {
                let ind = compute_indicators(cfg.nx, cfg.ny, cfg.nz, i, j, k);
                let (c, f, h) = (ind.core, ind.fracture, ind.halo);

                let val: f64 = match field {
                    RefField::Temperature => {
                        let base_temp = 25.0 + 35.0 * depth;
                        if cfg.null_scenario {
                            base_temp + 30.0 * c
                        } else {
                            base_temp
                                + 320.0 * pulse * c
                                + 180.0 * pulse * f
                                + 40.0 * h * (1.0 - 0.5 * pulse)
                        }
                    }
                    RefField::Hydration => {
                        let base_hyd = 0.85 - 0.10 * depth;
                        if cfg.null_scenario {
                            base_hyd
                        } else {
                            let drying = desic * (0.80 * f + 0.65 * c);
                            (base_hyd - drying + 0.08 * h).min(1.0).max(0.02)
                        }
                    }
                    RefField::H2Concentration => {
                        if cfg.null_scenario {
                            0.001
                        } else {
                            let water_present = (1.0 - 0.7 * desic).max(0.05);
                            let radiation_zone = c + 0.3 * h;
                            5.0 * radiation_zone * water_present + 1.8 * f * (1.0 - desic)
                        }
                    }
                    RefField::U235Ratio => 0.00725 - 0.00305 * c,
                    RefField::PolymerYield => {
                        if cfg.null_scenario {
                            0.0
                        } else {
                            let catalytic_surface = 0.7 * h + 0.5 * f;
                            let cycle_efficiency = 1.0 + 0.5 * (2.0 * PI * p).sin();
                            8.5 * catalytic_surface * cycle_efficiency
                        }
                    }
                    RefField::Compartments => {
                        if c > 0.45 {
                            1.0
                        } else if f > 0.35 {
                            if desic > 0.4 {
                                4.0
                            } else {
                                3.0
                            }
                        } else if h > 0.35 {
                            2.0
                        } else {
                            0.0
                        }
                    }
                };

                buffer.push(val as f32);
            }
        }
    }

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_snapshot_matches_fixture_spec() {
        // Spec Sec 2.1 recorded outputs at phase 0.15, active, b = 0.17
        let cfg = RefConfig {
            nx: 32,
            ny: 32,
            nz: 32,
            phase: 0.15,
            null_scenario: false,
            pulse_boundary: 0.17,
        };

        let temp = generate_reference_field(&cfg, RefField::Temperature);
        let s_temp = compute_stats(&temp);
        assert!((s_temp.min - 26.849241).abs() < 1e-4);
        assert!((s_temp.max - 216.861908).abs() < 1e-3);
        assert!((s_temp.mean - 84.661757).abs() < 1e-4);
        assert!((s_temp.pop_sd - 27.992193).abs() < 1e-4);

        let hyd = generate_reference_field(&cfg, RefField::Hydration);
        let s_hyd = compute_stats(&hyd);
        assert!((s_hyd.min - 0.020000).abs() < 1e-4);
        assert!((s_hyd.max - 0.881707).abs() < 1e-4);
        assert!((s_hyd.mean - 0.757855).abs() < 1e-4);
        assert!((s_hyd.pop_sd - 0.125662).abs() < 1e-4);

        let comp = generate_reference_field(&cfg, RefField::Compartments);
        let mut counts = [0usize; 5];
        for &v in &comp {
            counts[v as usize] += 1;
        }
        assert_eq!(counts, [7560, 3356, 20656, 0, 1196]);
    }
}
