//! Integration test suite for CF-2 conformance across all 20 cases.
//!
//! Evaluates oklo_core Rust API and C ABI against independent reference generator.
//! Tests relative numerical tolerance (1e-6), exact compartments match,
//! cross-language bitwise identity, and seed invariance.

use conformance_cf2::matrix::CF2_CASES;
use conformance_cf2::reference::{
    generate_reference_field, RefField,
};
use conformance_cf2::sha256::sha256_float32_array;
use conformance_cf2::{compare_compartments, compare_continuous};
use oklo_core::ffi::{oklo_sim_create, oklo_sim_destroy, oklo_sim_field, OkloSimConfig};
use oklo_core::{fill_field, Field, SimConfig};
use std::process::Command;

fn to_core_field(field: RefField) -> Field {
    match field {
        RefField::Temperature => Field::Temperature,
        RefField::Hydration => Field::Hydration,
        RefField::H2Concentration => Field::H2Concentration,
        RefField::U235Ratio => Field::U235Ratio,
        RefField::PolymerYield => Field::PolymerYield,
        RefField::Compartments => Field::Compartments,
    }
}

#[test]
fn test_cross_language_bitwise_identity_with_python() {
    // Run scripts/cf2_reference.py --verify and scripts/cf2_reference.py --hashes
    let output = Command::new("python3")
        .args(["scripts/cf2_reference.py", "--hashes"])
        .current_dir("../../")
        .output()
        .or_else(|_| {
            Command::new("python3")
                .args(["scripts/cf2_reference.py", "--hashes"])
                .output()
        })
        .expect("Failed to execute python3 scripts/cf2_reference.py");

    assert!(output.status.success(), "Python reference generator failed");
    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut py_hashes = std::collections::HashMap::new();
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            py_hashes.insert(parts[1].to_string(), parts[0].to_string());
        }
    }

    assert_eq!(py_hashes.len(), 120, "Expected 120 hashes from Python reference");

    // Compute all 120 field volumes in Rust reference and assert bitwise hash equality
    for case in &CF2_CASES {
        let ref_cfg = case.config();
        let scen = case.scenario_str();
        let b_str = case.boundary_str();
        let p_str = format!("{:.2}", case.phase);
        let idx_str = format!("{:02}", case.case_index);

        for &field in &RefField::ALL {
            let ref_data = generate_reference_field(&ref_cfg, field);
            let rust_hash = sha256_float32_array(&ref_data);

            let key = format!("case{}_p{}_{}_b{}_{}.bin", idx_str, p_str, scen, b_str, field.name());
            let py_hash = py_hashes.get(&key).expect(&format!("Missing hash for key: {}", key));

            assert_eq!(
                &rust_hash, py_hash,
                "Bitwise mismatch between Rust and Python reference for {}: Rust={} Python={}",
                key, rust_hash, py_hash
            );
        }
    }

    println!("PASSED: All 120 field volumes exhibit bitwise identity between Rust and Python.");
}

#[test]
fn test_all_20_cases_rust_api_against_reference() {
    let mut total_fields_checked = 0;

    for case in &CF2_CASES {
        let ref_cfg = case.config();
        let core_cfg = SimConfig {
            nx: 32,
            ny: 32,
            nz: 32,
            phase: case.phase,
            null_scenario: case.null_scenario,
            pulse_boundary: case.pulse_boundary,
            cycle_period_s: 10800.0,
        };

        for &ref_field in &RefField::ALL {
            let ref_data = generate_reference_field(&ref_cfg, ref_field);
            let core_field = to_core_field(ref_field);
            let mut core_data = Vec::with_capacity(32 * 32 * 32);
            fill_field(&core_cfg, core_field, &mut core_data);

            assert_eq!(core_data.len(), ref_data.len());

            if ref_field == RefField::Compartments {
                let comp_diff = compare_compartments(&core_data, &ref_data, &ref_cfg);
                assert!(
                    comp_diff.passed,
                    "Case {} Compartments failed with {} fatal mismatches",
                    case.id, comp_diff.fatal_mismatches
                );
                if comp_diff.borderline_mismatches > 0 {
                    println!(
                        "Case {} Compartments: logged {} borderline voxels",
                        case.id, comp_diff.borderline_mismatches
                    );
                }
            } else {
                let diff = compare_continuous(&core_data, &ref_data);
                assert!(
                    diff.passed,
                    "Case {} Field {} failed tolerance: max_abs={:e}, max_rel={:e}, worst_idx={}",
                    case.id,
                    ref_field.name(),
                    diff.max_abs_diff,
                    diff.max_rel_diff,
                    diff.worst_index
                );
            }

            total_fields_checked += 1;
        }
    }

    assert_eq!(total_fields_checked, 120);
    println!("PASSED: All 20 cases (120 field volumes) verified against Rust API within tolerance.");
}

#[test]
fn test_all_20_cases_c_abi_against_reference() {
    let mut total_fields_checked = 0;

    for case in &CF2_CASES {
        let ref_cfg = case.config();
        let c_cfg = OkloSimConfig {
            nx: 32,
            ny: 32,
            nz: 32,
            null_scenario: if case.null_scenario { 1 } else { 0 },
            phase: case.phase,
            pulse_boundary: case.pulse_boundary,
            cycle_period_s: 10800.0,
        };

        let sim = unsafe { oklo_sim_create(&c_cfg) };
        assert!(!sim.is_null(), "Failed to create OkloSim via C ABI for case {}", case.id);

        for &ref_field in &RefField::ALL {
            let ref_data = generate_reference_field(&ref_cfg, ref_field);
            let field_id = ref_field.id();

            let mut len = 0usize;
            let ptr = unsafe { oklo_sim_field(sim, field_id, &mut len) };
            assert!(!ptr.is_null(), "C ABI returned null for field {}", ref_field.name());
            assert_eq!(len, 32 * 32 * 32);

            let c_slice = unsafe { std::slice::from_raw_parts(ptr, len) };

            if ref_field == RefField::Compartments {
                let comp_diff = compare_compartments(c_slice, &ref_data, &ref_cfg);
                assert!(
                    comp_diff.passed,
                    "C ABI Case {} Compartments failed with {} fatal mismatches",
                    case.id, comp_diff.fatal_mismatches
                );
            } else {
                let diff = compare_continuous(c_slice, &ref_data);
                assert!(
                    diff.passed,
                    "C ABI Case {} Field {} failed tolerance: max_abs={:e}, max_rel={:e}",
                    case.id,
                    ref_field.name(),
                    diff.max_abs_diff,
                    diff.max_rel_diff
                );
            }

            total_fields_checked += 1;
        }

        unsafe { oklo_sim_destroy(sim) };
    }

    assert_eq!(total_fields_checked, 120);
    println!("PASSED: All 20 cases (120 field volumes) verified against C ABI within tolerance.");
}

#[test]
fn test_seed_invariance_control() {
    // Verify that the generator produces identical outputs regardless of history or configuration
    let cfg = SimConfig {
        nx: 32,
        ny: 32,
        nz: 32,
        phase: 0.15,
        null_scenario: false,
        pulse_boundary: 0.17,
        cycle_period_s: 10800.0,
    };

    let mut data1 = Vec::new();
    fill_field(&cfg, Field::Temperature, &mut data1);

    let mut data2 = Vec::new();
    fill_field(&cfg, Field::Temperature, &mut data2);

    assert_eq!(data1, data2, "Deterministic generator must produce identical arrays");
    let hash1 = sha256_float32_array(&data1);
    let hash2 = sha256_float32_array(&data2);
    assert_eq!(hash1, hash2);
    println!("PASSED: Seed and invocation invariance verified.");
}

#[test]
fn test_r0_1_sensitivity_diagnostic() {
    // Finding R0-1: At phase 0.15, active scenario, b=0.1667 vs b=0.1700
    // must detect a temperature difference of ~25.48 degrees C
    let cfg_js = SimConfig {
        nx: 32,
        ny: 32,
        nz: 32,
        phase: 0.15,
        null_scenario: false,
        pulse_boundary: 0.1667,
        cycle_period_s: 10800.0,
    };
    let cfg_py = SimConfig {
        nx: 32,
        ny: 32,
        nz: 32,
        phase: 0.15,
        null_scenario: false,
        pulse_boundary: 0.1700,
        cycle_period_s: 10800.0,
    };

    let mut temp_js = Vec::new();
    let mut temp_py = Vec::new();
    fill_field(&cfg_js, Field::Temperature, &mut temp_js);
    fill_field(&cfg_py, Field::Temperature, &mut temp_py);

    let mut max_diff = 0.0f32;
    for (a, b) in temp_js.iter().zip(temp_py.iter()) {
        let diff = (a - b).abs();
        if diff > max_diff {
            max_diff = diff;
        }
    }

    assert!(
        (max_diff - 25.48).abs() < 0.1,
        "Expected R0-1 temperature divergence near 25.48 C, observed {}",
        max_diff
    );
    println!("PASSED: R0-1 pulse boundary sensitivity diagnostic verified: max diff = {:.2} C", max_diff);
}
