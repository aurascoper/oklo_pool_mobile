//! MRC2014 writer for one field, following the CF-3 table in
//! CONFORMANCE_FIXTURE_SPEC.md: mode 2 (float32), little-endian, X fastest,
//! cell lengths n * 0.5 (1 header unit = 1 m by declared convention, R0-4),
//! NVERSION 20140, and header RMS holding the population sd (R0-3).

use crate::{compute_field, Field, SimConfig};

pub const HEADER_BYTES: usize = 1024;
/// Voxel step in metres, as in the revision 0 exporter.
pub const VOXEL_STEP_M: f32 = 0.5;

/// Population statistics of the stored float32 values: min, max, mean, sd.
pub fn stats(data: &[f32]) -> [f64; 4] {
    let n = data.len() as f64;
    let min = data.iter().copied().fold(f32::INFINITY, f32::min) as f64;
    let max = data.iter().copied().fold(f32::NEG_INFINITY, f32::max) as f64;
    let mean = data.iter().map(|&x| x as f64).sum::<f64>() / n;
    let var = data.iter().map(|&x| (x as f64 - mean).powi(2)).sum::<f64>() / n;
    [min, max, mean, var.sqrt()]
}

/// The CF-3 label, `Oklo <field> phase=<p> null=<bool>`, with p to 4 decimals.
pub fn label(cfg: &SimConfig, field: Field) -> String {
    let null = if cfg.null_scenario { "True" } else { "False" };
    format!("Oklo {} phase={:.4} null={null}", field.name(), cfg.phase)
}

/// The full file: 1024-byte header followed by the data block.
pub fn encode(cfg: &SimConfig, field: Field) -> Vec<u8> {
    let data = compute_field(cfg, field);
    let [min, max, mean, sd] = stats(&data);
    let mut h = vec![0u8; HEADER_BYTES];
    let mut put_i32 = |off: usize, v: i32| h[off..off + 4].copy_from_slice(&v.to_le_bytes());
    let (nx, ny, nz) = (cfg.nx as i32, cfg.ny as i32, cfg.nz as i32);
    put_i32(0, nx);
    put_i32(4, ny);
    put_i32(8, nz);
    put_i32(12, 2); // MODE float32
    put_i32(28, nx);
    put_i32(32, ny);
    put_i32(36, nz);
    put_i32(64, 1); // MAPC
    put_i32(68, 2); // MAPR
    put_i32(72, 3); // MAPS
    put_i32(88, 1); // ISPG
    put_i32(108, 20140); // NVERSION
    put_i32(220, 1); // NLABL
    let mut put_f32 = |off: usize, v: f32| h[off..off + 4].copy_from_slice(&v.to_le_bytes());
    put_f32(40, cfg.nx as f32 * VOXEL_STEP_M);
    put_f32(44, cfg.ny as f32 * VOXEL_STEP_M);
    put_f32(48, cfg.nz as f32 * VOXEL_STEP_M);
    put_f32(52, 90.0);
    put_f32(56, 90.0);
    put_f32(60, 90.0);
    put_f32(76, min as f32);
    put_f32(80, max as f32);
    put_f32(84, mean as f32);
    put_f32(216, sd as f32); // RMS word: population sd
    h[208..212].copy_from_slice(b"MAP ");
    h[212..216].copy_from_slice(&[0x44, 0x44, 0x00, 0x00]);
    let text = label(cfg, field);
    let mut lab = [b' '; 80];
    let n = text.len().min(80);
    lab[..n].copy_from_slice(&text.as_bytes()[..n]);
    h[224..304].copy_from_slice(&lab);

    h.reserve(data.len() * 4);
    for v in data {
        h.extend_from_slice(&v.to_le_bytes());
    }
    h
}
