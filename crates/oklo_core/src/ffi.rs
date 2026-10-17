//! C ABI over `Sim`. Header: include/oklo_core.h. Contract: docs/BRIDGING.md.
//! Keep the header in sync by hand when anything here changes, and bump
//! `ABI_VERSION` on any change to a signature, struct layout or field id.

use crate::{Field, Sim, SimConfig};
use std::ptr;

pub const ABI_VERSION: u32 = 1;

pub const OKLO_OK: i32 = 0;
pub const OKLO_ERR_NULL: i32 = -1;
pub const OKLO_ERR_INVALID: i32 = -2;

/// C mirror of `SimConfig`. Four u32 then three f64, so there is no padding.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct OkloSimConfig {
    pub nx: u32,
    pub ny: u32,
    pub nz: u32,
    /// 0 = active scenario, 1 = null scenario. Any other value is invalid.
    pub null_scenario: u32,
    pub phase: f64,
    pub pulse_boundary: f64,
    pub cycle_period_s: f64,
}

#[no_mangle]
pub extern "C" fn oklo_core_abi_version() -> u32 {
    ABI_VERSION
}

/// Returns NULL if `config` is NULL or invalid.
///
/// # Safety
/// `config` must be NULL or point to a readable `OkloSimConfig`.
#[no_mangle]
pub unsafe extern "C" fn oklo_sim_create(config: *const OkloSimConfig) -> *mut Sim {
    let Some(c) = config.as_ref() else {
        return ptr::null_mut();
    };
    if c.null_scenario > 1 {
        return ptr::null_mut();
    }
    let cfg = SimConfig {
        nx: c.nx,
        ny: c.ny,
        nz: c.nz,
        phase: c.phase,
        null_scenario: c.null_scenario == 1,
        pulse_boundary: c.pulse_boundary,
        cycle_period_s: c.cycle_period_s,
    };
    match Sim::new(cfg) {
        Ok(sim) => Box::into_raw(Box::new(sim)),
        Err(_) => ptr::null_mut(),
    }
}

/// Advance by `dt_s` seconds. Invalidates every pointer from `oklo_sim_field`.
///
/// # Safety
/// `sim` must be NULL or a live pointer from `oklo_sim_create`, not used
/// concurrently from another thread.
#[no_mangle]
pub unsafe extern "C" fn oklo_sim_step(sim: *mut Sim, dt_s: f64) -> i32 {
    let Some(sim) = sim.as_mut() else {
        return OKLO_ERR_NULL;
    };
    if !dt_s.is_finite() {
        return OKLO_ERR_INVALID;
    }
    match sim.step(dt_s) {
        Ok(()) => OKLO_OK,
        Err(_) => OKLO_ERR_INVALID,
    }
}

/// Borrow one field as `nx*ny*nz` f32 values, X fastest. Returns NULL for a
/// NULL `sim` or an unknown `field_id`. The pointer is valid until the next
/// `oklo_sim_step` or `oklo_sim_destroy` on the same `sim`.
///
/// # Safety
/// `sim` must be NULL or a live pointer from `oklo_sim_create`. `out_len`
/// must be NULL or writable.
#[no_mangle]
pub unsafe extern "C" fn oklo_sim_field(
    sim: *const Sim,
    field_id: u32,
    out_len: *mut usize,
) -> *const f32 {
    let (Some(sim), Some(field)) = (sim.as_ref(), Field::from_id(field_id)) else {
        return ptr::null();
    };
    let data = sim.field(field);
    if let Some(len) = out_len.as_mut() {
        *len = data.len();
    }
    data.as_ptr()
}

/// Current phase, or NaN for a NULL `sim`.
///
/// # Safety
/// `sim` must be NULL or a live pointer from `oklo_sim_create`.
#[no_mangle]
pub unsafe extern "C" fn oklo_sim_phase(sim: *const Sim) -> f64 {
    sim.as_ref().map_or(f64::NAN, |s| s.config().phase)
}

/// Free `sim`. NULL is accepted and ignored.
///
/// # Safety
/// `sim` must be NULL or a live pointer from `oklo_sim_create`, destroyed once.
#[no_mangle]
pub unsafe extern "C" fn oklo_sim_destroy(sim: *mut Sim) {
    if !sim.is_null() {
        drop(Box::from_raw(sim));
    }
}
