#!/usr/bin/env python3
"""Independent unit tests for the canonical Interface Falsification Bridge contract (v0.1.0).

Uses independent numerical integration (4th-order Runge-Kutta), 100-digit arbitrary-precision
Decimal arithmetic, and first-principles conservation identities to prevent circular verification.
"""

from __future__ import annotations

import copy
import decimal
import json
import math
import pathlib
import sys
import tempfile
import unittest

REPO_ROOT = pathlib.Path(__file__).resolve().parent.parent
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import validate_interface_falsification_contract as vfc  # noqa: E402


def _rk4_competing_precursor(k_syn: float, k_deact: float, dt: float, n_steps: int = 4000) -> tuple[float, float, float]:
    """Independently integrate dM/dt = -(k_syn + k_deact)*M, dP_syn/dt = k_syn*M, dP_deact/dt = k_deact*M."""
    h = dt / n_steps
    m, p_syn, p_deact = 1.0, 0.0, 0.0
    for _ in range(n_steps):
        k1_m = -(k_syn + k_deact) * m
        k2_m = -(k_syn + k_deact) * (m + 0.5 * h * k1_m)
        k3_m = -(k_syn + k_deact) * (m + 0.5 * h * k2_m)
        k4_m = -(k_syn + k_deact) * (m + h * k3_m)

        dm = (h / 6.0) * (k1_m + 2.0 * k2_m + 2.0 * k3_m + k4_m)
        m_mid = -dm / ((k_syn + k_deact) * h)
        m += dm
        p_syn += k_syn * h * m_mid
        p_deact += k_deact * h * m_mid
    return m, p_syn, p_deact


def _rk4_sequential_hydrolysis(k_syn: float, k_hyd: float, dt: float, n_steps: int = 4000) -> tuple[float, float, float]:
    """Independently integrate dM/dt = -k_syn*M, dP/dt = k_syn*M - k_hyd*P, dD/dt = k_hyd*P."""
    h = dt / n_steps
    m, p, d = 1.0, 0.0, 0.0

    def deriv(state: tuple[float, float, float]) -> tuple[float, float, float]:
        ms, ps, _ = state
        return (-k_syn * ms, k_syn * ms - k_hyd * ps, k_hyd * ps)

    for _ in range(n_steps):
        k1 = deriv((m, p, d))
        k2 = deriv((m + 0.5 * h * k1[0], p + 0.5 * h * k1[1], d + 0.5 * h * k1[2]))
        k3 = deriv((m + 0.5 * h * k2[0], p + 0.5 * h * k2[1], d + 0.5 * h * k2[2]))
        k4 = deriv((m + h * k3[0], p + h * k3[1], d + h * k3[2]))
        m += (h / 6.0) * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0])
        p += (h / 6.0) * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1])
        d += (h / 6.0) * (k1[2] + 2.0 * k2[2] + 2.0 * k3[2] + k4[2])
    return m, p, d


def _decimal_sequential_product(k_syn: float, k_hyd: float, dt: float) -> float:
    """100-digit Decimal reference for f_product(dt) = k_syn * (exp(-k_syn*dt) - exp(-k_hyd*dt)) / (k_hyd - k_syn)."""
    with decimal.localcontext() as ctx:
        ctx.prec = 100
        ks = decimal.Decimal(str(k_syn))
        kh = decimal.Decimal(str(k_hyd))
        t = decimal.Decimal(str(dt))
        if ks == kh:
            return float(ks * t * (-ks * t).exp())
        return float(ks * ((-ks * t).exp() - (-kh * t).exp()) / (kh - ks))


class TestInterfaceFalsificationContractV0(unittest.TestCase):
    def setUp(self) -> None:
        self.contract = json.loads(vfc.DEFAULT_CONTRACT_PATH.read_text(encoding="utf-8"))

    def test_canonical_contract_schema_passes_with_zero_errors(self) -> None:
        errors = vfc.validate_contract_dict(self.contract)
        self.assertEqual(errors, [])

    def test_corrupted_contract_mutations_fail_closed(self) -> None:
        c_agency = copy.deepcopy(self.contract)
        c_agency["epistemic_bridge_mapping"]["agency_or_perception_claimed_for_chemistry"] = True
        self.assertTrue(len(vfc.validate_contract_dict(c_agency)) > 0)

        # CAND-05 rejection: a gate with effect_threshold == None must never report PASS_FALSIFIER_NOT_TRIGGERED
        c_gate = copy.deepcopy(self.contract)
        c_gate["falsification_gates"][1]["effect_threshold"] = None
        c_gate["falsification_gates"][1]["current_evaluation_status"] = "PASS_FALSIFIER_NOT_TRIGGERED"
        self.assertTrue(len(vfc.validate_contract_dict(c_gate)) > 0)

        c_rad = copy.deepcopy(self.contract)
        c_rad["scientific_corrections"]["correction_3_radiolysis_dose_bases"]["allowed_dose_bases"][0][
            "allow_additional_water_partition_factor_f_w"
        ] = True
        self.assertTrue(len(vfc.validate_contract_dict(c_rad)) > 0)

    def test_f1_fiber_sufficiency_positive_negative_and_missing_threshold(self) -> None:
        fibers_ok = {
            "fiber_wet": [0.410, 0.422, 0.418],
            "fiber_dry": [0.105, 0.112],
        }
        res_ok = vfc.evaluate_f1_fiber_sufficiency(fibers_ok, uncertainty_u95=0.01, effect_threshold=0.05)
        self.assertEqual(res_ok["verdict"], "PASS_FALSIFIER_NOT_TRIGGERED")
        self.assertAlmostEqual(res_ok["max_intra_fiber_spread"], 0.012, places=12)

        fibers_coarse = {
            "fiber_wet": [0.15, 0.78],
            "fiber_dry": [0.10, 0.11],
        }
        res_fail = vfc.evaluate_f1_fiber_sufficiency(fibers_coarse, uncertainty_u95=0.01, effect_threshold=0.05)
        self.assertEqual(res_fail["verdict"], "FALSIFIED_PARTITION_TOO_COARSE")

        res_none = vfc.evaluate_f1_fiber_sufficiency(fibers_coarse, uncertainty_u95=0.01, effect_threshold=None)
        self.assertEqual(res_none["verdict"], "NOT_EVALUATED")

    def test_f2_pulse_timing_matched_budget_and_direction(self) -> None:
        res_pass = vfc.evaluate_f2_pulse_timing(
            pulsed_yield=0.62,
            continuous_matched_yield=0.30,
            uncertainty_q01_margin=0.08,
            pulsed_integrated_dose_gy=250.0,
            continuous_integrated_dose_gy=250.0,
            pulsed_thermal_exposure_k_s=1.2e6,
            continuous_thermal_exposure_k_s=1.2e6,
            effect_direction=1,
            effect_threshold=0.15,
        )
        self.assertEqual(res_pass["verdict"], "PASS_FALSIFIER_NOT_TRIGGERED")

        res_wrong_sign = vfc.evaluate_f2_pulse_timing(
            pulsed_yield=0.30,
            continuous_matched_yield=0.62,
            uncertainty_q01_margin=0.05,
            pulsed_integrated_dose_gy=250.0,
            continuous_integrated_dose_gy=250.0,
            pulsed_thermal_exposure_k_s=1.2e6,
            continuous_thermal_exposure_k_s=1.2e6,
            effect_direction=1,
            effect_threshold=0.10,
        )
        self.assertEqual(res_wrong_sign["verdict"], "FALSIFIED_TIMING_NO_EFFECT")

        res_unmatched = vfc.evaluate_f2_pulse_timing(
            pulsed_yield=0.62,
            continuous_matched_yield=0.30,
            uncertainty_q01_margin=0.05,
            pulsed_integrated_dose_gy=250.0,
            continuous_integrated_dose_gy=180.0,
            pulsed_thermal_exposure_k_s=1.2e6,
            continuous_thermal_exposure_k_s=1.2e6,
            effect_direction=1,
            effect_threshold=0.10,
        )
        self.assertEqual(res_unmatched["verdict"], "BLOCKED_ON_MISSING_CONTROL")

    def test_f3_scaffold_specificity_and_f4_selective_retention(self) -> None:
        # F3: Biotite vs inert quartz & alternative phyllosilicate under matched water chemistry
        res_f3_pass = vfc.evaluate_f3_scaffold_specificity(2.0e-6, 0.2e-6, 0.6e-6, True, 0.2e-6, 0.5e-6)
        self.assertEqual(res_f3_pass["verdict"], "PASS_FALSIFIER_NOT_TRIGGERED")
        res_f3_fail = vfc.evaluate_f3_scaffold_specificity(0.7e-6, 0.5e-6, 0.65e-6, True, 0.1e-6, 0.2e-6)
        self.assertEqual(res_f3_fail["verdict"], "FALSIFIED_NON_SPECIFIC_SURFACE")
        res_f3_block = vfc.evaluate_f3_scaffold_specificity(2.0e-6, 0.2e-6, 0.6e-6, False, 0.2e-6, 0.5e-6)
        self.assertEqual(res_f3_block["verdict"], "BLOCKED_ON_MISSING_CONTROL")
        res_f3_none = vfc.evaluate_f3_scaffold_specificity(2.0e-6, 0.2e-6, 0.6e-6, True, 0.2e-6, None)
        self.assertEqual(res_f3_none["verdict"], "NOT_EVALUATED")

        # F4: Selective retention vs conservative tracer
        res_f4_pass = vfc.evaluate_f4_selective_retention(25.0, 1.0, True, 0.5, 1.0)
        self.assertEqual(res_f4_pass["verdict"], "PASS_FALSIFIER_NOT_TRIGGERED")
        res_f4_fail = vfc.evaluate_f4_selective_retention(1.04, 1.0, True, 0.02, 0.10)
        self.assertEqual(res_f4_fail["verdict"], "FALSIFIED_PASSIVE_CONCENTRATION")
        res_f4_block = vfc.evaluate_f4_selective_retention(25.0, 1.0, False, 0.5, 1.0)
        self.assertEqual(res_f4_block["verdict"], "BLOCKED_ON_MISSING_CONTROL")
        res_f4_none = vfc.evaluate_f4_selective_retention(25.0, 1.0, True, 0.5, None)
        self.assertEqual(res_f4_none["verdict"], "NOT_EVALUATED")

    def test_f5_heritable_selection_vs_physical_sorting(self) -> None:
        res_sorting = vfc.evaluate_f5_heritable_selection(
            template_copying_rate_s_inv=0.0,
            transfer_cycles=10,
            replicating_freq_shift=0.40,
            sorting_control_freq_shift=0.05,
            standard_error=0.01,
            effect_threshold=0.05,
        )
        self.assertEqual(res_sorting["verdict"], "FALSIFIED_NON_HERITABLE_SORTING")

        res_heritable = vfc.evaluate_f5_heritable_selection(
            template_copying_rate_s_inv=2.5e-4,
            transfer_cycles=5,
            replicating_freq_shift=0.32,
            sorting_control_freq_shift=0.08,
            standard_error=0.03,
            effect_threshold=0.10,
        )
        self.assertEqual(res_heritable["verdict"], "PASS_FALSIFIER_NOT_TRIGGERED")

    def test_f6_independent_mass_balance_and_tautology_rejection(self) -> None:
        res_taut = vfc.evaluate_f6_independent_mass_balance(
            initial_inventory_mol=10.0,
            compartment_inventories_mol=[4.0, 6.0],
            closes_by_algebraic_definition=True,
            effect_threshold=1.0e-12,
        )
        self.assertEqual(res_taut["verdict"], "BLOCKED_ALGEBRAIC_TAUTOLOGY")

        res_ok = vfc.evaluate_f6_independent_mass_balance(
            initial_inventory_mol=1.0,
            compartment_inventories_mol=[0.0, 0.14652512, 0.85347488],
            closes_by_algebraic_definition=False,
            effect_threshold=1.0e-10,
        )
        self.assertEqual(res_ok["verdict"], "PASS_FALSIFIER_NOT_TRIGGERED")

    def test_correction_2_competing_and_sequential_kinetics_against_rk4_and_100digit_decimal(self) -> None:
        k_syn = 0.18
        k_deact = 0.42
        dt = 3.5

        closed_comp = vfc.evaluate_competing_precursor_synthesis(k_syn, k_deact, dt)
        m_rk4, p_syn_rk4, p_deact_rk4 = _rk4_competing_precursor(k_syn, k_deact, dt)
        self.assertAlmostEqual(closed_comp["p_precursor"], m_rk4, places=10)
        self.assertAlmostEqual(closed_comp["p_syn"], p_syn_rk4, places=10)
        self.assertAlmostEqual(closed_comp["p_deact"], p_deact_rk4, places=10)

        # CAND-06 rejection: neither unparenthesized formula nor clamped Boltzmann 1.0 is permitted
        dt_large = 50.0
        comp_large = vfc.evaluate_competing_precursor_synthesis(k_syn, k_deact, dt_large)
        expected_branch_fraction = k_syn / (k_syn + k_deact)
        unparenthesized_bug = 1.0 - math.exp(-(k_syn + k_deact) * dt_large) * expected_branch_fraction
        self.assertAlmostEqual(comp_large["p_syn"], expected_branch_fraction, places=12)
        self.assertGreater(abs(unparenthesized_bug - expected_branch_fraction), 0.65)
        self.assertLess(comp_large["p_syn"], 1.0)

        # Sequential product hydrolysis compared against RK4
        k_hyd = 0.31
        closed_seq = vfc.evaluate_sequential_product_hydrolysis(k_syn, k_hyd, dt)
        m_seq_rk4, p_seq_rk4, d_seq_rk4 = _rk4_sequential_hydrolysis(k_syn, k_hyd, dt)
        self.assertAlmostEqual(closed_seq["f_precursor"], m_seq_rk4, places=10)
        self.assertAlmostEqual(closed_seq["f_product"], p_seq_rk4, places=10)
        self.assertAlmostEqual(closed_seq["f_degraded"], d_seq_rk4, places=10)

        # CAND-01: 100-digit Decimal check across degenerate and near-degenerate |k_hyd - k_syn| * dt
        for delta_rate in (0.0, 1.0e-15, 1.0e-12, 1.0e-9, 1.0e-6, 1.0e-3, 0.25):
            kh_test = k_syn + delta_rate
            ref_dec = _decimal_sequential_product(k_syn, kh_test, dt)
            got_seq = vfc.evaluate_sequential_product_hydrolysis(k_syn, kh_test, dt)
            rel_err = abs(got_seq["f_product"] - ref_dec) / ref_dec
            self.assertLess(rel_err, 5.0e-16)
            self.assertLess(got_seq["closure_residual"], 1.0e-15)

    def test_correction_3_radiolysis_dose_basis_and_double_counting_rejection(self) -> None:
        res_pw = vfc.evaluate_radiolysis_energy_deposition(
            dose_basis="pore_water_absorbed_dose_rate",
            dose_rate_gy_s=0.25,
            porosity_eps=0.08,
            pore_water_density_kg_m3=1000.0,
            g_h2_molecules_per_100ev=0.45,
        )
        self.assertAlmostEqual(res_pw["q_abs_w_bulk_w_m3"], 20.0, places=12)

        with self.assertRaises(ValueError):
            vfc.evaluate_radiolysis_energy_deposition(
                dose_basis="pore_water_absorbed_dose_rate",
                dose_rate_gy_s=0.25,
                porosity_eps=0.08,
                water_energy_partition_f_w=0.03,
            )

        with self.assertRaises(ValueError):
            vfc.evaluate_radiolysis_energy_deposition(
                dose_basis="bulk_medium_absorbed_dose_rate",
                dose_rate_gy_s=0.25,
                porosity_eps=0.08,
                bulk_density_kg_m3=2886.0,
                water_energy_partition_f_w=None,
            )
        res_bulk = vfc.evaluate_radiolysis_energy_deposition(
            dose_basis="bulk_medium_absorbed_dose_rate",
            dose_rate_gy_s=0.25,
            porosity_eps=0.08,
            bulk_density_kg_m3=2886.0,
            water_energy_partition_f_w=0.031,
            f_w_provenance="Bragg-Gray mass-stopping-power ratio for 8% porosity water-saturated pelite",
        )
        self.assertAlmostEqual(res_bulk["q_abs_w_bulk_w_m3"], 2886.0 * 0.031 * 0.25, places=12)

    def test_psd_metric_requires_normalized_measure_space(self) -> None:
        with self.assertRaises(ValueError):
            vfc.evaluate_psd_metric([1.0, 0.0], [0.0, 0.0], [0.5, 0.2], event_threshold=0.5)

        res_psd = vfc.evaluate_psd_metric(
            values_a=[0.8, 0.2, 0.9, 0.1],
            values_b=[0.7, 0.6, 0.4, 0.1],
            measure_weights=[0.25, 0.25, 0.25, 0.25],
            event_threshold=0.5,
        )
        self.assertAlmostEqual(res_psd["d_psd"], 0.50, places=12)
        self.assertAlmostEqual(res_psd["hilbert_l2_distance"], math.sqrt(0.50), places=12)

    def test_cand04_julia_cross_repository_parity(self) -> None:
        parity_errors = vfc.validate_julia_model_parity(self.contract)
        self.assertEqual(parity_errors, [])

        with tempfile.NamedTemporaryFile("w", suffix=".jl", encoding="utf-8") as tmp:
            tmp.write('const INTERFACE_FALSIFICATION_CONTRACT_ID = "INTERFACE_FALSIFICATION_BRIDGE_v0"\n')
            tmp.write('const INTERFACE_FALSIFICATION_CONTRACT_VERSION = "v9.9.9"\n')
            tmp.flush()
            bad_errors = vfc.validate_julia_model_parity(self.contract, pathlib.Path(tmp.name))
            self.assertTrue(len(bad_errors) > 0)


if __name__ == "__main__":
    unittest.main()
