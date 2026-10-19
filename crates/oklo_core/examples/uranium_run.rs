//! Run uranium pre-registration v1 with Amendment A1: the 144-run sweep, B1,
//! T1, B2, the ledger, N1, C1 and C2; write one output file per run (v1
//! section 7) and a JSON receipt (v1 section 7, A1.8). A synthetic column, not
//! Oklo or the Franceville Basin; the results verify the declared equations,
//! they do not validate them against nature. Failures are recorded as failures.
//!
//! Usage: cargo run --release --example uranium_run -- <out_dir> <receipt.json> <meta.json>
//! meta.json: an object with provenance the program cannot compute itself
//! (spec hashes, git HEAD, rustc version, UTC time); it is embedded verbatim.

use oklo_core::uranium::*;
use std::fmt::Write as _;

fn num(x: f64) -> String {
    if x.is_finite() {
        format!("{x:e}")
    } else {
        "null".into()
    }
}

fn s(x: &str) -> String {
    format!("\"{}\"", x.replace('\\', "\\\\").replace('"', "\\\""))
}

fn status(pass: bool) -> &'static str {
    if pass {
        "pass"
    } else {
        "fail"
    }
}

fn arr(xs: &[f64]) -> String {
    format!("[{}]", xs.iter().map(|x| num(*x)).collect::<Vec<_>>().join(","))
}

fn bounds_json(b: &Bounds) -> String {
    let first = match b.first {
        Some(v) => format!("{{\"step\":{},\"cell\":{},\"what\":{},\"value\":{}}}", v.step, v.cell, s(v.what), num(v.value)),
        None => "null".into(),
    };
    format!(
        "{{\"status\":{},\"steps_checked\":{},\"violations\":{},\"c_min\":{},\"c_max\":{},\"s_min\":{},\"first_violation\":{first}}}",
        s(status(b.n1_pass())),
        b.steps,
        b.violations,
        num(b.c_min),
        num(b.c_max),
        num(b.s_min)
    )
}

fn write_run_file(path: &str, cfg: &RunConfig, col: &Column, r: &RunResult) {
    let mut o = String::new();
    write!(o, "{{\"label\":{},\"caveat\":{},\"n\":{},\"dx_m\":{},\"cell_centre\":\"(i + 0.5) dx\",", s(&cfg.label()), s(CAVEAT), col.n, num(col.dx())).unwrap();
    o.push_str("\"units\":{\"t\":\"s\",\"C\":\"mol per m3 of pore water\",\"S\":\"mol per kg of solid\",\"ledger\":\"mol per m2 of bulk cross-section\"},");
    let snaps: Vec<String> = r
        .snapshots
        .iter()
        .zip(OUTPUT_TAUS)
        .map(|(sn, tau)| format!("{{\"tau\":{},\"t\":{},\"C\":{},\"S\":{}}}", num(tau), num(sn.t), arr(&sn.c), arr(&sn.s)))
        .collect();
    write!(o, "\"snapshots\":[{}],", snaps.join(",")).unwrap();
    let bt: Vec<String> = r.breakthrough.iter().map(|(t, c)| format!("[{},{}]", num(*t), num(*c))).collect();
    write!(o, "\"breakthrough_outlet_C\":[{}],", bt.join(",")).unwrap();
    let led: Vec<String> = r.ledger.iter().map(|p| format!("[{},{},{},{},{}]", num(p.t), num(p.aqueous_sorbed), num(p.immobilized), num(p.inflow), num(p.outflow))).collect();
    write!(o, "\"ledger_columns\":[\"t\",\"aqueous_sorbed\",\"immobilized\",\"inflow\",\"outflow\"],\"ledger\":[{}]}}", led.join(",")).unwrap();
    std::fs::write(path, o).expect("write run file");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let [_, out_dir, receipt_path, meta_path] = args.as_slice() else {
        eprintln!("usage: uranium_run <out_dir> <receipt.json> <meta.json>");
        std::process::exit(2);
    };
    let meta = std::fs::read_to_string(meta_path).expect("read meta.json");
    std::fs::create_dir_all(out_dir).expect("create out_dir");
    let mut checks: Vec<String> = Vec::new();

    // A1.4: the nine grid sizes.
    let want_n = [81, 64, 64, 801, 80, 64, 8001, 800, 80];
    let got_n: Vec<usize> = VS.iter().flat_map(|&v| DS.iter().map(move |&d| grid_n(v, d, L_COL))).collect();
    let n_pass = got_n == want_n;
    checks.push(format!("{{\"id\":\"A1.4\",\"status\":{},\"expected\":{want_n:?},\"got\":{got_n:?}}}", s(status(n_pass))));
    eprintln!("A1.4 N {got_n:?}: {}", status(n_pass));

    // B1, T1, B2.
    let b1r = b1();
    checks.push(format!(
        "{{\"id\":\"B1\",\"status\":{},\"grids\":{:?},\"steps\":{:?},\"dt_s\":{},\"l2_errors\":{},\"orders\":{},\"pass_band\":[{},{}],\"n1\":{},\"worst_ledger_closure_rel\":{}}}",
        s(status(b1r.pass())), B1_GRIDS, b1r.steps, arr(&b1r.dt), arr(&b1r.errors), arr(&b1r.orders), num(ORDER_RANGE.0), num(ORDER_RANGE.1), s(status(b1r.n1_pass)), num(b1r.ledger_worst)
    ));
    eprintln!("B1 orders {:?}: {}", b1r.orders, status(b1r.pass()));
    let t1r = t1();
    checks.push(format!(
        "{{\"id\":\"T1\",\"status\":{},\"n\":{T1_N},\"steps\":{:?},\"e0\":{},\"e1\":{},\"order\":{},\"pass_band\":[{},{}],\"n1\":{}}}",
        s(status(t1r.pass())), t1r.steps, num(t1r.e[0]), num(t1r.e[1]), num(t1r.order), num(ORDER_RANGE.0), num(ORDER_RANGE.1), s(status(t1r.n1_pass))
    ));
    eprintln!("T1 order {}: {}", t1r.order, status(t1r.pass()));
    let b2r = b2();
    let (r1, r2) = b2_roots();
    checks.push(format!(
        "{{\"id\":\"B2\",\"status\":{},\"grids\":{:?},\"l2_errors\":{},\"orders\":{},\"pass_band\":[{},{}],\"max_residual_over_b0\":{},\"roots_per_m\":[{},{}]}}",
        s(status(b2r.pass())), B2_GRIDS, arr(&b2r.errors), arr(&b2r.orders), num(ORDER_RANGE.0), num(ORDER_RANGE.1), num(b2r.residual), num(r1), num(r2)
    ));
    eprintln!("B2 orders {:?}: {}", b2r.orders, status(b2r.pass()));

    // The sweep.
    let cfgs = sweep();
    let runs = par_map(&cfgs, |c| run_production(c).expect("valid column"));
    let mut run_json = Vec::new();
    let (mut ledger_fail, mut n1_fail) = (Vec::new(), Vec::new());
    let mut worst_ledger = 0.0f64;
    for (cfg, r) in cfgs.iter().zip(&runs) {
        let col = cfg.column().unwrap();
        let file = format!("{}.json", cfg.label());
        write_run_file(&format!("{out_dir}/{file}"), cfg, &col, r);
        let (lw, lp) = r.ledger_check();
        worst_ledger = worst_ledger.max(lw);
        if !lp {
            ledger_fail.push(cfg.label());
        }
        if !r.bounds.n1_pass() {
            n1_fail.push(cfg.label());
        }
        let outs: Vec<String> = r
            .ledger_at_outputs()
            .iter()
            .zip(OUTPUT_TAUS)
            .zip(&r.snapshots)
            .map(|((p, tau), sn)| {
                format!(
                    "{{\"tau\":{},\"t_s\":{},\"outlet_C\":{},\"aqueous_sorbed\":{},\"immobilized\":{},\"inflow\":{},\"outflow\":{},\"closure_rel\":{}}}",
                    num(tau), num(p.t), num(sn.c[col.n - 1]), num(p.aqueous_sorbed), num(p.immobilized), num(p.inflow), num(p.outflow), num(p.closure_rel())
                )
            })
            .collect();
        run_json.push(format!(
            "{{\"label\":{},\"v\":{},\"D_eff\":{},\"k_red\":{},\"K_d\":{},\"R\":{},\"N\":{},\"dx\":{},\"Pe_s\":{},\"Pe_cell\":{},\"Da\":{},\"tau_s\":{},\"m\":{},\"dt_s\":{},\"steps\":{},\"dt_binding\":{},\"n1\":{},\"ledger\":{{\"status\":{},\"worst_closure_rel_at_outputs\":{}}},\"outputs\":[{}],\"file\":{}}}",
            s(&cfg.label()), num(cfg.v), num(cfg.d), num(cfg.k_red), num(cfg.kd), num(col.r), col.n, num(col.dx()), num(pe_s(cfg.v, cfg.d, L_COL)), num(col.pe_cell()),
            num(cfg.k_red * (X_B - X_A) / cfg.v), num(cfg.tau()), r.landing.m, num(r.landing.dt), r.steps, s(binding_term(&col)), bounds_json(&r.bounds),
            s(status(lp)), num(lw), outs.join(","), s(&file)
        ));
    }
    checks.push(format!("{{\"id\":\"ledger\",\"status\":{},\"runs\":144,\"tolerance_rel\":{},\"worst_closure_rel\":{},\"failed_runs\":{ledger_fail:?}}}", s(status(ledger_fail.is_empty())), num(LEDGER_REL), num(worst_ledger)));
    eprintln!("ledger worst {worst_ledger:e}: {}", status(ledger_fail.is_empty()));

    // C1: the 36 runs with k_red = 0.
    let mut c1_fail = Vec::new();
    for (cfg, r) in cfgs.iter().zip(&runs).filter(|(c, _)| c.k_red == 0.0) {
        let zero = r.snapshots.iter().all(|sn| sn.s.iter().all(|x| *x == 0.0)) && r.ledger.iter().all(|p| p.immobilized == 0.0) && r.bounds.s_min == 0.0;
        if !(zero && r.ledger_check().1) {
            c1_fail.push(cfg.label());
        }
    }
    checks.push(format!("{{\"id\":\"C1\",\"status\":{},\"runs\":36,\"failed_runs\":{c1_fail:?}}}", s(status(c1_fail.is_empty()))));
    eprintln!("C1: {}", status(c1_fail.is_empty()));

    // C2: 36 configurations; the uncorrupted arm is the production run itself.
    let c2_cfgs = c2_configs();
    let models: Vec<&RunResult> = c2_cfgs.iter().map(|c| &runs[cfgs.iter().position(|x| x == c).unwrap()]).collect();
    let idx: Vec<usize> = (0..c2_cfgs.len()).collect();
    let c2r = par_map(&idx, |&i| c2(&c2_cfgs[i], models[i]));
    let c2_pass = c2r.iter().all(|r| r.pass());
    let c2_rows: Vec<String> = c2r
        .iter()
        .map(|r| {
            format!(
                "{{\"label\":{},\"status\":{},\"model\":{{\"ledger_worst\":{},\"s_min\":{},\"triggers_neither\":{}}},\"a_flip_c_reaction\":{{\"ledger_worst\":{},\"detected\":{}}},\"b_flip_s_source\":{{\"s_min\":{},\"detected\":{}}}}}",
                s(&r.cfg.label()), s(status(r.pass())), num(r.model_ledger_worst), num(r.model_s_min), r.model_clean, num(r.a_ledger_worst), r.a_detected, num(r.b_s_min), r.b_detected
            )
        })
        .collect();
    checks.push(format!("{{\"id\":\"C2\",\"status\":{},\"configurations\":[{}]}}", s(status(c2_pass)), c2_rows.join(",")));
    eprintln!("C2: {}", status(c2_pass));

    // N1 over every uncorrupted run: the sweep (which includes C1 and C2's model arms), B1 and T1.
    let n1_pass = n1_fail.is_empty() && b1r.n1_pass && t1r.n1_pass;
    checks.push(format!("{{\"id\":\"N1\",\"status\":{},\"production_failed_runs\":{n1_fail:?},\"b1\":{},\"t1\":{}}}", s(status(n1_pass)), s(status(b1r.n1_pass)), s(status(t1r.n1_pass))));
    eprintln!("N1: {}", status(n1_pass));

    let mut o = String::new();
    o.push_str("{\"name\":\"uranium_prereg_v1_A1_run\",");
    write!(o, "\"caveat\":{},", s(CAVEAT)).unwrap();
    o.push_str("\"statement\":\"Verification of the declared synthetic equations of uranium pre-registration v1 with Amendment A1, to the tested tolerances. It does not validate the assumed rates, sorption values or organic zone, and it is not a reconstruction of Oklo or the Franceville Basin. Every check below was run; no continuous integration ran.\",");
    write!(o, "\"meta\":{},", meta.trim()).unwrap();
    o.push_str("\"output_sampling\":\"breakthrough and ledger at every 0.05 tau landing point from t = 0 (101 points); C and S profiles at the six output times\",");
    write!(o, "\"checks\":[{}],", checks.join(",")).unwrap();
    write!(o, "\"runs\":[{}]}}", run_json.join(",")).unwrap();
    std::fs::write(receipt_path, o).expect("write receipt");
}
