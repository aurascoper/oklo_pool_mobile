//! Run Darcy pre-registration v1 with A1 to A4: the eleven configurations of
//! v1 section 3.3 at water states A and B, the refinement study, the zero-flow
//! control and checks D1 to D5; write the MRC outputs of v1 section 7 and a
//! JSON receipt. A hypothetical numerical experiment, not Oklo hydrogeology.
//! Failures are recorded as failures.
//!
//! Usage: cargo run --release --example darcy_run -- <out_dir> <receipt.json> <meta.json> <v1|a4>
//! meta.json: an object with provenance the program cannot compute itself
//! (spec hashes, git HEAD, rustc version, UTC time); it is embedded verbatim.
//! The last argument names the CG rule: v1 (1e-12) or a4 (1e-13, Amendment A4).

use oklo_core::darcy::*;
use oklo_core::mrc;
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

fn write_mrc(path: &str, data: &[f64], labels: &[&str]) {
    let v: Vec<f32> = data.iter().map(|x| *x as f32).collect();
    std::fs::write(path, mrc::encode_array([N as u32; 3], (L / N as f64) as f32, &v, labels)).expect("write mrc");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let [_, out_dir, receipt_path, meta_path, rule_arg] = args.as_slice() else {
        eprintln!("usage: darcy_run <out_dir> <receipt.json> <meta.json> <v1|a4>");
        std::process::exit(2);
    };
    let rule = SolverRule::parse(rule_arg).unwrap_or_else(|| panic!("rule must be v1 or a4, got {rule_arg:?}"));
    eprintln!("solver rule {} (CG true-residual tolerance {:e})", rule.name(), rule.tol());
    let meta = std::fs::read_to_string(meta_path).expect("read meta.json");
    std::fs::create_dir_all(out_dir).expect("create out_dir");

    let mask = zone_mask_fixture();
    let lam = lambda_range();
    let lams = [("low", lam.low), ("representative", lam.rep), ("high", lam.high)];
    let fa = STATE_A.rho_g_over_mu();
    let mut checks: Vec<String> = Vec::new();
    let mut cfg_json: Vec<String> = Vec::new();
    let mut base_q32 = f64::NAN;
    let mut all_d1_pass = true;
    let mut all_converged = true;

    for cfg in configurations() {
        let p = Problem::from_config(N, &mask, &cfg, &STATE_A, DH);
        let sol = cg(&p, rule);
        let d = d1(&p, &sol);
        all_d1_pass &= d.pass();
        all_converged &= sol.converged;
        if cfg.name == "base" {
            base_q32 = d.q_in;
        }
        let fl = fluxes(&p, &sol.h);
        let [qx, qy, qz] = cell_flux(&p, &fl);
        let qmag_a: Vec<f64> = (0..qx.len()).map(|c| (qx[c] * qx[c] + qy[c] * qy[c] + qz[c] * qz[c]).sqrt()).collect();
        let shares = layer_shares(&p, &mask, &fl);
        let dir = format!("{out_dir}/{}", cfg.name);
        std::fs::create_dir_all(&dir).expect("create config dir");

        let mut states_json = Vec::new();
        for st in STATES {
            let f = st.rho_g_over_mu() / fa;
            let qmag: Vec<f64> = qmag_a.iter().map(|q| q * f).collect();
            let pore: Vec<f64> = qmag.iter().zip(&mask).map(|(q, z)| q / flow_porosity(*z).value).collect();
            let tag = |what: &str| format!("Darcy {what} cfg={} state={}", cfg.name, st.name);
            let phi_label = "phi_z: core 0.125 sourced; sandstone, halo, conduit unsourced";
            write_mrc(&format!("{dir}/{}_head_m.mrc", st.name), &sol.h, &[&tag("head m"), CAVEAT]);
            for (name, q) in [("qx", &qx), ("qy", &qy), ("qz", &qz)] {
                let scaled: Vec<f64> = q.iter().map(|v| v * f).collect();
                write_mrc(&format!("{dir}/{}_{name}_m_s.mrc", st.name), &scaled, &[&tag(&format!("{name} m/s")), CAVEAT]);
            }
            write_mrc(&format!("{dir}/{}_pore_velocity_m_s.mrc", st.name), &pore, &[&tag("pore velocity m/s"), CAVEAT, phi_label]);

            let mut pe_json = Vec::new();
            for (lname, lv) in lams {
                let (max_all, max_conduit, median) = pe_stats(&qmag, &mask, &st, lv);
                pe_json.push(format!(
                    "{{\"lambda_case\": {}, \"lambda_m_w_mk\": {}, \"max_all_cells\": {}, \"max_conduit_cells\": {}, \"median_all_cells\": {}}}",
                    s(lname), num(lv), num(max_all), num(max_conduit), num(median)
                ));
            }
            let mut pv_json = Vec::new();
            for z in ZONES {
                let fp = flow_porosity(z);
                let vmax = pore.iter().zip(&mask).filter(|(_, m)| **m == z).map(|(v, _)| *v).fold(0.0f64, f64::max);
                pv_json.push(format!(
                    "{{\"zone\": {}, \"phi_z\": {}, \"phi_z_sourced\": {}, \"phi_z_basis\": {}, \"max_pore_velocity_m_s\": {}}}",
                    s(&format!("{z:?}")), num(fp.value), fp.sourced, s(fp.basis), num(vmax)
                ));
            }
            states_json.push(format!(
                "{{\"state\": {}, \"flux_factor_vs_A\": {}, \"q_in_m3_s\": {}, \"q_out_m3_s\": {}, \"pe_limitation\": {}, \"peclet\": [{}], \"pore_velocity\": [{}]}}",
                s(st.name), num(f), num(d.q_in * f), num(d.q_out * f), s(st.pe_limitation()), pe_json.join(", "), pv_json.join(", ")
            ));
        }
        let layer_json: Vec<String> = shares.iter().map(|(sh, ar)| format!("[{}, {}]", num(*sh), num(*ar))).collect();
        cfg_json.push(format!(
            "{{\"name\": {}, \"k_m2\": {{\"sandstone\": {}, \"core\": {}, \"halo\": {}, \"conduit\": {}}}, \
             \"cg\": {{\"iterations\": {}, \"restarts\": {}, \"true_relative_residual\": {}, \"converged\": {}}}, \
             \"d1\": {{\"status\": {}, \"eps\": {}, \"global\": {}, \"paths\": {}, \"r_inf_m3_s\": {}, \"r_2_m3_s\": {}, \"b_norm_over_q_ref\": {}}}, \
             \"conduit_share_top_face\": {}, \"conduit_share_and_area_by_layer\": [{}], \"states\": [{}]}}",
            s(&cfg.name), num(cfg.k[0]), num(cfg.k[1]), num(cfg.k[2]), num(cfg.k[3]),
            sol.iterations, sol.restarts, num(sol.rel_residual), sol.converged,
            s(status(d.pass())), num(d.eps), num(d.global), num(d.paths), num(d.r_inf), num(d.r_2), num(d.b_norm_over_q_ref),
            num(shares[N - 1].0), layer_json.join(", "), states_json.join(", ")
        ));
        eprintln!("{}: CG {} iterations, converged {}, D1 {}", cfg.name, sol.iterations, sol.converged, status(d.pass()));
    }

    // Checks D1 to D5, computed with the same code and thresholds as the tests.
    let base = configurations().into_iter().next().unwrap();
    let (zero_ok, _) = d1_zero_flow(&mask, &base, rule);
    checks.push(format!("{{\"id\": \"D1\", \"run\": true, \"status\": {}, \"detail\": \"all eleven configurations: eps <= 1e-10, global <= 1e-10, residual paths <= 1e-12; see configurations[].d1\"}}", s(status(all_d1_pass))));
    checks.push(format!("{{\"id\": \"D1-zero-flow\", \"run\": true, \"status\": {}}}", s(status(zero_ok))));
    checks.push(format!("{{\"id\": \"CG-convergence\", \"run\": true, \"status\": {}, \"detail\": \"all eleven configurations within 10 N^3 iterations at the pre-registered rule\"}}", s(status(all_converged))));

    let levels: Vec<D2Level> = [16, 32, 64].into_iter().map(|n| d2_level(n, rule)).collect();
    let p1 = (levels[0].e2 / levels[1].e2).log2();
    let p2 = (levels[1].e2 / levels[2].e2).log2();
    let d2_pass = [p1, p2].iter().all(|p| (D2_ORDER_RANGE.0..=D2_ORDER_RANGE.1).contains(p));
    let lv: Vec<String> = levels.iter().map(|l| format!("{{\"n\": {}, \"e2\": {}, \"einf\": {}, \"cg_iterations\": {}}}", l.n, num(l.e2), num(l.einf), l.solve_iterations)).collect();
    checks.push(format!("{{\"id\": \"D2\", \"run\": true, \"status\": {}, \"l2_orders\": [{}, {}], \"levels\": [{}]}}", s(status(d2_pass)), num(p1), num(p2), lv.join(", ")));

    for (id, series) in [("D3a", true), ("D3b", false)] {
        let d = d3(series, rule);
        checks.push(format!(
            "{{\"id\": {}, \"run\": true, \"status\": {}, \"q_rel_direct\": {}, \"head_err_direct_over_dh\": {}, \"q_rel_cg\": {}, \"cg_converged\": {}, \"tolerances\": {{\"direct\": 1e-12, \"cg\": 1e-10}}}}",
            s(id), s(status(d.pass())), num(d.q_rel_direct), num(d.head_err_direct), num(d.q_rel_cg), d.cg_converged
        ));
    }

    let pb = Problem::from_config(N, &mask, &base, &STATE_A, DH);
    let sb = cg(&pb, rule);
    let d4r = d4(&pb, &sb);
    checks.push(format!(
        "{{\"id\": \"D4\", \"run\": true, \"status\": {}, \"head_x\": {}, \"head_y\": {}, \"flux_x\": {}, \"flux_y\": {}}}",
        s(status(d4r.pass())), num(d4r.head_x), num(d4r.head_y), num(d4r.flux_x), num(d4r.flux_y)
    ));
    let d5r = d5(&mask, rule);
    checks.push(format!("{{\"id\": \"D5\", \"run\": true, \"status\": {}, \"max_deviation\": {}}}", s(status(d5r.pass())), num(d5r.max_dev)));

    // Refinement study, reported without a threshold.
    let fine = refine_mask(&mask, N);
    let pf = Problem::from_config(2 * N, &fine, &base, &STATE_A, DH);
    let sf = cg(&pf, rule);
    let q64: f64 = fluxes(&pf, &sf.h).bottom.iter().sum();
    let change = (q64 - base_q32) / base_q32;
    checks.push(format!(
        "{{\"id\": \"refinement\", \"run\": true, \"status\": \"reported, no threshold\", \"q_in_n32\": {}, \"q_in_n64\": {}, \"relative_change\": {}, \"cg_iterations_n64\": {}, \"cg_converged_n64\": {}}}",
        num(base_q32), num(q64), num(change), sf.iterations, sf.converged
    ));
    checks.push("{\"id\": \"peclet\", \"run\": true, \"status\": \"reported, no threshold (screening diagnostic)\"}".into());
    eprintln!("refinement: Q64/Q32 - 1 = {change:e}; D2 orders {p1:.4} {p2:.4}");

    let mut j = String::new();
    write!(
        j,
        "{{\n\"receipt\": {},\n\
         \"solver_rule\": {{\"name\": {}, \"cg_true_residual_tolerance\": {}, \"note\": {}}},\n\
         \"caveat\": {},\n\
         \"scope\": \"Hypothetical numerical experiment on the revision 0 fixture geometry. It does not describe the hydrogeology of the Oklo reactors at any time. Nothing was fitted. Pore velocities use unsourced flow porosities except the core and are not physically validated.\",\n\
         \"ci\": \"none; no continuous-integration run is implied\",\n\
         \"meta\": {},\n\
         \"porosity_roles\": {{\"thermal_property_phi_th\": {{\"lo\": {}, \"hi\": {}, \"enters\": \"lambda_m, hence Pe\"}}, \"flow_phi_z\": \"per zone, enters only pore velocity\"}},\n\
         \"lambda_m_w_mk\": {{\"low\": {}, \"representative\": {}, \"high\": {}, \"source\": \"Robertson 1988 USGS OFR 88-441 eq. 6, fig. 10, 300 K, 5 MPa\", \"range_includes\": \"phi_th 0.10 to 0.25 and quartz 0 to 100 percent\"}},\n\
         \"pe_definition\": \"Pe = rho_w c_p,w |q| L / lambda_m, L = 0.5 m voxel scale (A1.1); screening only\",\n\
         \"checks\": [\n{}\n],\n\
         \"specified_but_not_run\": [],\n\
         \"configurations\": [\n{}\n]\n}}\n",
        s(match rule {
            SolverRule::V1 => "Darcy pre-registration v1 with Amendments A1 to A3: implementation run under v1's CG rule",
            SolverRule::A4 => "Darcy pre-registration v1, Amendment A4 verification rerun (A1 to A4); a new experiment, not a retroactive validation of v1",
        }),
        s(rule.name()),
        num(rule.tol()),
        s(match rule {
            SolverRule::V1 => "v1 rule, 1e-12 relative to ||b||_2",
            SolverRule::A4 => "A4 rule, 1e-13 relative to ||b||_2; every acceptance threshold, case and parameter unchanged from v1",
        }),
        s(&format!("{CAVEAT}. Not a description of Oklo.")),
        meta.trim(),
        num(PHI_TH.lo), num(PHI_TH.hi),
        num(lam.low), num(lam.rep), num(lam.high),
        checks.join(",\n"),
        cfg_json.join(",\n")
    )
    .unwrap();
    std::fs::write(receipt_path, j).expect("write receipt");
    println!("wrote {receipt_path} and MRC files under {out_dir}");
}
