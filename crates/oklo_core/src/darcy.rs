//! Steady, single-phase, incompressible, isothermal Darcy flow on the revision 0
//! fixture geometry, implementing Darcy pre-registration v1 with Amendments A1,
//! A2 and A3 (docs/specs/). It is a hypothetical numerical experiment, not a
//! description of Oklo hydrogeology. Every constant comes from those frozen
//! specifications or their receipts; nothing here is fitted, and a failed check
//! is reported as a failure.

use crate::{compute_field, Field, SimConfig};

/// Grid of the fixture: 32 cells per side over 16 m.
pub const N: usize = 32;
pub const L: f64 = 16.0;
/// Declared head difference, bottom face minus top face (v1 section 4).
pub const DH: f64 = 1.0;
pub const G: f64 = 9.80665;
/// Péclet length scale, the voxel (A1.1).
pub const PE_LENGTH_M: f64 = 0.5;
/// Carried on every output (MRC label 2 and the receipt).
pub const CAVEAT: &str = "HYPOTHETICAL scenario, not Oklo; Darcy v1+A1-A3";

// Pre-registered CG acceptance rules: v1's 1e-12, and A4's 1e-13 for the
// separate A4 verification rerun. Nothing else differs between the two.
pub const CG_REL_TOL_V1: f64 = 1e-12;
pub const CG_REL_TOL_A4: f64 = 1e-13;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolverRule {
    V1,
    A4,
}

impl SolverRule {
    pub fn tol(self) -> f64 {
        match self {
            SolverRule::V1 => CG_REL_TOL_V1,
            SolverRule::A4 => CG_REL_TOL_A4,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            SolverRule::V1 => "v1",
            SolverRule::A4 => "a4",
        }
    }
    pub fn parse(s: &str) -> Option<SolverRule> {
        match s {
            "v1" => Some(SolverRule::V1),
            "a4" => Some(SolverRule::A4),
            _ => None,
        }
    }
    /// The rule named by OKLO_DARCY_RULE ("v1" or "a4"). A4.3 requires an
    /// explicit choice, so an unset or unknown value is an error.
    pub fn from_env() -> SolverRule {
        let v = std::env::var("OKLO_DARCY_RULE").unwrap_or_default();
        SolverRule::parse(&v).unwrap_or_else(|| panic!("set OKLO_DARCY_RULE to v1 or a4 (got {v:?})"))
    }
}

// Pre-registered acceptance thresholds (v1 sections 5 and 6, unchanged by A1 to A4).
pub const TOL_D1_EPS: f64 = 1e-10;
pub const TOL_D1_GLOBAL: f64 = 1e-10;
pub const TOL_D1_PATHS: f64 = 1e-12;
pub const D2_ORDER_RANGE: (f64, f64) = (1.9, 2.1);
pub const TOL_D3_DIRECT: f64 = 1e-12;
pub const TOL_D3_CG: f64 = 1e-10;
pub const TOL_D4: f64 = 1e-10;
pub const TOL_D5: f64 = 1e-10;

pub fn cg_max_iterations(n: usize) -> usize {
    10 * n * n * n
}

// ---------------------------------------------------------------- water states

/// One homogeneous water state (v1 section 3.2; c_p from A1.2), iapws 1.5.5.
#[derive(Clone, Copy, Debug)]
pub struct WaterState {
    pub name: &'static str,
    pub t_k: f64,
    pub p_mpa: f64,
    pub rho: f64,
    pub mu: f64,
    pub cp: f64,
}

pub const STATE_A: WaterState = WaterState { name: "A", t_k: 323.15, p_mpa: 30.0, rho: 1000.666795, mu: 5.528165e-4, cp: 4118.0976 };
pub const STATE_B: WaterState = WaterState { name: "B", t_k: 573.15, p_mpa: 30.0, rho: 750.658667, mu: 9.325487e-5, cp: 5080.3138 };
pub const STATES: [WaterState; 2] = [STATE_A, STATE_B];

impl WaterState {
    /// K = k * rho g / mu (v1 section 2).
    pub fn rho_g_over_mu(&self) -> f64 {
        self.rho * G / self.mu
    }
    pub fn rho_cp(&self) -> f64 {
        self.rho * self.cp
    }
    /// The temperature limitation A1.2 attaches to every Pe at this state.
    pub fn pe_limitation(&self) -> &'static str {
        if self.t_k > 473.15 {
            "lambda_m measured at 300 K and 5 MPa, applied uncorrected; 300 C lies outside Robertson's below-200 C range: unquantified temperature error"
        } else {
            "lambda_m measured at 300 K and 5 MPa, applied uncorrected for temperature and pressure"
        }
    }
}

// ---------------------------------------------------------------- porosity, two roles (A2)

/// Flow porosity phi_z, one per zone: enters only the pore velocity |q| / phi_z.
#[derive(Clone, Copy, Debug)]
pub struct FlowPorosity {
    pub value: f64,
    pub sourced: bool,
    pub basis: &'static str,
}

pub fn flow_porosity(zone: Zone) -> FlowPorosity {
    match zone {
        Zone::Core => FlowPorosity { value: 0.125, sourced: true, basis: "midpoint of 10-15% open porosity, Gauthier-Lafaye et al. 1996 p. 4834 citing Naudet 1991" },
        Zone::Sandstone => FlowPorosity { value: 0.15, sourced: false, basis: "revision 0 registry estimate, unsourced" },
        Zone::Halo => FlowPorosity { value: 0.06, sourced: false, basis: "revision 0 registry estimate, unsourced" },
        Zone::Conduit => FlowPorosity { value: 0.25, sourced: false, basis: "revision 0 registry estimate, unsourced" },
    }
}

/// Thermal-property porosity phi_th: a span used only to bound lambda_m (A1.2, A2.2).
#[derive(Clone, Copy, Debug)]
pub struct ThermalPorositySpan {
    pub lo: f64,
    pub hi: f64,
}

pub const PHI_TH: ThermalPorositySpan = ThermalPorositySpan { lo: 0.10, hi: 0.25 };
pub const QUARTZ_SPAN_PCT: (f64, f64) = (0.0, 100.0);

// ---------------------------------------------------------------- lambda_m (A1.2)

/// 1 CU = 1e-3 cal/(cm s C) = 0.4184 W/(m K) (Robertson 1988, p. 5).
pub const CU_W_PER_MK: f64 = 0.4184;
const ROB_KF: f64 = 1.48;
const ROB_KS: f64 = 3.63;
const ROB_S: f64 = 0.157;

/// Robertson 1988 equation (6) with the figure 10 intercepts for sandstone with
/// water in the pores, in W/(m K).
pub fn lambda_m(phi_th: f64, quartz_pct: f64) -> f64 {
    let g2 = (1.0 - phi_th) * (1.0 - phi_th);
    (ROB_KF + g2 * ((ROB_KS + quartz_pct * ROB_S) - ROB_KF)) * CU_W_PER_MK
}

#[derive(Clone, Copy, Debug)]
pub struct Range3 {
    pub low: f64,
    pub rep: f64,
    pub high: f64,
}

/// Low at (phi_th hi, quartz 0), high at (phi_th lo, quartz 100), representative
/// the geometric mean sqrt(low * high), A1's rule.
pub fn lambda_range() -> Range3 {
    let low = lambda_m(PHI_TH.hi, QUARTZ_SPAN_PCT.0);
    let high = lambda_m(PHI_TH.lo, QUARTZ_SPAN_PCT.1);
    Range3 { low, rep: (low * high).sqrt(), high }
}

// ---------------------------------------------------------------- permeability analogues (v1 3.1)

/// Decade tick positions (px) of the k (cm^2) axis of Groundwater Project
/// Figure 32, for log10 k_cm2 = -16 .. -3, from the v1 freeze receipt.
const TICKS_PX: [f64; 14] = [233.0, 320.0, 409.0, 498.0, 587.0, 675.0, 764.0, 853.0, 942.0, 1031.0, 1120.0, 1208.0, 1297.0, 1386.0];

/// Least-squares line x = a + b * log10(k_cm2) through the ticks.
pub fn calibration() -> (f64, f64) {
    let n = TICKS_PX.len() as f64;
    let d: Vec<f64> = (0..TICKS_PX.len()).map(|i| -16.0 + i as f64).collect();
    let md = d.iter().sum::<f64>() / n;
    let mx = TICKS_PX.iter().sum::<f64>() / n;
    let sdd: f64 = d.iter().map(|v| (v - md) * (v - md)).sum();
    let sdx: f64 = d.iter().zip(TICKS_PX.iter()).map(|(v, x)| (v - md) * (x - mx)).sum();
    let b = sdx / sdd;
    (mx - b * md, b)
}

pub fn k_m2_at_px(px: f64) -> f64 {
    let (a, b) = calibration();
    10f64.powf((px - a) / b) * 1e-4
}

/// A digitized permeability class: bar end pixels from the v1 freeze receipt.
#[derive(Clone, Copy, Debug)]
pub struct Analogue {
    pub class: &'static str,
    pub px_lo: f64,
    pub px_hi: f64,
}

pub const SANDSTONE_ANALOGUE: Analogue = Analogue { class: "Sandstone", px_lo: 492.0, px_hi: 878.0 };
pub const SHALE_ANALOGUE: Analogue = Analogue { class: "Shale", px_lo: 216.0, px_hi: 582.0 };
pub const FRACTURED_ANALOGUE: Analogue = Analogue { class: "Fractured Igneous & Metamorphic Rocks", px_lo: 508.0, px_hi: 1054.0 };

impl Analogue {
    /// Endpoints and log-midpoint (geometric mean of the endpoints) in m^2.
    pub fn k_range(&self) -> Range3 {
        let low = k_m2_at_px(self.px_lo);
        let high = k_m2_at_px(self.px_hi);
        Range3 { low, rep: (low * high).sqrt(), high }
    }
}

// ---------------------------------------------------------------- zones and configurations

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Zone {
    Sandstone = 0,
    Core = 1,
    Halo = 2,
    Conduit = 3,
}

pub const ZONES: [Zone; 4] = [Zone::Sandstone, Zone::Core, Zone::Halo, Zone::Conduit];

/// The fixture's compartment rule evaluated at cell centres in the null
/// scenario (v1 section 3.1). Zone 4 cannot occur there; it would be an error.
pub fn zone_mask_fixture() -> Vec<Zone> {
    let cfg = SimConfig { nx: N as u32, ny: N as u32, nz: N as u32, phase: 0.0, null_scenario: true, pulse_boundary: 0.17, cycle_period_s: 10800.0 };
    compute_field(&cfg, Field::Compartments)
        .into_iter()
        .map(|v| match v as u8 {
            0 => Zone::Sandstone,
            1 => Zone::Core,
            2 => Zone::Halo,
            3 => Zone::Conduit,
            other => panic!("zone {other} in the null-scenario mask"),
        })
        .collect()
}

/// Each cell of an n^3 mask copied to its eight children in a (2n)^3 mask.
pub fn refine_mask(mask: &[Zone], n: usize) -> Vec<Zone> {
    let m = 2 * n;
    let mut out = Vec::with_capacity(m * m * m);
    for k in 0..m {
        for j in 0..m {
            for i in 0..m {
                out.push(mask[idx(n, i / 2, j / 2, k / 2)]);
            }
        }
    }
    out
}

/// One configuration of v1 section 3.3: an intrinsic permeability per zone, m^2.
#[derive(Clone, Debug)]
pub struct Config {
    pub name: String,
    pub k: [f64; 4],
}

impl Config {
    pub fn k_of(&self, z: Zone) -> f64 {
        self.k[z as usize]
    }
}

/// Base, conduit ratio sweep 1, 10, 100, 1000, and six one-at-a-time endpoint
/// cases (core moves with sandstone). Eleven configurations.
pub fn configurations() -> Vec<Config> {
    let s = SANDSTONE_ANALOGUE.k_range();
    let h = SHALE_ANALOGUE.k_range();
    let f = FRACTURED_ANALOGUE.k_range();
    let base = [s.rep, s.rep, h.rep, f.rep];
    let mut out = vec![Config { name: "base".into(), k: base }];
    for r in [1.0, 10.0, 100.0, 1000.0] {
        let mut k = base;
        k[Zone::Conduit as usize] = r * s.rep;
        out.push(Config { name: format!("ratio_{r}"), k });
    }
    for (name, z, v) in [
        ("sandstone_core_low", None, s.low),
        ("sandstone_core_high", None, s.high),
        ("halo_low", Some(Zone::Halo), h.low),
        ("halo_high", Some(Zone::Halo), h.high),
        ("conduit_low", Some(Zone::Conduit), f.low),
        ("conduit_high", Some(Zone::Conduit), f.high),
    ] {
        let mut k = base;
        match z {
            None => {
                k[Zone::Sandstone as usize] = v;
                k[Zone::Core as usize] = v;
            }
            Some(z) => k[z as usize] = v,
        }
        out.push(Config { name: name.into(), k });
    }
    out
}

// ---------------------------------------------------------------- the discrete problem

pub fn idx(n: usize, i: usize, j: usize, k: usize) -> usize {
    (k * n + j) * n + i
}

/// Cell-centred finite volumes on [0, L]^3 with Dirichlet heads on the bottom
/// (dh) and top (0) faces and closed sides (v1 section 4).
pub struct Problem {
    pub n: usize,
    pub dx: f64,
    /// Hydraulic conductivity per cell, m/s.
    pub kc: Vec<f64>,
    pub dh: f64,
    /// Volumetric source per cell, 1/s (zero except in D2).
    pub src: Vec<f64>,
    // Face conductances, m^2/s.
    tx: Vec<f64>,
    ty: Vec<f64>,
    tz: Vec<f64>,
    tbot: Vec<f64>,
    ttop: Vec<f64>,
}

fn harmonic(a: f64, b: f64, dx: f64) -> f64 {
    2.0 * a * b / (a + b) * dx
}

impl Problem {
    pub fn new(n: usize, kc: Vec<f64>, dh: f64, src: Vec<f64>) -> Problem {
        assert_eq!(kc.len(), n * n * n);
        assert_eq!(src.len(), n * n * n);
        let dx = L / n as f64;
        let mut tx = vec![0.0; n * n * (n - 1)];
        let mut ty = vec![0.0; n * (n - 1) * n];
        let mut tz = vec![0.0; (n - 1) * n * n];
        for k in 0..n {
            for j in 0..n {
                for i in 0..n - 1 {
                    tx[(k * n + j) * (n - 1) + i] = harmonic(kc[idx(n, i, j, k)], kc[idx(n, i + 1, j, k)], dx);
                }
            }
        }
        for k in 0..n {
            for j in 0..n - 1 {
                for i in 0..n {
                    ty[(k * (n - 1) + j) * n + i] = harmonic(kc[idx(n, i, j, k)], kc[idx(n, i, j + 1, k)], dx);
                }
            }
        }
        for k in 0..n - 1 {
            for j in 0..n {
                for i in 0..n {
                    tz[(k * n + j) * n + i] = harmonic(kc[idx(n, i, j, k)], kc[idx(n, i, j, k + 1)], dx);
                }
            }
        }
        let mut tbot = vec![0.0; n * n];
        let mut ttop = vec![0.0; n * n];
        for j in 0..n {
            for i in 0..n {
                tbot[j * n + i] = 2.0 * kc[idx(n, i, j, 0)] * dx;
                ttop[j * n + i] = 2.0 * kc[idx(n, i, j, n - 1)] * dx;
            }
        }
        Problem { n, dx, kc, dh, src, tx, ty, tz, tbot, ttop }
    }

    /// A configuration on a zone mask at a water state.
    pub fn from_config(n: usize, mask: &[Zone], cfg: &Config, state: &WaterState, dh: f64) -> Problem {
        let f = state.rho_g_over_mu();
        let kc = mask.iter().map(|z| cfg.k_of(*z) * f).collect();
        Problem::new(n, kc, dh, vec![0.0; n * n * n])
    }

    pub fn cells(&self) -> usize {
        self.n * self.n * self.n
    }

    /// A h: per cell, sum over faces of T (h_c - h_neighbour), plus T_b h_c on
    /// each Dirichlet face. Symmetric positive definite.
    pub fn apply(&self, h: &[f64], out: &mut [f64]) {
        let n = self.n;
        out.iter_mut().for_each(|v| *v = 0.0);
        for k in 0..n {
            for j in 0..n {
                for i in 0..n - 1 {
                    let (a, b) = (idx(n, i, j, k), idx(n, i + 1, j, k));
                    let f = self.tx[(k * n + j) * (n - 1) + i] * (h[a] - h[b]);
                    out[a] += f;
                    out[b] -= f;
                }
            }
        }
        for k in 0..n {
            for j in 0..n - 1 {
                for i in 0..n {
                    let (a, b) = (idx(n, i, j, k), idx(n, i, j + 1, k));
                    let f = self.ty[(k * (n - 1) + j) * n + i] * (h[a] - h[b]);
                    out[a] += f;
                    out[b] -= f;
                }
            }
        }
        for k in 0..n - 1 {
            for j in 0..n {
                for i in 0..n {
                    let (a, b) = (idx(n, i, j, k), idx(n, i, j, k + 1));
                    let f = self.tz[(k * n + j) * n + i] * (h[a] - h[b]);
                    out[a] += f;
                    out[b] -= f;
                }
            }
        }
        for j in 0..n {
            for i in 0..n {
                out[idx(n, i, j, 0)] += self.tbot[j * n + i] * h[idx(n, i, j, 0)];
                out[idx(n, i, j, n - 1)] += self.ttop[j * n + i] * h[idx(n, i, j, n - 1)];
            }
        }
    }

    /// b: T_b dh on the bottom layer (the top head is 0), plus s_c dx^3.
    pub fn rhs(&self) -> Vec<f64> {
        let n = self.n;
        let vol = self.dx * self.dx * self.dx;
        let mut b: Vec<f64> = self.src.iter().map(|s| s * vol).collect();
        for j in 0..n {
            for i in 0..n {
                b[idx(n, i, j, 0)] += self.tbot[j * n + i] * self.dh;
            }
        }
        b
    }

    /// The linear initial guess h = dh (1 - z_c / L) (v1 section 5).
    pub fn linear_profile(&self) -> Vec<f64> {
        let n = self.n;
        let mut h = vec![0.0; self.cells()];
        for k in 0..n {
            let z = (k as f64 + 0.5) * self.dx;
            for j in 0..n {
                for i in 0..n {
                    h[idx(n, i, j, k)] = self.dh * (1.0 - z / L);
                }
            }
        }
        h
    }

    /// Dense copy of A, for the small D3 problems.
    pub fn dense(&self) -> Vec<f64> {
        let m = self.cells();
        let mut a = vec![0.0; m * m];
        let mut e = vec![0.0; m];
        let mut col = vec![0.0; m];
        for c in 0..m {
            e[c] = 1.0;
            self.apply(&e, &mut col);
            for r in 0..m {
                a[r * m + c] = col[r];
            }
            e[c] = 0.0;
        }
        a
    }
}

// ---------------------------------------------------------------- solvers

pub struct Solve {
    pub h: Vec<f64>,
    pub iterations: usize,
    pub restarts: usize,
    /// True ||b - A h||_2 / ||b||_2 at return (0 when ||b|| = 0).
    pub rel_residual: f64,
    pub converged: bool,
    pub b_norm: f64,
    pub method: &'static str,
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn true_residual(p: &Problem, b: &[f64], h: &[f64], ah: &mut [f64]) -> Vec<f64> {
    p.apply(h, ah);
    b.iter().zip(ah.iter()).map(|(bi, ai)| bi - ai).collect()
}

/// Unpreconditioned conjugate gradient in f64 from the linear profile. An
/// iterate is accepted only when the true residual meets the pre-registered
/// rule ||b - A h||_2 <= tol ||b||_2, with tol from `rule` (v1 1e-12, A4
/// 1e-13); when the recursive residual meets it but the true one does not,
/// CG restarts from the current h with the true residual. The iteration cap
/// counts every iteration. Not converging is returned as such, never retried
/// with other settings.
pub fn cg(p: &Problem, rule: SolverRule) -> Solve {
    let b = p.rhs();
    let b_norm = dot(&b, &b).sqrt();
    let mut h = p.linear_profile();
    let m = p.cells();
    let mut ah = vec![0.0; m];
    let max_it = cg_max_iterations(p.n);
    if b_norm == 0.0 {
        let r = true_residual(p, &b, &h, &mut ah);
        let ok = r.iter().all(|v| *v == 0.0);
        return Solve { h, iterations: 0, restarts: 0, rel_residual: 0.0, converged: ok, b_norm, method: "cg" };
    }
    let target = rule.tol() * b_norm;
    let mut r = true_residual(p, &b, &h, &mut ah);
    let mut d = r.clone();
    let mut rr = dot(&r, &r);
    let mut q = vec![0.0; m];
    let (mut it, mut restarts) = (0usize, 0usize);
    while it < max_it {
        if rr.sqrt() <= target {
            let rt = true_residual(p, &b, &h, &mut ah);
            let rtn = dot(&rt, &rt).sqrt();
            if rtn <= target {
                return Solve { h, iterations: it, restarts, rel_residual: rtn / b_norm, converged: true, b_norm, method: "cg" };
            }
            restarts += 1;
            r = rt;
            d = r.clone();
            rr = dot(&r, &r);
        }
        p.apply(&d, &mut q);
        let dq = dot(&d, &q);
        if dq <= 0.0 || !dq.is_finite() {
            break;
        }
        let alpha = rr / dq;
        for c in 0..m {
            h[c] += alpha * d[c];
            r[c] -= alpha * q[c];
        }
        let rr_new = dot(&r, &r);
        let beta = rr_new / rr;
        for c in 0..m {
            d[c] = r[c] + beta * d[c];
        }
        rr = rr_new;
        it += 1;
    }
    let rt = true_residual(p, &b, &h, &mut ah);
    let rtn = dot(&rt, &rt).sqrt();
    Solve { h, iterations: it, restarts, rel_residual: rtn / b_norm, converged: rtn <= target, b_norm, method: "cg" }
}

/// Dense Cholesky solve in f64 (D3 only).
pub fn cholesky(p: &Problem) -> Solve {
    let m = p.cells();
    let mut a = p.dense();
    let b = p.rhs();
    for c in 0..m {
        let mut s = a[c * m + c];
        for t in 0..c {
            s -= a[c * m + t] * a[c * m + t];
        }
        assert!(s > 0.0, "matrix not positive definite");
        let d = s.sqrt();
        a[c * m + c] = d;
        for r in c + 1..m {
            let mut s = a[r * m + c];
            for t in 0..c {
                s -= a[r * m + t] * a[c * m + t];
            }
            a[r * m + c] = s / d;
        }
    }
    let mut y = vec![0.0; m];
    for r in 0..m {
        let mut s = b[r];
        for t in 0..r {
            s -= a[r * m + t] * y[t];
        }
        y[r] = s / a[r * m + r];
    }
    let mut h = vec![0.0; m];
    for r in (0..m).rev() {
        let mut s = y[r];
        for t in r + 1..m {
            s -= a[t * m + r] * h[t];
        }
        h[r] = s / a[r * m + r];
    }
    let b_norm = dot(&b, &b).sqrt();
    let mut ah = vec![0.0; m];
    let rt = true_residual(p, &b, &h, &mut ah);
    let rel = if b_norm > 0.0 { dot(&rt, &rt).sqrt() / b_norm } else { 0.0 };
    Solve { h, iterations: 0, restarts: 0, rel_residual: rel, converged: true, b_norm, method: "cholesky" }
}

// ---------------------------------------------------------------- fluxes and residuals (A3)

/// Face fluxes in m^3/s. Interior faces: F = T (h_lower - h_upper), positive
/// along +x, +y, +z. Bottom: flux entering through z = 0, T_b (dh - h_c).
/// Top: flux leaving through z = L, T_b (h_c - 0). Both positive upward.
pub struct Fluxes {
    pub n: usize,
    pub fx: Vec<f64>,
    pub fy: Vec<f64>,
    pub fz: Vec<f64>,
    pub bottom: Vec<f64>,
    pub top: Vec<f64>,
}

pub fn fluxes(p: &Problem, h: &[f64]) -> Fluxes {
    let n = p.n;
    let mut fx = vec![0.0; p.tx.len()];
    let mut fy = vec![0.0; p.ty.len()];
    let mut fz = vec![0.0; p.tz.len()];
    for k in 0..n {
        for j in 0..n {
            for i in 0..n - 1 {
                let f = (k * n + j) * (n - 1) + i;
                fx[f] = p.tx[f] * (h[idx(n, i, j, k)] - h[idx(n, i + 1, j, k)]);
            }
        }
    }
    for k in 0..n {
        for j in 0..n - 1 {
            for i in 0..n {
                let f = (k * (n - 1) + j) * n + i;
                fy[f] = p.ty[f] * (h[idx(n, i, j, k)] - h[idx(n, i, j + 1, k)]);
            }
        }
    }
    for k in 0..n - 1 {
        for j in 0..n {
            for i in 0..n {
                let f = (k * n + j) * n + i;
                fz[f] = p.tz[f] * (h[idx(n, i, j, k)] - h[idx(n, i, j, k + 1)]);
            }
        }
    }
    let mut bottom = vec![0.0; n * n];
    let mut top = vec![0.0; n * n];
    for j in 0..n {
        for i in 0..n {
            bottom[j * n + i] = p.tbot[j * n + i] * (p.dh - h[idx(n, i, j, 0)]);
            top[j * n + i] = p.ttop[j * n + i] * h[idx(n, i, j, n - 1)];
        }
    }
    Fluxes { n, fx, fy, fz, bottom, top }
}

/// r_phys from the matrix: A h - b (A3.2).
pub fn residual_phys_matrix(p: &Problem, h: &[f64]) -> Vec<f64> {
    let mut ah = vec![0.0; p.cells()];
    p.apply(h, &mut ah);
    let b = p.rhs();
    ah.iter().zip(b.iter()).map(|(a, bi)| a - bi).collect()
}

/// r_phys from separately coded face fluxes: net outward flux - s_c dx^3 (A3.4).
pub fn residual_phys_faces(p: &Problem, fl: &Fluxes) -> Vec<f64> {
    let n = p.n;
    let vol = p.dx * p.dx * p.dx;
    let mut r = vec![0.0; p.cells()];
    for k in 0..n {
        for j in 0..n {
            for i in 0..n {
                let mut out = 0.0;
                if i + 1 < n {
                    out += fl.fx[(k * n + j) * (n - 1) + i];
                }
                if i > 0 {
                    out -= fl.fx[(k * n + j) * (n - 1) + i - 1];
                }
                if j + 1 < n {
                    out += fl.fy[(k * (n - 1) + j) * n + i];
                }
                if j > 0 {
                    out -= fl.fy[(k * (n - 1) + j - 1) * n + i];
                }
                if k + 1 < n {
                    out += fl.fz[(k * n + j) * n + i];
                } else {
                    out += fl.top[j * n + i];
                }
                if k > 0 {
                    out -= fl.fz[((k - 1) * n + j) * n + i];
                } else {
                    out -= fl.bottom[j * n + i];
                }
                r[idx(n, i, j, k)] = out - p.src[idx(n, i, j, k)] * vol;
            }
        }
    }
    r
}

/// Cell-centred Darcy flux (m/s): mean of each pair of opposite face fluxes
/// over dx^2 (v1 section 7). Closed side faces carry zero.
pub fn cell_flux(p: &Problem, fl: &Fluxes) -> [Vec<f64>; 3] {
    let n = p.n;
    let a = p.dx * p.dx;
    let m = p.cells();
    let (mut qx, mut qy, mut qz) = (vec![0.0; m], vec![0.0; m], vec![0.0; m]);
    for k in 0..n {
        for j in 0..n {
            for i in 0..n {
                let c = idx(n, i, j, k);
                let xm = if i > 0 { fl.fx[(k * n + j) * (n - 1) + i - 1] } else { 0.0 };
                let xp = if i + 1 < n { fl.fx[(k * n + j) * (n - 1) + i] } else { 0.0 };
                let ym = if j > 0 { fl.fy[(k * (n - 1) + j - 1) * n + i] } else { 0.0 };
                let yp = if j + 1 < n { fl.fy[(k * (n - 1) + j) * n + i] } else { 0.0 };
                let zm = if k > 0 { fl.fz[((k - 1) * n + j) * n + i] } else { fl.bottom[j * n + i] };
                let zp = if k + 1 < n { fl.fz[(k * n + j) * n + i] } else { fl.top[j * n + i] };
                qx[c] = 0.5 * (xm + xp) / a;
                qy[c] = 0.5 * (ym + yp) / a;
                qz[c] = 0.5 * (zm + zp) / a;
            }
        }
    }
    [qx, qy, qz]
}

// ---------------------------------------------------------------- checks

fn max_abs(v: &[f64]) -> f64 {
    v.iter().fold(0.0f64, |m, x| m.max(x.abs()))
}

/// D1 quantities for one solve (v1 D1 with A3.4).
#[derive(Clone, Debug)]
pub struct D1 {
    pub q_in: f64,
    pub q_out: f64,
    pub q_ref: f64,
    pub r_inf: f64,
    pub r_2: f64,
    pub eps: f64,
    pub global: f64,
    pub paths: f64,
    pub b_norm_over_q_ref: f64,
}

impl D1 {
    pub fn pass(&self) -> bool {
        self.q_ref > 0.0 && self.eps <= TOL_D1_EPS && self.global <= TOL_D1_GLOBAL && self.paths <= TOL_D1_PATHS
    }
}

pub fn d1(p: &Problem, s: &Solve) -> D1 {
    let fl = fluxes(p, &s.h);
    let rm = residual_phys_matrix(p, &s.h);
    let rf = residual_phys_faces(p, &fl);
    let q_in: f64 = fl.bottom.iter().sum();
    let q_out: f64 = fl.top.iter().sum();
    let q_ref = q_in.abs();
    let r_inf = max_abs(&rm);
    let r_2 = dot(&rm, &rm).sqrt();
    let diff: Vec<f64> = rf.iter().zip(rm.iter()).map(|(a, b)| a - b).collect();
    let rel = |x: f64| if q_ref > 0.0 { x / q_ref } else { f64::NAN };
    D1 { q_in, q_out, q_ref, r_inf, r_2, eps: rel(r_inf), global: rel((q_in - q_out).abs()), paths: rel(max_abs(&diff)), b_norm_over_q_ref: rel(s.b_norm) }
}

/// D1 zero-flow control: dh = 0 and s = 0 must give h = 0 and r_phys = 0 exactly.
pub fn d1_zero_flow(mask: &[Zone], cfg: &Config, rule: SolverRule) -> (bool, Solve) {
    let p = Problem::from_config(N, mask, cfg, &STATE_A, 0.0);
    let s = cg(&p, rule);
    let r = residual_phys_matrix(&p, &s.h);
    let ok = s.h.iter().all(|v| *v == 0.0) && r.iter().all(|v| *v == 0.0);
    (ok, s)
}

/// D2 manufactured solution: uniform K = 1e-6 m/s, A = 0.1 m, dh = 1 m.
pub const D2_K: f64 = 1e-6;
pub const D2_A: f64 = 0.1;

pub fn d2_exact(x: f64, y: f64, z: f64) -> f64 {
    use std::f64::consts::PI;
    DH * (1.0 - z / L) + D2_A * (PI * z / L).sin() * (PI * x / L).cos() * (PI * y / L).cos()
}

/// s_m = +(3 pi^2 K A / L^2) sin(pi z/L) cos(pi x/L) cos(pi y/L) (A3 sign convention).
pub fn d2_source(x: f64, y: f64, z: f64) -> f64 {
    use std::f64::consts::PI;
    3.0 * PI * PI * D2_K * D2_A / (L * L) * (PI * z / L).sin() * (PI * x / L).cos() * (PI * y / L).cos()
}

pub struct D2Level {
    pub n: usize,
    pub e2: f64,
    pub einf: f64,
    pub solve_iterations: usize,
    pub converged: bool,
}

pub fn d2_level(n: usize, rule: SolverRule) -> D2Level {
    let dx = L / n as f64;
    let m = n * n * n;
    let mut src = vec![0.0; m];
    for k in 0..n {
        for j in 0..n {
            for i in 0..n {
                let (x, y, z) = ((i as f64 + 0.5) * dx, (j as f64 + 0.5) * dx, (k as f64 + 0.5) * dx);
                src[idx(n, i, j, k)] = d2_source(x, y, z);
            }
        }
    }
    let p = Problem::new(n, vec![D2_K; m], DH, src);
    let s = cg(&p, rule);
    let (mut sum, mut einf) = (0.0, 0.0f64);
    for k in 0..n {
        for j in 0..n {
            for i in 0..n {
                let (x, y, z) = ((i as f64 + 0.5) * dx, (j as f64 + 0.5) * dx, (k as f64 + 0.5) * dx);
                let e = s.h[idx(n, i, j, k)] - d2_exact(x, y, z);
                sum += e * e * dx * dx * dx;
                einf = einf.max(e.abs());
            }
        }
    }
    D2Level { n, e2: (sum / (L * L * L)).sqrt(), einf, solve_iterations: s.iterations, converged: s.converged }
}

/// D3 layer conductivities (m/s) for N = 8.
pub const D3_K: [f64; 8] = [1e-5, 1e-7, 1e-6, 1e-8, 1e-5, 1e-7, 1e-6, 1e-8];
pub const D3_N: usize = 8;

/// D3a (series, K by layer in z) or D3b (parallel, K by column in x).
pub fn d3_problem(series: bool) -> Problem {
    let n = D3_N;
    let mut kc = vec![0.0; n * n * n];
    for k in 0..n {
        for j in 0..n {
            for i in 0..n {
                kc[idx(n, i, j, k)] = if series { D3_K[k] } else { D3_K[i] };
            }
        }
    }
    Problem::new(n, kc, DH, vec![0.0; n * n * n])
}

/// Exact total flux and exact cell-centre heads for D3.
pub fn d3_exact(series: bool) -> (f64, Vec<f64>) {
    let n = D3_N;
    let dx = L / n as f64;
    let mut h = vec![0.0; n * n * n];
    if series {
        let resist: f64 = D3_K.iter().map(|k| dx / k).sum();
        let q = DH / resist;
        for k in 0..n {
            let drop: f64 = D3_K[..k].iter().map(|kk| dx / kk).sum::<f64>() + dx / (2.0 * D3_K[k]);
            for j in 0..n {
                for i in 0..n {
                    h[idx(n, i, j, k)] = DH - q * drop;
                }
            }
        }
        (L * L * q, h)
    } else {
        for k in 0..n {
            let z = (k as f64 + 0.5) * dx;
            for j in 0..n {
                for i in 0..n {
                    h[idx(n, i, j, k)] = DH * (1.0 - z / L);
                }
            }
        }
        (D3_K.iter().map(|k| k * dx * DH).sum(), h)
    }
}

pub struct D3 {
    pub q_rel_direct: f64,
    pub head_err_direct: f64,
    pub q_rel_cg: f64,
    pub cg_converged: bool,
}

impl D3 {
    pub fn pass(&self) -> bool {
        self.q_rel_direct <= TOL_D3_DIRECT && self.head_err_direct <= TOL_D3_DIRECT && self.q_rel_cg <= TOL_D3_CG
    }
}

pub fn d3(series: bool, rule: SolverRule) -> D3 {
    let p = d3_problem(series);
    let (q_exact, h_exact) = d3_exact(series);
    let qin = |s: &Solve| fluxes(&p, &s.h).bottom.iter().sum::<f64>();
    let direct = cholesky(&p);
    let iter = cg(&p, rule);
    let head_err = direct.h.iter().zip(h_exact.iter()).fold(0.0f64, |m, (a, b)| m.max((a - b).abs())) / DH;
    D3 {
        q_rel_direct: (qin(&direct) - q_exact).abs() / q_exact,
        head_err_direct: head_err,
        q_rel_cg: (qin(&iter) - q_exact).abs() / q_exact,
        cg_converged: iter.converged,
    }
}

/// D4: mirror errors in x and in y, heads over dh and fluxes over Q_ref.
pub struct D4 {
    pub head_x: f64,
    pub head_y: f64,
    pub flux_x: f64,
    pub flux_y: f64,
}

impl D4 {
    pub fn pass(&self) -> bool {
        [self.head_x, self.head_y, self.flux_x, self.flux_y].iter().all(|v| *v <= TOL_D4)
    }
}

pub fn d4(p: &Problem, s: &Solve) -> D4 {
    let n = p.n;
    let fl = fluxes(p, &s.h);
    let q_ref = fl.bottom.iter().sum::<f64>().abs();
    let (mut hx, mut hy) = (0.0f64, 0.0f64);
    for k in 0..n {
        for j in 0..n {
            for i in 0..n {
                let h = s.h[idx(n, i, j, k)];
                hx = hx.max((h - s.h[idx(n, n - 1 - i, j, k)]).abs());
                hy = hy.max((h - s.h[idx(n, i, n - 1 - j, k)]).abs());
            }
        }
    }
    let (mut fxm, mut fym) = (0.0f64, 0.0f64);
    // x mirror: x faces reverse sign, y and z faces keep it.
    for k in 0..n {
        for j in 0..n {
            for i in 0..n - 1 {
                let a = fl.fx[(k * n + j) * (n - 1) + i];
                let b = fl.fx[(k * n + j) * (n - 1) + (n - 2 - i)];
                fxm = fxm.max((a + b).abs());
            }
        }
        for j in 0..n - 1 {
            for i in 0..n {
                let a = fl.fy[(k * (n - 1) + j) * n + i];
                let b = fl.fy[(k * (n - 1) + j) * n + (n - 1 - i)];
                fxm = fxm.max((a - b).abs());
                let c = fl.fy[(k * (n - 1) + (n - 2 - j)) * n + i];
                fym = fym.max((a + c).abs());
            }
        }
        for j in 0..n {
            for i in 0..n - 1 {
                let a = fl.fx[(k * n + j) * (n - 1) + i];
                let c = fl.fx[(k * n + (n - 1 - j)) * (n - 1) + i];
                fym = fym.max((a - c).abs());
            }
        }
    }
    for k in 0..n - 1 {
        for j in 0..n {
            for i in 0..n {
                let a = fl.fz[(k * n + j) * n + i];
                fxm = fxm.max((a - fl.fz[(k * n + j) * n + (n - 1 - i)]).abs());
                fym = fym.max((a - fl.fz[(k * n + (n - 1 - j)) * n + i]).abs());
            }
        }
    }
    for j in 0..n {
        for i in 0..n {
            for f in [&fl.bottom, &fl.top] {
                let a = f[j * n + i];
                fxm = fxm.max((a - f[j * n + (n - 1 - i)]).abs());
                fym = fym.max((a - f[(n - 1 - j) * n + i]).abs());
            }
        }
    }
    D4 { head_x: hx / DH, head_y: hy / DH, flux_x: fxm / q_ref, flux_y: fym / q_ref }
}

/// Per layer k: flux share through the z-faces above layer k attributed to
/// conduit cells (i, j, k), against the conduit area share of layer k.
pub fn layer_shares(p: &Problem, mask: &[Zone], fl: &Fluxes) -> Vec<(f64, f64)> {
    let n = p.n;
    (0..n)
        .map(|k| {
            let (mut total, mut conduit, mut count) = (0.0, 0.0, 0usize);
            for j in 0..n {
                for i in 0..n {
                    let f = if k + 1 < n { fl.fz[(k * n + j) * n + i] } else { fl.top[j * n + i] };
                    total += f;
                    if mask[idx(n, i, j, k)] == Zone::Conduit {
                        conduit += f;
                        count += 1;
                    }
                }
            }
            let share = if count == 0 { 0.0 } else { conduit / total };
            (share, count as f64 / (n * n) as f64)
        })
        .collect()
}

/// D5: all zones at the sandstone representative k, state A.
pub struct D5 {
    pub max_dev: f64,
    pub layers: Vec<(f64, f64)>,
    pub converged: bool,
}

impl D5 {
    pub fn pass(&self) -> bool {
        self.converged && self.max_dev <= TOL_D5
    }
}

pub fn d5(mask: &[Zone], rule: SolverRule) -> D5 {
    let ks = SANDSTONE_ANALOGUE.k_range().rep;
    let cfg = Config { name: "uniform".into(), k: [ks; 4] };
    let p = Problem::from_config(N, mask, &cfg, &STATE_A, DH);
    let s = cg(&p, rule);
    let fl = fluxes(&p, &s.h);
    let layers = layer_shares(&p, mask, &fl);
    let max_dev = layers.iter().fold(0.0f64, |m, (s, a)| m.max((s - a).abs()));
    D5 { max_dev, layers, converged: s.converged }
}

// ---------------------------------------------------------------- Péclet (A1.1)

/// Max over all cells, max over conduit cells, median over all cells (mean of
/// the two central values for an even count).
pub fn pe_stats(qmag: &[f64], mask: &[Zone], state: &WaterState, lambda: f64) -> (f64, f64, f64) {
    let f = state.rho_cp() * PE_LENGTH_M / lambda;
    let mut all: Vec<f64> = qmag.iter().map(|q| q * f).collect();
    let max_all = all.iter().copied().fold(0.0f64, f64::max);
    let max_conduit = qmag.iter().zip(mask).filter(|(_, z)| **z == Zone::Conduit).map(|(q, _)| q * f).fold(0.0f64, f64::max);
    all.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = all.len();
    let median = if m % 2 == 0 { 0.5 * (all[m / 2 - 1] + all[m / 2]) } else { all[m / 2] };
    (max_all, max_conduit, median)
}
