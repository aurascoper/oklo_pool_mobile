use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::sync::atomic::{AtomicI64, Ordering};

use jni::objects::{JClass, JFloatArray};
use jni::sys::{jdouble, jint, jlong};
use jni::JNIEnv;
use oklo_core::ffi::{self, OkloSimConfig, ABI_VERSION, OKLO_ERR_INVALID, OKLO_ERR_NULL};
use oklo_core::Field;

type SimPtr = usize;

// JNI handles are monotonically allocated integer keys, never native pointers.
// This makes invalid and repeated handles rejectable without dereferencing them.
static NEXT_HANDLE: AtomicI64 = AtomicI64::new(1);
static SIMS: OnceLock<Mutex<HashMap<jlong, SimPtr>>> = OnceLock::new();

fn sims() -> &'static Mutex<HashMap<jlong, SimPtr>> {
    SIMS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn invalid_handle() -> jlong {
    0
}

#[no_mangle]
pub extern "system" fn Java_org_oklo_pool_bridge_OkloCoreNative_abiVersion(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    ABI_VERSION as jint
}

#[no_mangle]
pub extern "system" fn Java_org_oklo_pool_bridge_OkloCoreNative_create(
    _env: JNIEnv,
    _class: JClass,
    nx: jint,
    ny: jint,
    nz: jint,
    null_scenario: jint,
    phase: jdouble,
    pulse_boundary: jdouble,
    cycle_period_s: jdouble,
) -> jlong {
    if nx <= 0 || ny <= 0 || nz <= 0 || !(0..=1).contains(&null_scenario) {
        return invalid_handle();
    }
    let config = OkloSimConfig {
        nx: nx as u32,
        ny: ny as u32,
        nz: nz as u32,
        null_scenario: null_scenario as u32,
        phase,
        pulse_boundary,
        cycle_period_s,
    };
    // SAFETY: config is a fully initialized local value valid for this call.
    let sim = unsafe { ffi::oklo_sim_create(&config) };
    if sim.is_null() {
        return invalid_handle();
    }

    let handle = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
    if handle <= 0 {
        // Exhaustion is practically unreachable; reject rather than reuse.
        // SAFETY: sim was just returned as an owned pointer by oklo_sim_create.
        unsafe { ffi::oklo_sim_destroy(sim) };
        return invalid_handle();
    }
    sims().lock().unwrap_or_else(|e| e.into_inner()).insert(handle, sim as usize);
    handle
}

#[no_mangle]
pub extern "system" fn Java_org_oklo_pool_bridge_OkloCoreNative_step(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
    dt_s: jdouble,
) -> jint {
    let guard = sims().lock().unwrap_or_else(|e| e.into_inner());
    let Some(sim) = guard.get(&handle).copied() else {
        return if handle == 0 { OKLO_ERR_NULL } else { OKLO_ERR_INVALID };
    };
    // SAFETY: the pointer is owned in SIMS and the mutex serializes access.
    unsafe { ffi::oklo_sim_step(sim as *mut oklo_core::Sim, dt_s) }
}

#[no_mangle]
pub extern "system" fn Java_org_oklo_pool_bridge_OkloCoreNative_copyField(
    env: JNIEnv,
    _class: JClass,
    handle: jlong,
    field_id: jint,
    out: JFloatArray,
) -> jint {
    let Some(field) = u32::try_from(field_id).ok().and_then(Field::from_id) else {
        return OKLO_ERR_INVALID;
    };
    let guard = sims().lock().unwrap_or_else(|e| e.into_inner());
    let Some(sim) = guard.get(&handle).copied() else {
        return if handle == 0 { OKLO_ERR_NULL } else { OKLO_ERR_INVALID };
    };
    let out_len = match env.get_array_length(&out) {
        Ok(n) => n as usize,
        Err(_) => return OKLO_ERR_INVALID,
    };
    let mut field_len = 0usize;
    // SAFETY: sim is live under the registry lock, and field is a valid enum.
    let values = unsafe { ffi::oklo_sim_field(sim as *const oklo_core::Sim, field as u32, &mut field_len) };
    if values.is_null() || out_len < field_len {
        return OKLO_ERR_INVALID;
    }
    // SAFETY: the C ABI guarantees field_len readable floats until step/destroy;
    // both operations are excluded while the registry lock is held.
    let slice = unsafe { std::slice::from_raw_parts(values, field_len) };
    match env.set_float_array_region(out, 0, slice) {
        Ok(()) => field_len as jint,
        Err(_) => OKLO_ERR_INVALID,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_oklo_pool_bridge_OkloCoreNative_phase(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) -> jdouble {
    let guard = sims().lock().unwrap_or_else(|e| e.into_inner());
    let Some(sim) = guard.get(&handle).copied() else {
        return f64::NAN;
    };
    // SAFETY: sim is live in SIMS and protected from concurrent destroy.
    unsafe { ffi::oklo_sim_phase(sim as *const oklo_core::Sim) }
}

#[no_mangle]
pub extern "system" fn Java_org_oklo_pool_bridge_OkloCoreNative_destroy(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    let mut guard = sims().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(sim) = guard.remove(&handle) {
        // SAFETY: remove transfers the unique live handle to this invocation.
        unsafe { ffi::oklo_sim_destroy(sim as *mut oklo_core::Sim) };
    }
}
