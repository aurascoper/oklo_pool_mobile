//! Write the six revision 0 fields at the tested case (phase 0.15, active, 32^3)
//! as raw little-endian f32, X fastest, no header, for viewer/index.html.
//!
//! Usage: cargo run --example dump_fields -- <out_dir> <pulse_boundary>
//! The pulse boundary is required: revision 0 has 0.1667 (GUI) and 0.17 (exporter).

use oklo_core::{compute_field, Field, SimConfig};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let [_, out_dir, b] = args.as_slice() else {
        eprintln!("usage: dump_fields <out_dir> <pulse_boundary>");
        std::process::exit(2);
    };
    let b: f64 = b.parse().expect("pulse_boundary must be a number");
    let cfg = SimConfig {
        nx: 32,
        ny: 32,
        nz: 32,
        phase: 0.15,
        null_scenario: false,
        pulse_boundary: b,
        cycle_period_s: 10800.0,
    };
    cfg.validate().expect("invalid configuration");
    std::fs::create_dir_all(out_dir).expect("create out_dir");
    for field in Field::ALL {
        let bytes: Vec<u8> = compute_field(&cfg, field)
            .iter()
            .flat_map(|x| x.to_le_bytes())
            .collect();
        let path = format!("{out_dir}/{}_b{b}.f32", field.name());
        std::fs::write(&path, bytes).expect("write field");
        println!("{path}");
    }
}
