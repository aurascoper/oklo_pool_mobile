# Bridging contract, ABI version 1

The C header crates/oklo_core/include/oklo_core.h is the only interface between the Rust core and the platform shells. It is hand-written. Any change to a function signature, the OkloSimConfig layout or a field id must change crates/oklo_core/src/ffi.rs, the header and OKLO_CORE_ABI_VERSION together, and a shell should refuse to run against a core whose oklo_core_abi_version() differs from the version it was built for.

## C ABI

| Function | Contract |
| --- | --- |
| oklo_core_abi_version() | Returns 1. |
| oklo_sim_create(config) | Returns an owned OkloSim pointer, or NULL if config is NULL or invalid. All six fields are computed before it returns. |
| oklo_sim_step(sim, dt_s) | Returns OKLO_OK (0), OKLO_ERR_NULL (-1) or OKLO_ERR_INVALID (-2) for a non-finite dt_s or a resulting phase outside [0, 1]. On success every field is recomputed in place. Currently aborts the process, because advance_phase is unwritten. |
| oklo_sim_field(sim, field_id, out_len) | Borrows nx*ny*nz floats, or returns NULL for a NULL sim or unknown id. out_len may be NULL. |
| oklo_sim_phase(sim) | The current phase, or NaN for NULL. |
| oklo_sim_destroy(sim) | Frees sim. NULL is ignored. Destroying twice is undefined. |

Configuration is validated in the core, not the shell. A config is invalid when any grid dimension is 0, the grid exceeds 128^3 voxels, the phase is outside [0, 1], the pulse boundary is outside the open interval (0, 1) or not a number, the cycle period is not a positive finite number, or null_scenario is anything other than 0 or 1. OkloSimConfig is four uint32 values followed by three doubles, so it has no padding on any target.

Ownership is simple: the caller owns the OkloSim pointer from create until destroy. A field pointer is borrowed from the sim and stays valid until the next step or destroy on that sim, so a shell that keeps data across frames must copy it, as the Swift wrapper does. The core does no locking, so a sim must be used from one thread at a time. Separate sims share nothing and may run on separate threads.

Every field is a float32 array with X fastest, index = k*nx*ny + j*nx + i, which matches the MRC files and the order a 3D texture upload expects. Field ids are 0 temperature, 1 hydration, 2 h2_concentration, 3 u235_ratio, 4 polymer_yield and 5 compartments. Compartments holds the zone numbers 0 to 4 as floats.

The core computes in float64 with the platform's libm and stores float32. On this Mac the result is byte-identical to the Python exporter, which also calls Apple's libm. Android's bionic libm is not guaranteed to round exp and sin identically, so Android results are expected to agree within the CF-2 tolerance, 1e-6 times max(1, |reference|), and may differ in the last bit. If bit identity across platforms is ever required, it needs a pure-Rust math library, which is a dependency decision for later.

A Rust panic cannot cross the C boundary. Since Rust 1.81 a panic in an extern "C" function aborts the process rather than unwinding into Swift or Kotlin, which is why calling step before advance_phase exists aborts rather than corrupting state.

## iOS and macOS

scripts/build_xcframework.sh builds liboklo_core.a for aarch64-apple-ios, aarch64-apple-ios-sim and aarch64-apple-darwin with deployment targets iOS 17 and macOS 14, and bundles them with the header and module.modulemap into ios/OkloCoreKit/OkloCoreFFI.xcframework. The Swift package exposes one class, OkloSim, which owns the pointer, destroys it in deinit, returns nil for a rejected config, and copies fields into [Float]. The app target, signing identity, bundle identifier and Metal viewer belong to M1.

## Android, contract only

Nothing for Android is built in M0. The SDK and NDK are not installed, and installing them needs approval. cargo-ndk is already present. The intended shape is a second crate that wraps the same core as a cdylib for arm64-v8a, exposing JNI functions for one Kotlin object. The package name is left for Hunter to choose and appears below as a placeholder.

```kotlin
package <package>

object OkloCoreNative {
    init { System.loadLibrary("oklo_core_jni") }

    external fun abiVersion(): Int
    /** 0 if the core rejects the configuration. */
    external fun create(nx: Int, ny: Int, nz: Int, nullScenario: Boolean,
                        phase: Double, pulseBoundary: Double, cyclePeriodS: Double): Long
    /** OKLO_OK, OKLO_ERR_NULL or OKLO_ERR_INVALID. */
    external fun step(handle: Long, dtS: Double): Int
    /** Copies one field into out, which must hold nx*ny*nz floats; returns the count or a negative error. */
    external fun copyField(handle: Long, fieldId: Int, out: FloatArray): Int
    external fun phase(handle: Long): Double
    external fun destroy(handle: Long)
}
```

The JNI symbols follow the standard mangling, Java_<package with dots as underscores>_OkloCoreNative_create and so on. copyField copies, and never hands Kotlin a pointer into Rust memory, because the garbage collector cannot track it. Google Play requires new apps and app updates to target Android 16 (API level 36) since 31 August 2026, with extensions available to 1 November 2026, so targetSdk is 36. minSdk is a product decision for later.
