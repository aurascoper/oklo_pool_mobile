/*
 * oklo_core C ABI, version 1.
 *
 * Hand-written. Keep in sync with crates/oklo_core/src/ffi.rs, and bump
 * OKLO_CORE_ABI_VERSION there and here on any change to a signature, struct
 * layout or field id. Ownership, lifetime and threading rules: docs/BRIDGING.md.
 *
 * The fields are revision 0 of the Oklo GUI conformance fixture: prescribed
 * analytic functions of grid position and cycle phase, not a physical
 * simulation. See docs/PROVENANCE.md.
 */
#ifndef OKLO_CORE_H
#define OKLO_CORE_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define OKLO_CORE_ABI_VERSION 1u

#define OKLO_OK 0
#define OKLO_ERR_NULL (-1)
#define OKLO_ERR_INVALID (-2)

/* Field ids, also the order of the six revision 0 MRC files. */
#define OKLO_FIELD_TEMPERATURE 0u
#define OKLO_FIELD_HYDRATION 1u
#define OKLO_FIELD_H2_CONCENTRATION 2u
#define OKLO_FIELD_U235_RATIO 3u
#define OKLO_FIELD_POLYMER_YIELD 4u
#define OKLO_FIELD_COMPARTMENTS 5u

typedef struct OkloSim OkloSim;

typedef struct OkloSimConfig {
    uint32_t nx;
    uint32_t ny;
    uint32_t nz;
    uint32_t null_scenario;  /* 0 = active, 1 = null; anything else is invalid */
    double phase;            /* cycle phase fraction in [0, 1] */
    double pulse_boundary;   /* required, in (0, 1); revision 0 has 0.1667 (JS) and 0.17 (Python) */
    double cycle_period_s;   /* seconds per cycle, > 0; used only by oklo_sim_step */
} OkloSimConfig;

uint32_t oklo_core_abi_version(void);

/* NULL if config is NULL or invalid. */
OkloSim *oklo_sim_create(const OkloSimConfig *config);

/* OKLO_OK, OKLO_ERR_NULL or OKLO_ERR_INVALID. Invalidates field pointers. */
int32_t oklo_sim_step(OkloSim *sim, double dt_s);

/* nx*ny*nz floats, X fastest; valid until the next step or destroy. NULL on bad input. */
const float *oklo_sim_field(const OkloSim *sim, uint32_t field_id, size_t *out_len);

/* NaN if sim is NULL. */
double oklo_sim_phase(const OkloSim *sim);

/* NULL is ignored. */
void oklo_sim_destroy(OkloSim *sim);

#ifdef __cplusplus
}
#endif

#endif /* OKLO_CORE_H */
