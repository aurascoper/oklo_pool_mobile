//! MRC2014 writer, following the CF-3 table in CONFORMANCE_FIXTURE_SPEC.md:
//! mode 2 (float32), little-endian, X fastest, NVERSION 20140, and header RMS
//! holding the population sd (R0-3). Cell lengths are n * step, with 1 header
//! unit = 1 m by declared convention (R0-4).

use crate::{compute_field, Field, SimConfig};

pub const HEADER_BYTES: usize = 1024;
/// Voxel step in metres, as in the revision 0 exporter.
pub const VOXEL_STEP_M: f32 = 0.5;
/// MRC2014 allows at most ten 80-byte labels.
pub const MAX_LABELS: usize = 10;

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

/// A complete MRC file for any float32 array of `nx * ny * nz` values,
/// X fastest, with 1 to 10 labels of at most 80 ASCII bytes each.
pub fn encode_array(dims: [u32; 3], step_m: f32, data: &[f32], labels: &[&str]) -> Vec<u8> {
    let [nx, ny, nz] = dims;
    assert_eq!(data.len(), nx as usize * ny as usize * nz as usize, "data length must equal nx*ny*nz");
    assert!(!labels.is_empty() && labels.len() <= MAX_LABELS, "MRC needs 1 to 10 labels");
    let [min, max, mean, sd] = stats(data);
    let mut h = vec![0u8; HEADER_BYTES];
    let mut put_i32 = |off: usize, v: i32| h[off..off + 4].copy_from_slice(&v.to_le_bytes());
    put_i32(0, nx as i32);
    put_i32(4, ny as i32);
    put_i32(8, nz as i32);
    put_i32(12, 2); // MODE float32
    put_i32(28, nx as i32);
    put_i32(32, ny as i32);
    put_i32(36, nz as i32);
    put_i32(64, 1); // MAPC
    put_i32(68, 2); // MAPR
    put_i32(72, 3); // MAPS
    put_i32(88, 1); // ISPG
    put_i32(108, 20140); // NVERSION
    put_i32(220, labels.len() as i32); // NLABL
    let mut put_f32 = |off: usize, v: f32| h[off..off + 4].copy_from_slice(&v.to_le_bytes());
    put_f32(40, nx as f32 * step_m);
    put_f32(44, ny as f32 * step_m);
    put_f32(48, nz as f32 * step_m);
    put_f32(52, 90.0);
    put_f32(56, 90.0);
    put_f32(60, 90.0);
    put_f32(76, min as f32);
    put_f32(80, max as f32);
    put_f32(84, mean as f32);
    put_f32(216, sd as f32); // RMS word: population sd
    h[208..212].copy_from_slice(b"MAP ");
    h[212..216].copy_from_slice(&[0x44, 0x44, 0x00, 0x00]);
    for (i, text) in labels.iter().enumerate() {
        assert!(text.is_ascii() && text.len() <= 80, "label {i} must be ASCII and at most 80 bytes");
        let start = 224 + 80 * i;
        let slot = &mut h[start..start + 80];
        slot.fill(b' ');
        slot[..text.len()].copy_from_slice(text.as_bytes());
    }

    h.reserve(data.len() * 4);
    for v in data {
        h.extend_from_slice(&v.to_le_bytes());
    }
    h
}

/// The full file for one revision 0 fixture field: header and data block.
pub fn encode(cfg: &SimConfig, field: Field) -> Vec<u8> {
    let data = compute_field(cfg, field);
    encode_array([cfg.nx, cfg.ny, cfg.nz], VOXEL_STEP_M, &data, &[&label(cfg, field)])
}
