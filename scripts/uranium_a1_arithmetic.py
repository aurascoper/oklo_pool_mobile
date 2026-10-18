"""Arithmetic for uranium pre-registration v1 Amendment A1 (spec arithmetic only, no model run).

Run: python3 -I scripts/uranium_a1_arithmetic.py

Builds the Crank-Nicolson matrices that v1's discretization defines, checks their sign
patterns under v1's step rule and under the A1 rule, and counts steps. It never time-steps
the model or looks at a solution.
"""
import math

L = 16.0
THETA = 0.15
RHO_B = (1 - THETA) * 2650.0
C0 = 1.0
XA, XB = 6.0, 10.0
VS = (1e-10, 1e-9, 1e-8)
DS = (1e-11, 1e-10, 1e-9)
KS = (0.0, 1e-11, 1e-10, 1e-9)
KDS = (0.0, 0.063, 6.3, 630.0)


def n_cells(v, D, Lc=L, nmin=64):
    pe_s = (v * Lc) / D
    return max(nmin, math.ceil(pe_s / 2)), pe_s


def chi(N, Lc):
    dx = Lc / N
    return [1.0 if XA <= (i + 0.5) * dx <= XB else 0.0 for i in range(N)]


def operator(v, D, k, N, Lc, chis):
    """Tridiagonal semi-discrete operator A (R dC/dt = A C + b) from v1 section 3."""
    dx = Lc / N
    lo, di, up = [0.0] * N, [0.0] * N, [0.0] * N
    for i in range(N):
        kc = k * chis[i]
        if i == 0:
            di[i] = -(3 * D / dx**2 + v / (2 * dx) + kc)
            up[i] = (D / dx - v / 2) / dx
        elif i == N - 1:
            di[i] = -(D / dx**2 + v / (2 * dx) + kc)
            lo[i] = (D / dx + v / 2) / dx
        else:
            di[i] = -(2 * D / dx**2 + kc)
            lo[i] = (D / dx + v / 2) / dx
            up[i] = (D / dx - v / 2) / dx
    return lo, di, up


def dt_rules(v, D, k, R, N, Lc, chis):
    dx = Lc / N
    lo, di, up = operator(v, D, k, N, Lc, chis)
    dt_adv = 0.5 * R * dx / v
    dt_pos = R / max(-x for x in di)
    return dt_adv, dt_pos, (lo, di, up)


def sign_check(R, dt, ops):
    """Explicit RI + dt/2 A >= 0 entrywise; implicit RI - dt/2 A a Z-matrix, row dominant."""
    lo, di, up = ops
    min_diag_frac = min(R + 0.5 * dt * d for d in di) / R
    min_off = min(min(lo[1:]), min(up[:-1]))
    max_off_rel = max(max(abs(x) for x in lo[1:]), max(abs(x) for x in up[:-1])) or 1.0
    dom = min(R - 0.5 * dt * di[i] - 0.5 * dt * (lo[i] + up[i]) for i in range(len(di)))
    return min_diag_frac, min_off, min_off / max_off_rel, dom > 0


def g_hi(d):
    return abs((1 - 2 * d) / (1 + 2 * d))


print("== v1 production sweep: grid and diffusion number (R cancels from d, steps and sign checks)")
print("v D Pe_s(f64) N Pe_cell d_v1 |g_hi|_v1 0.1tau_in_v1_steps")
rows = []
for v in VS:
    for D in DS:
        N, pe_s = n_cells(v, D)
        dx = L / N
        pc = v * dx / D
        d1 = 1 / (2 * pc)
        rows.append((v, D, N, pc))
        print(f"{v:.0e} {D:.0e} {pe_s!r} {N} {pc:.6g} {d1:.6g} {g_hi(d1):.4f} {0.2 * N:.6g}")

print("\n== A1 rule over all 144 runs: dt_rule = min(0.5 R dx/v, R/max|A_ii|), constant dt = I/ceil(I/dt_rule), I = 0.05 tau")
print("v D k  binding  d_A1(interior)  min_expl_diag/R  min_offdiag_signed  rel  M_dominant  steps_A1 (=100*ceil)  steps_v1(=10N)")
worst = (0, None)
all_ok = True
neg_off = []
checked = 0
min_mdf = float("inf")
for v, D, N, pc in rows:
    chis = chi(N, L)
    for k in KS:
        for Kd in KDS:
            R = 1 + RHO_B * Kd / THETA
            dta, dtp, ops = dt_rules(v, D, k, R, N, L, chis)
            rule = min(dta, dtp)
            tau = R * L / v
            I = 0.05 * tau
            m = math.ceil(I / rule)
            dt = I / m
            mdf, mo, rel, dom = sign_check(R, dt, ops)
            checked += 1
            min_mdf = min(min_mdf, mdf)
            ok = mdf >= 0.5 and dom
            all_ok &= ok
            if mo < 0:
                neg_off.append((v, D, k, Kd, N, mo, rel))
            steps = 100 * m
            if steps > worst[0]:
                worst = (steps, (v, D, k, Kd, N))
            if Kd == 0.0:
                dx = L / N
                print(f"{v:.0e} {D:.0e} {k:.0e} {'adv' if dta <= dtp else 'pos'} {D * dt / (R * dx * dx):.4f} {mdf:.4f} {mo:.3e} {rel:.1e} {dom} {steps} {10 * N}")
print("rows printed for K_d = 0 only; the loop checked all K_d values: runs checked =", checked)
print("min over all runs of min explicit diagonal / R:", repr(min_mdf))
print("all runs: explicit diagonal >= R/2 and implicit matrix row-dominant:", all_ok)
print("max steps over 144:", worst)
print("off-diagonals negative in f64 (Pe_cell at 2 within rounding):", len(neg_off))
for x in neg_off[:6]:
    print("   ", x)
print("distinct (v,D,N) with a negative f64 off-diagonal:", sorted({(a, b, e) for a, b, c, dd, e, f, g in neg_off}))

print("\n== B1 (R = 1, k = 0, v = D = 1e-9, L_B1 = 64 m, t* = 1.6e10 s)")
for N in (256, 512, 1024):
    v = D = 1e-9
    Lb = 64.0
    dx = Lb / N
    chis = [0.0] * N
    dta, dtp, ops = dt_rules(v, D, 0.0, 1.0, N, Lb, chis)
    ts = 1.6e10
    m = math.ceil(ts / min(dta, dtp))
    dt = ts / m
    mdf, mo, rel, dom = sign_check(1.0, dt, ops)
    print(f"N {N}: d_v1 {D * dta / dx**2:.4g} steps_v1 {ts / dta:.6g}  A1 steps {m} dt {dt:.6g} d_A1 {D * dt / dx**2:.4f} min_expl_diag {mdf:.4f} dominant {dom}")
    if N == 256:
        print(f"   T1 steps at dt_A1, dt_A1/2, dt_A1/4: {m}, {2 * m}, {4 * m}")

print("\n== C2 configurations (k_red = 1e-10): 3 v x 3 D x 4 K_d =", len(VS) * len(DS) * len(KDS))
print("S_ref = theta C0 / rho_b =", THETA * C0 / RHO_B, "mol/kg")
print("rho_b =", RHO_B)
