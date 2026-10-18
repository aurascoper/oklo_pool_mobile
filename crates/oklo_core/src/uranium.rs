//! Synthetic one-dimensional U(VI) reduction-transport column, implementing
//! uranium pre-registration v1 with Amendment A1 (docs/specs/). A synthetic
//! column, not a reconstruction of Oklo or the Franceville Basin; its rates,
//! dispersion and site inputs are declared assumptions, and its K_d values are
//! EPA sensitivity bounds. Every constant comes from those frozen
//! specifications or their parameter manifest; nothing here is fitted, and a
//! failed check is reported as a failure.

/// Column length, organic zone and inlet concentration (v1 section 4).
pub const L_COL: f64 = 16.0;
pub const X_A: f64 = 6.0;
pub const X_B: f64 = 10.0;
pub const C0: f64 = 1.0;
/// Porosity and grain density (v1 section 4, both assumptions).
pub const THETA: f64 = 0.15;
pub const GRAIN_DENSITY: f64 = 2650.0;
/// Dry bulk density, kg of solid per m3 of bulk medium.
pub const RHO_B: f64 = (1.0 - THETA) * GRAIN_DENSITY;
/// Scale for the S allowance of N1 and the C2(b) threshold (A1.5, A1.7), mol/kg.
pub const S_REF: f64 = THETA * C0 / RHO_B;

/// The full-factorial sweep of v1 section 4: 3 x 3 x 4 x 4 = 144 runs.
pub const VS: [f64; 3] = [1e-10, 1e-9, 1e-8];
pub const DS: [f64; 3] = [1e-11, 1e-10, 1e-9];
pub const KS: [f64; 4] = [0.0, 1e-11, 1e-10, 1e-9];
pub const KDS: [f64; 4] = [0.0, 0.063, 6.3, 630.0];

/// Grid rule floor (v1 section 3).
pub const N_MIN: usize = 64;
/// A1.3: production steps land on multiples of 0.05 tau; the six output times
/// of v1 section 4 (0.1, 0.25, 0.5, 1, 2 and 5 tau) are these multiples of it.
pub const LANDING_FRACTION: f64 = 0.05;
pub const OUTPUT_INTERVALS: [usize; 6] = [2, 5, 10, 20, 40, 100];
pub const OUTPUT_TAUS: [f64; 6] = [0.1, 0.25, 0.5, 1.0, 2.0, 5.0];

// Pre-registered acceptance thresholds (v1 section 6 and A1).
pub const ORDER_RANGE: (f64, f64) = (1.8, 2.2);
pub const LEDGER_REL: f64 = 1e-10;
pub const N1_REL: f64 = 1e-12;

/// Carried on every output.
pub const CAVEAT: &str = "SYNTHETIC column, not Oklo or the Franceville Basin; uranium prereg v1+A1";

/// Reasons a column is rejected before any step is taken.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColumnError {
    LengthNotPositive,
    TooFewCells,
    VelocityNotPositive,
    DiffusionNotPositive,
    RateNegative,
    RetardationBelowOne,
    MaskLength,
    /// v1 section 3 requires v dx / D_eff <= 2 in every run.
    CellPecletAboveTwo,
}

/// One column: geometry, transport, reaction and the organic-zone mask at cell
/// centres. Built only through `Column::new`, which validates it.
#[derive(Clone, Debug)]
pub struct Column {
    pub length: f64,
    pub n: usize,
    pub v: f64,
    pub d: f64,
    pub k_red: f64,
    pub r: f64,
    pub chi: Vec<bool>,
}

impl Column {
    pub fn new(length: f64, n: usize, v: f64, d: f64, k_red: f64, r: f64, chi: Vec<bool>) -> Result<Column, ColumnError> {
        if !(length.is_finite() && length > 0.0) {
            return Err(ColumnError::LengthNotPositive);
        }
        if n < 2 {
            return Err(ColumnError::TooFewCells);
        }
        if !(v.is_finite() && v > 0.0) {
            return Err(ColumnError::VelocityNotPositive);
        }
        if !(d.is_finite() && d > 0.0) {
            return Err(ColumnError::DiffusionNotPositive);
        }
        if !(k_red.is_finite() && k_red >= 0.0) {
            return Err(ColumnError::RateNegative);
        }
        if !(r.is_finite() && r >= 1.0) {
            return Err(ColumnError::RetardationBelowOne);
        }
        if chi.len() != n {
            return Err(ColumnError::MaskLength);
        }
        let col = Column { length, n, v, d, k_red, r, chi };
        if col.pe_cell() > 2.0 {
            return Err(ColumnError::CellPecletAboveTwo);
        }
        Ok(col)
    }

    pub fn dx(&self) -> f64 {
        self.length / self.n as f64
    }

    /// v dx / D_eff, evaluated as Pe_s / N with Pe_s in A1.4's order. The two
    /// are equal in exact arithmetic; evaluated as v * dx / D the six pairs that
    /// sit on Pe_cell = 2 come out at 2.0000000000000004 (finding 3 of
    /// docs/research/numerical_method_review_v1.md), while Pe_s / N reproduces
    /// the grid rule's guarantee. The operator's off-diagonal signs, which
    /// positivity needs, are tested directly.
    pub fn pe_cell(&self) -> f64 {
        pe_s(self.v, self.d, self.length) / self.n as f64
    }

    /// Largest step A1.2 allows for this column.
    pub fn dt_rule(&self) -> f64 {
        dt_rule(self.v, self.d, self.k_red, self.r, self.dx(), &self.chi)
    }
}

/// R = 1 + rho_b K_d / theta, equilibrium linear sorption of U(VI) only (v1 section 2).
pub fn retardation(kd: f64) -> f64 {
    1.0 + RHO_B * kd / THETA
}

/// Solute Péclet number in binary64, evaluated as (v * length) / d (A1.4).
pub fn pe_s(v: f64, d: f64, length: f64) -> f64 {
    (v * length) / d
}

/// N = max(64, ceil(Pe_s / 2)) (v1 section 3, evaluation order from A1.4).
pub fn grid_n(v: f64, d: f64, length: f64) -> usize {
    N_MIN.max((pe_s(v, d, length) / 2.0).ceil() as usize)
}

/// chi_org at cell centres: true for X_A <= x <= X_B (v1 section 2, A1.2).
pub fn organic_mask(n: usize, length: f64) -> Vec<bool> {
    let dx = length / n as f64;
    (0..n)
        .map(|i| {
            let x = (i as f64 + 0.5) * dx;
            (X_A..=X_B).contains(&x)
        })
        .collect()
}

/// A1.2: dt_rule = min(0.5 R dx / v, R / max_i |A_ii|), with the inlet,
/// interior and outlet diagonal magnitudes of v1's operator. This is the
/// largest step allowed, not necessarily the step used (A1.3 lands it).
pub fn dt_rule(v: f64, d: f64, k_red: f64, r: f64, dx: f64, chi: &[bool]) -> f64 {
    let max_diag = chi
        .iter()
        .enumerate()
        .map(|(i, &organic)| {
            let k = if organic { k_red } else { 0.0 };
            if i == 0 {
                3.0 * d / (dx * dx) + v / (2.0 * dx) + k
            } else if i + 1 == chi.len() {
                d / (dx * dx) + v / (2.0 * dx) + k
            } else {
                2.0 * d / (dx * dx) + k
            }
        })
        .fold(0.0_f64, f64::max);

    (0.5 * r * dx / v).min(r / max_diag)
}

/// A1.3: one constant step dt = interval / m with m = ceil(interval / dt_max).
#[derive(Clone, Copy, Debug)]
pub struct Landing {
    pub interval: f64,
    pub m: usize,
    pub dt: f64,
}

pub fn landing(interval: f64, dt_max: f64) -> Landing {
    let m = (interval / dt_max).ceil() as usize;
    Landing { interval, m, dt: interval / m as f64 }
}

/// One point of the production sweep.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RunConfig {
    pub v: f64,
    pub d: f64,
    pub k_red: f64,
    pub kd: f64,
}

impl RunConfig {
    pub fn column(&self) -> Result<Column, ColumnError> {
        let n = grid_n(self.v, self.d, L_COL);
        Column::new(L_COL, n, self.v, self.d, self.k_red, retardation(self.kd), organic_mask(n, L_COL))
    }

    /// tau = R L_col / v, one retarded advective travel time.
    pub fn tau(&self) -> f64 {
        retardation(self.kd) * L_COL / self.v
    }

    pub fn label(&self) -> String {
        format!("v{:e}_D{:e}_k{:e}_Kd{:e}", self.v, self.d, self.k_red, self.kd)
    }
}

/// The 144 runs, ordered by v, then D_eff, then k_red, then K_d.
pub fn sweep() -> Vec<RunConfig> {
    let mut out = Vec::with_capacity(144);
    for v in VS {
        for d in DS {
            for k_red in KS {
                for kd in KDS {
                    out.push(RunConfig { v, d, k_red, kd });
                }
            }
        }
    }
    out
}

/// Production landing (A1.3): interval 0.05 tau, so T_end = 5 tau is 100 m steps.
pub fn production_landing(cfg: &RunConfig, col: &Column) -> Landing {
    landing(LANDING_FRACTION * cfg.tau(), col.dt_rule())
}

// ---------------------------------------------------------------- operator (v1 section 3)

/// Tridiagonal matrix: `lo[i]` couples cell i to i-1 (lo[0] unused), `up[i]`
/// couples cell i to i+1 (up[n-1] unused).
#[derive(Clone, Debug)]
pub struct Tridiag {
    pub lo: Vec<f64>,
    pub di: Vec<f64>,
    pub up: Vec<f64>,
}

/// The semi-discrete system R dC/dt = A C + b, with b nonzero only in the
/// inlet cell, b_0 = b0.
#[derive(Clone, Debug)]
pub struct Operator {
    pub a: Tridiag,
    pub b0: f64,
}

/// Face j as a linear function of its neighbours, F_j = left C_{j-1} + right
/// C_j + constant, per unit pore area. Face 0 is the inlet, face n the outlet.
#[derive(Clone, Copy, Debug)]
struct Face {
    left: f64,
    right: f64,
    constant: f64,
}

fn faces(col: &Column) -> Vec<Face> {
    let (v, d, dx, n) = (col.v, col.d, col.dx(), col.n);
    let mut f = Vec::with_capacity(n + 1);
    // Inlet: advective value C0, diffusive D (C_1 - C0) / (dx / 2).
    f.push(Face { left: 0.0, right: -2.0 * d / dx, constant: (v + 2.0 * d / dx) * C0 });
    // Interior: central advective average, central diffusive difference.
    for _ in 1..n {
        f.push(Face { left: v / 2.0 + d / dx, right: v / 2.0 - d / dx, constant: 0.0 });
    }
    // Outlet: zero diffusive gradient, advective value of the last cell.
    f.push(Face { left: v, right: 0.0, constant: 0.0 });
    f
}

/// A and b assembled from the face rules: R dC_i/dt = (F_i - F_{i+1}) / dx - k_red chi_i C_i.
pub fn operator(col: &Column) -> Operator {
    let (n, dx) = (col.n, col.dx());
    let f = faces(col);
    let mut a = Tridiag { lo: vec![0.0; n], di: vec![0.0; n], up: vec![0.0; n] };
    for i in 0..n {
        let k = if col.chi[i] { col.k_red } else { 0.0 };
        a.di[i] = (f[i].right - f[i + 1].left) / dx - k;
        if i > 0 {
            a.lo[i] = f[i].left / dx;
        }
        if i + 1 < n {
            a.up[i] = -f[i + 1].right / dx;
        }
    }
    Operator { a, b0: f[0].constant / dx }
}

/// The n + 1 face fluxes F_j for a profile c, per unit pore area (mol m^-2 s^-1
/// of pore water); multiply by theta for flux per unit bulk cross-section.
pub fn face_fluxes(col: &Column, c: &[f64]) -> Vec<f64> {
    let n = col.n;
    faces(col)
        .iter()
        .enumerate()
        .map(|(j, f)| {
            let l = if j > 0 { f.left * c[j - 1] } else { 0.0 };
            let r = if j < n { f.right * c[j] } else { 0.0 };
            l + r + f.constant
        })
        .collect()
}

impl Operator {
    /// (A c + b)_i.
    pub fn apply(&self, c: &[f64]) -> Vec<f64> {
        let n = c.len();
        (0..n)
            .map(|i| {
                let mut y = self.a.di[i] * c[i];
                if i > 0 {
                    y += self.a.lo[i] * c[i - 1];
                }
                if i + 1 < n {
                    y += self.a.up[i] * c[i + 1];
                }
                if i == 0 {
                    y += self.b0;
                }
                y
            })
            .collect()
    }
}

/// The Crank-Nicolson pair for step dt: implicit M = R I - (dt/2) A and
/// explicit E = R I + (dt/2) A.
pub fn cn_matrices(col: &Column, op: &Operator, dt: f64) -> (Tridiag, Tridiag) {
    let h = 0.5 * dt;
    let m = Tridiag {
        lo: op.a.lo.iter().map(|x| -h * x).collect(),
        di: op.a.di.iter().map(|x| col.r - h * x).collect(),
        up: op.a.up.iter().map(|x| -h * x).collect(),
    };
    let e = Tridiag {
        lo: op.a.lo.iter().map(|x| h * x).collect(),
        di: op.a.di.iter().map(|x| col.r + h * x).collect(),
        up: op.a.up.iter().map(|x| h * x).collect(),
    };
    (m, e)
}
