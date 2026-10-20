//! 20-case CF-2 conformance matrix definitions.
//!
//! Evaluates 5 phases x 2 scenarios x 2 pulse boundaries across all 6 fields.

use crate::reference::RefConfig;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cf2Case {
    pub case_index: usize, // 1..=20
    pub id: &'static str,
    pub phase: f64,
    pub null_scenario: bool,
    pub pulse_boundary: f64,
}

impl Cf2Case {
    pub fn config(&self) -> RefConfig {
        RefConfig {
            nx: 32,
            ny: 32,
            nz: 32,
            phase: self.phase,
            null_scenario: self.null_scenario,
            pulse_boundary: self.pulse_boundary,
        }
    }

    pub fn scenario_str(&self) -> &'static str {
        if self.null_scenario {
            "null"
        } else {
            "active"
        }
    }

    pub fn boundary_str(&self) -> &'static str {
        if (self.pulse_boundary - 0.1667).abs() < 1e-5 {
            "0.1667"
        } else {
            "0.1700"
        }
    }
}

pub const CF2_CASES: [Cf2Case; 20] = [
    Cf2Case { case_index: 1,  id: "CF2-01", phase: 0.05, null_scenario: false, pulse_boundary: 0.1667 },
    Cf2Case { case_index: 2,  id: "CF2-02", phase: 0.05, null_scenario: false, pulse_boundary: 0.1700 },
    Cf2Case { case_index: 3,  id: "CF2-03", phase: 0.05, null_scenario: true,  pulse_boundary: 0.1667 },
    Cf2Case { case_index: 4,  id: "CF2-04", phase: 0.05, null_scenario: true,  pulse_boundary: 0.1700 },
    Cf2Case { case_index: 5,  id: "CF2-05", phase: 0.15, null_scenario: false, pulse_boundary: 0.1667 },
    Cf2Case { case_index: 6,  id: "CF2-06", phase: 0.15, null_scenario: false, pulse_boundary: 0.1700 },
    Cf2Case { case_index: 7,  id: "CF2-07", phase: 0.15, null_scenario: true,  pulse_boundary: 0.1667 },
    Cf2Case { case_index: 8,  id: "CF2-08", phase: 0.15, null_scenario: true,  pulse_boundary: 0.1700 },
    Cf2Case { case_index: 9,  id: "CF2-09", phase: 0.20, null_scenario: false, pulse_boundary: 0.1667 },
    Cf2Case { case_index: 10, id: "CF2-10", phase: 0.20, null_scenario: false, pulse_boundary: 0.1700 },
    Cf2Case { case_index: 11, id: "CF2-11", phase: 0.20, null_scenario: true,  pulse_boundary: 0.1667 },
    Cf2Case { case_index: 12, id: "CF2-12", phase: 0.20, null_scenario: true,  pulse_boundary: 0.1700 },
    Cf2Case { case_index: 13, id: "CF2-13", phase: 0.50, null_scenario: false, pulse_boundary: 0.1667 },
    Cf2Case { case_index: 14, id: "CF2-14", phase: 0.50, null_scenario: false, pulse_boundary: 0.1700 },
    Cf2Case { case_index: 15, id: "CF2-15", phase: 0.50, null_scenario: true,  pulse_boundary: 0.1667 },
    Cf2Case { case_index: 16, id: "CF2-16", phase: 0.50, null_scenario: true,  pulse_boundary: 0.1700 },
    Cf2Case { case_index: 17, id: "CF2-17", phase: 0.90, null_scenario: false, pulse_boundary: 0.1667 },
    Cf2Case { case_index: 18, id: "CF2-18", phase: 0.90, null_scenario: false, pulse_boundary: 0.1700 },
    Cf2Case { case_index: 19, id: "CF2-19", phase: 0.90, null_scenario: true,  pulse_boundary: 0.1667 },
    Cf2Case { case_index: 20, id: "CF2-20", phase: 0.90, null_scenario: true,  pulse_boundary: 0.1700 },
];
