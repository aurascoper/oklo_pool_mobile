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

// ---------------------------------------------------------------- Crank-Nicolson (v1 section 3, A1.2, A1.3)

/// Thomas factorization of a tridiagonal matrix, without pivoting. The
/// matrices factored here are strictly diagonally dominant M-matrices, for
/// which elimination without pivoting is stable.
#[derive(Clone, Debug)]
pub struct Thomas {
    lo: Vec<f64>,
    denom: Vec<f64>,
    cprime: Vec<f64>,
}

impl Thomas {
    pub fn new(m: &Tridiag) -> Thomas {
        let n = m.di.len();
        let mut denom = vec![0.0; n];
        let mut cprime = vec![0.0; n];
        denom[0] = m.di[0];
        cprime[0] = m.up[0] / denom[0];
        for i in 1..n {
            denom[i] = m.di[i] - m.lo[i] * cprime[i - 1];
            cprime[i] = if i + 1 < n { m.up[i] / denom[i] } else { 0.0 };
        }
        Thomas { lo: m.lo.clone(), denom, cprime }
    }

    /// Overwrites rhs with the solution x of M x = rhs.
    pub fn solve(&self, rhs: &mut [f64]) {
        let n = rhs.len();
        rhs[0] /= self.denom[0];
        for i in 1..n {
            rhs[i] = (rhs[i] - self.lo[i] * rhs[i - 1]) / self.denom[i];
        }
        for i in (0..n - 1).rev() {
            rhs[i] -= self.cprime[i] * rhs[i + 1];
        }
    }
}

/// One column's Crank-Nicolson step at a fixed dt: M c_new = E c + dt b.
#[derive(Clone, Debug)]
pub struct Integrator {
    pub dt: f64,
    e: Tridiag,
    thomas: Thomas,
    dt_b0: f64,
}

impl Integrator {
    pub fn new(col: &Column, op: &Operator, dt: f64) -> Integrator {
        let (m, e) = cn_matrices(col, op, dt);
        Integrator { dt, e, thomas: Thomas::new(&m), dt_b0: dt * op.b0 }
    }

    pub fn step(&self, c: &[f64], out: &mut [f64]) {
        let n = c.len();
        let e = &self.e;
        for i in 0..n {
            let mut y = e.di[i] * c[i];
            if i > 0 {
                y += e.lo[i] * c[i - 1];
            }
            if i + 1 < n {
                y += e.up[i] * c[i + 1];
            }
            out[i] = y;
        }
        out[0] += self.dt_b0;
        self.thomas.solve(out);
    }
}

/// First value outside the N1 bounds (A1.5).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Violation {
    pub step: usize,
    pub cell: usize,
    pub what: &'static str,
    pub value: f64,
}

/// N1 bookkeeping: extremes of C and S over every step and cell, and the
/// number of values outside -1e-12 C0 <= C <= (1 + 1e-12) C0, S >= -1e-12 S_ref.
#[derive(Clone, Copy, Debug)]
pub struct Bounds {
    pub c_min: f64,
    pub c_max: f64,
    pub s_min: f64,
    pub steps: usize,
    pub violations: usize,
    pub first: Option<Violation>,
}

impl Default for Bounds {
    fn default() -> Self {
        Bounds { c_min: f64::INFINITY, c_max: f64::NEG_INFINITY, s_min: f64::INFINITY, steps: 0, violations: 0, first: None }
    }
}

impl Bounds {
    fn flag(&mut self, step: usize, cell: usize, what: &'static str, value: f64) {
        self.violations += 1;
        if self.first.is_none() {
            self.first = Some(Violation { step, cell, what, value });
        }
    }

    pub fn observe_c(&mut self, step: usize, c: &[f64]) {
        for (i, &x) in c.iter().enumerate() {
            self.c_min = self.c_min.min(x);
            self.c_max = self.c_max.max(x);
            if !(x >= -N1_REL * C0) {
                self.flag(step, i, "C below -1e-12 C0", x);
            } else if !(x <= (1.0 + N1_REL) * C0) {
                self.flag(step, i, "C above (1 + 1e-12) C0", x);
            }
        }
    }

    pub fn observe_s(&mut self, step: usize, s: &[f64]) {
        for (i, &x) in s.iter().enumerate() {
            self.s_min = self.s_min.min(x);
            if !(x >= -N1_REL * S_REF) {
                self.flag(step, i, "S below -1e-12 S_ref", x);
            }
        }
    }

    pub fn n1_pass(&self) -> bool {
        self.violations == 0
    }
}

// ---------------------------------------------------------------- solid inventory and ledger (v1 sections 2, 3 and 6)

/// The ledger at one landing point, per unit bulk cross-section (mol/m2).
#[derive(Clone, Copy, Debug)]
pub struct LedgerPoint {
    pub step: usize,
    pub t: f64,
    /// Dissolved plus sorbed U(VI), sum of theta R C dx.
    pub aqueous_sorbed: f64,
    /// Immobilized uranium, sum of rho_b S dx.
    pub immobilized: f64,
    pub inflow: f64,
    pub outflow: f64,
}

impl LedgerPoint {
    /// Inventory minus (cumulative inflow - cumulative outflow); zero in exact arithmetic.
    pub fn closure(&self) -> f64 {
        self.aqueous_sorbed + self.immobilized - (self.inflow - self.outflow)
    }

    /// |closure| / cumulative inflow, the quantity v1's ledger check bounds by 1e-10.
    pub fn closure_rel(&self) -> f64 {
        self.closure().abs() / self.inflow
    }
}

/// C and S at one output time.
#[derive(Clone, Debug)]
pub struct Snapshot {
    pub step: usize,
    pub t: f64,
    pub c: Vec<f64>,
    pub s: Vec<f64>,
}

#[derive(Clone, Debug)]
pub struct RunResult {
    pub landing: Landing,
    pub steps: usize,
    /// Outlet value C_N (the outlet face's advective value) at every landing point, from t = 0.
    pub breakthrough: Vec<(f64, f64)>,
    /// Ledger at every landing point, from t = 0.
    pub ledger: Vec<LedgerPoint>,
    /// Snapshots at the requested landing points.
    pub snapshots: Vec<Snapshot>,
    pub bounds: Bounds,
}

impl RunResult {
    /// Ledger points at the snapshot times, where v1's closure check applies.
    pub fn ledger_at_outputs(&self) -> Vec<LedgerPoint> {
        self.snapshots.iter().map(|s| *self.ledger.iter().find(|p| p.step == s.step).expect("ledger at output")).collect()
    }

    /// Largest closure_rel at the output times, and whether every one is within LEDGER_REL.
    pub fn ledger_check(&self) -> (f64, bool) {
        let worst = self.ledger_at_outputs().iter().map(|p| p.closure_rel()).fold(0.0, f64::max);
        (worst, worst <= LEDGER_REL)
    }
}

/// Run a column from C = S = 0 for n_intervals landing intervals of m steps
/// each, keeping snapshots at the landing indices in `outputs`. S is advanced
/// with the same Crank-Nicolson weighting of theta k_red chi C as the C
/// equation, and the boundary fluxes are integrated with that weighting, so
/// the ledger closes in exact arithmetic (v1 section 3).
pub fn run(col: &Column, landing: Landing, n_intervals: usize, outputs: &[usize]) -> RunResult {
    let (n, dx, dt) = (col.n, col.dx(), landing.dt);
    let op = operator(col);
    let integ = Integrator::new(col, &op, dt);
    let inlet = faces(col)[0];
    let zone: Vec<usize> = (0..n).filter(|&i| col.chi[i]).collect();
    let s_gain = THETA * col.k_red * dt / (2.0 * RHO_B);

    let (mut c, mut next, mut s) = (vec![0.0; n], vec![0.0; n], vec![0.0; n]);
    let (mut inflow, mut outflow) = (0.0, 0.0);
    let f_in = |c: &[f64]| THETA * (inlet.constant + inlet.right * c[0]);
    let f_out = |c: &[f64]| THETA * col.v * c[n - 1];
    let point = |step: usize, c: &[f64], s: &[f64], inflow: f64, outflow: f64| LedgerPoint {
        step,
        t: step as f64 * dt,
        aqueous_sorbed: THETA * col.r * dx * c.iter().sum::<f64>(),
        immobilized: RHO_B * dx * s.iter().sum::<f64>(),
        inflow,
        outflow,
    };

    let mut res = RunResult { landing, steps: landing.m * n_intervals, breakthrough: vec![(0.0, 0.0)], ledger: vec![point(0, &c, &s, 0.0, 0.0)], snapshots: Vec::new(), bounds: Bounds::default() };
    if outputs.contains(&0) {
        res.snapshots.push(Snapshot { step: 0, t: 0.0, c: c.clone(), s: s.clone() });
    }
    for step in 1..=res.steps {
        integ.step(&c, &mut next);
        inflow += 0.5 * dt * (f_in(&c) + f_in(&next));
        outflow += 0.5 * dt * (f_out(&c) + f_out(&next));
        for &i in &zone {
            s[i] += s_gain * (c[i] + next[i]);
        }
        std::mem::swap(&mut c, &mut next);
        res.bounds.steps += 1;
        res.bounds.observe_c(step, &c);
        res.bounds.observe_s(step, &s);
        if step % landing.m == 0 {
            let t = step as f64 * dt;
            res.breakthrough.push((t, c[n - 1]));
            res.ledger.push(point(step, &c, &s, inflow, outflow));
            if outputs.contains(&(step / landing.m)) {
                res.snapshots.push(Snapshot { step, t, c: c.clone(), s: s.clone() });
            }
        }
    }
    res
}

/// A production run of v1 section 4: T_end = 5 tau, snapshots at the six output times.
pub fn run_production(cfg: &RunConfig) -> Result<RunResult, ColumnError> {
    let col = cfg.column()?;
    let l = production_landing(cfg, &col);
    Ok(run(&col, l, 100, &OUTPUT_INTERVALS))
}

// ---------------------------------------------------------------- B1 Ogata-Banks benchmark and T1 (v1 section 6, A1.6)

/// B1: a separate semi-infinite problem on a long column, not the production column.
pub const B1_LENGTH: f64 = 64.0;
pub const B1_V: f64 = 1e-9;
pub const B1_D: f64 = 1e-9;
pub const B1_T: f64 = 1.6e10;
pub const B1_WINDOW: f64 = 32.0;
pub const B1_GRIDS: [usize; 3] = [256, 512, 1024];
pub const T1_N: usize = 256;

/// Below this argument erfc is 1 - erf from the positive series; at or above
/// it, the continued fraction for erfcx.
pub const ERFC_SWITCH: f64 = 2.0;

/// erf(x) = (2/sqrt(pi)) exp(-x^2) sum_n 2^n x^(2n+1) / (2n+1)!!, whose terms
/// all share the sign of x, so the sum has no cancellation.
fn erf_series(x: f64) -> f64 {
    let x2 = x * x;
    let (mut term, mut sum, mut n) = (x, x, 0.0);
    while term.abs() > 1e-17 * sum.abs() {
        n += 1.0;
        term *= 2.0 * x2 / (2.0 * n + 1.0);
        sum += term;
    }
    std::f64::consts::FRAC_2_SQRT_PI * (-x2).exp() * sum
}

/// exp(x^2) erfc(x) for x >= ERFC_SWITCH from the continued fraction
/// erfc(x) = exp(-x^2) / sqrt(pi) / (x + (1/2) / (x + 1 / (x + (3/2) / (x + ...)))),
/// evaluated by the modified Lentz method.
fn erfcx_cf(x: f64) -> f64 {
    const TINY: f64 = 1e-300;
    let (mut f, mut c, mut d) = (x, x, 0.0);
    for n in 1..100_000 {
        let a = 0.5 * n as f64;
        d = x + a * d;
        d = if d == 0.0 { 1.0 / TINY } else { 1.0 / d };
        c = x + a / c;
        if c == 0.0 {
            c = TINY;
        }
        let delta = c * d;
        f *= delta;
        if (delta - 1.0).abs() <= 2.0 * f64::EPSILON {
            break;
        }
    }
    0.5 * std::f64::consts::FRAC_2_SQRT_PI / f
}

/// Complementary error function for real x.
pub fn erfc(x: f64) -> f64 {
    if x < 0.0 {
        2.0 - erfc(-x)
    } else if x < ERFC_SWITCH {
        1.0 - erf_series(x)
    } else {
        (-x * x).exp() * erfcx_cf(x)
    }
}

/// Scaled complementary error function exp(x^2) erfc(x), for x >= 0.
pub fn erfcx(x: f64) -> f64 {
    assert!(x >= 0.0, "erfcx is used here only for x >= 0");
    if x < ERFC_SWITCH {
        (x * x).exp() * (1.0 - erf_series(x))
    } else {
        erfcx_cf(x)
    }
}

/// Ogata and Banks 1961, equation 12: C / C0 = (1/2) [erfc((x - vt) / (2 sqrt(Dt)))
/// + exp(vx / D) erfc((x + vt) / (2 sqrt(Dt)))], with the second term
/// evaluated as exp(vx / D - z^2) erfcx(z) to avoid overflow of exp(vx / D).
pub fn ogata_banks(x: f64, t: f64, v: f64, d: f64) -> f64 {
    let s = 2.0 * (d * t).sqrt();
    let z1 = (x - v * t) / s;
    let z2 = (x + v * t) / s;
    0.5 * C0 * (erfc(z1) + (v * x / d - z2 * z2).exp() * erfcx(z2))
}

pub fn b1_column(n: usize) -> Column {
    Column::new(B1_LENGTH, n, B1_V, B1_D, 0.0, 1.0, vec![false; n]).expect("B1 column is valid")
}

/// sqrt(sum dx f_i^2) over the cells whose centres lie in 0 to B1_WINDOW.
pub fn window_l2(col: &Column, f: impl Fn(usize) -> f64) -> f64 {
    let dx = col.dx();
    (0..col.n).filter(|&i| (i as f64 + 0.5) * dx <= B1_WINDOW).map(|i| dx * f(i) * f(i)).sum::<f64>().sqrt()
}

#[derive(Clone, Debug)]
pub struct B1Result {
    pub steps: [usize; 3],
    pub dt: [f64; 3],
    pub errors: [f64; 3],
    pub orders: [f64; 2],
    pub n1_pass: bool,
    pub ledger_worst: f64,
}

impl B1Result {
    pub fn pass(&self) -> bool {
        self.orders.iter().all(|p| (ORDER_RANGE.0..=ORDER_RANGE.1).contains(p))
    }
}

/// B1: the Ogata-Banks problem at N = 256, 512, 1024, compared at t* over 0 to 32 m.
pub fn b1() -> B1Result {
    let mut r = B1Result { steps: [0; 3], dt: [0.0; 3], errors: [0.0; 3], orders: [0.0; 2], n1_pass: true, ledger_worst: 0.0 };
    for (g, &n) in B1_GRIDS.iter().enumerate() {
        let col = b1_column(n);
        let l = landing(B1_T, col.dt_rule());
        let run = run(&col, l, 1, &[1]);
        let c = &run.snapshots[0].c;
        let dx = col.dx();
        r.steps[g] = run.steps;
        r.dt[g] = l.dt;
        r.errors[g] = window_l2(&col, |i| c[i] - ogata_banks((i as f64 + 0.5) * dx, B1_T, B1_V, B1_D));
        r.n1_pass &= run.bounds.n1_pass();
        r.ledger_worst = r.ledger_worst.max(run.ledger_check().0);
    }
    r.orders = [(r.errors[0] / r.errors[1]).log2(), (r.errors[1] / r.errors[2]).log2()];
    r
}

#[derive(Clone, Debug)]
pub struct T1Result {
    pub steps: [usize; 3],
    pub e: [f64; 2],
    pub order: f64,
    pub n1_pass: bool,
}

impl T1Result {
    pub fn pass(&self) -> bool {
        (ORDER_RANGE.0..=ORDER_RANGE.1).contains(&self.order)
    }
}

/// T1: the B1 problem at N = 256 with dt_0 = t* / m_0, dt_0 / 2 and dt_0 / 4;
/// the order comes from successive differences, which cancel the shared
/// spatial error.
pub fn t1() -> T1Result {
    let col = b1_column(T1_N);
    let m0 = landing(B1_T, col.dt_rule()).m;
    let mut sols = Vec::new();
    let mut r = T1Result { steps: [0; 3], e: [0.0; 2], order: f64::NAN, n1_pass: true };
    for j in 0..3 {
        let m = m0 << j;
        let run = run(&col, Landing { interval: B1_T, m, dt: B1_T / m as f64 }, 1, &[1]);
        r.steps[j] = m;
        r.n1_pass &= run.bounds.n1_pass();
        sols.push(run.snapshots[0].c.clone());
    }
    r.e = [window_l2(&col, |i| sols[0][i] - sols[1][i]), window_l2(&col, |i| sols[1][i] - sols[2][i])];
    r.order = (r.e[0] / r.e[1]).log2();
    r
}

// ---------------------------------------------------------------- B2 finite-domain steady state (v1 section 6)

/// B2: the production boundary-value problem with uniform reaction.
pub const B2_LENGTH: f64 = 16.0;
pub const B2_V: f64 = 1e-9;
pub const B2_D: f64 = 1e-9;
pub const B2_K: f64 = 1e-10;
pub const B2_GRIDS: [usize; 3] = [64, 128, 256];

pub fn b2_column(n: usize) -> Column {
    Column::new(B2_LENGTH, n, B2_V, B2_D, B2_K, 1.0, vec![true; n]).expect("B2 column is valid")
}

/// r1, r2 = (v +- sqrt(v^2 + 4 D k)) / (2 D), the roots of D r^2 - v r - k = 0.
pub fn b2_roots() -> (f64, f64) {
    let q = (B2_V * B2_V + 4.0 * B2_D * B2_K).sqrt();
    ((B2_V + q) / (2.0 * B2_D), (B2_V - q) / (2.0 * B2_D))
}

/// C(x) = C0 [r2 e^(r2 L) e^(r1 x) - r1 e^(r1 L) e^(r2 x)] / [r2 e^(r2 L) - r1 e^(r1 L)].
pub fn b2_exact(x: f64) -> f64 {
    let (r1, r2) = b2_roots();
    let l = B2_LENGTH;
    C0 * (r2 * (r2 * l).exp() * (r1 * x).exp() - r1 * (r1 * l).exp() * (r2 * x).exp()) / (r2 * (r2 * l).exp() - r1 * (r1 * l).exp())
}

/// The discrete steady state, A C + b = 0 (dt -> infinity), solved directly.
pub fn steady_state(col: &Column) -> Vec<f64> {
    let op = operator(col);
    let neg = Tridiag { lo: op.a.lo.iter().map(|x| -x).collect(), di: op.a.di.iter().map(|x| -x).collect(), up: op.a.up.iter().map(|x| -x).collect() };
    let mut c = vec![0.0; col.n];
    c[0] = op.b0;
    Thomas::new(&neg).solve(&mut c);
    c
}

#[derive(Clone, Debug)]
pub struct B2Result {
    pub errors: [f64; 3],
    pub orders: [f64; 2],
    /// Largest |A C + b| relative to b0 over the three grids.
    pub residual: f64,
}

impl B2Result {
    pub fn pass(&self) -> bool {
        self.orders.iter().all(|p| (ORDER_RANGE.0..=ORDER_RANGE.1).contains(p))
    }
}

/// B2: discrete steady state against the exact solution at N = 64, 128, 256,
/// L2 error over the whole column.
pub fn b2() -> B2Result {
    let mut r = B2Result { errors: [0.0; 3], orders: [0.0; 2], residual: 0.0 };
    for (g, &n) in B2_GRIDS.iter().enumerate() {
        let col = b2_column(n);
        let c = steady_state(&col);
        let op = operator(&col);
        r.residual = r.residual.max(op.apply(&c).iter().fold(0.0f64, |a, x| a.max(x.abs())) / op.b0);
        let dx = col.dx();
        r.errors[g] = (0..n).map(|i| dx * (c[i] - b2_exact((i as f64 + 0.5) * dx)).powi(2)).sum::<f64>().sqrt();
    }
    r.orders = [(r.errors[0] / r.errors[1]).log2(), (r.errors[1] / r.errors[2]).log2()];
    r
}
