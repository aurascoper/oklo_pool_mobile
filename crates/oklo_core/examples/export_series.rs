//! Write the revision 0 fields over one cycle as MRC2014 volume series:
//! 60 frames, phase p = i/60, 3 minutes per frame under the exporter's 3.0 h
//! period. Frame 9 is p = 0.15, the case checked against the stored files.
//! These are samples of prescribed functions of phase, not simulated dynamics.
//!
//! Usage: cargo run --example export_series -- <out_dir> <pulse_boundary>
//! Writes <out_dir>/<field>_fNN.mrc and <out_dir>/series.json.

use oklo_core::{mrc, Field, SimConfig};
use std::fmt::Write as _;

const FRAMES: u32 = 60;
const PERIOD_MIN: f64 = 180.0;

/// Phase names and thresholds as export_oklo_mrc.py writes them (revision 0).
fn exporter_phase_name(p: f64) -> &'static str {
    if p < 0.17 {
        "Pulse/Critical Boiling"
    } else if p < 0.25 {
        "Desiccation/Dry Bake"
    } else if p < 0.83 {
        "Quiescent Cooldown"
    } else {
        "Infiltration/Re-wetting"
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let [_, out_dir, b] = args.as_slice() else {
        eprintln!("usage: export_series <out_dir> <pulse_boundary>");
        std::process::exit(2);
    };
    let b: f64 = b.parse().expect("pulse_boundary must be a number");
    std::fs::create_dir_all(out_dir).expect("create out_dir");

    let mut json = String::new();
    write!(
        json,
        "{{\"pulse_boundary\": {b}, \"frames\": {FRAMES}, \"period_minutes\": {PERIOD_MIN}, \
         \"note\": \"samples of revision 0 prescribed functions of phase, not dynamics\", \"frame_list\": ["
    )
    .unwrap();
    for i in 0..FRAMES {
        let phase = i as f64 / FRAMES as f64;
        let cfg = SimConfig {
            nx: 32,
            ny: 32,
            nz: 32,
            phase,
            null_scenario: false,
            pulse_boundary: b,
            cycle_period_s: PERIOD_MIN * 60.0,
        };
        cfg.validate().expect("invalid configuration");
        for field in Field::ALL {
            let path = format!("{out_dir}/{}_f{i:02}.mrc", field.name());
            std::fs::write(&path, mrc::encode(&cfg, field)).expect("write mrc");
        }
        let sep = if i == 0 { "" } else { ", " };
        write!(
            json,
            "{sep}{{\"frame\": {i}, \"phase\": {phase}, \"minutes\": {}, \"exporter_phase_name\": \"{}\"}}",
            phase * PERIOD_MIN,
            exporter_phase_name(phase)
        )
        .unwrap();
    }
    json.push_str("]}\n");
    std::fs::write(format!("{out_dir}/series.json"), json).expect("write series.json");
    println!("{out_dir}: {FRAMES} frames x {} fields", Field::ALL.len());
}
