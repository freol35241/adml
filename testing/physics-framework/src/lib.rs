//! Physics validation testing framework
//!
//! Provides utilities for validating the physical correctness of dynamical models.

use approx::relative_eq;

/// Assertions for physics-based validation
pub mod assertions {
    use super::*;

    /// Assert that energy is conserved within a tolerance
    pub fn assert_energy_conserved(initial_energy: f64, final_energy: f64, tolerance: f64) {
        let diff = (final_energy - initial_energy).abs();
        assert!(
            diff < tolerance,
            "Energy not conserved: ΔE = {}, tolerance = {}",
            diff,
            tolerance
        );
    }

    /// Assert that a steady state has been reached
    ///
    /// Checks if values in the window have stabilized within tolerance
    pub fn assert_steady_state(values: &[f64], window: usize, tolerance: f64) {
        assert!(
            values.len() >= window,
            "Not enough values to check steady state"
        );

        let recent = &values[values.len() - window..];
        let mean = recent.iter().sum::<f64>() / window as f64;

        for &value in recent {
            let diff = (value - mean).abs();
            assert!(
                diff < tolerance,
                "Values not in steady state: diff = {}, tolerance = {}",
                diff,
                tolerance
            );
        }
    }

    /// Assert that a value matches an expected value within relative tolerance
    pub fn assert_relative_eq(actual: f64, expected: f64, epsilon: f64) {
        assert!(
            relative_eq!(actual, expected, epsilon = epsilon),
            "Values not relatively equal: actual = {}, expected = {}, epsilon = {}",
            actual,
            expected,
            epsilon
        );
    }
}

/// Numerical methods for validation
pub mod numerical {
    /// Compare simulation results against an analytical solution
    pub fn compare_with_analytical<F>(
        simulation_results: &[(f64, f64)],
        analytical: F,
        tolerance: f64,
    ) -> bool
    where
        F: Fn(f64) -> f64,
    {
        for &(time, sim_value) in simulation_results {
            let analytical_value = analytical(time);
            let error = (sim_value - analytical_value).abs();
            if error > tolerance {
                eprintln!(
                    "Mismatch at t={}: sim={}, analytical={}, error={}",
                    time, sim_value, analytical_value, error
                );
                return false;
            }
        }
        true
    }

    /// Find peaks in a time series (for oscillation analysis)
    pub fn find_peaks(values: &[f64]) -> Vec<(usize, f64)> {
        let mut peaks = Vec::new();

        for i in 1..values.len() - 1 {
            if values[i] > values[i - 1] && values[i] > values[i + 1] {
                peaks.push((i, values[i]));
            }
        }

        peaks
    }
}
