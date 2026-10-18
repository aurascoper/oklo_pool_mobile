//! Revision 0 of the Oklo GUI conformance fixture.
//!
//! Ported from CONFORMANCE_FIXTURE_SPEC.md section 3 (spec sha256 587745ed...).
//! The operation order follows export_oklo_mrc.py (revision 0, sha256 40298111...),
//! so values computed here in f64 and stored as f32 can be compared voxel by
//! voxel with the six MRC files that script exported.
//!
//! Every field is a prescribed analytic function of grid position and cycle
//! phase. Nothing here is a physical simulation, a calibrated model, or
//! evidence about Oklo. See docs/PROVENANCE.md for what each field may and
//! may not be used to claim.

pub mod darcy;
pub mod ffi;
pub mod mrc;
pub mod uranium;

use std::f64::consts::PI;

/// Largest grid accepted, 128^3 voxels. Six f32 fields at this size use about
/// 50 MB, inside the 150 MB app budget.
pub const MAX_VOXELS: usize = 128 * 128 * 128;

/// The six revision 0 fields, in the order and with the ids used by the C ABI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Field {
    Temperature = 0,
    Hydration = 1,
    H2Concentration = 2,
    U235Ratio = 3,
    PolymerYield = 4,
    Compartments = 5,
}

impl Field {
    pub const ALL: [Field; 6] = [
        Field::Temperature,
        Field::Hydration,
        Field::H2Concentration,
        Field::U235Ratio,
        Field::PolymerYield,
        Field::Compartments,
    ];

    /// The field name used in the revision 0 MRC file names and labels.
    pub fn name(self) -> &'static str {
        match self {
            Field::Temperature => "temperature",
            Field::Hydration => "hydration",
            Field::H2Concentration => "h2_concentration",
            Field::U235Ratio => "u235_ratio",
            Field::PolymerYield => "polymer_yield",
            Field::Compartments => "compartments",
        }
    }

    pub fn from_id(id: u32) -> Option<Field> {
        Field::ALL.get(id as usize).copied()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimConfig {
    pub nx: u32,
    pub ny: u32,
    pub nz: u32,
    /// Cycle phase fraction in [0, 1].
    pub phase: f64,
    /// Null scenario: pulse and desiccation are both 0.
    pub null_scenario: bool,
    /// Pulse boundary b. Revision 0 has two values: 0.1667 in oklo_model.js
    /// and 0.17 in export_oklo_mrc.py (finding R0-1). Choosing one is a
    /// revision 1 decision, so there is deliberately no default.
    pub pulse_boundary: f64,
    /// Seconds per full cycle. Used only by `advance_phase`.
    pub cycle_period_s: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigError {
    EmptyGrid,
    GridTooLarge,
    PhaseOutOfRange,
    PulseBoundaryOutOfRange,
    PeriodNotPositive,
    StepNotFinite,
    NegativeStep,
}

impl SimConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.nx == 0 || self.ny == 0 || self.nz == 0 {
            return Err(ConfigError::EmptyGrid);
        }
        let voxels = (self.nx as usize)
            .checked_mul(self.ny as usize)
            .and_then(|v| v.checked_mul(self.nz as usize));
        if !matches!(voxels, Some(v) if v <= MAX_VOXELS) {
            return Err(ConfigError::GridTooLarge);
        }
        if !(0.0..=1.0).contains(&self.phase) {
            return Err(ConfigError::PhaseOutOfRange);
        }
        if !(self.pulse_boundary > 0.0 && self.pulse_boundary < 1.0) {
            return Err(ConfigError::PulseBoundaryOutOfRange);
        }
        if !(self.cycle_period_s.is_finite() && self.cycle_period_s > 0.0) {
            return Err(ConfigError::PeriodNotPositive);
        }
        Ok(())
    }

    pub fn voxels(&self) -> usize {
        self.nx as usize * self.ny as usize * self.nz as usize
    }
}

/// Pulse term P(p): `sin(pi*p/b)` for `p < b`, else `exp(-8*(p - b))`.
pub fn pulse(p: f64, b: f64) -> f64 {
    if p < b {
        ((p / b) * PI).sin()
    } else {
        (-(p - b) * 8.0).exp().max(0.0)
    }
}

/// Desiccation term D(p): `sin(pi*(p - 0.10)/0.25)` for `0.10 <= p <= 0.35`, else 0.
pub fn desiccation(p: f64) -> f64 {
    if (0.10..=0.35).contains(&p) {
        (((p - 0.10) / 0.25) * PI).sin()
    } else {
        0.0
    }
}

/// Map a time step onto the revision 0 cycle phase.
///
/// Contract adopted 2026-10-17:
/// - forward steps wrap: `p_next = rem_euclid(p + dt_s / period_s, 1)`, in [0, 1);
/// - a negative step is an error, never a backwards run;
/// - `period_s` is the configured `cycle_period_s`, and nothing else sets the time scale;
/// - `phase` must be finite and in [0, 1], `dt_s` finite, `period_s` positive and finite.
///
/// Phase 1 is accepted as a starting value, but every step returns the
/// canonical [0, 1) form, so a full-cycle step from 0 returns 0.
///
/// Revision 0 has no time axis: its fields depend only on phase, so this is
/// navigation along the cycle, not dynamics. The 3 h period most callers pass
/// is inherited from the exporter (the Meshik 2004 inference, which Hunter's
/// own lane did not reproduce from ENDF data). Stepping with it does not
/// validate that timing, and it does not change the pulse discontinuity at p = b.
pub fn advance_phase(phase: f64, dt_s: f64, period_s: f64) -> Result<f64, ConfigError> {
    if !(0.0..=1.0).contains(&phase) {
        return Err(ConfigError::PhaseOutOfRange);
    }
    if !(period_s.is_finite() && period_s > 0.0) {
        return Err(ConfigError::PeriodNotPositive);
    }
    if !dt_s.is_finite() {
        return Err(ConfigError::StepNotFinite);
    }
    if dt_s < 0.0 {
        return Err(ConfigError::NegativeStep);
    }
    let next = (phase + dt_s / period_s).rem_euclid(1.0);
    if !next.is_finite() {
        return Err(ConfigError::StepNotFinite);
    }
    Ok(next)
}

/// Compartment indicators at voxel (i, j, k). Spec section 3, geometry.
#[derive(Clone, Copy, Debug)]
pub struct Indicators {
    /// Core indicator C.
    pub core: f64,
    /// Conduit indicator F (0 below the core centre plane).
    pub fracture: f64,
    /// Halo indicator H.
    pub halo: f64,
}

pub fn indicators(cfg: &SimConfig, i: u32, j: u32, k: u32) -> Indicators {
    let (nx, ny, nz) = (cfg.nx as f64, cfg.ny as f64, cfg.nz as f64);
    let (i, j, k) = (i as f64, j as f64, k as f64);
    let cx = (nx - 1.0) / 2.0;
    let cy = (ny - 1.0) / 2.0;
    let cz_core = nz * 0.35;

    let z_norm = (k - cz_core) / (nz * 0.35);
    let y_norm = (j - cy) / (ny * 0.40);
    let x_norm = (i - cx) / (nx * 0.40);
    let r_core = (x_norm * x_norm + y_norm * y_norm + z_norm * z_norm).sqrt();

    let dx_frac = (i - cx) / (nx * 0.12);
    let dy_frac = (j - cy) / (ny * 0.12);
    let r_fracture = (dx_frac * dx_frac + dy_frac * dy_frac).sqrt();
    let fracture = if k >= cz_core {
        (-0.5 * r_fracture * r_fracture).exp()
    } else {
        0.0
    };

    Indicators {
        core: (-0.5 * (r_core / 0.6).powi(2)).exp(),
        fracture,
        halo: (-0.5 * ((r_core - 1.1) / 0.35).powi(2)).exp().max(0.0),
    }
}

/// Fill `out` with one field, X fastest: `index = k*nx*ny + j*nx + i`.
/// `out` keeps its capacity, so refilling the same buffer does not allocate.
pub fn fill_field(cfg: &SimConfig, field: Field, out: &mut Vec<f32>) {
    let (pulse_factor, desic) = if cfg.null_scenario {
        (0.0, 0.0)
    } else {
        (pulse(cfg.phase, cfg.pulse_boundary), desiccation(cfg.phase))
    };
    let p = cfg.phase;
    let nz = cfg.nz as f64;

    out.clear();
    out.reserve(cfg.voxels());
    for k in 0..cfg.nz {
        let depth = k as f64 / nz;
        for j in 0..cfg.ny {
            for i in 0..cfg.nx {
                let ind = indicators(cfg, i, j, k);
                let (c, f, h) = (ind.core, ind.fracture, ind.halo);
                let val = match field {
                    Field::Temperature => {
                        let base_temp = 25.0 + 35.0 * depth;
                        if cfg.null_scenario {
                            base_temp + 30.0 * c
                        } else {
                            let core_heat = 320.0 * pulse_factor * c;
                            let frac_heat = 180.0 * pulse_factor * f;
                            let diffused = 40.0 * h * (1.0 - pulse_factor * 0.5);
                            base_temp + core_heat + frac_heat + diffused
                        }
                    }
                    Field::Hydration => {
                        let base_hydration = 0.85 - 0.10 * depth;
                        if cfg.null_scenario {
                            base_hydration
                        } else {
                            let drying = desic * (0.80 * f + 0.65 * c);
                            (base_hydration - drying + 0.08 * h).min(1.0).max(0.02)
                        }
                    }
                    Field::H2Concentration => {
                        if cfg.null_scenario {
                            0.001
                        } else {
                            let water_present = (1.0 - desic * 0.7).max(0.05);
                            let radiation_zone = c + 0.3 * h;
                            let mut v = (0.40 * 12.5) * radiation_zone * water_present;
                            v += 1.8 * f * (1.0 - desic);
                            v
                        }
                    }
                    Field::U235Ratio => 0.00725 - 0.00305 * c,
                    Field::PolymerYield => {
                        if cfg.null_scenario {
                            0.0
                        } else {
                            let catalytic_surface = 0.7 * h + 0.5 * f;
                            let cycle_efficiency = 1.0 + 0.5 * (p * 2.0 * PI).sin();
                            8.5 * catalytic_surface * cycle_efficiency
                        }
                    }
                    Field::Compartments => {
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
                out.push(val as f32);
            }
        }
    }
}

pub fn compute_field(cfg: &SimConfig, field: Field) -> Vec<f32> {
    let mut out = Vec::new();
    fill_field(cfg, field, &mut out);
    out
}

/// A configured fixture with all six fields computed.
pub struct Sim {
    cfg: SimConfig,
    fields: [Vec<f32>; 6],
}

impl Sim {
    pub fn new(cfg: SimConfig) -> Result<Sim, ConfigError> {
        cfg.validate()?;
        let mut sim = Sim {
            cfg,
            fields: Default::default(),
        };
        sim.recompute();
        Ok(sim)
    }

    fn recompute(&mut self) {
        for field in Field::ALL {
            fill_field(&self.cfg, field, &mut self.fields[field as usize]);
        }
    }

    /// Advance the phase by `dt_s` seconds and recompute every field in place.
    /// On error nothing changes.
    pub fn step(&mut self, dt_s: f64) -> Result<(), ConfigError> {
        let mut next = self.cfg;
        next.phase = advance_phase(self.cfg.phase, dt_s, self.cfg.cycle_period_s)?;
        next.validate()?;
        self.cfg = next;
        self.recompute();
        Ok(())
    }

    pub fn field(&self, field: Field) -> &[f32] {
        &self.fields[field as usize]
    }

    pub fn config(&self) -> &SimConfig {
        &self.cfg
    }
}
