#!/usr/bin/env python3
"""Fail-closed validator for the canonical Interface Falsification Bridge contract (v0.1.0).

Verifies:
1. Schema completeness, allowed evidence classifications, and required units/provenance in
   data/contracts/interface_falsification_contract_v0.json.
2. Fail-closed gate behavior: any gate with effect_threshold is None returns NOT_EVALUATED.
3. Programmatic evaluators for all six falsification gates (F1 through F6).
4. Scientific Correction 1: separation of K-40 EC decay-branch fraction from evaluated
   1460.822 keV photon-emission probability (DDEP May 2025, DDEP 2010, and IAEA TRS-473 vs
   Chen 2017 ENSDF with shared DDEP 2025 alpha_T) via P_EC* = I_gamma*(1 + alpha_tot).
5. Scientific Correction 2: competing first-order precursor hazards vs sequential product hydrolysis
   kinetics (with near-degenerate Taylor conditioning for |k_hyd - k_syn|*dt < 1e-8), rejecting both
   the unparenthesized formula and force-clamped 100% Boltzmann saturation.
6. Scientific Correction 3: unambiguous radiolysis absorbed-dose basis (pore_water_absorbed_dose_rate
   vs bulk_medium_absorbed_dose_rate), rejecting double-application of water partition factor f_w.
7. Measure-space prerequisite for Hoffman (2006) probability-of-symmetric-difference (d_PSD) metric.
8. Cross-repository constant and contract-version parity with RadioPharmacology.jl/src/biotite_geochem_models.jl.
"""

from __future__ import annotations

import json
import math
import os
import pathlib
import re
import sys
from typing import Any, Mapping, Sequence


REPO_ROOT = pathlib.Path(__file__).resolve().parent.parent
DEFAULT_CONTRACT_PATH = REPO_ROOT / "data" / "contracts" / "interface_falsification_contract_v0.json"
DEFAULT_SPEC_PATH = REPO_ROOT / "docs" / "specs" / "INTERFACE_FALSIFICATION_BRIDGE_v0.md"
DEFAULT_JULIA_REPO_ROOT = pathlib.Path(
    os.environ.get(
        "RADIOPHARMACOLOGY_JL_DIR",
        pathlib.Path.home()
        / ".gemini"
        / "antigravity"
        / "scratch"
        / "RadioPharmacology.jl",
    )
).expanduser()
DEFAULT_JULIA_MODEL_PATH = (
    DEFAULT_JULIA_REPO_ROOT / "src" / "biotite_geochem_models.jl"
)
DEFAULT_JULIA_MIRROR_ROOT = (
    DEFAULT_JULIA_REPO_ROOT.parent / "RadioPharm.jl"
)

REQUIRED_EVIDENCE_CLASSES = (
    "empirically_supported_parameter",
    "source_reported_value",
    "phenomenological_surrogate",
    "uncalibrated_candidate_kinetic_law",
    "illustrative_simulation",
    "untested_hypothesis",
)

REQUIRED_GATE_IDS = ("F1", "F2", "F3", "F4", "F5", "F6")


def evaluate_f1_fiber_sufficiency(
    fiber_outcomes: Mapping[str, Sequence[float]],
    uncertainty_u95: float,
    effect_threshold: float | None,
) -> dict[str, Any]:
    """Evaluate Gate F1 (Interface / Fiber Sufficiency).

    Falsified if states mapped to the same coarse-grained fiber F_p produce
    outcomes diverging by more than effect_threshold once u95 is included.
    """
    if uncertainty_u95 < 0.0 or not math.isfinite(uncertainty_u95):
        raise ValueError("uncertainty_u95 must be finite and non-negative")
    if not fiber_outcomes:
        raise ValueError("fiber_outcomes must contain at least one fiber")

    max_spread = 0.0
    worst_fiber = ""
    for fiber_id, vals in fiber_outcomes.items():
        if len(vals) < 2:
            raise ValueError(f"Fiber {fiber_id!r} requires at least 2 microstates to test sufficiency")
        for v in vals:
            if not math.isfinite(v):
                raise ValueError(f"Non-finite outcome {v} in fiber {fiber_id!r}")
        spread = max(vals) - min(vals)
        if spread >= max_spread:
            max_spread = spread
            worst_fiber = fiber_id

    if effect_threshold is None:
        return {
            "gate_id": "F1",
            "verdict": "NOT_EVALUATED",
            "max_intra_fiber_spread": max_spread,
            "worst_fiber": worst_fiber,
        }
    if effect_threshold <= 0.0 or not math.isfinite(effect_threshold):
        raise ValueError("effect_threshold must be finite and positive when declared")

    upper_divergence = max_spread + uncertainty_u95
    verdict = (
        "PASS_FALSIFIER_NOT_TRIGGERED"
        if upper_divergence <= effect_threshold
        else "FALSIFIED_PARTITION_TOO_COARSE"
    )
    return {
        "gate_id": "F1",
        "verdict": verdict,
        "max_intra_fiber_spread": max_spread,
        "upper_divergence_with_u95": upper_divergence,
        "effect_threshold": effect_threshold,
        "worst_fiber": worst_fiber,
    }


def evaluate_f2_pulse_timing(
    pulsed_yield: float,
    continuous_matched_yield: float,
    uncertainty_q01_margin: float,
    pulsed_integrated_dose_gy: float,
    continuous_integrated_dose_gy: float,
    pulsed_thermal_exposure_k_s: float,
    continuous_thermal_exposure_k_s: float,
    effect_direction: int,
    effect_threshold: float | None,
    budget_rtol: float = 1.0e-6,
) -> dict[str, Any]:
    """Evaluate Gate F2 (Pulse Timing vs Matched Total Exposure)."""
    if effect_direction not in (1, -1):
        raise ValueError("effect_direction must be +1 or -1")
    if uncertainty_q01_margin < 0.0 or not math.isfinite(uncertainty_q01_margin):
        raise ValueError("uncertainty_q01_margin must be finite and non-negative")
    for val in (
        pulsed_yield,
        continuous_matched_yield,
        pulsed_integrated_dose_gy,
        continuous_integrated_dose_gy,
        pulsed_thermal_exposure_k_s,
        continuous_thermal_exposure_k_s,
    ):
        if not math.isfinite(val):
            raise ValueError("All yield and exposure inputs must be finite")

    dose_scale = max(abs(pulsed_integrated_dose_gy), abs(continuous_integrated_dose_gy), 1.0e-30)
    therm_scale = max(abs(pulsed_thermal_exposure_k_s), abs(continuous_thermal_exposure_k_s), 1.0e-30)
    if (
        abs(pulsed_integrated_dose_gy - continuous_integrated_dose_gy) / dose_scale > budget_rtol
        or abs(pulsed_thermal_exposure_k_s - continuous_thermal_exposure_k_s) / therm_scale > budget_rtol
    ):
        return {
            "gate_id": "F2",
            "verdict": "BLOCKED_ON_MISSING_CONTROL",
            "reason": "Pulsed and continuous arms do not have matched integrated dose or thermal exposure",
        }

    if effect_threshold is None:
        return {
            "gate_id": "F2",
            "verdict": "NOT_EVALUATED",
            "signed_yield_diff": effect_direction * (pulsed_yield - continuous_matched_yield),
        }
    if effect_threshold <= 0.0 or not math.isfinite(effect_threshold):
        raise ValueError("effect_threshold must be finite and positive when declared")

    signed_diff = effect_direction * (pulsed_yield - continuous_matched_yield)
    q01_lower = signed_diff - uncertainty_q01_margin
    verdict = (
        "PASS_FALSIFIER_NOT_TRIGGERED"
        if q01_lower > effect_threshold
        else "FALSIFIED_TIMING_NO_EFFECT"
    )
    return {
        "gate_id": "F2",
        "verdict": verdict,
        "signed_yield_diff": signed_diff,
        "q01_lower_bound": q01_lower,
        "effect_threshold": effect_threshold,
    }


def evaluate_f3_scaffold_specificity(
    biotite_yield_mol_m2: float | None,
    inert_quartz_yield_mol_m2: float | None,
    alt_phyllosilicate_yield_mol_m2: float | None,
    water_chemistry_matched: bool,
    uncertainty_q01_margin: float,
    effect_threshold: float | None,
) -> dict[str, Any]:
    """Evaluate Gate F3 (Scaffold-Specific Effect: Biotite vs Inert & Alternative Minerals)."""
    if uncertainty_q01_margin < 0.0 or not math.isfinite(uncertainty_q01_margin):
        raise ValueError("uncertainty_q01_margin must be finite and non-negative")
    if (
        biotite_yield_mol_m2 is None
        or inert_quartz_yield_mol_m2 is None
        or alt_phyllosilicate_yield_mol_m2 is None
        or not water_chemistry_matched
    ):
        return {
            "gate_id": "F3",
            "verdict": "BLOCKED_ON_MISSING_CONTROL",
            "reason": "Requires BET-normalized biotite, inert quartz, and alternative phyllosilicate controls under matched water chemistry",
        }
    for val in (biotite_yield_mol_m2, inert_quartz_yield_mol_m2, alt_phyllosilicate_yield_mol_m2):
        if val < 0.0 or not math.isfinite(val):
            raise ValueError("BET-normalized yields must be finite and non-negative")

    best_control = max(inert_quartz_yield_mol_m2, alt_phyllosilicate_yield_mol_m2)
    excess = biotite_yield_mol_m2 - best_control
    if effect_threshold is None:
        return {
            "gate_id": "F3",
            "verdict": "NOT_EVALUATED",
            "excess_over_best_control_mol_m2": excess,
        }
    if effect_threshold <= 0.0 or not math.isfinite(effect_threshold):
        raise ValueError("effect_threshold must be finite and positive when declared")

    q01_lower = excess - uncertainty_q01_margin
    verdict = (
        "PASS_FALSIFIER_NOT_TRIGGERED"
        if q01_lower > effect_threshold
        else "FALSIFIED_NON_SPECIFIC_SURFACE"
    )
    return {
        "gate_id": "F3",
        "verdict": verdict,
        "excess_over_best_control_mol_m2": excess,
        "q01_lower_bound": q01_lower,
        "effect_threshold": effect_threshold,
    }


def evaluate_f4_selective_retention(
    solute_retardation_r: float | None,
    conservative_tracer_r: float | None,
    peclet_and_evaporation_matched: bool,
    uncertainty_u95: float,
    effect_threshold: float | None,
) -> dict[str, Any]:
    """Evaluate Gate F4 (Selective Retention vs Passive Concentration / Washout)."""
    if uncertainty_u95 < 0.0 or not math.isfinite(uncertainty_u95):
        raise ValueError("uncertainty_u95 must be finite and non-negative")
    if solute_retardation_r is None or conservative_tracer_r is None or not peclet_and_evaporation_matched:
        return {
            "gate_id": "F4",
            "verdict": "BLOCKED_ON_MISSING_CONTROL",
            "reason": "Requires paired conservative tracer under matched Peclet number and evaporation rate",
        }
    if solute_retardation_r <= 0.0 or conservative_tracer_r <= 0.0:
        raise ValueError("Retardation factors must be positive")
    if not (math.isfinite(solute_retardation_r) and math.isfinite(conservative_tracer_r)):
        raise ValueError("Retardation factors must be finite")

    selective_excess = (solute_retardation_r / conservative_tracer_r) - 1.0
    if effect_threshold is None:
        return {
            "gate_id": "F4",
            "verdict": "NOT_EVALUATED",
            "selective_retardation_excess": selective_excess,
        }
    if effect_threshold <= 0.0 or not math.isfinite(effect_threshold):
        raise ValueError("effect_threshold must be finite and positive when declared")

    lower_bound = selective_excess - uncertainty_u95
    verdict = (
        "PASS_FALSIFIER_NOT_TRIGGERED"
        if lower_bound > effect_threshold
        else "FALSIFIED_PASSIVE_CONCENTRATION"
    )
    return {
        "gate_id": "F4",
        "verdict": verdict,
        "selective_retardation_excess": selective_excess,
        "lower_bound_with_u95": lower_bound,
        "effect_threshold": effect_threshold,
    }


def evaluate_f5_heritable_selection(
    template_copying_rate_s_inv: float,
    transfer_cycles: int,
    replicating_freq_shift: float,
    sorting_control_freq_shift: float,
    standard_error: float,
    effect_threshold: float | None,
) -> dict[str, Any]:
    """Evaluate Gate F5 (Heritable Selection vs Non-Heritable Physical Sorting)."""
    if template_copying_rate_s_inv < 0.0 or not math.isfinite(template_copying_rate_s_inv):
        raise ValueError("template_copying_rate_s_inv must be finite and non-negative")
    if standard_error < 0.0 or not math.isfinite(standard_error):
        raise ValueError("standard_error must be finite and non-negative")
    if transfer_cycles < 0:
        raise ValueError("transfer_cycles must be non-negative")

    if effect_threshold is None:
        return {
            "gate_id": "F5",
            "verdict": "NOT_EVALUATED",
            "template_copying_rate_s_inv": template_copying_rate_s_inv,
        }
    if effect_threshold <= 0.0 or not math.isfinite(effect_threshold):
        raise ValueError("effect_threshold must be finite and positive when declared")

    if template_copying_rate_s_inv == 0.0 or transfer_cycles < 2:
        return {
            "gate_id": "F5",
            "verdict": "FALSIFIED_NON_HERITABLE_SORTING",
            "reason": "Zero template copying rate or insufficient serial transfers; physical sorting cannot establish heritable evolution",
        }

    net_heritable_shift = (replicating_freq_shift - sorting_control_freq_shift) - 2.0 * standard_error
    verdict = (
        "PASS_FALSIFIER_NOT_TRIGGERED"
        if net_heritable_shift > effect_threshold
        else "FALSIFIED_NON_HERITABLE_SORTING"
    )
    return {
        "gate_id": "F5",
        "verdict": verdict,
        "net_heritable_shift_2se": net_heritable_shift,
        "effect_threshold": effect_threshold,
    }


def evaluate_f6_independent_mass_balance(
    initial_inventory_mol: float,
    compartment_inventories_mol: Sequence[float],
    closes_by_algebraic_definition: bool,
    effect_threshold: float | None,
) -> dict[str, Any]:
    """Evaluate Gate F6 (Independent Mass Balance & Decay-Chain Accounting)."""
    if initial_inventory_mol <= 0.0 or not math.isfinite(initial_inventory_mol):
        raise ValueError("initial_inventory_mol must be finite and positive")
    if not compartment_inventories_mol:
        raise ValueError("compartment_inventories_mol must not be empty")
    for c in compartment_inventories_mol:
        if c < 0.0 or not math.isfinite(c):
            raise ValueError("Each compartment inventory must be finite and non-negative")

    if closes_by_algebraic_definition:
        return {
            "gate_id": "F6",
            "verdict": "BLOCKED_ALGEBRAIC_TAUTOLOGY",
            "reason": "At least one compartment was defined by algebraic subtraction rather than independent accounting",
        }

    if effect_threshold is None:
        return {
            "gate_id": "F6",
            "verdict": "NOT_EVALUATED",
        }
    if effect_threshold <= 0.0 or not math.isfinite(effect_threshold):
        raise ValueError("effect_threshold must be finite and positive when declared")

    total_out = math.fsum(compartment_inventories_mol)
    rel_residual = abs(initial_inventory_mol - total_out) / initial_inventory_mol
    verdict = (
        "PASS_FALSIFIER_NOT_TRIGGERED"
        if rel_residual <= effect_threshold
        else "FALSIFIED_MASS_IMBALANCE"
    )
    return {
        "gate_id": "F6",
        "verdict": verdict,
        "relative_closure_residual": rel_residual,
        "effect_threshold": effect_threshold,
    }


def evaluate_competing_precursor_synthesis(
    k_syn_s_inv: float,
    k_deact_s_inv: float,
    dt_s: float,
) -> dict[str, float]:
    """Correction 2A: Competing first-order hazards from the same activated precursor.

    P_syn(dt) = (k_syn / (k_syn + k_deact)) * (1 - exp(-(k_syn + k_deact) * dt))
    P_deact(dt) = (k_deact / (k_syn + k_deact)) * (1 - exp(-(k_syn + k_deact) * dt))
    P_surviving_precursor(dt) = exp(-(k_syn + k_deact) * dt)
    """
    if k_syn_s_inv < 0.0 or k_deact_s_inv < 0.0 or dt_s < 0.0:
        raise ValueError("Rates and exposure interval dt_s must be non-negative")
    if not (math.isfinite(k_syn_s_inv) and math.isfinite(k_deact_s_inv) and math.isfinite(dt_s)):
        raise ValueError("Rates and dt_s must be finite")

    k_tot = k_syn_s_inv + k_deact_s_inv
    if k_tot == 0.0 or dt_s == 0.0:
        return {
            "p_precursor": 1.0,
            "p_syn": 0.0,
            "p_deact": 0.0,
            "closure_residual": 0.0,
        }

    p_precursor = math.exp(-k_tot * dt_s)
    reacted = -math.expm1(-k_tot * dt_s)
    p_syn = (k_syn_s_inv / k_tot) * reacted
    p_deact = (k_deact_s_inv / k_tot) * reacted
    closure_res = abs((p_precursor + p_syn + p_deact) - 1.0)
    return {
        "p_precursor": p_precursor,
        "p_syn": p_syn,
        "p_deact": p_deact,
        "closure_residual": closure_res,
    }


def evaluate_sequential_product_hydrolysis(
    k_syn_s_inv: float,
    k_hyd_s_inv: float,
    dt_s: float,
) -> dict[str, float]:
    """Correction 2B: Sequential synthesis followed by product hydrolysis (M -> P_chain -> D_deg).

    Uses 4th-order Taylor series for |k_hyd - k_syn| * dt < 1e-8 to prevent subnormal/near-equal
    cancellation and preserve full IEEE-754 Float64 accuracy across the k_syn == k_hyd limit.
    """
    if k_syn_s_inv < 0.0 or k_hyd_s_inv < 0.0 or dt_s < 0.0:
        raise ValueError("Rates and exposure interval dt_s must be non-negative")
    if not (math.isfinite(k_syn_s_inv) and math.isfinite(k_hyd_s_inv) and math.isfinite(dt_s)):
        raise ValueError("Rates and dt_s must be finite")

    f_precursor = math.exp(-k_syn_s_inv * dt_s)
    diff = abs(k_hyd_s_inv - k_syn_s_inv)
    r_min = min(k_syn_s_inv, k_hyd_s_inv)
    delta = diff * dt_s
    if delta < 1.0e-8:
        # (-expm1(-delta)) / delta = 1 - delta/2 + delta^2/6 - delta^3/24
        series = 1.0 - delta * (0.5 - delta * ((1.0 / 6.0) - delta / 24.0))
        exp_diff = dt_s * math.exp(-r_min * dt_s) * series
    else:
        exp_diff = math.exp(-r_min * dt_s) * (-math.expm1(-delta) / diff)

    f_product = k_syn_s_inv * exp_diff
    f_degraded = max(0.0, 1.0 - f_precursor - f_product)
    closure_res = abs((f_precursor + f_product + f_degraded) - 1.0)
    return {
        "f_precursor": f_precursor,
        "f_product": f_product,
        "f_degraded": f_degraded,
        "closure_residual": closure_res,
    }


def evaluate_radiolysis_energy_deposition(
    dose_basis: str,
    dose_rate_gy_s: float,
    porosity_eps: float,
    pore_water_density_kg_m3: float = 1000.0,
    bulk_density_kg_m3: float | None = None,
    water_energy_partition_f_w: float | None = None,
    f_w_provenance: str | None = None,
    g_h2_molecules_per_100ev: float = 0.45,
) -> dict[str, Any]:
    """Correction 3: Unambiguous radiolysis absorbed-dose basis."""
    if dose_rate_gy_s < 0.0 or not math.isfinite(dose_rate_gy_s):
        raise ValueError("dose_rate_gy_s must be finite and non-negative")
    if not (0.0 < porosity_eps < 1.0 and math.isfinite(porosity_eps)):
        raise ValueError("porosity_eps must be in (0.0, 1.0)")
    if pore_water_density_kg_m3 <= 0.0 or not math.isfinite(pore_water_density_kg_m3):
        raise ValueError("pore_water_density_kg_m3 must be positive")
    if g_h2_molecules_per_100ev < 0.0 or not math.isfinite(g_h2_molecules_per_100ev):
        raise ValueError("g_h2_molecules_per_100ev must be non-negative")

    g_mol_per_j = g_h2_molecules_per_100ev * 1.036426965e-7

    if dose_basis == "pore_water_absorbed_dose_rate":
        if water_energy_partition_f_w is not None:
            raise ValueError(
                "Double-counting error: water_energy_partition_f_w must not be applied when "
                "dose_basis is 'pore_water_absorbed_dose_rate' (D_dot_w is already pore-water dose rate)"
            )
        q_abs_w_bulk = porosity_eps * pore_water_density_kg_m3 * dose_rate_gy_s
        h2_rate_pore_water_mol_m3_s = pore_water_density_kg_m3 * dose_rate_gy_s * g_mol_per_j
    elif dose_basis == "bulk_medium_absorbed_dose_rate":
        if bulk_density_kg_m3 is None or bulk_density_kg_m3 <= 0.0 or not math.isfinite(bulk_density_kg_m3):
            raise ValueError("bulk_density_kg_m3 must be positive for bulk_medium_absorbed_dose_rate")
        if water_energy_partition_f_w is None or not (0.0 < water_energy_partition_f_w < 1.0):
            raise ValueError("water_energy_partition_f_w in (0, 1) is required for bulk_medium_absorbed_dose_rate")
        if not f_w_provenance or not f_w_provenance.strip():
            raise ValueError("f_w_provenance string is required for bulk_medium_absorbed_dose_rate")
        q_abs_w_bulk = bulk_density_kg_m3 * water_energy_partition_f_w * dose_rate_gy_s
        h2_rate_pore_water_mol_m3_s = (q_abs_w_bulk / porosity_eps) * g_mol_per_j
    else:
        raise ValueError(f"Unsupported dose_basis: {dose_basis!r}")

    return {
        "dose_basis": dose_basis,
        "q_abs_w_bulk_w_m3": q_abs_w_bulk,
        "h2_production_rate_pore_water_mol_m3_s": h2_rate_pore_water_mol_m3_s,
        "g_h2_mol_per_j": g_mol_per_j,
    }


def evaluate_psd_metric(
    values_a: Sequence[float],
    values_b: Sequence[float],
    measure_weights: Sequence[float],
    event_threshold: float,
) -> dict[str, float]:
    """Evaluate Hoffman (2006) probability-of-symmetric-difference metric on an explicit measure space."""
    if not (len(values_a) == len(values_b) == len(measure_weights)) or len(values_a) == 0:
        raise ValueError("values_a, values_b, and measure_weights must have equal non-zero length")
    if not math.isfinite(event_threshold):
        raise ValueError("event_threshold must be finite")

    for w in measure_weights:
        if w < 0.0 or not math.isfinite(w):
            raise ValueError("All measure_weights must be finite and non-negative")
    total_measure = math.fsum(measure_weights)
    if abs(total_measure - 1.0) > 1.0e-12:
        raise ValueError(f"measure_weights must be normalized to 1.0 (got {total_measure})")

    d_psd = 0.0
    for va, vb, mu_i in zip(values_a, values_b, measure_weights):
        in_a = va >= event_threshold
        in_b = vb >= event_threshold
        if in_a != in_b:
            d_psd += mu_i

    return {
        "d_psd": d_psd,
        "hilbert_l2_distance": math.sqrt(d_psd),
    }


def evaluate_k40_convention_fixture_discrepancy(
    photon_emission_prob: float,
    photon_emission_sigma: float,
    internal_conversion_alpha_tot: float,
    internal_conversion_alpha_sigma: float,
    fixture_branch_ec_gamma_frac: float,
    max_normalized_discrepancy: float = 2.0,
    fixture_branch_sigma: float | None = None,
    evaluation_edition_year: int = 2025,
    convention_id: str = "DDEP_2025",
    source_citation: str = "Mougeot (May 2025) DDEP",
    photon_emission_source: str | None = None,
    decay_branch_source: str | None = None,
    conversion_coefficient_source: str | None = None,
    alpha_tot_is_shared_ddep_adopted: bool | None = None,
) -> dict[str, Any]:
    """Evaluate K-40 photon-convention vs geochronology-fixture discrepancy (RT-01, Item 2A & RT-02)."""
    if not source_citation or not source_citation.strip():
        raise ValueError("source_citation must be non-empty")
    if not (1970 <= evaluation_edition_year <= 2100):
        raise ValueError("evaluation_edition_year must be in [1970, 2100]")
    if (convention_id == "DDEP_2004" or evaluation_edition_year < 2020) and abs(photon_emission_prob - 0.1034) < 1.0e-4 and "DDEP" in convention_id.upper():
        raise ValueError(
            "Provenance edition mismatch: I_gamma = 0.1034(7) belongs to the May 2025 DDEP evaluation (Mougeot, 2025), not a pre-2020 DDEP edition."
        )
    if photon_emission_sigma <= 0.0 or not math.isfinite(photon_emission_sigma):
        raise ValueError("photon_emission_sigma must be finite and positive")
    if max_normalized_discrepancy <= 0.0 or not math.isfinite(max_normalized_discrepancy):
        raise ValueError("max_normalized_discrepancy must be finite and positive")

    shared_alpha = (
        (convention_id not in ("DDEP_2025", "DDEP_2010"))
        if alpha_tot_is_shared_ddep_adopted is None
        else bool(alpha_tot_is_shared_ddep_adopted)
    )
    psrc = (photon_emission_source if photon_emission_source is not None else source_citation).strip()
    bsrc = (
        decay_branch_source
        if decay_branch_source is not None
        else (
            "Chen, J. (2017) Nuclear Data Sheets 140:1-364 (ENSDF A=40 evaluation): P_EC* = 10.67(11)%"
            if convention_id == "IAEA_ENSDF_2017"
            else source_citation
        )
    ).strip()
    csrc = (
        conversion_coefficient_source
        if conversion_coefficient_source is not None
        else (
            "Shared adopted value from DDEP May 2025 (Mougeot 2025 / Chiste & Be 2010: alpha_T = 10.28(15)e-5 = 1.028(15)e-4)"
            if shared_alpha
            else "Native DDEP evaluation (Mougeot May 2025 / Chiste & Be 2010: alpha_T = 10.28(15)e-5 = 1.028(15)e-4)"
        )
    ).strip()

    if not psrc or not bsrc or not csrc:
        raise ValueError("photon_emission_source, decay_branch_source, and conversion_coefficient_source must be non-empty")
    if convention_id in ("DDEP_2025", "DDEP_2010") and shared_alpha:
        raise ValueError(f"Provenance mismatch for {convention_id}: alpha_T is native to DDEP, so alpha_tot_is_shared_ddep_adopted must be False")
    if shared_alpha and ("shared" not in csrc.lower() and "adopted" not in csrc.lower()):
        raise ValueError(f"Provenance mismatch for {convention_id}: conversion_coefficient_source must document shared/adopted alpha_T")
    if not shared_alpha and "shared" in csrc.lower():
        raise ValueError(f"Provenance mismatch for {convention_id}: native conversion_coefficient_source must not claim a shared value")
    if convention_id == "IAEA_ENSDF_2017":
        if abs(internal_conversion_alpha_tot - 1.028e-4) < 1.0e-6 and not shared_alpha:
            raise ValueError("Provenance mismatch for IAEA_ENSDF_2017: alpha_tot_is_shared_ddep_adopted must be True")
        if "10.67" in psrc:
            raise ValueError("Provenance mismatch for IAEA_ENSDF_2017: photon_emission_source must not conflate Chen (2017) P_EC* = 10.67(11)%")
        if "10.67" not in bsrc:
            raise ValueError("Provenance mismatch for IAEA_ENSDF_2017: decay_branch_source must cite Chen (2017) P_EC* = 10.67(11)%")

    implied_p_ec = photon_emission_prob * (1.0 + internal_conversion_alpha_tot)
    implied_p_ec_sigma = math.hypot(
        (1.0 + internal_conversion_alpha_tot) * photon_emission_sigma,
        photon_emission_prob * internal_conversion_alpha_sigma,
    )
    branch_diff = abs(fixture_branch_ec_gamma_frac - implied_p_ec)
    one_sided_norm = branch_diff / photon_emission_sigma

    if fixture_branch_sigma is None:
        norm_disc = one_sided_norm
        metric_type = "one_sided_convention_sigma_normalized_discrepancy"
        fixture_unc_prop = False
        is_cal_z = False
    else:
        if fixture_branch_sigma <= 0.0 or not math.isfinite(fixture_branch_sigma):
            raise ValueError("fixture_branch_sigma must be finite and positive when supplied")
        combined_sigma = math.hypot(implied_p_ec_sigma, fixture_branch_sigma)
        norm_disc = branch_diff / combined_sigma
        metric_type = "two_sided_uncorrelated_sigma_normalized_discrepancy"
        fixture_unc_prop = True
        is_cal_z = True

    return {
        "convention_id": convention_id,
        "evaluation_edition_year": evaluation_edition_year,
        "photon_emission_source": psrc,
        "decay_branch_source": bsrc,
        "conversion_coefficient_source": csrc,
        "alpha_tot_is_shared_ddep_adopted": shared_alpha,
        "implied_ec_excited_branch_frac": implied_p_ec,
        "implied_ec_excited_branch_sigma": implied_p_ec_sigma,
        "branch_difference": branch_diff,
        "one_sided_normalized_discrepancy": one_sided_norm,
        "normalized_discrepancy": norm_disc,
        "max_normalized_discrepancy": max_normalized_discrepancy,
        "discrepancy_metric_type": metric_type,
        "fixture_branch_uncertainty_propagated": fixture_unc_prop,
        "is_calibrated_statistical_z_test": is_cal_z,
        "passes_acceptance_rule": norm_disc <= max_normalized_discrepancy,
    }


def validate_julia_model_parity(
    contract: Mapping[str, Any],
    julia_model_path: pathlib.Path = DEFAULT_JULIA_MODEL_PATH,
) -> list[str]:
    """Verify cross-repository parity between canonical JSON contract and RadioPharmacology.jl."""
    errors: list[str] = []
    if not julia_model_path.exists():
        return errors

    src = julia_model_path.read_text(encoding="utf-8")
    m_id = re.search(r'INTERFACE_FALSIFICATION_CONTRACT_ID\s*=\s*"([^"]+)"', src)
    m_ver = re.search(r'INTERFACE_FALSIFICATION_CONTRACT_VERSION\s*=\s*"([^"]+)"', src)
    if not m_id or m_id.group(1) != contract.get("contract_id"):
        errors.append("Julia INTERFACE_FALSIFICATION_CONTRACT_ID does not match canonical contract_id")
    if not m_ver or m_ver.group(1) != contract.get("contract_version"):
        errors.append("Julia INTERFACE_FALSIFICATION_CONTRACT_VERSION does not match canonical contract_version")

    # Check that DDEP 2025 (0.1034, 0.0007, 1.028e-4, 0.015e-4), DDEP 2010 (0.1055, 0.0011),
    # IAEA/ENSDF 2017 (0.1066, 0.0013 vs Chen 2017 P_EC* = 10.67(11)% and shared DDEP alpha_T),
    # and RT-03 reduced 4-state naming are present in Julia source
    for token in (
        "K40_GAMMA_DDEP_2025",
        "K40_GAMMA_DDEP_2010",
        "K40_GAMMA_IAEA_ENSDF",
        "0.1034",
        "0.0007",
        "0.1055",
        "0.0011",
        "0.1066",
        "0.0013",
        "10.67(11)%",
        "1.028e-4",
        "0.015e-4",
        "photon_emission_source",
        "decay_branch_source",
        "conversion_coefficient_source",
        "alpha_tot_is_shared_ddep_adopted",
        "4.962e-10",
        "0.581e-10",
        "4.9737e-10",
        "0.5757e-10",
        "pb207_reduced_4state_fraction_at_elapsed",
        "reduced_4state_bateman_chain_approximation",
    ):
        if token not in src:
            errors.append(f"Expected constant or symbol {token!r} missing from {julia_model_path}")

    if "3.07e-4" in src:
        errors.append(f"Stale internal conversion coefficient '3.07e-4' found in {julia_model_path}")

    # When validating the default RadioPharmacology.jl path, also verify bitwise parity with the RadioPharm.jl mirror
    if julia_model_path == DEFAULT_JULIA_MODEL_PATH:
        mirror_src_path = DEFAULT_JULIA_MIRROR_ROOT / "src" / "biotite_geochem_models.jl"
        if mirror_src_path.exists() and mirror_src_path.read_bytes() != julia_model_path.read_bytes():
            errors.append("RadioPharm.jl/src/biotite_geochem_models.jl is out of sync with RadioPharmacology.jl/src/biotite_geochem_models.jl")
        main_test_path = julia_model_path.parent.parent / "test" / "test_biotite_geochem_models.jl"
        mirror_test_path = DEFAULT_JULIA_MIRROR_ROOT / "test" / "test_biotite_geochem_models.jl"
        if main_test_path.exists() and mirror_test_path.exists() and mirror_test_path.read_bytes() != main_test_path.read_bytes():
            errors.append("RadioPharm.jl/test/test_biotite_geochem_models.jl is out of sync with RadioPharmacology.jl/test/test_biotite_geochem_models.jl")

    return errors


def validate_contract_dict(contract: Mapping[str, Any]) -> list[str]:
    """Validate schema, provenance, and scientific invariants of the contract dictionary."""
    errors: list[str] = []

    if contract.get("contract_id") != "INTERFACE_FALSIFICATION_BRIDGE_v0":
        errors.append("contract_id must be 'INTERFACE_FALSIFICATION_BRIDGE_v0'")
    if contract.get("contract_version") != "v0.1.0":
        errors.append("contract_version must be 'v0.1.0'")
    if contract.get("canonical_repository") != "aurascoper/oklo_pool_mobile":
        errors.append("canonical_repository must be 'aurascoper/oklo_pool_mobile'")

    ev_classes = contract.get("allowed_evidence_classifications", [])
    if tuple(ev_classes) != REQUIRED_EVIDENCE_CLASSES:
        errors.append("allowed_evidence_classifications does not match the 6 required classes")

    allowed_verdicts = set(contract.get("allowed_gate_verdicts", []))
    if "NOT_EVALUATED" not in allowed_verdicts:
        errors.append("NOT_EVALUATED must be in allowed_gate_verdicts")

    bridge = contract.get("epistemic_bridge_mapping", {})
    if bridge.get("agency_or_perception_claimed_for_chemistry") is not False:
        errors.append("agency_or_perception_claimed_for_chemistry must be strictly false")

    psd = bridge.get("psd_measure_space_contract", {})
    for req_flag in (
        "requires_explicit_sample_space_omega",
        "requires_event_threshold_predicate",
        "requires_normalized_measure_mu",
        "raw_unnormalized_voxel_euclidean_conflation_prohibited",
    ):
        if psd.get(req_flag) is not True:
            errors.append(f"psd_measure_space_contract.{req_flag} must be true")

    # Validate Correction 1: K-40 decay branch vs photon emission conventions
    sc = contract.get("scientific_corrections", {})
    c1 = sc.get("correction_1_nuclear_data_conventions", {})
    if c1.get("conflation_of_decay_branch_and_photon_intensity_prohibited") is not True:
        errors.append("conflation_of_decay_branch_and_photon_intensity_prohibited must be true")

    fixtures = c1.get("decay_fixtures", [])
    conventions = c1.get("photon_emission_conventions", [])
    if len(fixtures) < 2 or len(conventions) < 2:
        errors.append("correction_1 must include at least 2 decay_fixtures and 2 photon_emission_conventions")

    for fx in fixtures:
        ltot = fx["lambda_beta_per_yr"] + fx["lambda_ec_gamma_per_yr"] + fx["lambda_ec_ground_per_yr"]
        if abs(ltot - fx["lambda_total_per_yr"]) > 1.0e-22:
            errors.append(f"Fixture {fx['fixture_id']} lambda_total_per_yr does not match sum of partial constants")
        thalf = math.log(2.0) / ltot
        if abs(thalf - fx["t_half_yr"]) / thalf > 1.0e-12:
            errors.append(f"Fixture {fx['fixture_id']} t_half_yr does not match ln(2)/lambda_total")
        if fx.get("evidence_classification") not in REQUIRED_EVIDENCE_CLASSES:
            errors.append(f"Fixture {fx['fixture_id']} has invalid evidence_classification")

    for conv in conventions:
        cid = conv.get("convention_id", "<unknown>")
        yr = conv.get("evaluation_edition_year", 0)
        igam = conv["photon_emission_prob"]
        alpha = conv["internal_conversion_alpha_tot"]
        alpha_sig = conv.get("internal_conversion_alpha_sigma", -1.0)
        implied = igam * (1.0 + alpha)
        if not (1970 <= yr <= 2100):
            errors.append(f"Convention {cid} evaluation_edition_year must be in [1970, 2100]")
        if (cid == "DDEP_2004" or yr < 2020) and abs(igam - 0.1034) < 1.0e-4 and "DDEP" in cid.upper():
            errors.append(f"Convention {cid} mislabels May 2025 DDEP value 0.1034 as a pre-2020 DDEP edition")
        if alpha_sig <= 0.0:
            errors.append(f"Convention {cid} must specify positive internal_conversion_alpha_sigma")
        if conv.get("includes_internal_pair_creation") is not True:
            errors.append(f"Convention {cid} must set includes_internal_pair_creation to true")
        shared_flag = conv.get("alpha_tot_is_shared_ddep_adopted")
        if not isinstance(shared_flag, bool):
            errors.append(f"Convention {cid} must specify boolean alpha_tot_is_shared_ddep_adopted")
        for src_field in ("source_citation", "photon_emission_source", "decay_branch_source", "conversion_coefficient_source"):
            if not conv.get(src_field) or not str(conv.get(src_field)).strip():
                errors.append(f"Convention {cid} must provide a non-empty {src_field}")
        csrc_str = str(conv.get("conversion_coefficient_source", "")).lower()
        psrc_str = str(conv.get("photon_emission_source", ""))
        bsrc_str = str(conv.get("decay_branch_source", ""))
        if cid in ("DDEP_2025", "DDEP_2010"):
            if shared_flag is not False:
                errors.append(f"Convention {cid} evaluates alpha_T natively in DDEP and must set alpha_tot_is_shared_ddep_adopted to false")
            if "shared" in csrc_str:
                errors.append(f"Convention {cid} conversion_coefficient_source must not claim a shared adopted alpha_T")
        if shared_flag is True and ("shared" not in csrc_str and "adopted" not in csrc_str):
            errors.append(f"Convention {cid} has alpha_tot_is_shared_ddep_adopted=true so conversion_coefficient_source must explicitly document shared/adopted alpha_T")
        if cid == "IAEA_ENSDF_2017":
            if shared_flag is not True:
                errors.append("Convention IAEA_ENSDF_2017 must set alpha_tot_is_shared_ddep_adopted to true")
            if "10.67" not in bsrc_str or "Chen" not in bsrc_str:
                errors.append("Convention IAEA_ENSDF_2017 decay_branch_source must distinguish Chen (2017) ENSDF P_EC* = 10.67(11)%")
            if "IAEA" not in psrc_str or "10.66" not in psrc_str or "10.67" in psrc_str:
                errors.append("Convention IAEA_ENSDF_2017 photon_emission_source must attribute I_gamma = 10.66(13)% to IAEA compilation without conflating 10.67(11)%")
            if abs(float(conv.get("ensdf_2017_reported_ec_branch_prob", 0.0)) - 0.1067) > 1.0e-6:
                errors.append("Convention IAEA_ENSDF_2017 must record ensdf_2017_reported_ec_branch_prob == 0.1067")
            if abs(float(conv.get("ensdf_2017_reported_ec_branch_sigma", 0.0)) - 0.0011) > 1.0e-6:
                errors.append("Convention IAEA_ENSDF_2017 must record ensdf_2017_reported_ec_branch_sigma == 0.0011")
        if abs(implied - conv["implied_ec_excited_branch_frac"]) > 1.0e-12:
            errors.append(f"Convention {cid} implied_ec_excited_branch_frac != I_gamma*(1 + alpha_tot)")
        if igam >= implied:
            errors.append(f"Convention {cid} must have photon_emission_prob < implied_ec_excited_branch_frac")
        if conv.get("evidence_classification") not in REQUIRED_EVIDENCE_CLASSES:
            errors.append(f"Convention {cid} has invalid evidence_classification")

    acc_rule = c1.get("convention_acceptance_rule", {})
    if acc_rule.get("discrepancy_metric_type") != "one_sided_convention_sigma_normalized_discrepancy":
        errors.append("convention_acceptance_rule.discrepancy_metric_type must be 'one_sided_convention_sigma_normalized_discrepancy'")
    if acc_rule.get("default_max_normalized_discrepancy") != 2.0:
        errors.append("convention_acceptance_rule.default_max_normalized_discrepancy must be 2.0")
    if acc_rule.get("fixture_branch_uncertainty_propagated") is not False:
        errors.append("convention_acceptance_rule.fixture_branch_uncertainty_propagated must be false")
    if acc_rule.get("is_calibrated_statistical_z_test") is not False:
        errors.append("convention_acceptance_rule.is_calibrated_statistical_z_test must be false")

    # Validate Correction 2: Reaction kinetics topologies
    c2 = sc.get("correction_2_reaction_kinetics_models", {})
    topologies = {t["topology_id"]: t for t in c2.get("approved_candidate_topologies", [])}
    if "competing_precursor_hazards" not in topologies or "sequential_product_hydrolysis" not in topologies:
        errors.append("correction_2 must define both competing_precursor_hazards and sequential_product_hydrolysis")
    for tid, tspec in topologies.items():
        if tspec.get("evidence_classification") != "uncalibrated_candidate_kinetic_law":
            errors.append(f"Topology {tid} must carry evidence_classification 'uncalibrated_candidate_kinetic_law'")

    # Validate Correction 3: Radiolysis dose bases
    c3 = sc.get("correction_3_radiolysis_dose_bases", {})
    bases = {b["dose_basis_id"]: b for b in c3.get("allowed_dose_bases", [])}
    if "pore_water_absorbed_dose_rate" not in bases or "bulk_medium_absorbed_dose_rate" not in bases:
        errors.append("correction_3 must define both pore_water_absorbed_dose_rate and bulk_medium_absorbed_dose_rate")
    else:
        if bases["pore_water_absorbed_dose_rate"].get("allow_additional_water_partition_factor_f_w") is not False:
            errors.append("pore_water_absorbed_dose_rate must forbid additional water partition factor f_w")
        if bases["bulk_medium_absorbed_dose_rate"].get("allow_additional_water_partition_factor_f_w") is not True:
            errors.append("bulk_medium_absorbed_dose_rate must require water partition factor f_w")

    # Validate Gates F1-F6
    gates = contract.get("falsification_gates", [])
    seen_ids = [g.get("gate_id") for g in gates]
    if tuple(seen_ids) != REQUIRED_GATE_IDS:
        errors.append(f"falsification_gates must be ordered {REQUIRED_GATE_IDS}, got {seen_ids}")

    for g in gates:
        gid = g.get("gate_id", "<unknown>")
        if not g.get("observable_id") or not g.get("observable_units"):
            errors.append(f"Gate {gid} is missing observable_id or observable_units")
        if not g.get("required_controls"):
            errors.append(f"Gate {gid} must declare at least one required control")
        if g.get("missing_threshold_verdict") != "NOT_EVALUATED":
            errors.append(f"Gate {gid} must set missing_threshold_verdict to 'NOT_EVALUATED'")
        status = g.get("current_evaluation_status")
        if status not in allowed_verdicts:
            errors.append(f"Gate {gid} has invalid current_evaluation_status {status!r}")
        if g.get("effect_threshold") is None and status != "NOT_EVALUATED":
            errors.append(f"Gate {gid} has effect_threshold == null but status is {status!r} instead of 'NOT_EVALUATED'")

    return errors


def run_synthetic_self_tests() -> None:
    """Run synthetic positive and negative verification cases for F1 through F6 and Corrections 1-3."""
    # F1: tight fiber passes; coarse fiber fails; missing threshold returns NOT_EVALUATED
    res_f1_pass = evaluate_f1_fiber_sufficiency({"wet_low_dose": [0.81, 0.83, 0.82]}, 0.01, 0.05)
    assert res_f1_pass["verdict"] == "PASS_FALSIFIER_NOT_TRIGGERED"
    res_f1_fail = evaluate_f1_fiber_sufficiency({"wet_low_dose": [0.20, 0.85]}, 0.01, 0.05)
    assert res_f1_fail["verdict"] == "FALSIFIED_PARTITION_TOO_COARSE"
    res_f1_none = evaluate_f1_fiber_sufficiency({"wet_low_dose": [0.20, 0.85]}, 0.01, None)
    assert res_f1_none["verdict"] == "NOT_EVALUATED"

    # F2: matched exposure positive effect passes; mismatched exposure blocks; missing threshold returns NOT_EVALUATED
    res_f2_pass = evaluate_f2_pulse_timing(0.45, 0.20, 0.05, 100.0, 100.0, 3.0e5, 3.0e5, 1, 0.10)
    assert res_f2_pass["verdict"] == "PASS_FALSIFIER_NOT_TRIGGERED"
    res_f2_block = evaluate_f2_pulse_timing(0.45, 0.20, 0.05, 100.0, 80.0, 3.0e5, 3.0e5, 1, 0.10)
    assert res_f2_block["verdict"] == "BLOCKED_ON_MISSING_CONTROL"
    res_f2_none = evaluate_f2_pulse_timing(0.45, 0.20, 0.05, 100.0, 100.0, 3.0e5, 3.0e5, 1, None)
    assert res_f2_none["verdict"] == "NOT_EVALUATED"

    # F3: BET-normalized biotite vs quartz and muscovite controls
    res_f3_pass = evaluate_f3_scaffold_specificity(1.8e-6, 0.2e-6, 0.5e-6, True, 0.2e-6, 0.5e-6)
    assert res_f3_pass["verdict"] == "PASS_FALSIFIER_NOT_TRIGGERED"
    res_f3_fail = evaluate_f3_scaffold_specificity(0.6e-6, 0.5e-6, 0.55e-6, True, 0.1e-6, 0.2e-6)
    assert res_f3_fail["verdict"] == "FALSIFIED_NON_SPECIFIC_SURFACE"
    res_f3_block = evaluate_f3_scaffold_specificity(1.8e-6, 0.2e-6, 0.5e-6, False, 0.2e-6, 0.5e-6)
    assert res_f3_block["verdict"] == "BLOCKED_ON_MISSING_CONTROL"
    res_f3_none = evaluate_f3_scaffold_specificity(1.8e-6, 0.2e-6, 0.5e-6, True, 0.2e-6, None)
    assert res_f3_none["verdict"] == "NOT_EVALUATED"

    # F4: Selective retention vs passive concentration / conservative tracer
    res_f4_pass = evaluate_f4_selective_retention(12.0, 1.0, True, 0.2, 0.5)
    assert res_f4_pass["verdict"] == "PASS_FALSIFIER_NOT_TRIGGERED"
    res_f4_fail = evaluate_f4_selective_retention(1.05, 1.0, True, 0.02, 0.10)
    assert res_f4_fail["verdict"] == "FALSIFIED_PASSIVE_CONCENTRATION"
    res_f4_block = evaluate_f4_selective_retention(12.0, None, True, 0.2, 0.5)
    assert res_f4_block["verdict"] == "BLOCKED_ON_MISSING_CONTROL"
    res_f4_none = evaluate_f4_selective_retention(12.0, 1.0, True, 0.2, None)
    assert res_f4_none["verdict"] == "NOT_EVALUATED"

    # F5: zero copying rate fails as non-heritable physical sorting; replicating shift passes
    res_f5_sort = evaluate_f5_heritable_selection(0.0, 5, 0.15, 0.15, 0.01, 0.02)
    assert res_f5_sort["verdict"] == "FALSIFIED_NON_HERITABLE_SORTING"
    res_f5_pass = evaluate_f5_heritable_selection(1.0e-3, 5, 0.25, 0.05, 0.02, 0.05)
    assert res_f5_pass["verdict"] == "PASS_FALSIFIER_NOT_TRIGGERED"

    # F6: algebraic tautology is blocked; independent closure passes; large imbalance fails
    res_f6_taut = evaluate_f6_independent_mass_balance(1.0, [0.4, 0.6], True, 1.0e-10)
    assert res_f6_taut["verdict"] == "BLOCKED_ALGEBRAIC_TAUTOLOGY"
    res_f6_pass = evaluate_f6_independent_mass_balance(1.0, [0.25, 0.35, 0.40], False, 1.0e-12)
    assert res_f6_pass["verdict"] == "PASS_FALSIFIER_NOT_TRIGGERED"
    res_f6_fail = evaluate_f6_independent_mass_balance(1.0, [0.25, 0.35, 0.30], False, 1.0e-6)
    assert res_f6_fail["verdict"] == "FALSIFIED_MASS_IMBALANCE"

    # Correction 2: competing hazards vs sequential hydrolysis (including near-equal rate limit)
    comp = evaluate_competing_precursor_synthesis(0.2, 0.8, 100.0)
    assert abs(comp["p_syn"] - 0.2) < 1.0e-12
    assert comp["closure_residual"] < 1.0e-15
    seq = evaluate_sequential_product_hydrolysis(0.2, 0.5, 10.0)
    assert seq["closure_residual"] < 1.0e-15
    seq_near_deg = evaluate_sequential_product_hydrolysis(0.25, 0.25 + 1.0e-13, 2.0)
    assert seq_near_deg["closure_residual"] < 1.0e-15

    # Correction 3: radiolysis dose-basis enforcement
    rad_pw = evaluate_radiolysis_energy_deposition("pore_water_absorbed_dose_rate", 0.5, 0.10)
    assert abs(rad_pw["q_abs_w_bulk_w_m3"] - 50.0) < 1.0e-12
    try:
        evaluate_radiolysis_energy_deposition(
            "pore_water_absorbed_dose_rate", 0.5, 0.10, water_energy_partition_f_w=0.15
        )
        raise AssertionError("Expected ValueError when applying f_w to pore_water_absorbed_dose_rate")
    except ValueError:
        pass


def main() -> int:
    if not DEFAULT_CONTRACT_PATH.exists():
        print(f"ERROR: Contract file not found: {DEFAULT_CONTRACT_PATH}", file=sys.stderr)
        return 1
    if not DEFAULT_SPEC_PATH.exists():
        print(f"ERROR: Specification markdown not found: {DEFAULT_SPEC_PATH}", file=sys.stderr)
        return 1

    contract = json.loads(DEFAULT_CONTRACT_PATH.read_text(encoding="utf-8"))
    errors = validate_contract_dict(contract)
    errors.extend(validate_julia_model_parity(contract))
    if errors:
        for err in errors:
            print(f"CONTRACT_ERROR: {err}", file=sys.stderr)
        return 1

    run_synthetic_self_tests()
    print(
        f"VALIDATED {contract['contract_id']} ({contract['contract_version']}): "
        f"{len(contract['falsification_gates'])} falsification gates (F1-F6), "
        f"{len(contract['allowed_evidence_classifications'])} evidence classes, "
        f"cross-repository Julia parity, and synthetic F1-F6 + Corrections 1-3 self-tests PASSED."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
