//! WebAssembly exports for the viewer. The grid is fixed at 32^3, the size of
//! the revision 0 fixture. Values come from oklo_core unchanged; this crate
//! only moves them across the wasm boundary.

use oklo_core::{desiccation, fill_field, pulse, Field, SimConfig};

pub const VOXELS: usize = 32 * 32 * 32;

/// Allocate room for `n` f32 values in wasm memory. Free with `oklo_free`.
#[no_mangle]
pub extern "C" fn oklo_alloc(n: usize) -> *mut f32 {
    let mut v = Vec::<f32>::with_capacity(n);
    let p = v.as_mut_ptr();
    std::mem::forget(v);
    p
}

/// # Safety
/// `p` must come from `oklo_alloc(n)` with the same `n`, freed once.
#[no_mangle]
pub unsafe extern "C" fn oklo_free(p: *mut f32, n: usize) {
    drop(Vec::from_raw_parts(p, 0, n));
}

/// Fill `out` (VOXELS f32 values, X fastest) with one field. Returns 0, or -2
/// for an unknown field id or an invalid configuration.
///
/// # Safety
/// `out` must point to at least VOXELS writable f32 values.
#[no_mangle]
pub unsafe extern "C" fn oklo_compute(field_id: u32, phase: f64, null_scenario: u32, pulse_boundary: f64, out: *mut f32) -> i32 {
    let Some(field) = Field::from_id(field_id) else {
        return -2;
    };
    let cfg = SimConfig {
        nx: 32,
        ny: 32,
        nz: 32,
        phase,
        null_scenario: null_scenario == 1,
        pulse_boundary,
        cycle_period_s: 10800.0,
    };
    if null_scenario > 1 || cfg.validate().is_err() {
        return -2;
    }
    let mut buf = Vec::with_capacity(VOXELS);
    fill_field(&cfg, field, &mut buf);
    std::ptr::copy_nonoverlapping(buf.as_ptr(), out, VOXELS);
    0
}

/// The prescribed pulse term P(p) for the time-series strip.
#[no_mangle]
pub extern "C" fn oklo_pulse(phase: f64, pulse_boundary: f64) -> f64 {
    pulse(phase, pulse_boundary)
}

/// The prescribed desiccation term D(p) for the time-series strip.
#[no_mangle]
pub extern "C" fn oklo_desiccation(phase: f64) -> f64 {
    desiccation(phase)
}
